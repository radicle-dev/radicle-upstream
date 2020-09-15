use std::collections::HashSet;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::time::Duration;

use librad::net::peer::types::Gossip;
use librad::uri::RadUrn;

pub mod waiting_room;

type PeerId = String;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Repo {
    urn: RadUrn,
    name: String,
}

const MAX_QUERIES: usize = 1;
const MAX_CLONES: usize = 1;
const PERIOD: Duration = Duration::from_secs(1); // Not for the whole request but for re-request

pub fn exponential_backoff(attempts: Attempts, interval: Duration) -> Duration {
    let exp = u32::try_from(attempts.queries + attempts.clones).unwrap_or(u32::MAX);
    Duration::from_millis(u64::pow(2, exp)) + interval
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum State {
    Found(PeerId),
    Cloning(PeerId),
    Cloned(Repo),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Created;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Canceled;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TimedOut;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Requested;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Fulfilled;

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

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("the URN found '{actual}' was not the expected URN '{expected}'")]
    UrnMismatch { expected: RadUrn, actual: RadUrn },
    #[error("the request was not found to be in the cloning state and so it cannot be fulfilled")]
    ExpectedCloningState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Request<S, T> {
    urn: RadUrn,
    state: PhantomData<S>,
    found_peers: HashSet<PeerId>,
    attempts: Attempts,
    timestamp: T,
    internal: Option<State>,
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
    fn coerce<R>(self, timestamp: T) -> Request<R, T> {
        Request {
            urn: self.urn,
            state: PhantomData,
            found_peers: self.found_peers,
            attempts: self.attempts,
            timestamp,
            internal: None,
        }
    }
}

impl<S, T> Request<S, T> {
    pub fn urn(&self) -> &RadUrn {
        &self.urn
    }
}

impl<T> Request<Created, T> {
    pub fn new(urn: RadUrn, timestamp: T) -> Self {
        Self {
            urn,
            state: PhantomData,
            found_peers: HashSet::new(),
            attempts: Attempts::new(),
            timestamp,
            internal: None,
        }
    }

    pub fn request(self, timestamp: T) -> Request<Requested, T> {
        self.coerce(timestamp)
    }
}

impl<T> Request<Requested, T> {
    pub fn attempt(mut self, attempt: Attempt) -> Self {
        match attempt {
            Attempt::Query => self.attempts.queries += 1,
            Attempt::CloneRepo => self.attempts.clones += 1,
        }
        self
    }

    pub fn found_peer(mut self, peer: PeerId, timestamp: T) -> Self {
        self.found_peers.insert(peer.clone());
        self.internal = Some(State::Found(peer));
        self.timestamp = timestamp;
        self
    }

    pub fn cloning(mut self, peer: PeerId, timestamp: T) -> Self {
        self.internal = Some(State::Cloning(peer));
        self.timestamp = timestamp;
        self
    }

    pub fn cancel(self, timestamp: T) -> Request<Canceled, T> {
        self.coerce(timestamp)
    }

    pub fn timed_out(self, timestamp: T) -> Request<TimedOut, T> {
        self.coerce(timestamp)
    }

    pub fn fulfilled(self, repo: Repo, timestamp: T) -> Result<Request<Fulfilled, T>, Error>
    where
        T: Clone,
    {
        if repo.urn != self.urn {
            return Err(Error::UrnMismatch {
                expected: self.urn,
                actual: repo.urn,
            });
        }

        match self.internal {
            Some(State::Cloning(_)) => { /* all good */ }
            _ => return Err(Error::ExpectedCloningState),
        }

        Ok(Request {
            urn: self.urn,
            state: PhantomData,
            found_peers: self.found_peers,
            attempts: self.attempts,
            timestamp,
            internal: Some(State::Cloned(repo)),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SomeRequest<T> {
    Created(Request<Created, T>),
    Requested(Request<Requested, T>),
    Fulfilled(Request<Fulfilled, T>),
    Canceled(Request<Canceled, T>),
    TimedOut(Request<TimedOut, T>),
}

impl<T> From<Request<Created, T>> for SomeRequest<T> {
    fn from(request: Request<Created, T>) -> Self {
        Self::Created(request)
    }
}

impl<T> From<Request<Requested, T>> for SomeRequest<T> {
    fn from(request: Request<Requested, T>) -> Self {
        Self::Requested(request)
    }
}

impl<T> From<Request<Fulfilled, T>> for SomeRequest<T> {
    fn from(request: Request<Fulfilled, T>) -> Self {
        Self::Fulfilled(request)
    }
}

impl<T> From<Request<Canceled, T>> for SomeRequest<T> {
    fn from(request: Request<Canceled, T>) -> Self {
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
            SomeRequest::Fulfilled(request) => request.urn(),
            SomeRequest::Canceled(request) => request.urn(),
            SomeRequest::TimedOut(request) => request.urn(),
        }
    }
}
