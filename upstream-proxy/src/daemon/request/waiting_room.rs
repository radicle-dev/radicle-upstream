// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! The black box tracker of [`Request`]s and their lifecycles.

// I reserve the right to not match all the arms when picking out a single case, thank you very
// much.
#![allow(clippy::wildcard_enum_match_arm)]

use std::{
    cmp::PartialOrd,
    collections::HashMap,
    convert::TryFrom,
    ops::{Add, Mul},
};

use either::Either;
use serde::{Deserialize, Serialize};

use librad::{
    git::{identities::Revision, Urn},
    PeerId,
};

use crate::daemon::request::{
    Clones, Queries, Request, RequestState, SomeRequest, Status, TimedOut,
};

/// The maximum number of query attempts that can be made for a single request.
const MAX_QUERIES: Queries = Queries::Infinite;

/// The maximum number of clone attempts that can be made for a single request.
const MAX_CLONES: Clones = Clones::Infinite;

/// An error that can occur when interacting with the [`WaitingRoom`] API.
#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// When looking up a `Urn` in the [`WaitingRoom`] it was missing.
    #[error("the URN '{0}' was not found in the waiting room")]
    MissingUrn(Urn),

    /// When performing an operation on the a [`Request`] in the [`WaitingRoom`]
    /// it was found to be in the wrong state for the desired operation.
    ///
    /// For example, if we tried to call `cloning` on a request that has only
    /// been created then this would be an invalid transition.
    #[error("the state fetched '{0}' from the waiting room was not one of the expected states")]
    StateMismatch(RequestState),

    /// The [`Request`] timed out when performing an operation on it by
    /// exceeding the number of attempts it was allowed to make.
    #[error("encountered {timeout} time out after {attempts:?} attempts")]
    TimeOut {
        /// What kind of the time out that occurred.
        timeout: TimedOut,
        /// The number of attempts that were made when we timed out.
        attempts: Option<usize>,
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

/// Holds either the newly created request or the request already present for
/// the requested urn.
pub type Created<T> = Either<SomeRequest<T>, SomeRequest<T>>;

/// A `WaitingRoom` knows about a set of `Request`s that have been made, and can
/// look them up via their `Urn`.
///
/// It keeps track of these states as the user tells the waiting room what is
/// happening to the request on the outside.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WaitingRoom<T, D> {
    /// The set of requests keyed by their `Urn`. This helps us keep only unique
    /// requests in the waiting room.
    #[serde(bound = "T: serde_millis::Milliseconds")]
    requests: HashMap<Revision, SomeRequest<T>>,

    /// The configuration of the waiting room.
    config: Config<D>,
}

/// The `Config` for the waiting room tells it what are the maximum number of
/// query and clone attempts that can be made for a single request.
///
/// The recommended approach to initialising the `Config` is to use its
/// `Default` implementation, i.e. `Config::default()`, followed by setting the
/// `delta`, since the usual default values for number values are `0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config<D> {
    /// The maximum number of query attempts that can be made.
    pub max_queries: Queries,
    /// The maximum number of clone attempts that can be made.
    pub max_clones: Clones,
    /// The minimum elapsed time between some provided time and a request's
    /// timestamp. For example, if we had the following setup:
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
    pub fn get(&self, urn: &Urn) -> Option<&SomeRequest<T>> {
        self.requests.get(&urn.id)
    }

    /// Permanently remove a request from the `WaitingRoom`. If the `urn` did
    /// exist in the `WaitingRoom` then the request will be returned.
    ///
    /// Otherwise, it will return `None` if no such request existed.
    pub fn remove(&mut self, urn: &Urn) -> Option<SomeRequest<T>> {
        self.requests.remove(&urn.id)
    }

    /// This will return the request for the given `urn` if one exists in the
    /// `WaitingRoom`.
    ///
    /// If there is no such `urn` then it create a fresh `Request` using the
    /// `urn` and `timestamp` and it will return `None`.
    pub fn request(&mut self, urn: &Urn, timestamp: T) -> Either<SomeRequest<T>, SomeRequest<T>>
    where
        T: Clone,
    {
        match self.get(urn) {
            None => {
                let request = SomeRequest::Created(Request::new(urn.clone(), timestamp));
                self.requests.insert(urn.id, request.clone());
                Either::Left(request)
            },
            Some(request) => Either::Right(request.clone()),
        }
    }

    /// Transition the `Request` found at the provided `urn` and call the
    /// transition function to move it into its `Next` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the transition function supplied returns an error.
    fn transition<Prev, Next>(
        &mut self,
        matcher: impl FnOnce(SomeRequest<T>) -> Option<Prev>,
        transition: impl FnOnce(Prev) -> Either<Request<TimedOut, T>, Next>,
        urn: &Urn,
    ) -> Result<(), Error>
    where
        T: Clone,
        Prev: Clone,
        Next: Into<SomeRequest<T>> + Clone,
    {
        match self.get(urn) {
            None => Err(Error::MissingUrn(urn.clone())),
            Some(request) => match request.clone().transition(matcher, transition) {
                Either::Right(Either::Right(next)) => {
                    self.requests.insert(urn.id, next.into());
                    Ok(())
                },
                Either::Right(Either::Left(timeout)) => {
                    self.requests.insert(urn.id, timeout.clone().into());
                    Err(timeout.into())
                },
                Either::Left(mismatch) => Err(Error::StateMismatch((&mismatch).into())),
            },
        }
    }

    /// Tell the `WaitingRoom` that a query was made for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Created` state then it will
    /// transition to the `IsRequested` state.
    ///
    /// If the underlying `Request` was in the `IsRequested` state then it
    /// increments the query attempt.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the underlying `Request` timed out.
    pub fn queried(&mut self, urn: &Urn, timestamp: T) -> Result<(), Error>
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
            |previous| previous,
            urn,
        )
    }

    /// Tell the `WaitingRoom` that a `peer` was found for the given `urn`.
    ///
    /// If the underlying `Request` was in the `IsRequested` state then it will
    /// transition to the `Found` state.
    ///
    /// If the underlying `Request` was in the `Found` or `Cloning` state then
    /// it add this `peer` to the set of found peers.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn found(&mut self, urn: &Urn, remote_peer: PeerId, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Requested(request) => {
                    Some(request.into_found(remote_peer, timestamp).into())
                },
                SomeRequest::Found(request) => {
                    let some_request: SomeRequest<T> = request.found(remote_peer, timestamp).into();
                    Some(some_request)
                },
                SomeRequest::Cloning(request) => {
                    let some_request: SomeRequest<T> = request.found(remote_peer, timestamp).into();
                    Some(some_request)
                },
                _ => None,
            },
            Either::Right,
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we are attempting a clone from the `peer`
    /// for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Found` state then it will
    /// transition to the `Cloning` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    ///   * If the underlying `Request` timed out.
    pub fn cloning(&mut self, urn: &Urn, remote_peer: PeerId, timestamp: T) -> Result<(), Error>
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
            |previous| previous.cloning(max_queries, max_clones, remote_peer, timestamp),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we failed the attempt to clone from the
    /// `peer` for the given `urn`.
    ///
    /// If the underlying `Request` was in the `Cloning` state then it will
    /// transition to the `Found` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    #[allow(clippy::needless_pass_by_value)]
    pub fn cloning_failed(
        &mut self,
        urn: &Urn,
        remote_peer: PeerId,
        timestamp: T,
        reason: Box<dyn std::error::Error>,
    ) -> Result<(), Error>
    where
        T: Clone,
    {
        #[allow(clippy::non_ascii_literal)]
        let reason_str = anyhow::Chain::new(reason.as_ref())
            .fold("".to_string(), |acc, e| format!("{} ⮩ {}", acc, e));
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Either::Right(previous.failed(remote_peer, reason_str.clone(), timestamp)),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we successfully cloned the given `urn`.
    ///
    /// If the underlying `Request` was in the `Cloning` state then it will
    /// transition to the `Cloned` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn cloned(&mut self, urn: &Urn, remote_peer: PeerId, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| match request {
                SomeRequest::Cloning(request) => Some(request),
                _ => None,
            },
            |previous| Either::Right(previous.cloned(remote_peer, timestamp)),
            urn,
        )
    }

    /// Tell the `WaitingRoom` that we are cancelling the request for the given
    /// `urn`.
    ///
    /// If the underlying `Request` was in the `{Created, IsRequested, Found,
    /// Cloning, Cancelled}` state then it will transition to the
    /// `Cancelled` state.
    ///
    /// # Errors
    ///
    ///   * If the `urn` was not in the `WaitingRoom`.
    ///   * If the underlying `Request` was not in the expected state.
    pub fn canceled(&mut self, urn: &Urn, timestamp: T) -> Result<(), Error>
    where
        T: Clone,
    {
        self.transition(
            |request| request.cancel(timestamp).right(),
            Either::Right,
            urn,
        )
    }

    /// Return the list of all `Urn`/`SomeRequest` pairs in the `WaitingRoom`.
    pub fn iter(&self) -> impl Iterator<Item = (Urn, &SomeRequest<T>)> {
        self.requests
            .iter()
            .map(|(id, request)| (Urn::new(*id), request))
    }

    /// Filter the `WaitingRoom` by:
    ///   * Choosing which [`RequestState`] you are looking for
    pub fn filter_by_state(
        &self,
        request_state: RequestState,
    ) -> impl Iterator<Item = (Urn, &SomeRequest<T>)> {
        self.iter()
            .filter(move |(_, request)| RequestState::from(*request) == request_state.clone())
    }

    /// Find the first occurring request based on the call to
    /// [`WaitingRoom::filter_by_state`].
    pub fn find_by_state(&self, request_state: RequestState) -> Option<(Urn, &SomeRequest<T>)> {
        self.filter_by_state(request_state).next()
    }

    /// Get the next `Request` that is in a query state, i.e. `Created` or
    /// `Requested`.
    ///
    /// In the case of the `Requested` state we check if:
    ///   * The request is a fresh request that hasn't had an attempt to clone yet
    ///   * Or the elapsed time between the `timestamp` and the `Request`'s timestamp is greater
    ///     than the `delta` provided in the [`Config`].
    pub fn next_query(&self, timestamp: T) -> Option<Urn>
    where
        T: Add<D, Output = T> + PartialOrd + Clone,
        D: Mul<u32, Output = D> + Ord + Clone,
    {
        let backoff = |tries: Queries| match tries {
            Queries::Max(i) => self.config.delta.clone() * u32::try_from(i).unwrap_or(u32::MAX),
            Queries::Infinite => self.config.delta.clone(),
        };
        let created = self.find_by_state(RequestState::Created);
        let requested = self
            .filter_by_state(RequestState::Requested)
            .find(move |(_, request)| {
                request.timestamp().clone() + backoff(request.attempts().queries) <= timestamp
            });

        created.or(requested).map(|(urn, _request)| urn)
    }

    /// Get the next `Request` that is in the the `Found` state and the status
    /// of the peer is `Available`.
    pub fn next_clone(&self) -> Option<(Urn, PeerId)> {
        self.find_by_state(RequestState::Found)
            .and_then(|(urn, request)| match request {
                SomeRequest::Found(request) => {
                    request.iter().find_map(|(peer_id, status)| match status {
                        Status::Available => Some((urn.clone(), *peer_id)),
                        _ => None,
                    })
                },
                _ => None,
            })
    }
}

impl<T, D> WaitingRoom<T, D>
where
    T: Clone,
{
    /// The state of requests currently in the waiting room
    pub fn requests(&self) -> HashMap<Revision, SomeRequest<T>> {
        self.requests.clone()
    }
}
