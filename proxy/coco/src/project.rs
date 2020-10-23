//! Project creation data and functions.

use serde::Serialize;

use librad::peer::PeerId;
use radicle_surf::vcs::git::git2;

use crate::config;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &str) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch, git2::BranchType::Local)?;
    branch.set_upstream(Some(&format!("{}/{}", config::RAD_REMOTE, default_branch)))
}

/// Relation of the peer to the project.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    /// Replicating, but not participating.
    Tracker,
    /// Participated with unique changes.
    Contributor,
    /// Part of the set of maintainers.
    Maintainer,
}

/// Distinct views on a prpject.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Peer<U> {
    /// Represents the local peer.
    #[serde(rename_all = "camelCase")]
    Local {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// Encoded state of replication.
        status: ReplicationStatus<U>,
    },
    /// Represents a remote peer.
    #[serde(rename_all = "camelCase")]
    Remote {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// Encoded state of replication.
        status: ReplicationStatus<U>,
    },
}

impl<U> Peer<U> {
    pub fn map<V, F>(self, f: F) -> Peer<V>
    where
        F: FnOnce(U) -> V,
    {
        match self {
            Self::Local { peer_id, status } => Peer::Local {
                peer_id,
                status: status.map(f),
            },
            Self::Remote { peer_id, status } => Peer::Remote {
                peer_id,
                status: status.map(f),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ReplicationStatus<U> {
    NotReplicated,
    #[serde(rename_all = "camelCase")]
    Replicated {
        role: Role,
        user: U,
    },
}

impl<U> ReplicationStatus<U> {
    pub fn map<V, F>(self, f: F) -> ReplicationStatus<V>
    where
        F: FnOnce(U) -> V,
    {
        match self {
            Self::NotReplicated => ReplicationStatus::NotReplicated,
            Self::Replicated { role, user } => ReplicationStatus::Replicated {
                role,
                user: f(user),
            },
        }
    }
}
