// I reserve the right to not match all the arms when picking out a single case, thank you very
// much.
#![allow(clippy::wildcard_enum_match_arm)]

use std::collections::HashMap;

use either::Either;
use rand::{seq::IteratorRandom as _, Rng};
use serde::{Deserialize, Serialize};

use librad::{peer::PeerId, uri::RadUrn};

use crate::request::{
    sequence_result, Cloned, Clones, IsCreated, Queries, Request, RequestKind, SomeRequest,
    TimedOut, MAX_CLONES, MAX_QUERIES,
};

/// An error that can occur when interacting with the [`WaitingRoom`] API.
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// When looking up a `RadUrn` in the [`WaitingRoom`] it was missing.
    #[error("the URN '{0}' was not found in the waiting room")]
    MissingUrn(RadUrn),

    /// When performing an operation on the a [`Request`] in the [`WaitingRoom`] it was found to be
    /// in the wrong state for the desired operation.
    ///
    /// For example, if we tried to call `cloning` on a request that has only been created then
    /// this would be an invalid transition.
    #[error("the state fetched '{0}' from the waiting room was not one of the expected states")]
    StateMismatch(RequestKind),

    /// The [`Request`] timed out when performing an operation on it by exceeding the number of
    /// attempts it was allowed to make.
    #[error("encountered {timeout} time out after {attempts} attempts")]
    TimeOut {
        /// What kind of the time out that occurred.
        timeout: TimedOut,
        /// The number of attempts that were made when we timed out.
        attempts: usize,
    },
}

impl<T> From<Request<TimedOut, T>> for Error {
    fn from(other: Request<TimedOut, T>) -> Self {
        match &other.state {
            TimedOut::Query => Error::TimeOut {
                timeout: other.state,
                attempts: other.attempts.queries.into(),
            },
            TimedOut::Clone => Error::TimeOut {
                timeout: other.state,
                attempts: other.attempts.clones.into(),
            },
        }
    }
}

/// A `WaitingRoom` knows about a set of `Request`s that have been made, and can look them up via
/// their `RadUrn`.
///
/// It keeps track of these states as the user tells the waiting room what is happening to the
/// request on the outside.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitingRoom<T> {
    /// The set of requests keyed by their `RadUrn`. This helps us keep only unique requests in the
    /// waiting room.
    requests: HashMap<RadUrn, SomeRequest<T>>,

    /// The configuration of the waiting room.
    config: Config,
}

/// The `Config` for the waiting room tells it what are the maximum number of query and clone
/// attempts that can be made for a single request.
///
/// The recommended approach to initialising the `Config` is to use its `Default` implementation,
/// i.e. `Config::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// The maximum number of query attempts that can be made.
    pub max_queries: Queries,
    /// The maximum number of clone attempts that can be made.
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

/// The `Strategy` enumeration is for picking a strategy when selecting a `Request` from the
/// [`WaitingRoom`]. Specifically, it's used in [`WaitingRoom::next`] and [`WaitingRoom::ready`]
/// for selecting a `Request` that is in the state `IsCreated` and `Cloned`, respectively.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Strategy<R> {
    /// Select the first from the iterator.
    First,
    /// Select the newest from the iterator.
    Newest,
    /// Select the oldest from the iterator.
    Oldest,
    /// Select an item from the iterator at random.
    Random(R),
}

impl<R> Strategy<R> {
    /// Get back a `T` based off of the `Strategy`.
    ///
    /// If the iterator is empty then `None` is returned.
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
    /// Initialise a new `WaitingRoom` with the supplied `config`.
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            requests: HashMap::new(),
            config,
        }
    }

    /// Get the underlying [`SomeRequest`] for the given `urn`.
    ///
    /// Returns `None` if there is no such request.
    #[must_use]
    pub fn get(&self, urn: &RadUrn) -> Option<&SomeRequest<T>> {
        self.requests.get(urn)
    }

    /// Create a fresh [`Request`] with the given `urn`.
    ///
    /// If the `Request` already existed in the `WaitingRoom` then we get back the original request
    /// as `Some`. This means we did nothing with the `urn` or `timestamp`.
    pub fn create(&mut self, urn: RadUrn, timestamp: T) -> Option<SomeRequest<T>>
    where
        T: Clone,
    {
        match self.get(&urn) {
            None => {
                let request = SomeRequest::Created(Request::new(urn.clone(), timestamp));
                self.requests.insert(urn, request);
                None
            },
            Some(request) => Some(request.clone()),
        }
    }

    /// Transition the `Request` found at the provided `urn` and call the transition function to
    /// move it into its `Next` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the transition function supplied returns an error.
    fn transition<Prev, Next>(
        &mut self,
        matcher: impl FnOnce(SomeRequest<T>) -> Option<Prev>,
        transition: impl FnOnce(Prev) -> Result<Next, Error>,
        urn: &RadUrn,
    ) -> Result<(), Error>
    where
        T: Clone,
        Prev: Clone,
        Next: Into<SomeRequest<T>> + Clone,
    {
        match self.requests.get(urn) {
            None => Err(Error::MissingUrn(urn.clone())),
            Some(request) => {
                match sequence_result(request.clone().transition(matcher, transition))? {
                    Either::Right(next) => {
                        self.requests.insert(urn.clone(), next.into());
                        Ok(())
                    },
                    Either::Left(mismatch) => Err(Error::StateMismatch((&mismatch).into())),
                }
            },
        }
    }

    /// Tell the `WaitingRoom` that a query was made for the given `urn`.
    ///
    /// If the underlying `Request` was in the `IsCreated` state then it will transition to the
    /// `IsRequested` state.
    ///
    /// If the underlying `Request` was in the `IsRequested` state then it increments the query
    /// attempt.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the underlying `Request` timed out.
    pub fn queried(&mut self, urn: &RadUrn, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        let max_queries = self.config.max_queries;
        let max_clones = self.config.max_clones;
        self.transition(
            |request| match request {
                SomeRequest::Created(request) => Some(Either::Right(request.request(timestamp))),
                SomeRequest::Requested(request) => {
                    Some(request.queried(max_queries, max_clones, timestamp))
                },
                _ => None,
            },
            |previous| match previous {
                Either::Left(timeout) => Err(timeout.into()),
                Either::Right(request) => Ok(request),
            },
            urn,
        )
    }

    /// Tell the `WaitingRoom` that a `peer` was found for the given `urn`.
    ///
    /// If the underlying `Request` was in the `IsRequested` state then it will transition to the
    /// `Found` state.
    ///
    /// If the underlying `Request` was in the `Found` or `Cloning` state then it add this `peer`
    /// to the set of found peers.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn found(&mut self, urn: &RadUrn, peer: PeerId, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => Some(request.first_peer(peer, timestamp).into()),
                SomeRequest::Found(request) => {
                    let some_request: SomeRequest<T> = request.found(peer, timestamp).into();
                    Some(some_request)
                },
                SomeRequest::Cloning(request) => {
                    let some_request: SomeRequest<T> = request.found(peer, timestamp).into();
                    Some(some_request)
                },
                _ => None,
            },
            Ok,
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we are attempting a clone from the `peer` for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Found` state then it will transition to the
    /// `Cloning` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the underlying `Request` timed out.
    pub fn cloning(&mut self, urn: &RadUrn, peer: PeerId, timestamp: T) -> Result<(), Error>
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
            |previous| match previous.cloning(max_queries, max_clones, peer, timestamp) {
                Either::Left(timeout) => Err(timeout.into()),
                Either::Right(request) => Ok(request),
            },
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we failed the attempt to clone from the `peer` for the given
    /// `urn`.
    ///
    /// If the underlying `Request` was in the `Found` state then it will transition to the
    /// `Cloning` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn failed(&mut self, peer: PeerId, urn: &RadUrn, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.failed(peer, timestamp)),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we successfully cloned the `found_repo` for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Cloning` state then it will transition to the
    /// `Cloned` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn cloned(&mut self, urn: &RadUrn, found_repo: RadUrn, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.cloned(found_repo, timestamp)),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we are cancelling the request for the given `urn`.
    ///
    /// If the underlying `Request` was in the `{IsCreated, IsRequested, Found, Cloning,
    /// IsCanceled}` state then it will transition to the `IsCanceled` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn canceled(&mut self, urn: &RadUrn, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(|request| request.cancel(timestamp).right(), Ok, urn)
    }

    /// Return the list of all `RadUrn`s in the `WaitingRoom`.
    pub fn list(&self) -> impl Iterator<Item = &RadUrn> {
        self.requests.keys()
    }

    /// Filter the requests in the waiting room based on the passed in `matcher`.
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

    /// Return a `Request` that is in the `IsCreated` state, if any, based off of the supplied
    /// `strategy`.
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

    /// Return a `Request` that is in the `Cloned` state, if any, based off of the supplied
    /// `strategy`.
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
    use proptest::{collection, prelude::prop_assert_eq, proptest};

    use super::*;

    #[test]
    fn happy_path_of_full_request() -> Result<(), Box<dyn error::Error + 'static>> {
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(Config::default());
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer = PeerId::from(SecretKey::new());
        let request = waiting_room.create(urn.clone(), ());

        assert_eq!(request, None);

        let strategy: Strategy<rand::rngs::StdRng> = Strategy::First;
        let created = waiting_room.next(strategy);
        assert_eq!(created, Some(&Request::new(urn.clone(), ())),);

        waiting_room.queried(&urn, ())?;
        let expected = SomeRequest::Requested(Request::new(urn.clone(), ()).request(()));
        assert_eq!(waiting_room.get(&urn), Some(&expected));

        waiting_room.found(&urn, peer.clone(), ())?;
        let expected = SomeRequest::Found(
            Request::new(urn.clone(), ())
                .request(())
                .first_peer(peer.clone(), ()),
        );
        assert_eq!(waiting_room.get(&urn), Some(&expected));

        waiting_room.cloning(&urn, peer.clone(), ())?;
        let expected = SomeRequest::Cloning(
            Request::new(urn.clone(), ())
                .request(())
                .first_peer(peer.clone(), ())
                .cloning(MAX_QUERIES, MAX_CLONES, peer.clone(), ())
                .unwrap_right(),
        );
        assert_eq!(waiting_room.get(&urn), Some(&expected));

        let found_repo = urn.clone();

        waiting_room.cloned(&urn, found_repo.clone(), ())?;
        let expected = SomeRequest::Cloned(
            Request::new(urn.clone(), ())
                .request(())
                .first_peer(peer.clone(), ())
                .cloning(MAX_QUERIES, MAX_CLONES, peer, ())
                .unwrap_right()
                .cloned(found_repo, ()),
        );
        assert_eq!(waiting_room.get(&urn), Some(&expected));

        Ok(())
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
        for _ in 0..NUM_QUERIES {
            waiting_room.queried(&urn, ())?;
        }

        assert_eq!(
            waiting_room.queried(&urn, ()),
            Err(Error::TimeOut {
                timeout: TimedOut::Query,
                attempts: 17,
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
        let peer = PeerId::from(SecretKey::new());

        let _ = waiting_room.create(urn.clone(), ());
        waiting_room.queried(&urn, ())?;
        waiting_room.found(&urn, peer.clone(), ())?;
        waiting_room.cloning(&urn, peer.clone(), ())?;

        for _ in 1..NUM_CLONES {
            waiting_room.failed(peer.clone(), &urn, ())?;
            waiting_room.cloning(&urn, peer.clone(), ())?;
        }

        waiting_room.failed(peer.clone(), &urn, ())?;
        assert_eq!(
            waiting_room.cloning(&urn, peer, ()),
            Err(Error::TimeOut {
                timeout: TimedOut::Clone,
                attempts: 17,
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
        let peer = PeerId::from(SecretKey::new());

        // created
        let _ = waiting_room.create(urn.clone(), ());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Canceled(
                Request::new(urn.clone(), ()).cancel(())
            ))
        );

        // requested
        let is_requested = Request::new(urn.clone(), ()).request(());
        waiting_room.insert(urn.clone(), is_requested.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Canceled(is_requested.clone().cancel(())))
        );

        // found
        let found = is_requested.first_peer(peer.clone(), ());
        waiting_room.insert(urn.clone(), found.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Canceled(found.clone().cancel(())))
        );

        // cloning
        let cloning = found
            .cloning(config.max_queries, config.max_clones, peer, ())
            .unwrap_right();
        waiting_room.insert(urn.clone(), cloning.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Canceled(cloning.clone().cancel(())))
        );

        // cloned
        let cloned = cloning.cloned(urn.clone(), ());
        waiting_room.insert(urn.clone(), cloned);
        assert_eq!(
            waiting_room.canceled(&urn, ()),
            Err(Error::StateMismatch(RequestKind::Cloned))
        );

        // cancel
        let cancelled = Request::new(urn.clone(), ()).cancel(());
        waiting_room.insert(urn.clone(), cancelled.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Canceled(cancelled))
        );

        Ok(())
    }

    #[test]
    fn can_get_request_that_is_ready() -> Result<(), Box<dyn error::Error + 'static>> {
        let config = Config::default();
        let mut waiting_room: WaitingRoom<()> = WaitingRoom::new(config);
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer = PeerId::from(SecretKey::new());
        let strategy: Strategy<rand::rngs::StdRng> = Strategy::First;

        let ready = waiting_room.ready(strategy.clone());
        assert_eq!(ready, None);

        let _ = waiting_room.create(urn.clone(), ());
        waiting_room.queried(&urn, ())?;
        waiting_room.found(&urn, peer.clone(), ())?;
        waiting_room.cloning(&urn, peer.clone(), ())?;
        waiting_room.cloned(&urn, urn.clone(), ())?;

        let ready = waiting_room.ready(strategy);
        let expected = Request::new(urn.clone(), ())
            .request(())
            .first_peer(peer.clone(), ())
            .cloning(config.max_queries, config.max_clones, peer, ())
            .unwrap_right()
            .cloned(urn, ());
        assert_eq!(ready, Some(&expected));

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
        fn prop_strategy_first(xs in collection::vec(0_u32..1000, 1..100)) {
            let (got, expected) = strategy_first(&xs);
            prop_assert_eq!(got, expected);
        }

        #[test]
        fn prop_strategy_newest(xs in collection::vec(0_u32..1000, 1..100)) {
            let (got, expected) = strategy_newest(&xs);
            prop_assert_eq!(got, expected);
        }

        #[test]
        fn prop_strategy_oldest(xs in collection::vec(0_u32..1000, 1..100)) {
            let (got, expected) = strategy_oldest(&xs);
            prop_assert_eq!(got, expected);
        }
    }
}
