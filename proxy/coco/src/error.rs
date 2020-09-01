//! Collection of all crate errors.

use std::io;
use std::path;

use librad::git::{repo, storage};
use librad::meta::entity;
use librad::net;
use librad::uri;
use radicle_surf::file_system;
use radicle_surf::vcs::git;
use radicle_surf::vcs::git::git2;

/// Error emitted by one of the modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Peer accept error.
    #[error(transparent)]
    Accept(#[from] net::peer::AcceptError),

    /// Peer bootstrap error.
    #[error(transparent)]
    Bootstrap(#[from] net::peer::BootstrapError),

    /// Seed DNS failed to resolve to an address.
    #[error("the seed '{0}' failed to resolve to an address")]
    DnsLookupFailed(String),

    /// An existing project is being created, but we couldn't get the `name` of the project, i.e.
    /// the final suffix of the file path.
    #[error(
        "the existing path provided '{0}' was empty, and we could not get the project name from it"
    )]
    EmptyExistingPath(path::PathBuf),

    /// We expect at least one [`crate::source::Revisions`] when looking at a project, however the
    /// computation found none.
    #[error(
        "while trying to get user revisions we could not find any, there should be at least one"
    )]
    EmptyRevisions,

    /// Returned when an attempt to create an identity was made and there is one present.
    #[error("the identity '{0}' already exits")]
    EntityExists(uri::RadUrn),

    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// Seed input is invalid.
    #[error("the seed '{0}' is invalid: {:1}")]
    InvalidSeed(String, Option<librad::peer::conversion::Error>),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Kv error.
    #[error(transparent)]
    Kv(#[from] kv::Error),

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

    /// Configured default branch for the project is missing.
    #[error(
        "the default branch '{branch}' supplied was not found for the repository at '{repo_path}'"
    )]
    MissingDefaultBranch {
        /// The repository path we're setting up.
        repo_path: path::PathBuf,
        /// The default branch that was expected to be found.
        branch: String,
    },

    /// Trying to find a file path which could not be found.
    #[error("the path '{0}' was not found")]
    PathNotFound(file_system::Path),

    /// Repo error.
    #[error(transparent)]
    Repo(#[from] repo::Error),

    /// Storage error.
    #[error(transparent)]
    Storage(#[from] storage::Error),

    /// Originated from `radicle-surf`.
    #[error(transparent)]
    SurfFFS(#[from] file_system::Error),

    /// Originated from `radicle_surf`.
    #[error(transparent)]
    SurfGit(#[from] git::error::Error),

    /// Emitted when the parsing of a [`librad::uri::Path`] failed.
    #[error(transparent)]
    UriParse(#[from] uri::path::ParseError),

    /// Verifcation error.
    #[error(transparent)]
    Verification(#[from] entity::HistoryVerificationError),
}
