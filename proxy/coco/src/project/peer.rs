//! The `peer` module models the data representing a peer's relation to a project.
//!
//! A [`Peer`] can be `Local` or `Remote`, it can be `NotReplicated` or `Replicated`, and it
//! can be a `Tracker`, `Contributor`, or `Maintainer` of the project.
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

/// A peer is split between a `Local` peer and a `Remote` peer. The `Local` variant corresponds to
/// the user browsing from their own machine. The `Remote` variant corresponds to a peer that they
/// have connected with by exchanging data over the newtork.
///
/// Both variants are keyed by their `PeerId` which identifies them in the network.
///
/// The `status` field is left generic so that we can use a combination of [`Status`] and
/// [`Replicated`]. When `S` is `Status` it means that the `Peer` could be in one of two states:
/// `NotReplicated` or `Replicated`. When `S` is `Replicated` it means the peer is definitely
/// `Replicated` on the local peer's machine.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Peer<S> {
    /// Represents the local peer.
    #[serde(rename_all = "camelCase")]
    Local {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// State of replication.
        status: S,
    },
    /// Represents a remote peer.
    #[serde(rename_all = "camelCase")]
    Remote {
        /// [`PeerId`] of the peer.
        peer_id: PeerId,
        /// State of replication.
        status: S,
    },
}

impl<U> Peer<Status<U>> {
    #![allow(clippy::use_self)]
    /// Refine a `Peer` into a `Replicated` peer.
    ///
    /// This will return `Some` if the `Status` was `Replicated`, and `None` otherwise.
    ///
    /// This can be used in tandem with `filter_map` if we want to turn an iterator of
    /// `Peer<Status<U>>` into an iterator of `Peer<Replicated<U>>`.
    pub fn replicated(self) -> Option<Peer<Replicated<U>>>
    where
        U: Clone,
    {
        match self {
            Self::Local {
                peer_id,
                status: Status::Replicated(replicated),
            } => Some(Peer::Local {
                peer_id,
                status: replicated,
            }),
            Self::Remote {
                peer_id,
                status: Status::Replicated(replicated),
            } => Some(Peer::Remote {
                peer_id,
                status: replicated,
            }),
            Self::Local { .. } | Self::Remote { .. } => None,
        }
    }

    /// Get the user details for a `Remote` peer.
    ///
    /// This will return `Some` if the `Peer` is `Replicated`, and `None` otherwise.
    #[allow(clippy::missing_const_for_fn)]
    pub fn replicated_remote(self) -> Option<(PeerId, U)> {
        match self {
            Self::Remote {
                peer_id,
                status: Status::Replicated(Replicated { user, .. }),
            } => Some((peer_id, user)),
            Self::Local { .. } | Self::Remote { .. } => None,
        }
    }
}

impl<S> Peer<S> {
    /// Get the [`PeerId`] of the `Peer`, regardless whether they are `Local` or `Remote`.
    pub const fn peer_id(&self) -> PeerId {
        match self {
            Self::Local { peer_id, .. } | Self::Remote { peer_id, .. } => *peer_id,
        }
    }

    /// Get the `status` of the `Peer`, regardless whether they are `Local` or `Remote`.
    pub const fn status(&self) -> &S {
        match self {
            Self::Local { status, .. } | Self::Remote { status, .. } => status,
        }
    }
}

#[allow(clippy::use_self)]
impl<S> Peer<S> {
    /// Apply the function `f` to the `status` of the `Peer`. This allows us to easily change the
    /// underlying type of the `Peer` without changing the `peer_id` field.
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

/// If data has been replicated locally we should be able to determine the [`Role`] the peer had
/// with this project as well as their user metadata.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Replicated<U> {
    /// The role this peer has with the project.
    pub role: Role,
    /// The user metadata the peer used with the project.
    pub user: U,
}

#[allow(clippy::use_self)]
impl<U> Replicated<U> {
    /// Apply the supplied function `f` to the `user` field of `Replicated`. This leaves the `role`
    /// field untouched.
    ///
    /// This is useful for changing the `user` type by augmenting the data inside with the supplied
    /// function.
    pub fn map<V, F>(self, f: F) -> Replicated<V>
    where
        F: FnOnce(U) -> V,
    {
        Replicated {
            role: self.role,
            user: f(self.user),
        }
    }
}

/// `Status` represents the state of data that relates a peer and some project.
///
/// If it is `NotReplicated`, it means the peer is being tracked but we have not received
/// any data relating to them yet.
///
/// If it is `Replicated`, it means the data has been replicated and we have the associated role
/// and user metadata for this peer.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Status<U> {
    /// No data has been replicated locally (yet).
    NotReplicated,
    /// The data has been replicated locally, and so we can determine the `Role` and `User`.
    Replicated(Replicated<U>),
}

impl<U> Status<U> {
    /// Helper for constructing the `Status::Replicated` variant.
    pub const fn replicated(role: Role, user: U) -> Self {
        Self::Replicated(Replicated { role, user })
    }
}

#[allow(clippy::use_self)]
impl<U> Status<U> {
    /// Apply the supplied function `f` to [`Replicated`], otherwise it leaves the `NotReplicated`
    /// variant untouched.
    pub fn map<V, F>(self, f: F) -> Status<V>
    where
        F: FnOnce(U) -> V,
    {
        match self {
            Self::NotReplicated => Status::NotReplicated,
            Self::Replicated(replicated) => Status::Replicated(replicated.map(f)),
        }
    }
}
