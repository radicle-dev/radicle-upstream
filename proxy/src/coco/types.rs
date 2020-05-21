use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

use librad::surf;
use librad::surf::git::git2;

/// Branch name representation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Branch(pub(super) String);

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tag name representation.
///
/// We still need full tag support.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag(pub(super) String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of a person (e.g. committer, author, signer) from a repository. Usually
/// extracted from a signature.
pub struct Person {
    /// Name part of the commit signature.
    pub name: String,
    /// Email part of the commit signature.
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
    /// The committer of the commit.
    pub committer: Person,
    /// The recorded time of the committer signature. This is a convenience alias until we
    /// expose the actual author and commiter signatures.
    pub committer_time: git2::Time,
}

impl Commit {
    /// Returns the commit description text. This is the text after the one-line summary.
    #[must_use]
    pub fn description(&self) -> &str {
        self.message
            .strip_prefix(&self.summary)
            .unwrap_or(&self.message)
            .trim()
    }
}

impl From<&surf::git::Commit> for Commit {
    fn from(commit: &surf::git::Commit) -> Self {
        let avatar = |input: &String| {
            let mut s = DefaultHasher::new();
            input.hash(&mut s);

            format!(
                "https://avatars.dicebear.com/v2/jdenticon/{}.svg",
                s.finish().to_string()
            )
        };

        Self {
            sha1: commit.id,
            author: Person {
                name: commit.author.name.clone(),
                email: commit.author.email.clone(),
                avatar: avatar(&commit.author.email),
            },
            summary: commit.summary.clone(),
            message: commit.message.clone(),
            committer: Person {
                name: commit.committer.name.clone(),
                email: commit.committer.email.clone(),
                avatar: avatar(&commit.committer.email),
            },
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
    /// Absolute path to the object from the root of the repo.
    pub path: String,
}

impl Blob {
    /// Indicates if the content of the [`Blob`] is binary.
    #[must_use]
    pub fn is_binary(&self) -> bool {
        self.content == BlobContent::Binary
    }
}

/// Variants of blob content.
#[derive(PartialEq)]
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
