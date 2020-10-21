//! Capture `State` related error variants.

use librad::{
    git::{
        repo,
        types::{namespace, NamespacedRef, Single},
    },
    meta::entity,
    net,
    uri::{self, RadUrn},
};
use radicle_surf::vcs::git::git2;

use crate::source;

/// Errors that may occur when interacting with [`super::State`].
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

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

    /// Peer API error
    #[error(transparent)]
    PeerApi(#[from] net::peer::ApiError),

    #[error(transparent)]
    ReferenceName(#[from] librad::git::ext::reference::name::Error),

    /// Repo error.
    #[error(transparent)]
    Repo(#[from] repo::Error),

    /// An error occurred when interacting with the source code of a project.
    #[error(transparent)]
    Source(#[from] source::Error),

    /// Storage error.
    #[error(transparent)]
    Storage(#[from] storage::Error),

    #[error(transparent)]
    Transport(#[from] librad::git::local::transport::Error),

    #[error("uhoh")]
    TransportTimeout,

    /// Emitted when the parsing of a [`librad::uri::Path`] failed.
    #[error(transparent)]
    UriParse(#[from] uri::path::ParseError),

    /// Verifcation error.
    #[error(transparent)]
    Verification(#[from] entity::HistoryVerificationError),

    /// There were no references for a Browser to be initialised.
    #[error("we could not find a default branch for '{name}@{urn}'")]
    NoDefaultBranch {
        /// Name of the project.
        name: String,
        /// RadUrn of the project.
        urn: RadUrn,
    },

    /// Could not find a `NamespacedRef` when searching for it in the `Storage`.
    #[error("we could not find the '{reference}'")]
    MissingRef {
        /// The reference that we looked for in the `Storage`.
        reference: NamespacedRef<namespace::Legacy, Single>,
    },
}

impl Error {
    /// Easily create an [`storage::Error::AlreadyExists`] exists error.
    #[must_use = "you made it, you use it"]
    pub const fn already_exists(urn: RadUrn) -> Self {
        Self::Storage(storage::Error::AlreadyExists(urn))
    }
}

/// Re-export the underlying [`storage::Error`] so that consumers don't need to add `librad` as a
/// dependency to match on the variant. Instead, they can import `coco::state::error::storage`.
pub mod storage {
    pub use librad::git::storage::Error;
}

/// Re-export the underlying [`blob::Error`] so that consumers don't need to add `librad` as a
/// dependency to match on the variant. Instead, they can import `coco::state::error::blob`.
pub mod blob {
    pub use librad::git::ext::blob::Error;
}
