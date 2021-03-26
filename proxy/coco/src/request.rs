//! An API for keeping track of requests and their state transitions.
//!
//! See [`Request`] and [`waiting_room::WaitingRoom`] for a high-level view of the API.

// We need to allow this because there's a bug and clippy doesn't realise that the type parameter
// is changing during state transitions.
// See https://github.com/rust-lang/rust-clippy/issues/4859 for more information.
#![allow(clippy::use_self)]

use std::{collections::HashMap, ops::Deref};

use either::Either;
use serde::{Deserialize, Serialize};

use librad::{git::Urn, net::protocol::gossip::Payload, peer::PeerId};

pub mod existential;
pub use existential::SomeRequest;
pub mod states;
pub use states::*;
pub mod waiting_room;

/// Private trait for sealing the traits we use here.
mod sealed;

/// A `Request` represents the lifetime of requesting an identity in the network via its
/// [`Urn`].
///
/// The `Request`'s state is represented by the `S` type parameter. This parameter makes sure that
/// a `Request` transitions through specific states in a type safe manner.
///
/// These transitions are pictured below:
///
/// ```text
///      +----------------------------------v
///      |                             +---------+
///      |                   +-------->+cancelled+<------+
///      |                   |         +----+----+       |
///      |                   |              ^            |
///      |                   |              |            |
/// +----+----+       +------+--+       +---+-+      +---+---+       +------+
/// | created +------>+requested+------>+found+----->+cloning+------>+cloned|
/// +---------+       +------+--+       +--+--+      +---+---+       +------+
///                          |  ^-------+  |  ^------+   |
///                          |    failed   |   failed    |
///                          |             v             |
///                          |          +--+------+      |
///                          +--------->+timed out+------+
///                                     +---------+
/// ```
///
/// The `T` type parameter represents some timestamp that is chosen by the user of the `Request`
/// API. Note that it makes it easy to test by just choosing `()` for the timestamp.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<S, T> {
    /// The identifier of the identity on the network.
    urn: Urn,
    /// The number of attempts this request has made to complete its job.
    attempts: Attempts,
    /// The timestamp of the latest action to be taken on this request.
    #[serde(with = "serde_millis", bound = "T: serde_millis::Milliseconds")]
    timestamp: T,
    /// The state of the request, as mentioned above.
    state: S,
}

impl<S, T> Deref for Request<S, T> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S, T> From<Request<S, T>> for Payload {
    fn from(request: Request<S, T>) -> Self {
        Self {
            urn: request.urn,
            rev: None,
            origin: None,
        }
    }
}

impl<S, T> Request<S, T> {
    /// Get the [`Urn`] that this `Request` is searching for.
    pub const fn urn(&self) -> &Urn {
        &self.urn
    }

    /// Get the the current timestamp of the `Request`.
    pub const fn timestamp(&self) -> &T {
        &self.timestamp
    }

    /// Transition this `Request` into an `Cancelled` state. We can only transition a particular
    /// subset of the states which are: `{Created, Requested, Found, Cloning, Cancelled}`.
    ///
    /// That is, attempting to cancel a `Cloned` `Request` is not permitted and will complain at
    /// compile time.
    pub fn cancel(self, timestamp: T) -> Request<Cancelled, T>
    where
        S: Cancel,
    {
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: self.state.cancel(),
        }
    }

    /// If a state keeps track of found peers then it can transition back to itself by adding a
    /// `PeerId` to the existing set of peers.
    pub fn found(mut self, peer: PeerId, timestamp: T) -> Request<S, T>
    where
        S: HasPeers,
    {
        self.state.peers().entry(peer).or_insert(Status::Available);
        self.timestamp = timestamp;
        self
    }

    /// A `Request` transitions into a timed out state if it exceeds the maximum number of queries
    /// or maximum number of clones. Otherwise, the `Request` proceeds as normal.
    ///
    /// The subset of states that can transition to the `TimedOut` state consist of
    /// `{Requested, Found, Cloning}`.
    pub fn timed_out(
        mut self,
        max_queries: Queries,
        max_clones: Clones,
        timestamp: T,
    ) -> Either<Self, Request<TimedOut, T>>
    where
        S: TimeOut,
    {
        if self.attempts.queries > max_queries {
            Either::Right(Request {
                urn: self.urn,
                attempts: self.attempts,
                timestamp,
                state: self.state.time_out(TimedOut::Query),
            })
        } else if self.attempts.clones > max_clones {
            Either::Right(Request {
                urn: self.urn,
                attempts: self.attempts,
                timestamp,
                state: self.state.time_out(TimedOut::Clone),
            })
        } else {
            self.timestamp = timestamp;
            Either::Left(self)
        }
    }

    /// When a `Request` is queried, we increment the `queries` count -- tracked via the
    /// `attempts` of the `Request`. If incrementing this count makes it exceed the maximum then
    /// the `Request` transitions into the `TimedOut` state.
    pub fn queried(
        mut self,
        max_queries: Queries,
        max_clones: Clones,
        timestamp: T,
    ) -> Either<Request<TimedOut, T>, Self>
    where
        S: TimeOut + QueryAttempt,
    {
        self.attempts.queries += 1;
        self.timed_out(max_queries, max_clones, timestamp).flip()
    }
}

impl<T> Request<Created, T> {
    /// Create a fresh `Request` for the given `urn`.
    ///
    /// Once this request has been made, we can transition this `Request` to the `Requested`
    /// state by calling [`Request::request`].
    #[allow(clippy::missing_const_for_fn, clippy::needless_pass_by_value)]
    pub fn new(urn: Urn, timestamp: T) -> Self {
        let urn = Urn { path: None, ..urn };
        Self {
            urn,
            attempts: Attempts::new(),
            timestamp,
            state: Created {},
        }
    }

    /// Transition the `Request` from the `Created` state to the `Requested` state.
    ///
    /// This signifies that the `Request` has been queried and will be looking for peers to fulfill
    /// the request.
    ///
    /// The number of queries is incremented by 1.
    pub fn request(self, timestamp: T) -> Request<Requested, T> {
        Request {
            urn: self.urn,
            attempts: Attempts {
                queries: self.attempts.queries + 1,
                ..self.attempts
            },
            timestamp,
            state: Requested {
                peers: HashMap::new(),
            },
        }
    }
}

impl<T> Request<Requested, T> {
    /// Transition the `Request` from the `Requested` state to the `Found` state.
    ///
    /// This signifies that the `Request` found its first peer and will be ready to attempt to
    /// clone from the peer.
    pub fn into_found(self, peer: PeerId, timestamp: T) -> Request<Found, T> {
        let mut peers = self.state.peers;
        peers.entry(peer).or_insert(Status::Available);
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Found { peers },
        }
    }
}

impl<T> Request<Found, T> {
    /// Transition the `Request` from the `Found` state to the `Cloning` state.
    ///
    /// This signifies that the `Request` is attempting to clone from the provided `peer`.
    pub fn cloning(
        self,
        max_queries: Queries,
        max_clones: Clones,
        peer: PeerId,
        timestamp: T,
    ) -> Either<Request<TimedOut, T>, Request<Cloning, T>>
    where
        T: Clone,
    {
        let mut peers = self.state.peers;
        peers
            .entry(peer)
            .and_modify(|status| *status = status.join(Status::InProgress))
            .or_insert(Status::InProgress);
        let this = Request {
            urn: self.urn,
            attempts: Attempts {
                queries: self.attempts.queries,
                clones: self.attempts.clones + 1,
            },
            timestamp: timestamp.clone(),
            state: Cloning { peers },
        };
        this.timed_out(max_queries, max_clones, timestamp).flip()
    }

    /// Transition the `Request` from the `Found` back to the `Requested` state.
    ///
    /// This signifies that the `Request` has exhausted its list of peers to attempt cloning from
    /// and needs to re-attempt the request for the search.
    ///
    /// Should be used in tandem with [`HasPeers::all_failed`] to ensure that we transition back
    /// when all peers have failed to clone.
    #[allow(clippy::missing_const_for_fn)]
    pub fn failed(self) -> Either<Request<Requested, T>, Request<Found, T>> {
        if self.state.all_failed() {
            Either::Left(Request {
                urn: self.urn,
                attempts: self.attempts,
                timestamp: self.timestamp,
                state: Requested {
                    peers: self.state.peers,
                },
            })
        } else {
            Either::Right(self)
        }
    }
}

impl<T> Request<Cloning, T> {
    /// Transition from the `Cloning` state back to the `Found` state.
    ///
    /// This signifies that the `peer` failed to clone the identity and we mark it as failed.
    pub fn failed(
        self,
        peer: PeerId,
        timestamp: T,
    ) -> Either<Request<Requested, T>, Request<Found, T>> {
        let mut peers = self.state.peers;
        peers
            .entry(peer)
            .and_modify(|status| *status = status.join(Status::Failed))
            .or_insert(Status::Failed);
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Found { peers },
        }
        .failed()
    }

    /// Transition from the `Cloning` to the `Cloned` state.
    ///
    /// This signifies that the clone was successful and that the whole request was successful,
    /// congratulations.
    #[allow(clippy::missing_const_for_fn)]
    pub fn cloned(self, remote_peer: PeerId, timestamp: T) -> Request<Cloned, T> {
        Request {
            urn: self.urn.clone(),
            attempts: self.attempts,
            timestamp,
            state: Cloned { remote_peer },
        }
    }
}
