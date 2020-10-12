//! Inspect state and perform actions on a running peer.

use std::time::Instant;

use tokio::sync::{mpsc, oneshot};

use librad::uri::RadUrn;

use crate::request;

/// Requests sent to the peer.
#[derive(Debug)]
pub enum Request {
    /// Request a urn to be fetched from the network.
    Urn(
        RadUrn,
        Instant,
        oneshot::Sender<Option<request::SomeRequest<Instant>>>,
    ),
}

/// Returned responses from the peer.
#[derive(Debug)]
pub enum Response {
    /// Response to a urn request.
    Urn(
        oneshot::Sender<Option<request::SomeRequest<Instant>>>,
        Option<request::SomeRequest<Instant>>,
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

    /// Initiate a new request for the `urn`.
    pub async fn request_urn(
        &mut self,
        urn: &RadUrn,
        time: Instant,
    ) -> Option<request::SomeRequest<Instant>> {
        let (sender, receiver) = oneshot::channel::<Option<request::SomeRequest<Instant>>>();

        self.sender
            .send(Request::Urn(urn.clone(), time, sender))
            .await
            .expect("peer is gone");

        receiver.await.expect("receiver is gone")
    }
}
