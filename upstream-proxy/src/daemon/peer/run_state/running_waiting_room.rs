// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

use either::Either;
use librad::{git::Urn, identities::Revision, PeerId};
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use crate::daemon::request::SomeRequest;

use super::{
    command, control, waiting_room::Error as WaitingRoomError, Command, Event as RunStateEvent,
    WaitingRoom,
};
use tokio::sync::oneshot::Sender;

use serde::Serialize;

/// Events that can affect the state of the waiting room
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Event {
    /// A request was created for a urn
    Created {
        /// The urn bein requested
        urn: Urn,
    },
    /// A query was initiated for a urn
    Queried {
        /// The urn bein queried
        urn: Urn,
    },
    /// A peer was found who claims to have a urn
    Found {
        /// The urn that was found
        urn: Urn,
        /// The peer who claims to have it
        peer: PeerId,
    },
    /// Cloning was initiated for a urn and peer
    Cloning {
        /// The urn we are cloning
        urn: Urn,
        /// The peer we are cloning from
        peer: PeerId,
    },
    /// Cloning failed for a urn and peer
    CloningFailed {
        /// The urn that failed
        urn: Urn,
        /// The peer we failed to clone from
        peer: PeerId,
        /// A description of why the cloning failed
        reason: String,
    },
    /// Cloning succeeded for a urn and peer
    Cloned {
        /// The urn we cloned
        urn: Urn,
        /// The peer we cloned from
        peer: PeerId,
    },
    /// A request for a urn was canceled
    Canceled {
        /// The urn that was canceled
        urn: Urn,
    },
    /// A request was timed out
    TimedOut {
        /// The urn that timed out
        urn: Urn,
        /// The attempts that were made before the timeout
        attempts: Option<usize>,
    },
}

/// `RunningWaitingRoom` is an adapter from the interface of `WaitingRoom` to
/// the language of commands which is spoken by `RunState`. Whenever `RunState`
/// needs to talk to `WaitingRoom` it does so via a wrapper method on
/// `RunningWaitingRoom`. These wrapper methods contain the logic to convert
/// the values  returned by `WaitingRoom` methods into `Vec<Command>`.
pub(super) struct RunningWaitingRoom {
    waiting_room: WaitingRoom<SystemTime, Duration>,
}

impl RunningWaitingRoom {
    pub const fn new(waiting_room: WaitingRoom<SystemTime, Duration>) -> Self {
        Self { waiting_room }
    }

    pub fn cancel(
        &mut self,
        urn: Urn,
        timestamp: SystemTime,
        sender: Sender<Result<Option<SomeRequest<SystemTime>>, WaitingRoomError>>,
    ) -> Vec<Command> {
        let state_before = self.waiting_room.requests();
        match self.waiting_room.canceled(&urn, timestamp) {
            Ok(()) => {
                let request = self.waiting_room.remove(&urn);
                let state_after = self.waiting_room.requests();
                let transition = WaitingRoomTransition {
                    timestamp,
                    state_before,
                    state_after,
                    event: Event::Canceled { urn },
                };
                vec![
                    Command::Control(command::Control::Respond(control::Response::CancelSearch(
                        sender,
                        Ok(request),
                    ))),
                    Command::PersistWaitingRoom(self.waiting_room.clone()),
                    Command::EmitEvent(transition.into()),
                ]
            },
            Err(e) => vec![
                Command::Control(command::Control::Respond(control::Response::CancelSearch(
                    sender,
                    Err(e),
                ))),
                Command::PersistWaitingRoom(self.waiting_room.clone()),
            ],
        }
    }

    pub fn request(
        &mut self,
        urn: Urn,
        timestamp: SystemTime,
        sender: Sender<Either<SomeRequest<SystemTime>, SomeRequest<SystemTime>>>,
    ) -> Vec<Command> {
        let state_before = self.waiting_room.requests();
        let request = self.waiting_room.request(&urn, timestamp);
        let state_after = self.waiting_room.requests();
        match request {
            Either::Left(request) => {
                let transition = WaitingRoomTransition {
                    timestamp,
                    state_before,
                    state_after,
                    event: Event::Created { urn: urn.clone() },
                };
                vec![
                    Command::Control(command::Control::Respond(control::Response::StartSearch(
                        sender,
                        Either::Left(request),
                    ))),
                    Command::EmitEvent(RunStateEvent::RequestCreated(urn)),
                    Command::EmitEvent(transition.into()),
                ]
            },
            Either::Right(request) => vec![
                Command::Control(command::Control::Respond(control::Response::StartSearch(
                    sender,
                    Either::Right(request),
                ))),
                Command::EmitEvent(RunStateEvent::RequestCreated(urn)),
            ],
        }
    }

    pub fn get(&self, urn: &Urn) -> Option<&SomeRequest<SystemTime>> {
        self.waiting_room.get(urn)
    }

    /// Return the list of all `Urn`/`SomeRequest` pairs in the `WaitingRoom`.
    pub fn iter(&self) -> impl Iterator<Item = (Urn, &SomeRequest<SystemTime>)> {
        self.waiting_room.iter()
    }

    pub fn found(&mut self, urn: &Urn, remote_peer: PeerId, timestamp: SystemTime) -> Vec<Command> {
        Self::simple_command_helper(
            &mut self.waiting_room,
            urn,
            Event::Found {
                urn: urn.clone(),
                peer: remote_peer,
            },
            timestamp,
            |w| w.found(urn, remote_peer, timestamp),
        )
    }

    /// Issue "query" and "clone" requests for requests that are next in the
    /// queue.
    pub fn tick(&mut self, timestamp: SystemTime) -> Vec<Command> {
        let mut cmds = Vec::with_capacity(2);

        if let Some(urn) = self.waiting_room.next_query(timestamp) {
            cmds.push(Command::Request(command::Request::Query(urn)));
        }
        if let Some((urn, remote_peer)) = self.waiting_room.next_clone() {
            cmds.push(Command::Request(command::Request::Clone(urn, remote_peer)));
        }

        cmds
    }

    pub fn cloning(
        &mut self,
        urn: &Urn,
        remote_peer: PeerId,
        timestamp: SystemTime,
    ) -> Vec<Command> {
        Self::simple_command_helper(
            &mut self.waiting_room,
            urn,
            Event::Cloning {
                urn: urn.clone(),
                peer: remote_peer,
            },
            timestamp,
            |w| w.cloning(urn, remote_peer, timestamp),
        )
    }

    pub fn cloned(
        &mut self,
        urn: &Urn,
        remote_peer: PeerId,
        timestamp: SystemTime,
    ) -> Vec<Command> {
        Self::simple_command_helper(
            &mut self.waiting_room,
            urn,
            Event::Cloned {
                urn: urn.clone(),
                peer: remote_peer,
            },
            timestamp,
            |w| w.cloned(urn, remote_peer, timestamp),
        )
    }

    pub fn queried(&mut self, urn: &Urn, timestamp: SystemTime) -> Vec<Command> {
        Self::simple_command_helper(
            &mut self.waiting_room,
            urn,
            Event::Queried { urn: urn.clone() },
            timestamp,
            |w| w.queried(urn, timestamp),
        )
    }

    pub fn cloning_failed(
        &mut self,
        urn: &Urn,
        remote_peer: PeerId,
        timestamp: SystemTime,
        reason: Box<dyn std::error::Error>,
    ) -> Vec<Command> {
        Self::simple_command_helper(
            &mut self.waiting_room,
            urn,
            Event::CloningFailed {
                urn: urn.clone(),
                peer: remote_peer,
                reason: reason.to_string(),
            },
            timestamp,
            |w| w.cloning_failed(urn, remote_peer, timestamp, reason),
        )
    }

    fn simple_command_helper<F>(
        waiting_room: &mut WaitingRoom<SystemTime, Duration>,
        urn: &Urn,
        event: Event,
        timestamp: SystemTime,
        f: F,
    ) -> Vec<Command>
    where
        F: FnOnce(&mut WaitingRoom<SystemTime, Duration>) -> Result<(), WaitingRoomError>,
    {
        let state_before = waiting_room.requests();
        let result = f(waiting_room);
        let state_after = waiting_room.requests();
        let mut commands = Vec::with_capacity(4);
        commands.push(Command::PersistWaitingRoom(waiting_room.clone()));
        match result {
            Ok(()) => {
                commands.push(Command::EmitEvent(
                    WaitingRoomTransition {
                        timestamp,
                        state_before,
                        state_after,
                        event,
                    }
                    .into(),
                ));
                commands
            },
            Err(WaitingRoomError::TimeOut { attempts, .. }) => {
                commands.push(Command::EmitEvent(
                    WaitingRoomTransition {
                        timestamp,
                        state_before,
                        state_after,
                        event: Event::TimedOut {
                            urn: urn.clone(),
                            attempts,
                        },
                    }
                    .into(),
                ));
                commands.push(Command::Request(command::Request::TimedOut(urn.clone())));
                commands
            },
            // FIXME(alexjg): Figure out how to report this error to the client
            Err(error) => {
                tracing::warn!(?error, "waiting room error");
                Vec::new()
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct WaitingRoomTransition<T> {
    pub timestamp: T,
    pub state_before: HashMap<Revision, SomeRequest<T>>,
    pub state_after: HashMap<Revision, SomeRequest<T>>,
    pub event: Event,
}
