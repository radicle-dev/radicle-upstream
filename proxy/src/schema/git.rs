use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use librad::paths::Paths;
use radicle_surf::git::git2;

use crate::schema::error::Error;

pub fn init_project(
    librad_paths: &Paths,
    path: String,
    name: String,
    description: String,
    default_branch: String,
    img_url: String,
) -> Result<(librad::git::ProjectId, librad::meta::Project), Error> {
    let key = librad::keys::device::Key::new();
    let peer_id = librad::peer::PeerId::from(key.public());
    let founder = librad::meta::contributor::Contributor::new();
    let sources = git2::Repository::open(std::path::Path::new(&path))?;
    let img = url::Url::parse(&img_url)?;
    let mut meta = librad::meta::Project::new(&name, &peer_id);

    meta.description = Some(description.to_string());
    meta.default_branch = default_branch.to_string();
    meta.add_rel(librad::meta::Relation::Url("img_url".to_string(), img));

    let id = librad::git::GitProject::init(&librad_paths, &key, &sources, meta.clone(), founder)?;

    Ok((id, meta))
}

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

// FIXME(xla): This should be a `std::convert::TryFrom` and needs to be addressed together with
//             consistent error handling.
impl From<&git2::Commit<'_>> for Commit {
    fn from(commit: &git2::Commit) -> Self {
        let signature = commit.author();
        let email = signature.email().unwrap_or("invalid email");

        let mut s = DefaultHasher::new();
        email.hash(&mut s);

        let avatar = format!(
            "https://avatars.dicebear.com/v2/jdenticon/{}.svg",
            s.finish().to_string()
        );

        Self {
            sha1: commit.id().to_string(),
            author: Person {
                name: signature.name().unwrap_or("invalid name").into(),
                email: email.into(),
                avatar: avatar.into(),
            },
            summary: commit.summary().unwrap_or("invalid subject").into(),
            message: commit.message().unwrap_or("invalid message").into(),
            committer_time: commit.time().seconds().to_string(),
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
    pub last_commit: Commit,
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
