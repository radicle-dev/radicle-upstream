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
pub enum Peer<S> {
    /// Represents the local peer.
    #[serde(rename_all = "camelCase")]
    Local {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// Encoded state of replication.
        status: S,
    },
    /// Represents a remote peer.
    #[serde(rename_all = "camelCase")]
    Remote {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// Encoded state of replication.
        status: S,
    },
}

impl<U> Peer<Status<U>> {
    pub fn replicated(self) -> Option<Peer<Replicated<U>>>
    where
        U: Clone,
    {
        match self {
            Peer::Local {
                peer_id,
                status: Status::Replicated(replicated),
            } => Some(Peer::Local {
                peer_id,
                status: replicated,
            }),
            Peer::Remote {
                peer_id,
                status: Status::Replicated(replicated),
            } => Some(Peer::Remote {
                peer_id,
                status: replicated,
            }),
            _ => None,
        }
    }

    pub fn replicated_remote(self) -> Option<(PeerId, U)> {
        match self {
            Self::Remote {
                peer_id,
                status: Status::Replicated(Replicated { user, .. }),
            } => Some((peer_id, user)),
            _ => None,
        }
    }
}

impl<S> Peer<S> {
    pub fn peer_id(&self) -> PeerId {
        match self {
            Peer::Local { peer_id, .. } => *peer_id,
            Peer::Remote { peer_id, .. } => *peer_id,
        }
    }

    pub fn status(&self) -> &S {
        match self {
            Peer::Local { status, .. } => &status,
            Peer::Remote { status, .. } => &status,
        }
    }

    pub fn map<T, F>(self, f: F) -> Peer<T>
    where
        F: FnOnce(S) -> T,
    {
        match self {
            Self::Local { peer_id, status } => Peer::Local {
                peer_id,
                status: f(status),
            },
            Self::Remote { peer_id, status } => Peer::Remote {
                peer_id,
                status: f(status),
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Replicated<U> {
    pub role: Role,
    pub user: U,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Status<U> {
    NotReplicated,
    Replicated(Replicated<U>),
}

impl<U> Status<U> {
    pub fn replicated(role: Role, user: U) -> Self {
        Status::Replicated(Replicated { role, user })
    }
}

impl<U> Status<U> {
    pub fn map<V, F>(self, f: F) -> Status<V>
    where
        F: FnOnce(U) -> V,
    {
        match self {
            Self::NotReplicated => Status::NotReplicated,
            Self::Replicated(Replicated { role, user }) => Status::replicated(role, f(user)),
        }
    }
}
