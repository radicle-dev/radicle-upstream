// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Capture `State` related error variants.

use librad::{
    git::{
        tracking,
        types::{One, Reference},
        Urn,
    },
    net,
};
use std::{convert::Infallible, panic};

/// Errors that may occur when interacting with [`librad::net::peer::Peer`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while trying to create a working copy of a project.
    #[error(transparent)]
    Create(#[from] crate::daemon::project::create::Error),

    /// An error occurred while performing the checkout of a project.
    #[error(transparent)]
    Checkout(#[from] crate::daemon::project::checkout::Error),

    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An interaction involving an identity failed.
    #[error(transparent)]
    Identities(#[from] Box<librad::git::identities::Error>),

    /// An interaction involving
    /// [`librad::git::identities::local::LocalIdentity`] failed.
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

    /// The [`librad::git::identities::Person`] was not found for the provided
    /// [`Urn`].
    #[error("person not found for '{0}'")]
    PersonNotFound(Urn),

    /// The [`librad::git::identities::Project`] was not found for the provided
    /// [`Urn`].
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
    Replication(#[from] librad::net::peer::error::Replicate),

    /// Peer storage error.
    #[error(transparent)]
    PeerStorage(#[from] net::peer::error::Storage),

    /// Peer storage error.
    #[error(transparent)]
    Storage(#[from] storage::Error),

    /// An interaction with the config file for the storage failed.
    #[error(transparent)]
    StorageConfig(#[from] librad::git::storage::config::Error),

    /// Error while performing tracking operation
    #[error(transparent)]
    Tracking(#[from] Tracking),

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

    /// A spawned task was cancelled
    #[error("spawned task cancelled")]
    TaskCancelled,
}

/// Error while performing tracking operation
#[derive(Debug, thiserror::Error)]
pub enum Tracking {
    /// An error occurred when attempting to track a peer.
    #[error(transparent)]
    Track(#[from] tracking::error::Track),

    /// An error occurred when attempting to get tracked peers.
    #[error(transparent)]
    Tracked(#[from] tracking::error::TrackedPeers),

    /// An error occurred when attempting to untrack a peer.
    #[error(transparent)]
    Untrack(#[from] tracking::error::Untrack),
}

impl From<tracking::error::Track> for Error {
    fn from(err: tracking::error::Track) -> Self {
        Self::Tracking(err.into())
    }
}

impl From<tracking::error::Untrack> for Error {
    fn from(err: tracking::error::Untrack) -> Self {
        Self::Tracking(err.into())
    }
}

impl From<tracking::error::TrackedPeers> for Error {
    fn from(err: tracking::error::TrackedPeers) -> Self {
        Self::Tracking(err.into())
    }
}

impl From<Infallible> for Error {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}

impl From<librad::git::identities::Error> for Error {
    fn from(err: librad::git::identities::Error) -> Self {
        Self::Identities(Box::new(err))
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        if err.is_cancelled() {
            Self::TaskCancelled
        } else if err.is_panic() {
            panic::resume_unwind(err.into_panic())
        } else {
            unreachable!("unexpected join error: {:?}", err)
        }
    }
}

/// Re-export the underlying [`storage::Error`] so that consumers don't need to
/// add `librad` as a dependency to match on the variant. Instead, they can
/// import `coco::state::error::storage`.
pub mod storage {
    pub use librad::git::storage::Error;
}

/// Re-export the underlying [`blob::Error`] so that consumers don't need to add
/// `librad` as a dependency to match on the variant. Instead, they can import
/// `coco::state::error::blob`.
pub mod blob {
    pub use librad::git_ext::blob::Error;
}
