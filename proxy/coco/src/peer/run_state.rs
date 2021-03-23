//! State machine to manage the current mode of operation during peer lifecycle.

use std::{
    collections::HashSet,
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
    peer::PeerId,
};

use crate::{
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

/// Events external subscribers can observe for internal peer operations.
#[allow(clippy::large_enum_variant)]
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
    /// Sync with a peer completed.
    PeerSynced(PeerId),
    /// Request fullfilled with a successful clone.
    RequestCloned(Urn, PeerId),
    /// Request is being cloned from a peer.
    RequestCloning(Urn, PeerId),
    /// Request for the URN was created and is pending submission to the network.
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
}

impl MaybeFrom<&Input> for Event {
    fn maybe_from(input: &Input) -> Option<Self> {
        match input {
            Input::Announce(input::Announce::Succeeded(updates)) => {
                Some(Self::Announced(updates.clone()))
            },
            Input::PeerSync(input::Sync::Succeeded(peer_id)) => Some(Self::PeerSynced(*peer_id)),
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
    /// Local peer is listening on a socket but has not connected to any peers yet.
    Started,
    /// The local peer lost its connections to all its peers.
    Offline,
    /// The local peer is operational and is able to interact with the peers it has connected to.
    #[serde(rename_all = "camelCase")]
    Online {
        /// Number of connected peers.
        connected: usize,
    },
}

/// State kept for a running local peer.
pub struct RunState {
    /// Tracking remote peers that have an active connection.
    connected_peers: HashSet<PeerId>,
    listen_addrs: Vec<SocketAddr>,
    /// Current internal status.
    pub status: Status,
    stats: net::protocol::event::downstream::Stats,
    /// Timestamp of last status change.
    status_since: SystemTime,
    syncs: HashSet<PeerId>,
    /// Current set of requests.
    waiting_room: WaitingRoom<SystemTime, Duration>,
}

impl RunState {
    /// Constructs a new state.
    #[cfg(test)]
    fn construct(
        connected_peers: HashSet<PeerId>,
        status: Status,
        status_since: SystemTime,
        syncs: HashSet<PeerId>,
    ) -> Self {
        Self {
            connected_peers,
            listen_addrs: vec![],
            stats: downstream::Stats::default(),
            status,
            status_since,
            syncs,
            waiting_room: WaitingRoom::new(waiting_room::Config::default()),
        }
    }

    /// Creates a new `RunState` initialising it with the provided `config` and `waiting_room`.
    pub fn new(waiting_room: WaitingRoom<SystemTime, Duration>) -> Self {
        Self {
            connected_peers: HashSet::new(),
            listen_addrs: vec![],
            stats: downstream::Stats::default(),
            status: Status::Stopped,
            status_since: SystemTime::now(),
            syncs: HashSet::new(),
            waiting_room,
        }
    }

    /// Applies the `input` and based on the current state, transforms to the new state and in some
    /// cases produes commands which should be executed in the appropriate subroutines.
    pub fn transition(&mut self, input: Input) -> Vec<Command> {
        log::trace!("TRANSITION START: {:?} {:?}", input, self.status);

        let cmds = match input {
            Input::Announce(announce_input) => self.handle_announce(announce_input),
            Input::Control(control_input) => self.handle_control(control_input),
            Input::ListenAddrs(addrs) => self.handle_listen_addrs(addrs),
            Input::Protocol(protocol_event) => self.handle_protocol(protocol_event),
            Input::PeerSync(peer_sync_input) => self.handle_peer_sync(&peer_sync_input),
            Input::Request(request_input) => self.handle_request(request_input),
            Input::Stats(stats_input) => self.handle_stats(stats_input),
        };

        log::trace!("TRANSITION END: {:?} {:?}", self.status, cmds);

        cmds
    }

    /// Handle [`input::Announce`]s.
    fn handle_announce(&mut self, input: input::Announce) -> Vec<Command> {
        match (&self.status, input) {
            // Announce new updates while the peer is online.
            (Status::Online { .. } | Status::Started { .. }, input::Announce::Tick)
                if self.stats.connected_peers > 0 && self.stats.membership_active > 0 =>
            {
                vec![Command::Announce]
            }
            _ => vec![],
        }
    }

    /// Handle [`input::Control`]s.
    fn handle_control(&mut self, input: input::Control) -> Vec<Command> {
        match input {
            input::Control::CancelRequest(urn, timestamp, sender) => {
                let request = self
                    .waiting_room
                    .canceled(&urn, timestamp)
                    .map(|()| self.waiting_room.remove(&urn));
                vec![
                    Command::Control(command::Control::Respond(control::Response::CancelSearch(
                        sender, request,
                    ))),
                    Command::PersistWaitingRoom(self.waiting_room.clone()),
                ]
            },
            input::Control::CreateRequest(urn, time, sender) => {
                let request = self.waiting_room.request(&urn, time);
                vec![
                    Command::Control(command::Control::Respond(control::Response::StartSearch(
                        sender, request,
                    ))),
                    Command::EmitEvent(Event::RequestCreated(urn)),
                ]
            },
            input::Control::GetRequest(urn, sender) => {
                vec![Command::Control(command::Control::Respond(
                    control::Response::GetSearch(sender, self.waiting_room.get(&urn).cloned()),
                ))]
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

    /// Handle [`input::Sync`]s.
    fn handle_peer_sync(&mut self, input: &input::Sync) -> Vec<Command> {
        match input {
            input::Sync::Tick => {
                let mut cmds = vec![];

                for peer_id in &self.connected_peers {
                    if self.syncs.get(peer_id).is_none() {
                        cmds.push(Command::SyncPeer(*peer_id));
                    }
                }

                cmds
            },
            input::Sync::Started(peer_id) => {
                self.syncs.insert(*peer_id);
                vec![]
            },
            input::Sync::Succeeded(peer_id) | input::Sync::Failed(peer_id) => {
                self.syncs.remove(peer_id);
                vec![]
            },
        }
    }

    /// Handle [`ProtocolEvent`]s.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_protocol(&mut self, event: ProtocolEvent) -> Vec<Command> {
        match (&self.status, event) {
            (Status::Stopped, ProtocolEvent::Endpoint(upstream::Endpoint::Up { .. })) => {
                self.status = Status::Started;
                self.status_since = SystemTime::now();

                vec![]
            },
            (_, ProtocolEvent::Endpoint(upstream::Endpoint::Down)) => {
                self.status = Status::Stopped;
                self.status_since = SystemTime::now();

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
                        if let Err(waiting_room::Error::TimeOut { .. }) =
                            self.waiting_room.found(&urn, peer_id, SystemTime::now())
                        {
                            cmds.push(Command::Request(command::Request::TimedOut(urn.clone())));
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
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_request(&mut self, input: input::Request) -> Vec<Command> {
        match (&self.status, input) {
            // Check for new query and clone requests.
            (Status::Online { .. }, input::Request::Tick) => {
                let mut cmds = Vec::with_capacity(2);

                if let Some(urn) = self.waiting_room.next_query(SystemTime::now()) {
                    cmds.push(Command::Request(command::Request::Query(urn)));
                    cmds.push(Command::PersistWaitingRoom(self.waiting_room.clone()));
                }
                if let Some((urn, remote_peer)) = self.waiting_room.next_clone() {
                    cmds.push(Command::Request(command::Request::Clone(urn, remote_peer)));
                    cmds.push(Command::PersistWaitingRoom(self.waiting_room.clone()));
                }
                cmds
            },
            // FIXME(xla): Come up with a strategy for the results returned by the waiting room.
            (_, input::Request::Cloning(urn, remote_peer)) => self
                .waiting_room
                .cloning(&urn, remote_peer, SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (_, input::Request::Cloned(urn, remote_peer)) => self
                .waiting_room
                .cloned(&urn, remote_peer, SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (_, input::Request::Queried(urn)) => self
                .waiting_room
                .queried(&urn, SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (
                _,
                input::Request::Failed {
                    remote_peer,
                    reason,
                    urn,
                },
            ) => {
                log::warn!("Cloning failed with: {}", reason);
                self.waiting_room
                    .cloning_failed(&urn, remote_peer, SystemTime::now())
                    .map_or_else(
                        |error| Self::handle_waiting_room_timeout(urn, &error),
                        |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                    )
            },
            _ => vec![],
        }
    }

    fn handle_stats(&mut self, input: input::Stats) -> Vec<Command> {
        match (&self.status, input) {
            (_, input::Stats::Tick) => vec![Command::Stats],
            (status, input::Stats::Values(connected_peers, stats)) => {
                match status {
                    Status::Online { .. } if stats.connected_peers == 0 => {
                        self.status = Status::Offline;
                        self.status_since = SystemTime::now();
                    },
                    Status::Offline if stats.connected_peers > 0 => {
                        self.status = Status::Online {
                            connected: stats.connected_peers,
                        };
                    },
                    Status::Started if stats.connected_peers > 0 => {
                        self.status = Status::Online {
                            connected: stats.connected_peers,
                        };
                        self.status_since = SystemTime::now();
                    },
                    _ => {},
                };

                self.connected_peers = connected_peers.into_iter().collect();
                self.stats = stats;

                vec![]
            },
        }
    }

    /// Handle [`waiting_room::Error`]s.
    fn handle_waiting_room_timeout(urn: Urn, error: &waiting_room::Error) -> Vec<Command> {
        log::warn!("WaitingRoom::Error : {}", error);
        match error {
            waiting_room::Error::TimeOut { .. } => {
                vec![Command::Request(command::Request::TimedOut(urn))]
            },
            _ => vec![],
        }
    }
}

#[allow(clippy::needless_update, clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{
        collections::{BTreeSet, HashSet},
        net::SocketAddr,
        str::FromStr,
        time::SystemTime,
    };

    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;
    use tokio::sync::oneshot;

    use librad::{
        git::Urn,
        git_ext::Oid,
        keys::SecretKey,
        net::{
            self,
            peer::ProtocolEvent,
            protocol::{
                broadcast,
                event::{
                    downstream,
                    upstream::{Endpoint, Gossip},
                },
                gossip::Payload,
            },
        },
        peer::PeerId,
    };

    use super::{command, input, Command, Input, RunState, Status};

    #[test]
    fn transition_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());

        let cmds = state.transition(Input::Protocol(ProtocolEvent::Endpoint(Endpoint::Up {
            listen_addrs: vec![addr],
        })));
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Started { .. });

        Ok(())
    }

    #[test]
    fn transition_to_online() {
        let status = Status::Started;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());

        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Stats(input::Stats::Values(
                vec![peer_id],
                downstream::Stats {
                    connections_total: 1,
                    connected_peers: 1,
                    membership_active: 1,
                    membership_passive: 1,
                },
            )))
        };
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Online { .. });
    }

    #[test]
    fn transition_to_offline_when_last_peer_disconnects() {
        let peer_id = PeerId::from(SecretKey::new());
        let status = Status::Online { connected: 0 };
        let status_since = SystemTime::now();
        let mut state = RunState::construct(
            Some(peer_id).into_iter().collect(),
            status,
            status_since,
            HashSet::new(),
        );

        let _cmds = state.transition(Input::Stats(input::Stats::Values(
            vec![],
            downstream::Stats::default(),
        )));
        assert_matches!(state.status, Status::Offline);
    }

    #[test]
    fn issue_announce_while_online_and_active_membering() {
        let status = Status::Online { connected: 1 };
        let status_since = SystemTime::now();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());

        let cmds = state.transition(Input::Announce(input::Announce::Tick));
        assert!(cmds.is_empty(), "expected no command");

        state.stats = librad::net::protocol::event::downstream::Stats {
            connected_peers: 1,
            membership_active: 1,
            ..librad::net::protocol::event::downstream::Stats::default()
        };
        let cmds = state.transition(Input::Announce(input::Announce::Tick));

        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::Announce);
    }

    #[test]
    fn dont_announce_with_inactive_member() {
        let status = Status::Online { connected: 1 };
        let status_since = SystemTime::now();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());

        state.stats = librad::net::protocol::event::downstream::Stats {
            connected_peers: 0,
            membership_active: 0,
            membership_passive: 1,
            ..librad::net::protocol::event::downstream::Stats::default()
        };

        let cmds = state.transition(Input::Announce(input::Announce::Tick));
        assert!(cmds.is_empty(), "expected no command");
    }

    #[test]
    fn dont_announce_when_offline() {
        let status = Status::Offline;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());
        let cmds = state.transition(Input::Announce(input::Announce::Tick));

        assert!(cmds.is_empty(), "expected no command");
    }

    #[test]
    fn issue_query_when_requested_and_online() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let urn: Urn = Urn::new(Oid::from_str("7ab8629dd6da14dcacde7f65b3d58cd291d7e235")?);

        let status = Status::Online { connected: 1 };
        let status_since = SystemTime::now();
        let (response_sender, _) = oneshot::channel();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());
        state.transition(Input::Control(input::Control::CreateRequest(
            urn.clone(),
            SystemTime::now(),
            response_sender,
        )));

        let cmds = state.transition(Input::Request(input::Request::Tick));
        assert_matches!(
            cmds.first().unwrap(),
            Command::Request(command::Request::Query(have)) => {
                assert_eq!(*have, urn);
            }
        );

        Ok(())
    }

    #[test]
    fn issue_clone_when_found() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let urn: Urn = Urn::new(Oid::from_str("7ab8629dd6da14dcacde7f65b3d58cd291d7e235")?);
        let peer_id = PeerId::from(SecretKey::new());
        let addr = "127.0.0.0:80".parse()?;

        let status = Status::Online { connected: 0 };
        let status_since = SystemTime::now();
        let (response_sender, _) = oneshot::channel();
        let mut state = RunState::construct(HashSet::new(), status, status_since, HashSet::new());

        state.transition(Input::Control(input::Control::CreateRequest(
            urn.clone(),
            SystemTime::now(),
            response_sender,
        )));
        assert_matches!(
            state
                .transition(Input::Request(input::Request::Queried(urn.clone())))
                .first(),
            Some(Command::PersistWaitingRoom(_))
        );
        // Gossip(Box<upstream::Gossip<SocketAddr, gossip::Payload>>),
        assert_matches!(
            state
                .transition(Input::Protocol(ProtocolEvent::Gossip(Box::new(
                    Gossip::Put {
                        provider: librad::net::protocol::PeerInfo {
                            advertised_info: net::protocol::PeerAdvertisement::new(addr),
                            peer_id,
                            seen_addrs: BTreeSet::new(),
                        },
                        payload: Payload {
                            urn: urn.clone(),
                            origin: None,
                            rev: None
                        },
                        result: broadcast::PutResult::Applied(Payload {
                            urn: urn.clone(),
                            origin: None,
                            rev: None,
                        }),
                    }
                ))))
                .first(),
            Some(Command::Include(_))
        );

        let cmds = state.transition(Input::Request(input::Request::Tick));
        assert_matches!(
            cmds.first().unwrap(),
            Command::Request(command::Request::Clone(remote_urn, remote_peer)) => {
                assert_eq!(remote_urn.clone(), urn);
                assert_eq!(*remote_peer, peer_id);
            }
        );

        Ok(())
    }

    #[test]
    fn issue_syncs() {
        let num_peers = 5;

        let mut connected_peers = HashSet::new();
        for _ in 0..num_peers {
            connected_peers.insert(PeerId::from(SecretKey::new()));
        }

        let status = Status::Online {
            connected: num_peers,
        };
        let status_since = SystemTime::now();
        let mut state = RunState::construct(connected_peers, status, status_since, HashSet::new());

        let cmds = state.transition(Input::PeerSync(input::Sync::Tick));

        assert_eq!(cmds.len(), num_peers);
    }
}
