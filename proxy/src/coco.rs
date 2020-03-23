//! Abstractions and utilities for git interactions through the API.

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use librad::git;
use librad::keys;
use librad::meta::{self, common::Url};
use librad::paths::Paths;
use librad::peer;
use librad::project;
use librad::surf;
use librad::surf::git::git2;

use crate::error;

/// Branch name representation.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Branch(String);

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tag name representation.
///
/// We still need full tag support.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag(String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of a person (e.g. committer, author, signer) from a repository. Usually
/// extracted from a signature.
pub struct Person {
    /// Name part of the commit signature commit.
    pub name: String,
    /// Email part of the commit signature commit.
    pub email: String,
    /// Reference (url/uri) to a persons avatar image.
    pub avatar: String,
}

/// Representation of a code commit.
pub struct Commit {
    /// Identifier of the commit in the form of a sha1 hash. Often referred to as oid or object
    /// id.
    pub sha1: git2::Oid,
    /// The author of the commit.
    pub author: Person,
    /// The summary of the commit message body.
    pub summary: String,
    /// The entire commit message body.
    pub message: String,
    /// The recorded time of the committer signature. This is a convenience alias until we
    /// expose the actual author and commiter signatures.
    pub committer_time: git2::Time,
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
            sha1: commit.id,
            author: Person {
                name: commit.author.name.clone(),
                email: commit.author.email.clone(),
                avatar,
            },
            summary: commit.summary.clone(),
            message: commit.message.clone(),
            committer_time: commit.author.time,
        }
    }
}

/// Git object types.
///
/// `shafiul.github.io/gitbook/1_the_git_object_model.html`
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum ObjectType {
    /// References a list of other trees and blobs.
    Tree,
    /// Used to store file data.
    Blob,
}

/// Set of extra information we carry for blob and tree objects returned from the API.
pub struct Info {
    /// Name part of an object.
    pub name: String,
    /// The type of the object.
    pub object_type: ObjectType,
    /// The last commmit that touched this object.
    pub last_commit: Option<Commit>,
}

/// File data abstraction.
pub struct Blob {
    /// Actual content of the file, if the content is ASCII.
    pub content: BlobContent,
    /// Extra info for the file.
    pub info: Info,
}

/// Variants of blob content.
pub enum BlobContent {
    /// Content is ASCII and can be passed as a string.
    Ascii(String),
    /// Content is binary and needs special treatment.
    Binary,
}

/// Result of a directory listing, carries other trees and blobs.
pub struct Tree {
    /// Absolute path to the tree object from the repo root.
    pub path: String,
    /// Entries listed in that tree result.
    pub entries: Vec<TreeEntry>,
    /// Extra info for the tree object.
    pub info: Info,
}

/// Entry in a Tree result.
pub struct TreeEntry {
    /// Extra info for the entry.
    pub info: Info,
    /// Absolute path to the object from the root of the repo.
    pub path: String,
}

/// Returns the [`Blob`] for a file at `revision` under `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interactions fail.
pub fn blob(paths: &Paths, id: &str, revision: &str, path: &str) -> Result<Blob, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let project = project::Project::open(paths, &project_id)?;

    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    // Best effort to guess the revision.
    browser.revspec(revision)?;

    let root = browser.get_directory()?;

    let mut p = surf::file_system::Path::from_str(path)?;

    let file = root.find_file(&p).ok_or_else(|| {
        surf::file_system::error::Error::Path(surf::file_system::error::Path::Empty)
    })?;

    let mut commit_path = surf::file_system::Path::root();
    commit_path.append(&mut p);

    let last_commit = browser.last_commit(&commit_path)?.map(|c| Commit::from(&c));
    let (_rest, last) = p.split_last();
    let content = match std::str::from_utf8(&file.contents) {
        Ok(content) => BlobContent::Ascii(content.to_string()),
        Err(_) => BlobContent::Binary,
    };

    Ok(Blob {
        content,
        info: Info {
            name: last.to_string(),
            object_type: ObjectType::Blob,
            last_commit,
        },
    })
}

/// Given a project id to a repo returns the list of branches.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
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
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Given a path to a repo returns the list of branches.
///
/// # Errors
///
/// Will return [`error::Error`] if the repository doesn't exist.
pub fn local_branches(repo_path: &str) -> Result<Vec<Branch>, error::Error> {
    let repo = surf::git::Repository::new(repo_path)?;
    let browser = surf::git::Browser::new(repo)?;
    let mut branches = browser
        .list_branches(None)?
        .into_iter()
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Retrieves the [`Commit`] for the given `sha1`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commit(paths: &Paths, id: &str, sha1: &str) -> Result<Commit, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let project = project::Project::open(paths, &project_id)?;
    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    browser.commit(surf::git::Oid::from_str(sha1)?)?;

    let history = browser.get();
    let commit = history.first();

    Ok(Commit::from(commit))
}

/// Retrieves the list of [`Tag`] for the given project `id`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn tags(paths: &Paths, id: &str) -> Result<Vec<Tag>, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let project = project::Project::open(paths, &project_id)?;
    let browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    let tag_names = browser.list_tags()?;

    let mut tags: Vec<Tag> = tag_names
        .into_iter()
        .map(|tag_name| Tag(tag_name.name().to_string()))
        .collect();

    tags.sort();

    Ok(tags)
}

/// Retrieve the [`Tree`] for the given `revision` and directory `prefix`.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the surf interactions fail.
pub fn tree(paths: &Paths, id: &str, revision: &str, prefix: &str) -> Result<Tree, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let project = project::Project::open(paths, &project_id)?;

    let mut browser = match project {
        project::Project::Git(git_project) => git_project.browser()?,
    };

    browser.revspec(revision)?;

    let mut path = if prefix == "/" || prefix == "" {
        surf::file_system::Path::root()
    } else {
        surf::file_system::Path::from_str(prefix)?
    };

    let root_dir = browser.get_directory()?;
    let prefix_dir = if path.is_root() {
        root_dir
    } else {
        root_dir.find_directory(&path).ok_or_else(|| {
            surf::file_system::error::Error::Path(surf::file_system::error::Path::Empty)
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
                        surf::file_system::error::Error::Label(
                            surf::file_system::error::Label::Empty,
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
        Some(Commit::from(browser.get().first()))
    } else {
        let mut commit_path = surf::file_system::Path::root();
        commit_path.append(&mut path);

        browser.last_commit(&commit_path)?.map(|c| Commit::from(&c))
    };
    let name = if path.is_root() {
        "".into()
    } else {
        let (_first, last) = path.split_last();
        last.to_string()
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

/// Retrieves project metadata.
///
/// # Errors
///
/// Will return [`error::Error`] if the project for the given `id` doesn't exist.
pub fn get_project_meta(paths: &Paths, id: &str) -> Result<meta::Project, error::Error> {
    let project_id = project::ProjectId::from_str(id)?;
    let meta = project::Project::show(paths, &project_id)?;

    Ok(meta)
}

/// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
#[must_use]
pub fn list_projects(paths: &Paths) -> Vec<(project::ProjectId, meta::Project)> {
    let mut projects = project::Project::list(paths)
        .map(|id| {
            (
                id.clone(),
                project::Project::show(paths, &id).expect("unable to get project meta"),
            )
        })
        .collect::<Vec<(project::ProjectId, meta::Project)>>();

    projects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    projects
}

/// Initialize a [`librad::project::Project`] in the location of the given `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the git2 repository is not present for the `path` or any of the
/// librad interactions fail.
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

/// Initialize a (FIXME: add type) at the given path.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the git interactions break.
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

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the git interaction fail, or the initialisation of the
/// coco project.
pub fn replicate_platinum(
    tmp_dir: &tempfile::TempDir,
    librad_paths: &Paths,
    name: &str,
    description: &str,
    default_branch: &str,
    img_url: &str,
) -> Result<(git::ProjectId, meta::Project), error::Error> {
    // Craft the absolute path to git-platinum fixtures.
    let mut platinum_path = env::current_dir()?;
    platinum_path.push("../fixtures/git-platinum");
    let mut platinum_from = String::from("file://");
    platinum_from.push_str(platinum_path.to_str().expect("unable get path"));

    // Construct path for fixtures to clone into.
    let platinum_into = tmp_dir.path().join("git-platinum");

    // Clone a copy into temp directory.
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(&platinum_from, platinum_into.as_path())
        .expect("unable to clone fixtures repo");

    let platinum_surf_repo = surf::git::Repository::new(
        platinum_into
            .to_str()
            .expect("unable to convert into string"),
    )?;
    let platinum_browser = surf::git::Browser::new(platinum_surf_repo)?;

    let tags = platinum_browser
        .list_tags()
        .expect("unable to get list of tags")
        .iter()
        .map(|t| format!("+refs/tags/{}", t.name()))
        .collect::<Vec<String>>();

    {
        let branches = platinum_repo.branches(Some(git2::BranchType::Remote))?;

        for branch in branches {
            let (branch, _branch_type) = branch?;
            let name = &branch
                .name()
                .expect("unable to get branch name")
                .expect("branch not present")
                .get(7..)
                .expect("unable to extract branch name");
            let oid = branch.get().target().expect("can't find OID");
            let commit = platinum_repo.find_commit(oid)?;

            if *name != "master" {
                platinum_repo.branch(name, &commit, false)?;
            }
        }
    }

    // Init as rad project.
    let (id, meta) = init_project(
        librad_paths,
        platinum_into.to_str().expect("unable to get path"),
        name,
        description,
        default_branch,
        img_url,
    )?;
    let mut rad_remote = platinum_repo.find_remote("rad")?;

    // Push all tags to rad remote.
    rad_remote.push(&tags.iter().map(String::as_str).collect::<Vec<_>>(), None)?;
    // Push dev branch.
    rad_remote.push(&["+refs/heads/dev"], None)?;

    Ok((id, meta))
}

/// Creates a small set of projects in [`Paths`].
///
/// # Errors
///
/// Will error if filesystem access is not granted or broken for the configured
/// [`librad::paths::Paths`].
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
