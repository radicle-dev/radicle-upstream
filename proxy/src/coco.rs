//! Abstractions and utilities for git interactions through the API.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use librad::git;
use librad::keys;
use librad::meta::{self, common::Url};
use librad::paths::Paths;
use librad::peer;
use librad::project;
use radicle_surf as surf;
use radicle_surf::git::git2;

use crate::error;

/// Branch name representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, GraphQLScalarValue)]
pub struct Branch(pub String);

/// Tag name representation.
///
/// We still need full tag support.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, GraphQLScalarValue)]
pub struct Tag(pub String);

/// Representation of a person (e.g. committer, author, signer) from a repository. Usually
/// extracted from a signature.
#[derive(GraphQLObject)]
pub struct Person {
    /// Name part of the commit signature commit.
    name: String,
    /// Email part of the commit signature commit.
    email: String,
    /// Reference (url/uri) to a persons avatar image.
    avatar: String,
}

/// Representation of a code commit.
#[derive(GraphQLObject)]
pub struct Commit {
    /// Identifier of the commit in the form of a sha1 hash. Often referred to as oid or object
    /// id.
    sha1: String,
    /// The author of the commit.
    author: Person,
    /// The summary of the commit message body.
    summary: String,
    /// The entire commit message body.
    message: String,
    /// The recorded time of the committer signature. This is a convenience alias until we
    /// expose the actual author and commiter signatures.
    committer_time: String,
}

impl From<&surf::git::Commit> for Commit {
    fn from(commit: &surf::git::Commit) -> Self {
        let mut s = DefaultHasher::new();
        commit.author.email.hash(&mut s);

        let avatar = format!(
            "https://avatars.dicebear.com/v2/jdenticon/{}.svg",
            s.finish().to_string()
        );

        Self {
            sha1: commit.id.to_string(),
            author: Person {
                name: commit.author.name.clone(),
                email: commit.author.email.clone(),
                avatar,
            },
            summary: commit.summary.clone(),
            message: commit.message.clone(),
            committer_time: commit.author.time.seconds().to_string(),
        }
    }
}

/// Git object types.
///
/// `shafiul.github.io/gitbook/1_the_git_object_model.html`
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, GraphQLEnum)]
pub enum ObjectType {
    /// References a list of other trees and blobs.
    Tree,
    /// Used to store file data.
    Blob,
}

/// Set of extra information we carry for blob and tree objects returned from the API.
#[derive(GraphQLObject)]
pub struct Info {
    /// Name part of an object.
    pub name: String,
    /// The type of the object.
    pub object_type: ObjectType,
    /// The last commmit that touched this object.
    pub last_commit: Option<Commit>,
}

/// File data abstraction.
#[derive(GraphQLObject)]
pub struct Blob {
    /// Best-effort guess if the content is binary.
    pub binary: bool,
    /// Actual content of the file, if the content is ASCII.
    pub content: Option<String>,
    /// Extra info for the file.
    pub info: Info,
}

/// Result of a directory listing, carries other trees and blobs.
#[derive(GraphQLObject)]
pub struct Tree {
    /// Absolute path to the tree object from the repo root.
    pub path: String,
    /// Entries listed in that tree result.
    pub entries: Vec<TreeEntry>,
    /// Extra info for the tree object.
    pub info: Info,
}

/// Entry in a Tree result.
#[derive(GraphQLObject)]
pub struct TreeEntry {
    /// Extra info for the entry.
    pub info: Info,
    /// Absolute path to the object from the root of the repo.
    pub path: String,
}

pub fn blob(paths: &Paths, id: &str, revision: &str, path: &str) -> Result<Blob, error::Error> {
    let project_id = project::ProjectId::from_str(&id)?;
    let project = project::Project::open(&paths, &project_id)?;

    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    // Best effort to guess the revision.
    if let Err(_err) = browser
        .branch(surf::git::BranchName::new(&revision))
        .or(browser.commit(surf::git::Sha1::new(&revision)))
        .or(browser.tag(surf::git::TagName::new(&revision)))
    {
        return Err(error::Error::Git(surf::git::error::Error::NotBranch));
    };

    let root = browser.get_directory()?;

    let mut p = surf::file_system::Path::from_str(&path)?;

    let file = root.find_file(&p).ok_or_else(|| {
        radicle_surf::file_system::error::Error::Path(radicle_surf::file_system::error::Path::Empty)
    })?;

    let mut commit_path = surf::file_system::Path::root();
    commit_path.append(&mut p);

    let last_commit = browser.last_commit(&commit_path)?.map(|c| Commit::from(&c));
    let (_rest, last) = p.split_last();
    let (binary, content) = {
        let res = std::str::from_utf8(&file.contents);

        match res {
            Ok(content) => (false, Some(content.to_string())),
            Err(_) => (true, None),
        }
    };

    Ok(Blob {
        binary,
        content,
        info: Info {
            name: last.label,
            object_type: ObjectType::Blob,
            last_commit,
        },
    })
}

/// Given a project id to a repo returns the list of branches.
pub fn branches(paths: &Paths, id: &str) -> Result<Vec<Branch>, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let project = project::Project::open(paths, &project_id)?;
    let browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    let mut branches = browser
        .list_branches(None)
        .expect("Getting branches failed")
        .into_iter()
        .map(|b| Branch(b.name.name()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Given a path to a repo returns the list of branches.
pub fn local_branches(repo_path: &str) -> Result<Vec<Branch>, error::Error> {
    let repo = surf::git::Repository::new(repo_path)?;
    let browser = surf::git::Browser::new(repo)?;
    let mut branches = browser
        .list_branches(None)?
        .into_iter()
        .map(|b| Branch(b.name.name()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

pub fn commit(paths: &Paths, id: &str, sha1: &str) -> Result<Commit, error::Error> {
    let project_id = project::ProjectId::from_str(&id)?;
    let project = project::Project::open(paths, &project_id)?;
    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    browser.commit(radicle_surf::git::Sha1::new(sha1))?;

    let history = browser.get_history();
    let commit = history.0.first();

    Ok(Commit::from(commit))
}

pub fn tags(paths: &Paths, id: &str) -> Result<Vec<Tag>, error::Error> {
    let project_id = project::ProjectId::from_str(&id)?;
    let project = project::Project::open(&paths, &project_id)?;
    let browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    let mut tag_names = browser.list_tags()?;
    tag_names.sort();

    let mut tags: Vec<Tag> = tag_names
        .into_iter()
        .map(|tag_name| Tag(tag_name.name()))
        .collect();

    tags.sort();

    Ok(tags)
}

pub fn tree(paths: &Paths, id: &str, revision: &str, prefix: &str) -> Result<Tree, error::Error> {
    let project_id = project::ProjectId::from_str(&id)?;
    let project = project::Project::open(&paths, &project_id)?;

    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    if let Err(_err) = browser
        .branch(surf::git::BranchName::new(&revision))
        .or(browser.commit(surf::git::Sha1::new(&revision)))
        .or(browser.tag(surf::git::TagName::new(&revision)))
    {
        return Err(error::Error::Git(surf::git::error::Error::NotBranch));
    };

    let mut path = if prefix == "/" || prefix == "" {
        surf::file_system::Path::root()
    } else {
        surf::file_system::Path::from_str(&prefix)?
    };

    let root_dir = browser.get_directory()?;
    let prefix_dir = if path.is_root() {
        root_dir
    } else {
        root_dir.find_directory(&path).ok_or_else(|| {
            radicle_surf::file_system::error::Error::Path(
                radicle_surf::file_system::error::Path::Empty,
            )
        })?
    };
    let mut prefix_contents = prefix_dir.list_directory();
    prefix_contents.sort();

    let entries_results: Result<Vec<TreeEntry>, error::Error> = prefix_contents
        .iter()
        .map(|(label, system_type)| {
            let mut entry_path = if path.is_root() {
                let label_path =
                    nonempty::NonEmpty::from_slice(&[label.clone()]).ok_or_else(|| {
                        radicle_surf::file_system::error::Error::Label(
                            radicle_surf::file_system::error::Label::Empty,
                        )
                    })?;
                surf::file_system::Path(label_path)
            } else {
                let mut p = path.clone();
                p.push(label.clone());
                p
            };
            let mut commit_path = surf::file_system::Path::root();
            commit_path.append(&mut entry_path);

            let last_commit = browser.last_commit(&commit_path)?.map(|c| Commit::from(&c));
            let info = Info {
                name: label.to_string(),
                object_type: match system_type {
                    surf::file_system::SystemType::Directory => ObjectType::Tree,
                    surf::file_system::SystemType::File => ObjectType::Blob,
                },
                last_commit,
            };

            Ok(TreeEntry {
                info,
                path: entry_path.to_string(),
            })
        })
        .collect();

    let mut entries = entries_results?;

    // We want to ensure that in the response Tree entries come first. `Ord` being derived on
    // the enum ensures Variant declaration order.
    //
    // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
    entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

    let last_commit = if path.is_root() {
        Some(Commit::from(browser.get_history().0.first()))
    } else {
        let mut commit_path = surf::file_system::Path::root();
        commit_path.append(&mut path);

        browser.last_commit(&commit_path)?.map(|c| Commit::from(&c))
    };
    let name = if path.is_root() {
        "".into()
    } else {
        let (_first, last) = path.split_last();
        last.label
    };
    let info = Info {
        name,
        object_type: ObjectType::Tree,
        last_commit,
    };

    Ok(Tree {
        path: prefix.to_string(),
        entries,
        info,
    })
}

pub fn get_project_meta(paths: &Paths, id: &str) -> Result<meta::Project, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let meta = project::Project::show(&paths, &project_id)?;

    Ok(meta)
}

pub fn list_projects(paths: &Paths) -> Vec<(project::ProjectId, meta::Project)> {
    let mut projects = project::Project::list(&paths)
        .map(|id| {
            (
                id.clone(),
                project::Project::show(&paths, &id).expect("unable to get project meta"),
            )
        })
        .collect::<Vec<(project::ProjectId, meta::Project)>>();

    projects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    projects
}

/// Initialize a [`Project`] in the location of the given `path`.
pub fn init_project(
    librad_paths: &Paths,
    path: &str,
    name: &str,
    description: &str,
    default_branch: &str,
    img_url: &str,
) -> Result<(git::ProjectId, meta::Project), error::Error> {
    let key = keys::device::Key::new();
    let peer_id = peer::PeerId::from(key.public());
    let founder = meta::contributor::Contributor::new();
    let sources = git2::Repository::open(std::path::Path::new(path))?;
    let img = Url::parse(img_url)?;
    let mut meta = meta::Project::new(name, &peer_id);

    meta.description = Some(description.to_string());
    meta.default_branch = default_branch.to_string();
    meta.add_rel(meta::Relation::Url("img_url".to_string(), img));

    let id = git::GitProject::init(librad_paths, &key, &sources, meta.clone(), founder)?;

    Ok((id, meta))
}

/// Initialize a [`git2::GitRepository`] at the given path.
pub fn init_repo(path: String) -> Result<(), error::Error> {
    let repo = git2::Repository::init(path)?;

    // First use the config to initialize a commit signature for the user.
    let sig = repo.signature()?;
    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // For our purposes, we'll leave the index empty for now.
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

/// Creates a small set of projects in [`Paths`].
pub fn setup_fixtures(librad_paths: &Paths, root: &str) -> Result<(), error::Error> {
    let infos = vec![
            (
                "monokel",
                "A looking glass into the future",
                "master",
                "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
            ),
            (
                "Monadic",
                "Open source organization of amazing things.",
                "master",
                "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
            ),
            (
                "open source coin",
                "Research for the sustainability of the open source community.",
                "master",
                "https://avatars0.githubusercontent.com/u/31632242",
            ),
            (
                "radicle",
                "Decentralized open source collaboration",
                "master",
                "https://avatars0.githubusercontent.com/u/48290027",
            ),
        ];

    for info in infos {
        let path = format!("{}/{}/{}", root, "repos", info.0);
        std::fs::create_dir_all(path.clone())?;

        init_repo(path.clone())?;
        init_project(librad_paths, &path, info.0, info.1, info.2, info.3)?;
    }

    Ok(())
}
