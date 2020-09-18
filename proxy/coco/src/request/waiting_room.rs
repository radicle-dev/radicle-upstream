use std::collections::HashMap;

use either::Either;
use rand::{seq::IteratorRandom as _, Rng};
use serde::{Deserialize, Serialize};

use librad::peer::PeerId;
use librad::uri::RadUrn;

use crate::request::{
    Cloned, Clones, Cloning, Found, IsCanceled, IsCreated, IsRequested, Queries, Request,
    SomeRequest, TimedOut, MAX_CLONES, MAX_QUERIES,
};

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("the URN '{0}' was not found in the waiting room")]
    MissingUrn(RadUrn),
    #[error("the state fetched from the waiting room was not the expected state")]
    StateMismatch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitingRoom<T> {
    requests: HashMap<RadUrn, SomeRequest<T>>,
    max_queries: Queries,
    max_clones: Clones,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Strategy<R> {
    First,
    Newest,
    Oldest,
    Random(R),
}

impl<R> Strategy<R> {
    pub fn next<'a, S: 'a, T: 'a + Clone + Ord>(
        self,
        mut requests: impl Iterator<Item = &'a Request<S, T>>,
    ) -> Option<&'a Request<S, T>>
    where
        R: Rng,
    {
        match self {
            Self::First => requests.next(),
            Self::Newest => requests.max_by_key(|request| request.timestamp.clone()),
            Self::Oldest => requests.min_by_key(|request| request.timestamp.clone()),
            Self::Random(mut rng) => requests.choose(&mut rng),
        }
    }
}

// TODO(finto): Test scenario of "running" a request and updating the waiting room. Testing state
// transitions.
impl<T> WaitingRoom<T> {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            max_queries: MAX_QUERIES,
            max_clones: MAX_CLONES,
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
        matcher: impl FnOnce(SomeRequest<T>) -> Option<Prev>,
        transition: impl FnOnce(Prev) -> Next,
        urn: &RadUrn,
    ) -> Result<Next, Error>
    where
        T: Clone,
        Prev: Clone,
        Next: Into<SomeRequest<T>> + Clone,
    {
        match self.requests.get(urn) {
            None => Err(Error::MissingUrn(urn.clone())),
            Some(request) => match request.clone().transition(matcher, transition) {
                Either::Right(next) => {
                    self.requests.insert(urn.clone(), next.clone().into());
                    Ok(next)
                }
                Either::Left(_mismatch) => Err(Error::StateMismatch),
            },
        }
    }

    pub fn requested(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
    ) -> Result<Request<IsRequested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            },
            |previous| previous.request(timestamp),
            urn,
        )
    }

    pub fn queried(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<IsRequested, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| previous.queried(timestamp),
            urn,
        )
    }

    pub fn first_peer(
        &mut self,
        urn: &RadUrn,
        peer: PeerId,
        timestamp: T,
    ) -> Result<Request<Found, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| previous.first_peer(peer, timestamp),
            urn,
        )
    }

    pub fn cloning(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<Cloning, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Found(request) => Some(request),
                _ => None,
            },
            |previous| previous.cloning(timestamp),
            urn,
        )
    }

    pub fn failed(
        &mut self,
        peer_id: PeerId,
        urn: &RadUrn,
        timestamp: T,
    ) -> Result<Request<Found, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| previous.failed(peer_id, timestamp),
            urn,
        )
    }

    pub fn cloned(
        &mut self,
        urn: &RadUrn,
        found_repo: RadUrn,
        timestamp: T,
    ) -> Result<Request<Cloned, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| previous.cloned(found_repo, timestamp),
            urn,
        )
    }

    pub fn canceled(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<IsCanceled, T>, Error>
    where
        T: Clone,
    {
        self.transition(
            |request| request.clone().cancel(timestamp).right(),
            |prev| prev,
            urn,
        )
    }

    // TODO(finto): These semantics aren't quite right. We "may" have timed_out or we may not have.
    // So if we `Error` it's not correct.
    pub fn timed_out(&mut self, urn: &RadUrn, timestamp: T) -> Result<Request<TimedOut, T>, Error>
    where
        T: Clone,
    {
        let max_queries = self.max_queries;
        let max_clones = self.max_clones;
        self.transition(
            |request| {
                request
                    .clone()
                    .timed_out(max_queries, max_clones, timestamp)
                    .right()
            },
            |prev| prev,
            urn,
        )
    }

    pub fn list(&self) -> impl Iterator<Item = &RadUrn> {
        self.requests.keys()
    }

    fn filter<R: Rng, S>(
        &self,
        mut matcher: impl FnMut(&SomeRequest<T>) -> Option<&Request<S, T>>,
        strategy: Strategy<R>,
    ) -> Option<&Request<S, T>>
    where
        T: Clone + Ord,
    {
        strategy.next(
            self.requests
                .iter()
                .filter_map(|(_, request)| matcher(request)),
        )
    }

    pub fn next<R: Rng>(&self, strategy: Strategy<R>) -> Option<&Request<IsCreated, T>>
    where
        T: Clone + Ord,
    {
        self.filter(
            |request| match request {
                SomeRequest::Created(request) => Some(request),
                _ => None,
            },
            strategy,
        )
    }

    pub fn ready<R: Rng>(&self, strategy: Strategy<R>) -> Option<&Request<Cloned, T>>
    where
        T: Clone + Ord,
    {
        self.filter(
            |request| match request {
                SomeRequest::Cloned(request) => Some(request),
                _ => None,
            },
            strategy,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use librad::{keys::SecretKey, peer::PeerId, uri::RadUrn};

    #[test]
    fn happy_path_of_full_request() {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new();
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer_id = PeerId::from(SecretKey::new());
        let request = waiting_room.create(urn.clone(), ());

        assert_eq!(request, None);

        let strategy: Strategy<rand::rngs::StdRng> = Strategy::First;
        let created = waiting_room.next(strategy);
        assert_eq!(created, Some(&Request::new(urn.clone(), ())),);

        let requested = waiting_room.requested(&urn, ());
        assert_eq!(requested, Ok(Request::new(urn.clone(), ()).request(())));

        let found = waiting_room.first_peer(&urn, peer_id.clone(), ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .first_peer(peer_id.clone(), ());
        assert_eq!(found, Ok(expected),);

        let requested = waiting_room.cloning(&urn, ());
        let expected = Request::new(urn.clone(), ())
            .request(())
            .first_peer(peer_id.clone(), ())
            .cloning(());
        assert_eq!(requested, Ok(expected));

        let found_repo = urn.clone();

        let fulfilled = waiting_room.cloned(&urn, found_repo.clone(), ());
        let expected = Request::new(urn, ())
            .request(())
            .first_peer(peer_id.clone(), ())
            .cloning(())
            .cloned(found_repo, ());
        assert_eq!(fulfilled, Ok(expected));
    }
}
