use std::{collections::HashSet, convert::TryFrom, marker::PhantomData, time::Duration};

use librad::{net::peer::types::Gossip, peer::PeerId, uri::RadUrn};

pub mod waiting_room;

const MAX_QUERIES: usize = 1;
const MAX_CLONES: usize = 1;
const PERIOD: Duration = Duration::from_secs(1); // Not for the whole request but for re-request

pub fn exponential_backoff(attempts: Attempts, interval: Duration) -> Duration {
    let exp = u32::try_from(attempts.queries + attempts.clones).unwrap_or(u32::MAX);
    Duration::from_millis(u64::pow(2, exp)) + interval
}

#[derive(Clone, Debug, PartialEq)]
pub struct Found {
    peer_id: PeerId,
    found_peers: HashSet<PeerId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cloning {
    peer_id: PeerId,
    found_peers: HashSet<PeerId>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Cloned {
    repo: RadUrn,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Created;
pub type IsCreated = PhantomData<Created>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Canceled;
pub type IsCanceled = PhantomData<Canceled>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TimedOut;
pub type IsTimedOut = PhantomData<TimedOut>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Requested;
pub type IsRequested = PhantomData<Requested>;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Attempts {
    queries: usize, // how often we gossip
    clones: usize,  // how often we try to clone
}

impl Attempts {
    pub fn new() -> Self {
        Attempts {
            queries: 0,
            clones: 0,
        }
    }
}

pub enum Attempt {
    Query,
    CloneRepo,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("the URN found '{actual}' was not the expected URN '{expected}'")]
    UrnMismatch { expected: RadUrn, actual: RadUrn },
}

#[derive(Clone, Debug, PartialEq)]
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

impl<S, T> Request<PhantomData<S>, T> {
    fn coerce<R>(self, timestamp: T) -> Request<PhantomData<R>, T> {
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: PhantomData,
        }
    }
}

impl<S, T> Request<S, T> {
    pub fn urn(&self) -> &RadUrn {
        &self.urn
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
        self.coerce(timestamp)
    }
}

impl<T> Request<IsRequested, T> {
    pub fn found_peer(self, peer_id: PeerId, timestamp: T) -> Request<Found, T> {
        let found_peers = vec![peer_id.clone()].into_iter().collect();
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Found {
                peer_id,
                found_peers,
            },
        }
    }
}

impl<T> Request<Found, T> {
    pub fn cloning(self, peer_id: PeerId, timestamp: T) -> Request<Cloning, T> {
        Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Cloning {
                peer_id,
                found_peers: self.state.found_peers,
            },
        }
    }

    pub fn clone_attempt(mut self) -> Self {
        self.attempts.clones += 1;
        self
    }
}

impl<T> Request<IsRequested, T> {
    pub fn cancel(self, timestamp: T) -> Request<IsCanceled, T> {
        self.coerce(timestamp)
    }

    pub fn timed_out(self, timestamp: T) -> Request<IsTimedOut, T> {
        self.coerce(timestamp)
    }

    pub fn query_attempt(mut self) -> Self {
        self.attempts.queries += 1;
        self
    }
}

impl<T> Request<Cloning, T> {
    pub fn cloned(self, repo: RadUrn, timestamp: T) -> Result<Request<Cloned, T>, Error> {
        if repo != self.urn {
            return Err(Error::UrnMismatch {
                expected: self.urn,
                actual: repo,
            });
        }

        Ok(Request {
            urn: self.urn,
            attempts: self.attempts,
            timestamp,
            state: Cloned { repo },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SomeRequest<T> {
    Created(Request<IsCreated, T>),
    Requested(Request<IsRequested, T>),
    Found(Request<Found, T>),
    Cloning(Request<Cloning, T>),
    Cloned(Request<Cloned, T>),
    Canceled(Request<IsCanceled, T>),
    TimedOut(Request<IsTimedOut, T>),
}

impl<T> From<Request<IsCreated, T>> for SomeRequest<T> {
    fn from(request: Request<IsCreated, T>) -> Self {
        Self::Created(request)
    }
}

impl<T> From<Request<IsRequested, T>> for SomeRequest<T> {
    fn from(request: Request<IsRequested, T>) -> Self {
        Self::Requested(request)
    }
}

impl<T> From<Request<Found, T>> for SomeRequest<T> {
    fn from(request: Request<Found, T>) -> Self {
        Self::Found(request)
    }
}

impl<T> From<Request<Cloning, T>> for SomeRequest<T> {
    fn from(request: Request<Cloning, T>) -> Self {
        Self::Cloning(request)
    }
}

impl<T> From<Request<Cloned, T>> for SomeRequest<T> {
    fn from(request: Request<Cloned, T>) -> Self {
        Self::Cloned(request)
    }
}

impl<T> From<Request<IsCanceled, T>> for SomeRequest<T> {
    fn from(request: Request<IsCanceled, T>) -> Self {
        Self::Canceled(request)
    }
}

impl<T> From<Request<IsTimedOut, T>> for SomeRequest<T> {
    fn from(request: Request<IsTimedOut, T>) -> Self {
        Self::TimedOut(request)
    }
}

impl<T> SomeRequest<T> {
    pub fn urn(&self) -> &RadUrn {
        match self {
            SomeRequest::Created(request) => request.urn(),
            SomeRequest::Requested(request) => request.urn(),
            SomeRequest::Found(request) => request.urn(),
            SomeRequest::Cloning(request) => request.urn(),
            SomeRequest::Cloned(request) => request.urn(),
            SomeRequest::Canceled(request) => request.urn(),
            SomeRequest::TimedOut(request) => request.urn(),
        }
    }
}
