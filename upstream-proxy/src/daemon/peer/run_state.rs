// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! State machine to manage the current mode of operation during peer lifecycle.

use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use serde::Serialize;

use librad::{
    git::Urn,
    net::{
        self,
        peer::{PeerInfo, ProtocolEvent},
        protocol::{
            broadcast::PutResult,
            event::{downstream, upstream},
            gossip::Payload,
        },
    },
    PeerId,
};

use crate::daemon::{
    convert::MaybeFrom,
    peer::{announcement, control},
    request::waiting_room::{self, WaitingRoom},
};

pub mod command;
pub use command::Command;

pub mod config;
pub use config::Config;

pub mod input;
pub use input::Input;

mod running_waiting_room;
pub use running_waiting_room::Event as WaitingRoomEvent;
use running_waiting_room::{RunningWaitingRoom, WaitingRoomTransition};

/// Events external subscribers can observe for internal peer operations.
#[derive(Clone, Debug)]
pub enum Event {
    /// Announcement subroutine completed and emitted the enclosed updates.
    Announced(announcement::Updates),
    /// A fetch originated by a gossip message succeeded
    GossipFetched {
        /// Provider of the fetched update.
        provider: PeerInfo<SocketAddr>,
        /// Cooresponding gossip message.
        gossip: Payload,
        /// Result of the storage fetch.
        result: PutResult<Payload>,
    },
    /// An event from the underlying coco network stack.
    /// FIXME(xla): Align variant naming to indicate observed occurrences.
    Protocol(ProtocolEvent),
    /// Request fullfilled with a successful clone.
    RequestCloned(Urn, PeerId),
    /// Request is being cloned from a peer.
    RequestCloning(Urn, PeerId),
    /// Request for the URN was created and is pending submission to the
    /// network.
    RequestCreated(Urn),
    /// Request for the URN was submitted to the network.
    RequestQueried(Urn),
    /// Waiting room interval ticked.
    RequestTick,
    /// The request for [`Urn`] timed out.
    RequestTimedOut(Urn),
    /// The [`Status`] of the peer changed.
    StatusChanged {
        /// The old status
        old: Status,
        /// The net status
        new: Status,
    },
    /// A state change occurred in the waiting room
    WaitingRoomTransition(WaitingRoomTransition<SystemTime>),
}

impl From<WaitingRoomTransition<SystemTime>> for Event {
    fn from(transition: WaitingRoomTransition<SystemTime>) -> Self {
        Self::WaitingRoomTransition(transition)
    }
}

impl MaybeFrom<&Input> for Event {
    fn maybe_from(input: &Input) -> Option<Self> {
        match input {
            Input::Announce(input::Announce::Succeeded(updates)) => {
                Some(Self::Announced(updates.clone()))
            },
            Input::Protocol(protocol_event) => match protocol_event {
                ProtocolEvent::Gossip(gossip) => match &**gossip {
                    upstream::Gossip::Put {
                        provider,
                        payload,
                        result,
                    } => Some(Self::GossipFetched {
                        provider: provider.clone(),
                        gossip: payload.clone(),
                        result: result.clone(),
                    }),
                },
                event => Some(Self::Protocol(event.clone())),
            },
            Input::Request(input::Request::Cloned(urn, remote_peer)) => {
                Some(Self::RequestCloned(urn.clone(), *remote_peer))
            },
            Input::Request(input::Request::Cloning(urn, remote_peer)) => {
                Some(Self::RequestCloning(urn.clone(), *remote_peer))
            },
            Input::Request(input::Request::Queried(urn)) => Some(Self::RequestQueried(urn.clone())),
            Input::Request(input::Request::Tick) => Some(Self::RequestTick),
            Input::Request(input::Request::TimedOut(urn)) => {
                Some(Self::RequestTimedOut(urn.clone()))
            },
            _ => None,
        }
    }
}

/// The current status of the local peer and its relation to the network.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Status {
    /// Nothing is setup, not even a socket to listen on.
    Stopped,
    /// Local peer is listening on a socket but has not connected to any peers
    /// yet.
    Started,
    /// The local peer lost its connections to all its peers.
    Offline,
    /// The local peer is operational and is able to interact with the peers it
    /// has connected to.
    #[serde(rename_all = "camelCase")]
    Online {
        /// Connected peers
        connected_peers: HashMap<PeerId, Vec<SocketAddr>>,
    },
}

/// State kept for a running local peer.
pub struct RunState {
    listen_addrs: Vec<SocketAddr>,
    /// Current internal status.
    pub status: Status,
    stats: net::protocol::event::downstream::Stats,
    /// Current set of requests.
    waiting_room: RunningWaitingRoom,
}

impl RunState {
    /// Creates a new `RunState` initialising it with the provided `config` and
    /// `waiting_room`.
    pub fn new(waiting_room: WaitingRoom<SystemTime, Duration>) -> Self {
        Self {
            listen_addrs: vec![],
            stats: downstream::Stats::default(),
            status: Status::Stopped,
            waiting_room: RunningWaitingRoom::new(waiting_room),
        }
    }

    /// Applies the `input` and based on the current state, transforms to the
    /// new state and in some cases produes commands which should be
    /// executed in the appropriate subroutines.
    pub fn transition(&mut self, input: Input) -> Vec<Command> {
        tracing::trace!(?input, status = ?self.status, "transition start");

        let cmds = match input {
            Input::Announce(announce_input) => self.handle_announce(announce_input),
            Input::Control(control_input) => self.handle_control(control_input),
            Input::ListenAddrs(addrs) => self.handle_listen_addrs(addrs),
            Input::Protocol(protocol_event) => self.handle_protocol(protocol_event),
            Input::Request(request_input) => self.handle_request(request_input),
            Input::Stats(stats_input) => self.handle_stats(stats_input),
        };

        tracing::trace!(?cmds, status = ?self.status, "transition end");

        cmds
    }

    /// Handle [`input::Announce`]s.
    fn handle_announce(&mut self, input: input::Announce) -> Vec<Command> {
        match (&self.status, input) {
            // Announce new updates while the peer is online.
            (Status::Online { .. } | Status::Started { .. }, input::Announce::Tick)
                if !self.stats.connected_peers.is_empty() && self.stats.membership_active > 0 =>
            {
                vec![Command::Announce]
            },
            _ => vec![],
        }
    }

    /// Handle [`input::Control`]s.
    fn handle_control(&mut self, input: input::Control) -> Vec<Command> {
        match input {
            input::Control::CancelRequest(urn, timestamp, sender) => {
                self.waiting_room.cancel(urn, timestamp, sender)
            },
            input::Control::CreateRequest(urn, time, sender) => {
                self.waiting_room.request(urn, time, sender)
            },
            input::Control::ListRequests(sender) => vec![Command::Control(
                command::Control::Respond(control::Response::ListSearches(
                    sender,
                    self.waiting_room
                        .iter()
                        .map(|pair| pair.1.clone())
                        .collect::<Vec<_>>(),
                )),
            )],
            input::Control::ListenAddrs(sender) => {
                vec![Command::Control(command::Control::Respond(
                    control::Response::ListenAddrs(sender, self.listen_addrs.clone()),
                ))]
            },
            input::Control::Status(sender) => vec![Command::Control(command::Control::Respond(
                control::Response::CurrentStatus(sender, self.status.clone()),
            ))],
        }
    }

    fn handle_listen_addrs(&mut self, addrs: Vec<SocketAddr>) -> Vec<Command> {
        self.listen_addrs = addrs;
        vec![]
    }

    /// Handle [`ProtocolEvent`]s.
    fn handle_protocol(&mut self, event: ProtocolEvent) -> Vec<Command> {
        match (&self.status, event) {
            (Status::Stopped, ProtocolEvent::Endpoint(upstream::Endpoint::Up { .. })) => {
                self.status = Status::Started;

                vec![]
            },
            (_, ProtocolEvent::Endpoint(upstream::Endpoint::Down)) => {
                self.status = Status::Stopped;

                vec![]
            },
            (_, ProtocolEvent::Gossip(gossip)) => {
                let mut cmds = vec![];

                match *gossip {
                    // FIXME(xla): Find out if we care about the result variance.
                    upstream::Gossip::Put {
                        payload: Payload { urn, .. },
                        provider: PeerInfo { peer_id, .. },
                        result,
                    } => {
                        if self.waiting_room.get(&urn).is_some() {
                            cmds.extend(self.waiting_room.found(&urn, peer_id, SystemTime::now()));
                        }

                        if let PutResult::Applied(_) = result {
                            cmds.push(Command::Include(urn));
                        }
                    },
                }

                cmds
            },
            _ => vec![],
        }
    }

    /// Handle [`input::Request`]s.
    fn handle_request(&mut self, input: input::Request) -> Vec<Command> {
        match (&self.status, input) {
            // Check for new query and clone requests.
            (Status::Online { .. }, input::Request::Tick) => {
                self.waiting_room.tick(SystemTime::now())
            },
            (_, input::Request::Cloning(urn, remote_peer)) => {
                self.waiting_room
                    .cloning(&urn, remote_peer, SystemTime::now())
            },
            (_, input::Request::Cloned(urn, remote_peer)) => {
                self.waiting_room
                    .cloned(&urn, remote_peer, SystemTime::now())
            },
            (_, input::Request::Queried(urn)) => self.waiting_room.queried(&urn, SystemTime::now()),
            (
                _,
                input::Request::Failed {
                    remote_peer,
                    reason,
                    urn,
                },
            ) => {
                tracing::warn!(?reason, "cloning failed");
                self.waiting_room
                    .cloning_failed(&urn, remote_peer, SystemTime::now(), reason)
            },
            _ => vec![],
        }
    }

    fn handle_stats(&mut self, input: input::Stats) -> Vec<Command> {
        match (&self.status, input) {
            (_, input::Stats::Tick) => vec![Command::Stats],
            (status, input::Stats::Values(stats)) => {
                match status {
                    Status::Online { .. } if stats.connected_peers.is_empty() => {
                        self.status = Status::Offline;
                    },
                    Status::Offline if !stats.connected_peers.is_empty() => {
                        self.status = Status::Online {
                            connected_peers: stats.connected_peers.clone(),
                        };
                    },
                    Status::Started if !stats.connected_peers.is_empty() => {
                        self.status = Status::Online {
                            connected_peers: stats.connected_peers.clone(),
                        };
                    },
                    _ => {},
                };

                self.stats = stats;

                vec![]
            },
        }
    }
}
