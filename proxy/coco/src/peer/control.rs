//! Inspect state and perform actions on a running peer.

use std::time::Instant;

use either::Either;
use tokio::sync::{mpsc, oneshot};

use librad::identities::Urn;

use crate::{request, request::waiting_room};

use super::run_state::Status;

/// Requests sent to the peer.
#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Request {
    /// Request the current peer status.
    CurrentStatus(oneshot::Sender<Status>),

    /// Cancel an ongoing project search.
    CancelSearch(
        Urn,
        Instant,
        oneshot::Sender<Result<Option<request::SomeRequest<Instant>>, waiting_room::Error>>,
    ),
    /// Get a project search.
    GetSearch(Urn, oneshot::Sender<Option<request::SomeRequest<Instant>>>),
    /// List all project searches.
    ListSearches(oneshot::Sender<Vec<request::SomeRequest<Instant>>>),
    /// Initiate a search for a project on the network.
    StartSearch(
        Urn,
        Instant,
        oneshot::Sender<waiting_room::Created<Instant>>,
    ),
}

/// Returned responses from the peer.
#[derive(Debug)]
pub enum Response {
    /// Response to a status request.
    CurrentStatus(oneshot::Sender<Status>, Status),

    /// Response to a cancel project search request.
    CancelSearch(
        oneshot::Sender<Result<Option<request::SomeRequest<Instant>>, waiting_room::Error>>,
        Result<Option<request::SomeRequest<Instant>>, waiting_room::Error>,
    ),
    /// Response to get project search request.
    GetSearch(
        oneshot::Sender<Option<request::SomeRequest<Instant>>>,
        Option<request::SomeRequest<Instant>>,
    ),
    /// Response to list project searches request.
    ListSearches(
        oneshot::Sender<Vec<request::SomeRequest<Instant>>>,
        Vec<request::SomeRequest<Instant>>,
    ),
    /// Response to a start project search request.
    StartSearch(
        oneshot::Sender<waiting_room::Created<Instant>>,
        waiting_room::Created<Instant>,
    ),
}

/// A handle to inspect state and perform actions on a running peer.
#[derive(Clone)]
pub struct Control {
    /// Channel to send requests to the peer.
    sender: mpsc::Sender<Request>,
}

impl Control {
    /// Construct a new [`Control`] handle.
    #[must_use = "take control"]
    pub const fn new(sender: mpsc::Sender<Request>) -> Self {
        Self { sender }
    }

    /// Initiate a new request for the status.
    pub async fn current_status(&mut self) -> Status {
        let (sender, receiver) = oneshot::channel::<Status>();

        self.sender
            .send(Request::CurrentStatus(sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Cancel an ongoing search for a project.
    ///
    /// # Errors
    ///
    /// * if the waiting room returns an error
    pub async fn cancel_project_request(
        &mut self,
        urn: &Urn,
        timestamp: Instant,
    ) -> Result<Option<request::SomeRequest<Instant>>, waiting_room::Error> {
        let (sender, receiver) = oneshot::channel();

        self.sender
            .send(Request::CancelSearch(urn.clone(), timestamp, sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Initiate a new request to fetch a project from the network.
    pub async fn get_project_request(
        &mut self,
        urn: &Urn,
    ) -> Option<request::SomeRequest<Instant>> {
        let (sender, receiver) = oneshot::channel::<Option<request::SomeRequest<Instant>>>();

        self.sender
            .send(Request::GetSearch(urn.clone(), sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Initiate a new reuest for the list of existing project requests.
    pub async fn get_project_requests(&mut self) -> Vec<request::SomeRequest<Instant>> {
        let (sender, receiver) = oneshot::channel::<Vec<request::SomeRequest<Instant>>>();

        self.sender
            .send(Request::ListSearches(sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Initiate a new request for the `urn`.
    pub async fn request_project(
        &mut self,
        urn: &Urn,
        timestamp: Instant,
    ) -> request::SomeRequest<Instant> {
        let (sender, receiver) = oneshot::channel::<waiting_room::Created<Instant>>();

        self.sender
            .send(Request::StartSearch(urn.clone(), timestamp, sender))
            .await
            .expect("peer is gone");

        match receiver.await.expect("receiver is gone") {
            Either::Left(req) | Either::Right(req) => req,
        }
    }
}
