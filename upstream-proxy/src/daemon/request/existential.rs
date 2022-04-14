// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! The enumeration of different [`super::Request`] states unified under a
//! single enum called [`SomeRequest`].
use librad::PeerId;
use serde::{Deserialize, Serialize};

use super::{
    Attempts, Cancelled, Cloned, Cloning, Created, Either, Found, Request, RequestState, Requested,
    TimedOut,
};

use super::Status;

/// Since a `Request` is parameterised over its state, it makes it difficult to
/// talk about a `Request` in general without the compiler complaining at us.
/// For example, we cannot have something like `vec![created, requested,
/// cloning, timedout]` since they all have distinct types where they differ in
/// states.
///
/// To allow us to do this we unify all the states into `SomeRequest` where each
/// state is a variant in the enumeration.
///
/// When we pattern match we get back the request parameterised over the
/// specific state and can work in a type safe manner with this request.
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

    /// We can cancel an underlying `Request` if it is allowed to be cancelled.
    /// In the case that it is allowed, then we get back the cancelled
    /// request in the `Right` variant. Otherwise we get back our original
    /// `SomeRequest` in the `Left` variant.
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

    /// If we have some way of picking a specific `Request` from `SomeRequest`
    /// and a function that transitions that `Request` into a next state
    /// then we follow that transition.
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

    /// Get any peers associated with this request
    pub fn peers(&self) -> Option<&std::collections::HashMap<PeerId, Status>> {
        match self {
            SomeRequest::Created(_)
            | SomeRequest::Requested(_)
            | SomeRequest::Cloned(_)
            | SomeRequest::Cancelled(_)
            | SomeRequest::TimedOut(..) => None,
            SomeRequest::Found(f) => Some(&f.peers),
            SomeRequest::Cloning(c) => Some(&c.peers),
        }
    }
}
