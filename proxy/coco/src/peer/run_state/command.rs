use std::time::{Duration, Instant};

use librad::{identities::Urn, peer::PeerId};

use crate::{peer::control, request::waiting_room::WaitingRoom};

/// Instructions to issue side-effectful operations which are the results from state transitions.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Command {
    /// Start the announcement subroutine.
    Announce,
    /// Answer control requests.
    Control(Control),
    /// Update the include file for the provided `RadUrn`.
    Include(Urn),
    /// Tell the subroutine to persist the `WaitingRoom`.
    PersistWaitingRoom(WaitingRoom<Instant, Duration>),
    /// Fulfill request commands.
    Request(Request),
    /// Initiate a full sync with `PeerId`.
    SyncPeer(PeerId),
    /// Start sync timeout.
    StartSyncTimeout(Duration),
    /// Emit an external event to all subscribers
    EmitEvent(super::Event),
}

/// Reactions for incoming control requests.
#[derive(Debug)]
pub enum Control {
    /// Send a response corresponding to a control request.
    Respond(control::Response),
}

/// Commands issued when requesting an identity from the network.
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Tell the subroutine to attempt a clone from the given `RadUrl`.
    Clone(Urn, PeerId),
    /// Tell the subroutine that we should query for the given `RadUrn` on the network.
    Query(Urn),
    /// The request for [`RadUrn`] timed out.
    TimedOut(Urn),
}

impl From<Request> for Command {
    fn from(other: Request) -> Self {
        Self::Request(other)
    }
}
