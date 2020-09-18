use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
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

    pub fn cancel(self, timestamp: T) -> Either<SomeRequest<T>, Request<IsCanceled, T>> {
        match self {
            SomeRequest::Created(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Requested(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Found(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Cloning(request) => Either::Right(request.cancel(timestamp)),
            SomeRequest::Canceled(request) => Either::Right(request.cancel(timestamp)),
            request => Either::Left(request),
        }
    }

    pub fn timed_out(
        self,
        max_queries: usize,
        max_clones: usize,
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
