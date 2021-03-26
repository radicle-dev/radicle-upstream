//! Capture `State` related error variants.

use librad::{
    git::{
        types::{One, Reference},
        Urn,
    },
    net,
};
use radicle_surf::vcs::git::git2;
use std::convert::Infallible;

use crate::source;

/// Errors that may occur when interacting with [`librad::net::peer::Peer`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while trying to create a working copy of a project.
    #[error(transparent)]
    Create(#[from] crate::project::create::Error),

    /// An error occurred while performing the checkout of a project.
    #[error(transparent)]
    Checkout(#[from] crate::project::checkout::Error),

    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An attempt to create an identity failed.
    #[error("failed to create identity")]
    IdentityCreationFailed,

    /// An interaction involving an identity failed.
    #[error(transparent)]
    Identities(#[from] librad::git::identities::Error),

    /// An interaction involving [`librad::git::identities::local::LocalIdentity`] failed.
    #[error(transparent)]
    IdentitiesLocal(#[from] librad::git::identities::local::Error),

    /// An error occurred building include files.
    #[error(transparent)]
    Include(#[from] librad::git::include::Error),

    /// The namespace was expected in a reference but was not found.
    #[error("missing namespace in reference")]
    MissingNamespace,

    /// An operation relied on a default owner being set, but it was not.
    #[error("this operation depends on the present of a default owner")]
    MissingOwner,

    /// The [`librad::git::identities::Person`] was not found for the provided [`Urn`].
    #[error("person not found for '{0}'")]
    PersonNotFound(Urn),

    /// The [`librad::git::identities::Project`] was not found for the provided [`Urn`].
    #[error("project not found for '{0}'")]
    ProjectNotFound(Urn),

    /// Failed to parse a reference.
    #[error(transparent)]
    ReferenceName(#[from] librad::git_ext::reference::name::Error),

    /// An action involving `rad/signed_refs` resulted in an error.
    #[error(transparent)]
    Refs(#[from] librad::git::refs::stored::Error),

    /// An error occurred when attempting to replicate data from another peer.
    #[error(transparent)]
    Replication(#[from] librad::git::replication::Error),

    /// An error occurred when interacting with the source code of a project.
    #[error(transparent)]
    Source(#[from] source::Error),

    /// Peer storage error.
    #[error(transparent)]
    PeerStorage(#[from] net::peer::StorageError),

    /// Peer storage error.
    #[error(transparent)]
    Storage(#[from] storage::Error),

    /// An interaction with the config file for the storage failed.
    #[error(transparent)]
    StorageConfig(#[from] librad::git::storage::config::Error),

    /// An error occurred interacting with the `radicle_surf` package.
    #[error(transparent)]
    Surf(#[from] radicle_surf::git::error::Error),

    /// An error occurred when attempting to track or untrack a peer.
    #[error(transparent)]
    Tracking(#[from] librad::git::tracking::Error),

    /// Attempted to create an identity that already exists.
    #[error("the URN `{0}` already exists")]
    IdentityExists(Urn),

    /// There were no references for a Browser to be initialised.
    #[error("we could not find a default branch for '{name}@{urn}'")]
    NoDefaultBranch {
        /// Name of the project.
        name: String,
        /// RadUrn of the project.
        urn: Urn,
    },

    /// Could not find a `NamespacedRef` when searching for it in the `Storage`.
    #[error("we could not find the '{reference}'")]
    MissingRef {
        /// The reference that we looked for in the `Storage`.
        reference: Reference<One>,
    },

    /// A document payload extension was malformed
    #[error(transparent)]
    MalformedPayloadExt(#[from] librad::identities::payload::ExtError),
}

impl From<Infallible> for Error {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
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
    pub use librad::git_ext::blob::Error;
}
