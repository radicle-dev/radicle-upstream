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
    config: Config,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub max_queries: Queries,
    pub max_clones: Clones,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_queries: MAX_QUERIES,
            max_clones: MAX_CLONES,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Strategy<R> {
    First,
    Newest,
    Oldest,
    Random(R),
}

impl<R> Strategy<R> {
    pub fn next<'a, T: 'a, U: Ord>(
        self,
        mut items: impl Iterator<Item = &'a T>,
        mut key: impl FnMut(&T) -> U,
    ) -> Option<&'a T>
    where
        R: Rng,
    {
        match self {
            Self::First => items.next(),
            Self::Newest => items.max_by_key(|i| key(i)),
            Self::Oldest => items.min_by_key(|i| key(i)),
            Self::Random(mut rng) => items.choose(&mut rng),
        }
    }
}

impl<T> WaitingRoom<T> {
    pub fn new(config: Config) -> Self {
        Self {
            requests: HashMap::new(),
            config,
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

    pub fn queried_requested(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
    ) -> Result<Either<Request<TimedOut, T>, Request<IsRequested, T>>, Error>
    where
        T: Clone,
    {
        let max_queries = self.config.max_queries;
        let max_clones = self.config.max_clones;
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request),
                _ => None,
            },
            |previous| previous.queried(max_queries, max_clones, timestamp),
            urn,
        )
    }

    pub fn queried_found(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
    ) -> Result<Either<Request<TimedOut, T>, Request<Found, T>>, Error>
    where
        T: Clone,
    {
        let max_queries = self.config.max_queries;
        let max_clones = self.config.max_clones;
        self.transition(
            |request| match request {
                SomeRequest::Found(request) => Some(request),
                _ => None,
            },
            |previous| previous.queried(max_queries, max_clones, timestamp),
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

    pub fn cloning(
        &mut self,
        urn: &RadUrn,
        timestamp: T,
    ) -> Result<Either<Request<TimedOut, T>, Request<Cloning, T>>, Error>
    where
        T: Clone,
    {
        let max_queries = self.config.max_queries;
        let max_clones = self.config.max_clones;
        self.transition(
            |request| match request {
                SomeRequest::Found(request) => Some(request),
                _ => None,
            },
            |previous| previous.cloning(max_queries, max_clones, timestamp),
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
            |request| request.timestamp.clone(),
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

    #[cfg(test)]
    pub fn insert<R>(&mut self, urn: RadUrn, request: R)
    where
        R: Into<SomeRequest<T>>,
    {
        self.requests.insert(urn, request.into());
    }
}

#[cfg(test)]
mod test {
    use std::error;

    use librad::{keys::SecretKey, peer::PeerId, uri::RadUrn};
    use pretty_assertions::assert_eq;
    use proptest::{collection, prelude::prop_assert_eq};

    use super::*;
    use crate::request::Attempts;

    // TODO(finto): Test queried_found

    #[test]
    fn happy_path_of_full_request() {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(Config::default());
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
            .cloning(MAX_QUERIES, MAX_CLONES, ());
        assert_eq!(requested, Ok(expected));

        let found_repo = urn.clone();

        let fulfilled = waiting_room.cloned(&urn, found_repo.clone(), ());
        let expected = Request::new(urn, ())
            .request(())
            .first_peer(peer_id.clone(), ())
            .cloning(MAX_QUERIES, MAX_CLONES, ())
            .unwrap_right()
            .cloned(found_repo, ());
        assert_eq!(fulfilled, Ok(expected));
    }

    #[test]
    fn cannot_create_twice() {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(Config::default());
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        waiting_room.create(urn.clone(), ());
        let request = waiting_room.create(urn.clone(), ());
        assert_eq!(request, Some(SomeRequest::Created(Request::new(urn, ()))));
    }

    #[test]
    fn timeout_on_requests() -> Result<(), Box<dyn error::Error + 'static>> {
        const NUM_QUERIES: usize = 16;
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(Config {
            max_queries: Queries::new(NUM_QUERIES),
            max_clones: Clones::new(0),
        });
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");

        let _ = waiting_room.create(urn.clone(), ());
        let mut request = Either::Right(waiting_room.requested(&urn, ())?);

        for _ in 0..NUM_QUERIES {
            request = waiting_room.queried_requested(&urn, ())?;
        }

        assert_eq!(
            request,
            Either::Left(Request {
                urn,
                attempts: Attempts {
                    queries: Queries::new(NUM_QUERIES + 1),
                    clones: Clones::new(0),
                },
                timestamp: (),
                state: TimedOut::Query,
            })
        );

        Ok(())
    }

    #[test]
    fn timeout_on_clones() -> Result<(), Box<dyn error::Error + 'static>> {
        const NUM_CLONES: usize = 16;
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(Config {
            max_queries: Queries::new(1),
            max_clones: Clones::new(NUM_CLONES),
        });
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer_id = PeerId::from(SecretKey::new());

        let _ = waiting_room.create(urn.clone(), ());
        let _ = waiting_room.requested(&urn, ())?;
        let _ = waiting_room.first_peer(&urn, peer_id.clone(), ())?;
        let mut request = waiting_room.cloning(&urn, ())?;

        for _ in 0..NUM_CLONES {
            let _ = waiting_room.failed(peer_id.clone(), &urn, ())?;
            request = waiting_room.cloning(&urn, ())?;
        }

        assert_eq!(
            request,
            Either::Left(Request {
                urn,
                attempts: Attempts {
                    queries: Queries::new(1),
                    clones: Clones::new(NUM_CLONES + 1),
                },
                timestamp: (),
                state: TimedOut::Clone,
            })
        );

        Ok(())
    }

    #[test]
    fn cancel_transitions() -> Result<(), Box<dyn error::Error + 'static>> {
        let config = Config::default();
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(config);
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer_id = PeerId::from(SecretKey::new());

        // created
        let _ = waiting_room.create(urn.clone(), ());
        let request = waiting_room.canceled(&urn, ())?;
        assert_eq!(request, Request::new(urn.clone(), ()).cancel(()));

        // requested
        let is_requested = Request::new(urn.clone(), ()).request(());
        waiting_room.insert(urn.clone(), is_requested.clone());
        let request = waiting_room.canceled(&urn, ())?;
        assert_eq!(request, is_requested.clone().cancel(()));

        // found
        let found = is_requested.first_peer(peer_id.clone(), ());
        waiting_room.insert(urn.clone(), found.clone());
        let request = waiting_room.canceled(&urn, ())?;
        assert_eq!(request, found.clone().cancel(()));

        // cloning
        let cloning = found
            .cloning(config.max_queries, config.max_clones, ())
            .unwrap_right();
        waiting_room.insert(urn.clone(), cloning.clone());
        let request = waiting_room.canceled(&urn, ())?;
        assert_eq!(request, cloning.clone().cancel(()));

        // cloned
        let cloned = cloning.cloned(urn.clone(), ());
        waiting_room.insert(urn.clone(), cloned.clone());
        let request = waiting_room.canceled(&urn, ());
        assert_eq!(request, Err(Error::StateMismatch));

        // cancel
        let cancelled = Request::new(urn.clone(), ()).cancel(());
        waiting_room.insert(urn.clone(), cancelled.clone());
        let request = waiting_room.canceled(&urn, ())?;
        assert_eq!(request, cancelled);

        Ok(())
    }

    #[test]
    fn can_get_request_that_is_ready() -> Result<(), Box<dyn error::Error + 'static>> {
        let config = Config::default();
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(config);
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer_id = PeerId::from(SecretKey::new());
        let strategy: Strategy<rand::rngs::StdRng> = Strategy::First;

        let ready = waiting_room.ready(strategy.clone());
        assert_eq!(ready, None);

        let _ = waiting_room.create(urn.clone(), ());
        waiting_room.requested(&urn, ())?;
        waiting_room.first_peer(&urn, peer_id, ())?;
        waiting_room.cloning(&urn, ())?;
        let cloned = waiting_room.cloned(&urn, urn.clone(), ())?;

        let ready = waiting_room.ready(strategy);
        assert_eq!(ready, Some(&cloned));

        Ok(())
    }

    fn strategy_first(xs: &[u32]) -> (Option<&u32>, Option<&u32>) {
        let first = xs.first();
        let strategy: Strategy<rand::rngs::StdRng> = Strategy::First;

        (strategy.next(xs.iter(), |i| *i), first)
    }

    fn strategy_newest(xs: &[u32]) -> (Option<&u32>, Option<&u32>) {
        let newest = xs.iter().max();
        let strategy: Strategy<rand::rngs::StdRng> = Strategy::Newest;

        (strategy.next(xs.iter(), |i| *i), newest)
    }

    fn strategy_oldest(xs: &[u32]) -> (Option<&u32>, Option<&u32>) {
        let oldest = xs.iter().min();
        let strategy: Strategy<rand::rngs::StdRng> = Strategy::Oldest;

        (strategy.next(xs.iter(), |i| *i), oldest)
    }

    proptest! {
        #[test]
        fn prop_strategy_first(xs in collection::vec(0u32..1000, 1..100)) {
            let (got, expected) = strategy_first(&xs);
            prop_assert_eq!(got, expected);
        }

        #[test]
        fn prop_strategy_newest(xs in collection::vec(0u32..1000, 1..100)) {
            let (got, expected) = strategy_newest(&xs);
            prop_assert_eq!(got, expected);
        }

        #[test]
        fn prop_strategy_oldest(xs in collection::vec(0u32..1000, 1..100)) {
            let (got, expected) = strategy_oldest(&xs);
            prop_assert_eq!(got, expected);
        }
    }
}
