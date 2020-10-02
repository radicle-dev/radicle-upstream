//! Collection of all crate errors.

use std::io;

use librad::{git::repo, meta::entity, net, uri};
use radicle_surf::vcs::git::git2;

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

    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An error occured building include files.
    #[error(transparent)]
    Include(#[from] librad::git::include::Error),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

    /// Peer API error
    #[error(transparent)]
    PeerApi(#[from] net::peer::ApiError),

    /// Repo error.
    #[error(transparent)]
    Repo(#[from] repo::Error),

    /// An error occurred when interacting with the source code of a project.
    #[error(transparent)]
    Source(#[from] crate::source::Error),

    /// Storage error.
    #[error(transparent)]
    Storage(#[from] storage::Error),

    /// Emitted when the parsing of a [`librad::uri::Path`] failed.
    #[error(transparent)]
    UriParse(#[from] uri::path::ParseError),

    /// Verifcation error.
    #[error(transparent)]
    Verification(#[from] entity::HistoryVerificationError),
}
