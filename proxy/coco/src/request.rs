use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::time::Duration;

use librad::uri::RadUrn;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Request<S, T> {
    urn: RadUrn,
    state: PhantomData<S>,
    found_peers: HashSet<PeerId>,
    attempts: Attempts,
    timestamp: T,
    internal: Option<State>,
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

    // TODO(finto): Error state of urn not matching
    pub fn fulfilled(self, repo: Repo, timestamp: T) -> Result<Request<Fulfilled, T>, ()>
    where
        T: Clone,
    {
        if repo.urn != self.urn {
            return Err(());
        }

        match self.internal {
            Some(State::Cloning(_)) => { /* all good */ }
            _ => return Err(()),
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

pub struct WaitingRoom<T> {
    requests: HashMap<RadUrn, SomeRequest<T>>,
}

// TODO(finto): Map out the transitions
// TODO(finto): Safe state transitions
// TODO(finto): Gossip impl for Request
// TODO(finto): Exponential backoff calculation
// TODO(finto): Consider fairness of selection, e.g. rando sample, fifo, lifo, instant
// comparison, etc.
// TODO(finto): Test scenario of "running" a request and updating the waiting room. Testing state
// transitions.
// TODO(finto): De/Serialize for waiting room.
impl<T> WaitingRoom<T> {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }

    pub fn create(&mut self, urn: RadUrn, timestamp: T) -> Option<SomeRequest<T>>
    where
        T: Clone,
    {
        match self.requests.get(&urn) {
            None => {
                let request = SomeRequest::Created(Request::new(urn.clone(), timestamp));
                self.requests.insert(urn, request);
                None
            }
            Some(request) => Some(request.clone()),
        }
    }

    fn transition<Prev, Next>(
        &mut self,
        matcher: impl FnOnce(&SomeRequest<T>) -> Option<&Prev>,
        transition: impl FnOnce(Prev) -> Result<Next, ()>,
        urn: &RadUrn,
    ) -> Result<Next, ()>
    where
        Prev: Clone,
        Next: Into<SomeRequest<T>> + Clone,
    {
        match self.requests.get(urn) {
            None => Err(()),
            Some(request) => match matcher(request) {
                Some(previous) => {
                    let next = transition(previous.clone())?;
                    self.requests.insert(urn.clone(), next.clone().into());
                    Ok(next)
                }
                None => Err(()),
            },
        }
    }

    pub fn requested(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<Requested, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.request(timestamp)),
            urn,
        )
    }

    pub fn attempted(&mut self, urn: &RadUrn, attempt: Attempt) -> Result<Request<Requested, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.attempt(attempt)),
            urn,
        )
    }

    pub fn found_peer(
        &mut self,
        urn: &RadUrn,
        peer: PeerId,
        timestamp: T,
    ) -> Result<Request<Requested, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.found_peer(peer, timestamp)),
            urn,
        )
    }

    pub fn cloning(
        &mut self,
        urn: &RadUrn,
        peer: PeerId,
        timestamp: T,
    ) -> Result<Request<Requested, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.cloning(peer, timestamp)),
            urn,
        )
    }

    pub fn fulfilled(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
        repo: Repo,
    ) -> Result<Request<Fulfilled, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| previous.fulfilled(repo, timestamp),
            urn,
        )
    }

    pub fn canceled(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<Canceled, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |prev| Ok(prev.cancel(timestamp)),
            urn,
        )
    }

    pub fn timed_out(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<TimedOut, T>, ()>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |prev| Ok(prev.timed_out(timestamp)),
            urn,
        )
    }

    pub fn list(&self) -> impl Iterator<Item = &RadUrn> {
        self.requests.keys()
    }

    pub fn next(&self) -> Option<&Request<Created, T>> {
        self.requests
            .iter()
            .filter_map(|(_, request)| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            })
            .next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use librad::uri::RadUrn;

    #[test]
    fn happy_path_of_full_request() {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new();
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let request = waiting_room.create(urn.clone(), ());

        assert_eq!(request, None);

        let created = waiting_room.next();
        assert_eq!(created, Some(&Request::new(urn.clone(), ())),);

        let requested = waiting_room.requested(&urn, ());
        assert_eq!(requested, Ok(Request::new(urn.clone(), ()).request(())));

        let requested = waiting_room.attempted(&urn, Attempt::Query);
        assert_eq!(
            requested,
            Ok(Request::new(urn.clone(), ())
                .request(())
                .attempt(Attempt::Query))
        );

        let found = waiting_room.found_peer(&urn, "peer1".to_string(), ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ());
        assert_eq!(found, Ok(expected),);

        let requested = waiting_room.attempted(&urn, Attempt::CloneRepo);
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo);
        assert_eq!(requested, Ok(expected));

        let requested = waiting_room.cloning(&urn, "peer1".to_string(), ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo)
            .cloning("peer1".to_string(), ());
        assert_eq!(requested, Ok(expected));

        let repo = Repo {
            urn: urn.clone(),
            name: "next-please!".to_string(),
        };

        let fulfilled = waiting_room.fulfilled(&urn, (), repo.clone());
        let expected = Request::new(urn, ())
            .request(())
            .attempt(Attempt::Query)
            .found_peer("peer1".to_string(), ())
            .attempt(Attempt::CloneRepo)
            .cloning("peer1".to_string(), ())
            .fulfilled(repo, ());
        assert_eq!(fulfilled, expected);
    }
}
