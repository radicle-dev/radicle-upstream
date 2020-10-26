//! Inspect state and perform actions on a running peer.

use std::time::Instant;

use either::Either;
use tokio::sync::{mpsc, oneshot};

use librad::uri::RadUrn;

use crate::{request, request::waiting_room};

use super::run_state::Status;

/// Requests sent to the peer.
#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum Request {
    /// Request the current status.
    CurrentStatus(oneshot::Sender<Status>),
    /// Request a project request.
    GetProjectRequest(
        RadUrn,
        oneshot::Sender<Option<request::SomeRequest<Instant>>>,
    ),
    /// Request the current project requests.
    GetProjectRequests(oneshot::Sender<Vec<request::SomeRequest<Instant>>>),
    /// Request a urn to be fetched from the network.
    Urn(
        RadUrn,
        Instant,
        oneshot::Sender<waiting_room::Created<Instant>>,
    ),
}

/// Returned responses from the peer.
#[derive(Debug)]
pub enum Response {
    /// Response to a status request.
    CurrentStatus(oneshot::Sender<Status>, Status),
    /// Response to get project request request.
    GetProjectRequest(
        oneshot::Sender<Option<request::SomeRequest<Instant>>>,
        Option<request::SomeRequest<Instant>>,
    ),
    /// Response to list project requests request.
    GetProjectRequests(
        oneshot::Sender<Vec<request::SomeRequest<Instant>>>,
        Vec<request::SomeRequest<Instant>>,
    ),
    /// Response to a urn request.
    Urn(
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

    /// Initiate a new reuest to fetch a project request.
    pub async fn get_project_request(
        &mut self,
        urn: &RadUrn,
    ) -> Option<request::SomeRequest<Instant>> {
        let (sender, receiver) = oneshot::channel::<Option<request::SomeRequest<Instant>>>();

        self.sender
            .send(Request::GetProjectRequest(urn.clone(), sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Initiate a new reuest for the list of existing project requests.
    pub async fn get_project_requests(&mut self) -> Vec<request::SomeRequest<Instant>> {
        let (sender, receiver) = oneshot::channel::<Vec<request::SomeRequest<Instant>>>();

        self.sender
            .send(Request::GetProjectRequests(sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }

    /// Initiate a new request for the `urn`.
    pub async fn request_urn(
        &mut self,
        urn: &RadUrn,
        time: Instant,
    ) -> request::SomeRequest<Instant> {
        let (sender, receiver) = oneshot::channel::<waiting_room::Created<Instant>>();

        self.sender
            .send(Request::Urn(urn.clone(), time, sender))
            .await
            .expect("peer is gone");

        match receiver.await.expect("receiver is gone") {
            Either::Left(req) | Either::Right(req) => req,
        }
    }
}
