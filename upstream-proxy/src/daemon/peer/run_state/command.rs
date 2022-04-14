// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

use std::time::{Duration, SystemTime};

use librad::{git::Urn, PeerId};

use crate::daemon::{peer::control, request::waiting_room::WaitingRoom};

/// Instructions to issue side-effectful operations which are the results from
/// state transitions.
#[derive(Debug)]
pub enum Command {
    /// Start the announcement subroutine.
    Announce,
    /// Answer control requests.
    Control(Control),
    /// Update the include file for the provided [`Urn`].
    Include(Urn),
    /// Tell the subroutine to persist the [`WaitingRoom`].
    PersistWaitingRoom(WaitingRoom<SystemTime, Duration>),
    /// Fulfill request commands.
    Request(Request),
    Stats,
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
    /// Tell the subroutine to attempt a clone from the given [`Urn`] and
    /// [`PeerId`].
    Clone(Urn, PeerId),
    /// Tell the subroutine that we should query for the given [`Urn`] on the
    /// network.
    Query(Urn),
    /// The request for [`Urn`] timed out.
    TimedOut(Urn),
}

impl From<Request> for Command {
    fn from(other: Request) -> Self {
        Self::Request(other)
    }
}
