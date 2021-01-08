//! The enumeration of different [`super::Request`] states unified under a single enum called
//! [`SomeRequest`].

// I reserve the right to not match all the arms when picking out particular cases, thank you very
// much.
#![allow(clippy::wildcard_enum_match_arm)]

use serde::{Deserialize, Serialize};

use super::{
    Attempts, Cancelled, Cloned, Clones, Cloning, Created, Either, Found, Queries, Request,
    RequestState, Requested, TimedOut, Urn,
};

/// Since a `Request` is parameterised over its state, it makes it difficult to talk about a
/// `Request` in general without the compiler complaining at us. For example, we cannot have
/// something like `vec![created, requested, cloning, timedout]` since they all have distinct types
/// where they differ in states.
///
/// To allow us to do this we unify all the states into `SomeRequest` where each state is a variant
/// in the enumeration.
///
/// When we pattern match we get back the request parameterised over the specific state and can
/// work in a type safe manner with this request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    bound = "T: serde_millis::Milliseconds",
    rename_all = "camelCase",
    tag = "type"
)]
pub enum SomeRequest<T> {
    /// The `Request` has been created.
    Created(Request<Created, T>),

    /// The `Request` has been requested.
    Requested(Request<Requested, T>),

    /// The `Request` has found a peer and is possibly searching for more.
    Found(Request<Found, T>),

    /// The `Request` is attempting to clone from a peer.
    Cloning(Request<Cloning, T>),

    /// The `Request` has successfully cloned from a peer.
    Cloned(Request<Cloned, T>),

    /// The `Request` has been cancelled.
    Cancelled(Request<Cancelled, T>),

    /// The `Request` has timed out on querying or cloning.
    TimedOut(Request<TimedOut, T>),
}

impl<T> From<&SomeRequest<T>> for RequestState {
    fn from(other: &SomeRequest<T>) -> RequestState {
        match other {
            SomeRequest::Created(_) => Self::Created,
            SomeRequest::Requested(_) => Self::Requested,
            SomeRequest::Found(_) => Self::Found,
            SomeRequest::Cloning(_) => Self::Cloning,
            SomeRequest::Cloned(_) => Self::Cloned,
            SomeRequest::Cancelled(_) => Self::Cancelled,
            SomeRequest::TimedOut(_) => Self::TimedOut,
        }
    }
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

impl<T> From<Request<Cancelled, T>> for SomeRequest<T> {
    fn from(request: Request<Cancelled, T>) -> Self {
        Self::Cancelled(request)
    }
}

impl<T> From<Request<TimedOut, T>> for SomeRequest<T> {
    fn from(request: Request<TimedOut, T>) -> Self {
        Self::TimedOut(request)
    }
}

impl<T, L: Into<SomeRequest<T>>, R: Into<SomeRequest<T>>> From<Either<L, R>> for SomeRequest<T> {
    fn from(other: Either<L, R>) -> Self {
        other.either(L::into, R::into)
    }
}

impl<T> SomeRequest<T> {
    /// Get the `Urn` of whatever kind of [`Request`] is below.
    pub const fn urn(&self) -> &Urn {
        match self {
            SomeRequest::Created(request) => request.urn(),
            SomeRequest::Requested(request) => request.urn(),
            SomeRequest::Found(request) => request.urn(),
            SomeRequest::Cloning(request) => request.urn(),
            SomeRequest::Cloned(request) => request.urn(),
            SomeRequest::Cancelled(request) => request.urn(),
            SomeRequest::TimedOut(request) => request.urn(),
        }
    }

    /// Get the [`Attempts`] of whatever kind of [`Request`] is below.
    pub const fn attempts(&self) -> &Attempts {
        match self {
            SomeRequest::Created(request) => &request.attempts,
            SomeRequest::Requested(request) => &request.attempts,
            SomeRequest::Found(request) => &request.attempts,
            SomeRequest::Cloning(request) => &request.attempts,
            SomeRequest::Cloned(request) => &request.attempts,
            SomeRequest::Cancelled(request) => &request.attempts,
            SomeRequest::TimedOut(request) => &request.attempts,
        }
    }

    /// Get the current timestamp of the underlying `Request`.
    pub const fn timestamp(&self) -> &T {
        match self {
            SomeRequest::Created(request) => request.timestamp(),
            SomeRequest::Requested(request) => request.timestamp(),
            SomeRequest::Found(request) => request.timestamp(),
            SomeRequest::Cloning(request) => request.timestamp(),
            SomeRequest::Cloned(request) => request.timestamp(),
            SomeRequest::Cancelled(request) => request.timestamp(),
            SomeRequest::TimedOut(request) => request.timestamp(),
        }
    }

    /// We can cancel an underlying `Request` if it is allowed to be cancelled. In the case that it
    /// is allowed, then we get back the cancelled request in the `Right` variant. Otherwise we get
    /// back our original `SomeRequest` in the `Left` variant.
    pub fn cancel(self, timestamp: T) -> Either<SomeRequest<T>, Request<Cancelled, T>> {
        match self {
            SomeRequest::Created(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Requested(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Found(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Cloning(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Cancelled(request) => Either::Right(request.cancel(timestamp)),
            request => Either::Left(request),
        }
    }

    /// We can see if our underlying `Request` timed out if it is in a state where a time out can
    /// occur. In the case that it can time out, then we get back the timed out request in the
    /// `Right` variant. Otherwise we get back our original `SomeRequest` in the `Left` variant.
    pub fn timed_out(
        self,
        max_queries: Queries,
        max_clones: Clones,
        timestamp: T,
    ) -> Either<SomeRequest<T>, Request<TimedOut, T>> {
        match self {
            SomeRequest::Requested(request) => request
                .timed_out(max_queries, max_clones, timestamp)
                .map_left(SomeRequest::Requested),
            SomeRequest::Found(request) => request
                .timed_out(max_queries, max_clones, timestamp)
                .map_left(SomeRequest::Found),
            SomeRequest::Cloning(request) => request
                .timed_out(max_queries, max_clones, timestamp)
                .map_left(SomeRequest::Cloning),
            request => Either::Left(request),
        }
    }

    /// If we have some way of picking a specific `Request` from `SomeRequest` and a function that
    /// transitions that `Request` into a next state then we follow that transition.
    ///
    /// If not we leave the `SomeRequest` as is.
    pub fn transition<Prev, Next>(
        self,
        matcher: impl FnOnce(SomeRequest<T>) -> Option<Prev>,
        transition: impl FnOnce(Prev) -> Next,
    ) -> Either<SomeRequest<T>, Next>
    where
        T: Clone,
    {
        match matcher(self.clone()) {
            Some(previous) => Either::Right(transition(previous)),
            None => Either::Left(self),
        }
    }
}
