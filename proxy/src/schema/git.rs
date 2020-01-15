//! Abstractions and utilities for git interactions through the API.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use librad::meta::common::Url;
use librad::paths::Paths;
use radicle_surf as surf;
use radicle_surf::git::git2;

use crate::schema::error::Error;

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

/// Given a path to a repo returns the list of branches.
pub fn branches(repo_path: &str) -> Result<Vec<Branch>, Error> {
    let repo = surf::git::Repository::new(repo_path)?;
    let browser = surf::git::Browser::new(repo)?;
    let mut branches = browser
        .list_branches(None)
        .expect("Getting branches failed")
        .into_iter()
        .map(|b| Branch(b.name.name()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Initialize a [`librad::Project`] in the location of the given `path`.
pub fn init_project(
    librad_paths: &Paths,
    path: &str,
    name: &str,
    description: &str,
    default_branch: &str,
    img_url: &str,
) -> Result<(librad::git::ProjectId, librad::meta::Project), Error> {
    let key = librad::keys::device::Key::new();
    let peer_id = librad::peer::PeerId::from(key.public());
    let founder = librad::meta::contributor::Contributor::new();
    let sources = git2::Repository::open(std::path::Path::new(path))?;
    let img = Url::parse(img_url)?;
    let mut meta = librad::meta::Project::new(name, &peer_id);

    meta.description = Some(description.to_string());
    meta.default_branch = default_branch.to_string();
    meta.add_rel(librad::meta::Relation::Url("img_url".to_string(), img));

    let id = librad::git::GitProject::init(librad_paths, &key, &sources, meta.clone(), founder)?;

    Ok((id, meta))
}

/// Initialize a [`git2::GitRepository`] at the given path.
pub fn init_repo(path: String) -> Result<(), Error> {
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

/// Creates a small set of projects in [`librad::Paths`].
pub fn setup_fixtures(librad_paths: &Paths, root: &str) -> Result<(), Error> {
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
                "stable",
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
                "dev",
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
