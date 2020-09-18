use std::{collections::HashMap, convert::TryFrom, marker::PhantomData, time::Duration};

use either::Either;
use serde::{Deserialize, Serialize};

use librad::{net::peer::types::Gossip, peer::PeerId, uri::RadUrn};

pub mod states;
pub use states::*;
pub mod existential;
pub mod waiting_room;
pub use existential::SomeRequest;

mod sealed;

const MAX_QUERIES: Queries = Queries::new(1);
const MAX_CLONES: Clones = Clones::new(1);
const PERIOD: Duration = Duration::from_secs(1); // Not for the whole request but for re-request

pub fn exponential_backoff(attempt: usize, interval: Duration) -> Duration {
    let exp = u32::try_from(attempt).unwrap_or(u32::MAX);
    Duration::from_millis(u64::pow(2, exp)) + interval
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<S, T> {
    urn: RadUrn,
    attempts: Attempts,
    timestamp: T,
    state: S,
}

impl<S, T> From<Request<S, T>> for Gossip {
    fn from(request: Request<S, T>) -> Self {
        Self {
            urn: request.urn,
            rev: None,
            origin: None,
        }
    }
}

impl<S, T> Request<S, T> {
    pub fn urn(&self) -> &RadUrn {
        &self.urn
    }

    pub fn cancel(self, timestamp: T) -> Request<IsCanceled, T>
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

    pub fn found_peer(mut self, peer_id: PeerId, timestamp: T) -> Request<S, T>
    where
        S: HasPeers,
    {
        self.state
            .peers()
            .entry(peer_id)
            .or_insert(Status::Available);
        self.timestamp = timestamp;
        self
    }

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
}

impl<T> Request<IsCreated, T> {
    pub fn new(urn: RadUrn, timestamp: T) -> Self {
        Self {
            urn,
            attempts: Attempts::new(),
            timestamp,
            state: PhantomData,
        }
    }

    pub fn request(self, timestamp: T) -> Request<IsRequested, T> {
        Request {
            urn: self.urn,
            attempts: Attempts {
                queries: self.attempts.queries + 1,
                ..self.attempts
            },
            timestamp,
            state: PhantomData,
        }
    }
}

impl<T> Request<IsRequested, T> {
    pub fn first_peer(self, peer_id: PeerId, timestamp: T) -> Request<Found, T> {
        let mut peers = HashMap::new();
        peers.insert(peer_id, Status::Available);
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Found { peers },
        }
    }

    pub fn queried(mut self, timestamp: T) -> Self {
        self.attempts.queries += 1;
        self.timestamp = timestamp;
        self
    }
}

impl<T> Request<Found, T> {
    pub fn cloning(self, timestamp: T) -> Request<Cloning, T> {
        Request {
            urn: self.urn,
            attempts: Attempts {
                queries: self.attempts.queries,
                clones: self.attempts.clones + 1,
            },
            timestamp,
            state: Cloning {
                peers: self.state.peers,
            },
        }
    }
}

impl<T> Request<Cloning, T> {
    pub fn failed(self, peer_id: PeerId, timestamp: T) -> Request<Found, T> {
        let mut peers = self.state.peers;
        // TODO(finto): It's weird if it didn't exist but buh
        peers
            .entry(peer_id)
            .and_modify(|status| *status = Status::Failed)
            .or_insert(Status::Failed);
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Found { peers },
        }
    }

    pub fn cloned(self, repo: RadUrn, timestamp: T) -> Request<Cloned, T> {
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Cloned { repo },
        }
    }
}
