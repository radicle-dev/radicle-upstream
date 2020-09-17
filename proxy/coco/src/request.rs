use std::{collections::HashMap, convert::TryFrom, marker::PhantomData, time::Duration};

use librad::{net::peer::types::Gossip, peer::PeerId, uri::RadUrn};

pub mod waiting_room;

const MAX_QUERIES: usize = 1;
const MAX_CLONES: usize = 1;
const PERIOD: Duration = Duration::from_secs(1); // Not for the whole request but for re-request

pub fn exponential_backoff(attempt: usize, interval: Duration) -> Duration {
    let exp = u32::try_from(attempt).unwrap_or(u32::MAX);
    Duration::from_millis(u64::pow(2, exp)) + interval
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Available,
    InProgress,
    Failed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Found {
    peers: HashMap<PeerId, Status>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cloning {
    peers: HashMap<PeerId, Status>,
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

// TODO(finto): Better naming to please the people who will inevitably give out about it.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Kind {
    Query,
    Clone,
}

// TODO(finto): Time outs for multiple operations
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TimedOut {
    kind: Kind,
}

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
        let mut this = self.coerce(timestamp);
        this.attempts.queries + 1;
        this
    }
}

impl<T> Request<IsRequested, T> {
    pub fn found_peer(self, peer_id: PeerId, timestamp: T) -> Request<Found, T> {
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
        self.attempts.queries + 1;
        self
    }
}

impl<T> Request<Found, T> {
    pub fn cloning(self, peer_id: PeerId, timestamp: T) -> Request<Cloning, T> {
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

    pub fn found_peer(mut self, peer_id: PeerId, timestamp: T) -> Self {
        self.state.peers.entry(peer_id).or_insert(Status::Available);
        self
    }
}

/* TODO(finto): Try this out TYPE FAMILIES
 * trait Cancelable
 *
 * impl<S: Cancelable>
 *
 * trait CanFindPeer
 */
impl<T> Request<IsRequested, T> {
    // TODO(finto): Everything except cloned
    pub fn cancel(self, timestamp: T) -> Request<IsCanceled, T> {
        self.coerce(timestamp)
    }

    pub fn timed_out(self, timestamp: T) -> Request<TimedOut, T> {
        todo!()
    }

    pub fn query_attempt(mut self) -> Self {
        self.attempts.queries += 1;
        self
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

    pub fn cloned(self, repo: RadUrn, timestamp: T) -> Result<Request<Cloned, T>, Error> {
        // TODO(finto): Consider removing this and assume that it's for the correct RadUrn
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

    pub fn found_peer(mut self, peer_id: PeerId, timestamp: T) -> Self {
        self.state.peers.entry(peer_id).or_insert(Status::Available);
        self
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
    TimedOut(Request<TimedOut, T>),
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

impl<T> From<Request<TimedOut, T>> for SomeRequest<T> {
    fn from(request: Request<TimedOut, T>) -> Self {
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
