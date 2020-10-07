//! The black box tracker of [`Request`]s and their lifecycles.

// I reserve the right to not match all the arms when picking out a single case, thank you very
// much.
#![allow(clippy::wildcard_enum_match_arm)]

use std::{collections::HashMap, ops::Sub};

use either::Either;
use serde::{Deserialize, Serialize};

use librad::{
    hash::Hash,
    uri::{self, RadUrl, RadUrn},
};

use crate::request::{
    sequence_result, Clones, Queries, Request, RequestState, SomeRequest, Status, TimedOut,
};

pub mod stream;

/// The maximum number of query attempts that can be made for a single request.
const MAX_QUERIES: Queries = Queries::new(5);

/// The maximum number of clone attempts that can be made for a single request.
const MAX_CLONES: Clones = Clones::new(5);

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
    StateMismatch(RequestState),

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
pub struct WaitingRoom<T, D> {
    /// The set of requests keyed by their `RadUrn`. This helps us keep only unique requests in the
    /// waiting room.
    #[serde(bound = "T: serde_millis::Milliseconds")]
    requests: HashMap<Hash, SomeRequest<T>>,

    /// The configuration of the waiting room.
    config: Config<D>,
}

/// The `Config` for the waiting room tells it what are the maximum number of query and clone
/// attempts that can be made for a single request.
///
/// The recommended approach to initialising the `Config` is to use its `Default` implementation,
/// i.e. `Config::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config<D> {
    /// The maximum number of query attempts that can be made.
    pub max_queries: Queries,
    /// The maximum number of clone attempts that can be made.
    pub max_clones: Clones,
    /// The minimum elapsed time between some provided time and a request's timestamp.
    /// For example, if we had the following setup:
    ///   * `delta = 1`
    ///   * `now = 3`
    ///   * `request.timestamp = 2`
    /// then the `delta` would be compared against `now - request.timestamp`.
    pub delta: D,
}

impl<D> Default for Config<D>
where
    D: Default,
{
    fn default() -> Self {
        Self {
            max_queries: MAX_QUERIES,
            max_clones: MAX_CLONES,
            delta: D::default(),
        }
    }
}

impl<T, D> WaitingRoom<T, D> {
    /// Initialise a new `WaitingRoom` with the supplied `config`.
    #[must_use]
    pub fn new(config: Config<D>) -> Self {
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
        self.requests.get(&urn.id)
    }

    /// Get the [`Request::elapsed`] time between the `timestamp` provided and the current timestamp
    /// of the underlying `Request`.
    ///
    /// If the `urn` could not be found then `None` is returned.
    pub fn elapsed(&self, urn: &RadUrn, timestamp: T) -> Option<D>
    where
        T: Sub<T, Output = D> + Clone,
    {
        Some(self.get(urn)?.elapsed(timestamp))
    }

    /// This will return the request for the given `urn` if one exists in the `WaitingRoom`.
    ///
    /// If there is no such `urn` then it create a fresh `Request` using the `urn` and `timestamp`
    /// and it will return `None`.
    pub fn request(&mut self, urn: RadUrn, timestamp: T) -> Option<SomeRequest<T>>
    where
        T: Clone,
    {
        match self.get(&urn) {
            None => {
                let request = SomeRequest::Created(Request::new(urn.clone(), timestamp));
                self.requests.insert(urn.id, request);
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
        match self.get(urn) {
            None => Err(Error::MissingUrn(urn.clone())),
            Some(request) => {
                match sequence_result(request.clone().transition(matcher, transition))? {
                    Either::Right(next) => {
                        self.requests.insert(urn.id.clone(), next.into());
                        Ok(())
                    },
                    Either::Left(mismatch) => Err(Error::StateMismatch((&mismatch).into())),
                }
            },
        }
    }

    /// Tell the `WaitingRoom` that a query was made for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Created` state then it will transition to the
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
    pub fn found(&mut self, RadUrl { urn, authority }: RadUrl, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => {
                    Some(request.into_found(authority, timestamp).into())
                },
                SomeRequest::Found(request) => {
                    let some_request: SomeRequest<T> = request.found(authority, timestamp).into();
                    Some(some_request)
                },
                SomeRequest::Cloning(request) => {
                    let some_request: SomeRequest<T> = request.found(authority, timestamp).into();
                    Some(some_request)
                },
                _ => None,
            },
            Ok,
            &urn,
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
    pub fn cloning(&mut self, RadUrl { urn, authority }: RadUrl, timestamp: T) -> Result<(), Error>
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
            |previous| match previous.cloning(max_queries, max_clones, authority, timestamp) {
                Either::Left(timeout) => Err(timeout.into()),
                Either::Right(request) => Ok(request),
            },
            &urn,
        )
    }

    /// Tell the `WaitingRoom` that we failed the attempt to clone from the `peer` for the given
    /// `urn`.
    ///
    /// If the underlying `Request` was in the `Cloning` state then it will transition to the
    /// `Found` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn cloning_failed(
        &mut self,
        RadUrl { urn, authority }: RadUrl,
        timestamp: T,
    ) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.failed(authority, timestamp)),
            &urn,
        )
    }

    /// Tell the `WaitingRoom` that we successfully cloned the given `urn`.
    ///
    /// If the underlying `Request` was in the `Cloning` state then it will transition to the
    /// `Cloned` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn cloned(&mut self, urn: &RadUrn, url: RadUrl, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Ok(previous.cloned(url, timestamp)),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we are cancelling the request for the given `urn`.
    ///
    /// If the underlying `Request` was in the `{Created, IsRequested, Found, Cloning,
    /// Cancelled}` state then it will transition to the `Cancelled` state.
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

    /// Return the list of all `RadUrn`/`SomeRequest` pairs in the `WaitingRoom`.
    pub fn iter(&self) -> impl Iterator<Item = (RadUrn, &SomeRequest<T>)> {
        self.requests.iter().map(|(hash, request)| {
            (
                RadUrn::new(hash.clone(), uri::Protocol::Git, uri::Path::empty()),
                request,
            )
        })
    }

    /// Filter the `WaitingRoom` by:
    ///   * Choosing which [`RequestState`] you are looking for
    pub fn filter_by_state(
        &self,
        request_state: RequestState,
    ) -> impl Iterator<Item = (RadUrn, &SomeRequest<T>)> {
        self.iter()
            .filter(move |(_, request)| RequestState::from(*request) == request_state.clone())
    }

    /// Find the first occurring request based on the call to [`WaitingRoom::filter_by_state`].
    pub fn find_by_state(&self, request_state: RequestState) -> Option<(RadUrn, &SomeRequest<T>)> {
        self.filter_by_state(request_state).next()
    }

    /// Get the next `Request` that is in a query state, i.e. `Created` or `Requested`.
    ///
    /// In the case of the `Requested` state we check if:
    ///   * The request is a fresh request that hasn't had an attempt to clone yet
    ///   * Or the elapsed time between the `timestamp` and the `Request`'s timestamp is greater
    ///     than the `delta` provided in the [`Config`].
    pub fn next_query(&self, timestamp: T) -> Option<RadUrn>
    where
        T: Sub<T, Output = D> + Clone,
        D: Ord + Clone,
    {
        let created = self.find_by_state(RequestState::Created);
        let requested = self
            .filter_by_state(RequestState::Requested)
            .find(move |(_, request)| {
                request.elapsed(timestamp.clone()) >= self.config.delta.clone()
                    || request.attempts().clones == Clones::new(0)
            });

        created.or(requested).map(|(urn, _request)| urn)
    }

    /// Get the next `Request` that is in the the `Found` state and the status of the peer is
    /// `Available`.
    pub fn next_clone(&self) -> Option<RadUrl> {
        self.find_by_state(RequestState::Found)
            .and_then(|(urn, request)| match request {
                SomeRequest::Found(request) => {
                    request.iter().find_map(|(peer_id, status)| match status {
                        Status::Available => Some(urn.clone().into_rad_url(peer_id.clone())),
                        _ => None,
                    })
                },
                _ => None,
            })
    }

    #[cfg(test)]
    pub fn insert<R>(&mut self, urn: RadUrn, request: R)
    where
        R: Into<SomeRequest<T>>,
    {
        self.requests.insert(urn.id, request.into());
    }
}

#[cfg(test)]
mod test {
    use std::error;

    use assert_matches::assert_matches;
    use librad::{
        keys::SecretKey,
        peer::PeerId,
        uri::{RadUrl, RadUrn},
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn happy_path_of_full_request() -> Result<(), Box<dyn error::Error + 'static>> {
        let mut waiting_room: WaitingRoom<usize, usize> = WaitingRoom::new(Config::default());
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer = PeerId::from(SecretKey::new());
        let url = RadUrl {
            urn,
            authority: peer,
        };
        let request = waiting_room.request(url.urn.clone(), 0);

        assert_eq!(request, None);

        let created = waiting_room.find_by_state(RequestState::Created);
        assert_eq!(
            created,
            Some((
                url.urn.clone(),
                &SomeRequest::Created(Request::new(url.urn.clone(), 0))
            )),
        );

        waiting_room.queried(&url.urn, 0)?;
        let expected = SomeRequest::Requested(Request::new(url.urn.clone(), 0).request(0));
        assert_eq!(waiting_room.get(&url.urn), Some(&expected));

        waiting_room.found(url.clone(), 0)?;
        let expected = SomeRequest::Found(
            Request::new(url.urn.clone(), 0)
                .request(0)
                .into_found(url.authority.clone(), 0),
        );
        assert_eq!(waiting_room.get(&url.urn), Some(&expected));

        waiting_room.cloning(url.clone(), 0)?;
        let expected = SomeRequest::Cloning(
            Request::new(url.urn.clone(), 0)
                .request(0)
                .into_found(url.authority.clone(), 0)
                .cloning(MAX_QUERIES, MAX_CLONES, url.authority.clone(), 0)
                .unwrap_right(),
        );
        assert_eq!(waiting_room.get(&url.urn), Some(&expected));

        waiting_room.cloned(&url.urn, url.clone(), 0)?;
        let expected = SomeRequest::Cloned(
            Request::new(url.urn.clone(), 0)
                .request(0)
                .into_found(url.authority.clone(), 0)
                .cloning(MAX_QUERIES, MAX_CLONES, url.authority.clone(), 0)
                .unwrap_right()
                .cloned(url.clone(), 0),
        );
        assert_eq!(waiting_room.get(&url.urn), Some(&expected));

        Ok(())
    }

    #[test]
    fn cannot_create_twice() -> Result<(), Box<dyn error::Error>> {
        let mut waiting_room: WaitingRoom<(), ()> = WaitingRoom::new(Config::default());
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        waiting_room.request(urn.clone(), ());
        let request = waiting_room.request(urn.clone(), ());

        assert_eq!(
            request,
            Some(SomeRequest::Created(Request::new(urn.clone(), ())))
        );

        waiting_room.queried(&urn, ())?;
        let request = waiting_room.request(urn.clone(), ());

        assert_eq!(
            request,
            Some(SomeRequest::Requested(Request::new(urn, ()).request(())))
        );

        Ok(())
    }

    #[test]
    fn timeout_on_requests() -> Result<(), Box<dyn error::Error + 'static>> {
        const NUM_QUERIES: usize = 16;
        let mut waiting_room: WaitingRoom<(), ()> = WaitingRoom::new(Config {
            max_queries: Queries::new(NUM_QUERIES),
            max_clones: Clones::new(0),
            delta: (),
        });
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");

        let _ = waiting_room.request(urn.clone(), ());
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

    #[allow(clippy::indexing_slicing)]
    #[test]
    fn timeout_on_clones() -> Result<(), Box<dyn error::Error + 'static>> {
        const NUM_CLONES: usize = 16;
        let mut waiting_room: WaitingRoom<(), ()> = WaitingRoom::new(Config {
            max_queries: Queries::new(1),
            max_clones: Clones::new(NUM_CLONES),
            delta: (),
        });
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");

        let mut peers = vec![];
        for _ in 0..=NUM_CLONES {
            peers.push(RadUrl {
                urn: urn.clone(),
                authority: PeerId::from(SecretKey::new()),
            });
        }

        let _ = waiting_room.request(urn.clone(), ());
        waiting_room.queried(&urn, ())?;

        for url in &peers {
            waiting_room.found(url.clone(), ())?;
        }

        for url in &peers[0..NUM_CLONES] {
            waiting_room.cloning(url.clone(), ())?;
            waiting_room.cloning_failed(url.clone(), ())?;
        }

        assert_eq!(
            waiting_room.cloning(
                peers
                    .last()
                    .expect("unless you changed NUM_CLONES to < -1 we should be fine here. qed.")
                    .clone(),
                ()
            ),
            Err(Error::TimeOut {
                timeout: TimedOut::Clone,
                attempts: 17,
            })
        );

        Ok(())
    }

    #[test]
    fn cloning_fails_back_to_requested() -> Result<(), Box<dyn error::Error + 'static>> {
        const NUM_CLONES: usize = 5;
        let mut waiting_room: WaitingRoom<(), ()> = WaitingRoom::new(Config {
            max_queries: Queries::new(1),
            max_clones: Clones::new(NUM_CLONES),
            delta: (),
        });
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");

        let mut peers = vec![];
        for _ in 0..NUM_CLONES {
            peers.push(RadUrl {
                urn: urn.clone(),
                authority: PeerId::from(SecretKey::new()),
            });
        }

        let _ = waiting_room.request(urn.clone(), ());
        waiting_room.queried(&urn, ())?;

        for url in peers {
            waiting_room.found(url.clone(), ())?;
            waiting_room.cloning(url.clone(), ())?;
            waiting_room.cloning_failed(url, ())?;
        }

        assert_matches!(waiting_room.get(&urn), Some(SomeRequest::Requested(_)));

        Ok(())
    }

    #[test]
    fn cancel_transitions() -> Result<(), Box<dyn error::Error + 'static>> {
        let config = Config::default();
        let mut waiting_room: WaitingRoom<(), ()> = WaitingRoom::new(config);
        let urn: RadUrn = "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
            .parse()
            .expect("failed to parse the urn");
        let peer = PeerId::from(SecretKey::new());

        // created
        let _ = waiting_room.request(urn.clone(), ());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Cancelled(
                Request::new(urn.clone(), ()).cancel(())
            ))
        );

        // requested
        let is_requested = Request::new(urn.clone(), ()).request(());
        waiting_room.insert(urn.clone(), is_requested.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Cancelled(is_requested.clone().cancel(())))
        );

        // found
        let found = is_requested.into_found(peer.clone(), ());
        waiting_room.insert(urn.clone(), found.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Cancelled(found.clone().cancel(())))
        );

        // cloning
        let cloning = found
            .cloning(config.max_queries, config.max_clones, peer.clone(), ())
            .unwrap_right();
        waiting_room.insert(urn.clone(), cloning.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Cancelled(cloning.clone().cancel(())))
        );

        // cloned
        let cloned = cloning.cloned(
            RadUrl {
                urn: urn.clone(),
                authority: peer,
            },
            (),
        );
        waiting_room.insert(urn.clone(), cloned);
        assert_eq!(
            waiting_room.canceled(&urn, ()),
            Err(Error::StateMismatch(RequestState::Cloned))
        );

        // cancel
        let cancelled = Request::new(urn.clone(), ()).cancel(());
        waiting_room.insert(urn.clone(), cancelled.clone());
        waiting_room.canceled(&urn, ())?;
        assert_eq!(
            waiting_room.get(&urn),
            Some(&SomeRequest::Cancelled(cancelled))
        );

        Ok(())
    }

    #[test]
    fn can_get_request_that_is_ready() -> Result<(), Box<dyn error::Error + 'static>> {
        let config = Config::default();
        let mut waiting_room: WaitingRoom<usize, usize> = WaitingRoom::new(config);
        let url = RadUrl {
            urn: "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe"
                .parse()
                .expect("failed to parse the urn"),
            authority: PeerId::from(SecretKey::new()),
        };

        let ready = waiting_room.find_by_state(RequestState::Cloned);
        assert_eq!(ready, None);

        let _ = waiting_room.request(url.urn.clone(), 0);
        waiting_room.queried(&url.urn, 0)?;
        waiting_room.found(url.clone(), 0)?;
        waiting_room.cloning(url.clone(), 0)?;
        waiting_room.cloned(&url.urn, url.clone(), 0)?;

        let ready = waiting_room.find_by_state(RequestState::Cloned);
        let expected = SomeRequest::Cloned(
            Request::new(url.urn.clone(), 0)
                .request(0)
                .into_found(url.authority.clone(), 0)
                .cloning(
                    config.max_queries,
                    config.max_clones,
                    url.authority.clone(),
                    0,
                )
                .unwrap_right()
                .cloned(url.clone(), 0),
        );
        assert_eq!(ready, Some((url.urn, &expected)));

        Ok(())
    }
}
