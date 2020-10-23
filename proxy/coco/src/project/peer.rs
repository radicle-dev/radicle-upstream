use serde::Serialize;

use librad::peer::PeerId;

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
    pub fn replicated(&self) -> Option<Self>
    where
        U: Clone,
    {
        match self {
            Peer::Local {
                status: ReplicationStatus::Replicated { .. },
                ..
            } => Some(self.clone()),
            Peer::Remote {
                status: ReplicationStatus::Replicated { .. },
                ..
            } => Some(self.clone()),
            _ => None,
        }
    }

    pub fn replicated_remote(&self) -> Option<(PeerId, U)>
    where
        U: Clone,
    {
        match self {
            Self::Remote {
                peer_id,
                status: ReplicationStatus::Replicated { user, .. },
            } => Some((*peer_id, user.clone())),
            _ => None,
        }
    }
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
