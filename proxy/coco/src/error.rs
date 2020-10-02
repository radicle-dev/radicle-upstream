//! Collection of all crate errors.

use std::io;

use librad::{git::repo, meta::entity, net, uri};
use radicle_surf::{
    file_system,
    vcs::{git, git::git2},
};

/// Re-export [`librad::git::storage::Error`] under the `coco::error` namespace.
pub mod storage {
    pub use librad::git::storage::Error;
    use librad::uri::RadUrn;

    /// Easily create an [`storage::Error::AlreadyExists`] exists error.
    #[must_use = "you made it, you use it"]
    pub const fn already_exists(urn: RadUrn) -> super::Error {
        super::Error::Storage(Error::AlreadyExists(urn))
    }
}

/// Error emitted by one of the modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Peer accept error.
    #[error(transparent)]
    Accept(#[from] net::peer::AcceptError),

    /// Peer bootstrap error.
    #[error(transparent)]
    Bootstrap(#[from] net::peer::BootstrapError),

    /// An error occurred while trying to create a working copy of a project.
    #[error(transparent)]
    Create(#[from] crate::project::create::Error),

    /// An error occurred while performing the checkout of a project.
    #[error(transparent)]
    Checkout(#[from] crate::project::checkout::Error),

    /// Seed DNS failed to resolve to an address.
    #[error("the seed '{0}' failed to resolve to an address")]
    DnsLookupFailed(String),

    /// We expect at least one [`crate::source::Revisions`] when looking at a project, however the
    /// computation found none.
    #[error(
        "while trying to get user revisions we could not find any, there should be at least one"
    )]
    EmptyRevisions,

    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An error occured building include files.
    #[error(transparent)]
    Include(#[from] librad::git::include::Error),

    /// Seed input is invalid.
    #[error("the seed '{0}' is invalid: {:1}")]
    InvalidSeed(String, Option<librad::peer::conversion::Error>),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

    /// Peer API error
    #[error(transparent)]
    PeerApi(#[from] net::peer::ApiError),

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

    /// When trying to query a repositories branches, but there are none.
    #[error("The repository has no branches")]
    NoBranches,
}
