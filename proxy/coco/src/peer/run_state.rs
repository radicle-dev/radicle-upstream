//! State machine to manage the current mode of operation during peer lifecycle.

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use serde::Serialize;

use librad::{
    net::{
        gossip::{Has, Info, PutResult},
        peer::{FetchInfo, Gossip, PeerEvent},
        protocol::ProtocolEvent,
    },
    peer::PeerId,
    uri::{RadUrl, RadUrn},
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
        provider: PeerId,
        /// Cooresponding gossip message.
        gossip: Gossip,
        /// Result of the storage fetch.
        result: PutResult<Gossip>,
    },
    /// An event from the underlying coco network stack.
    /// FIXME(xla): Align variant naming to indicate observed occurrences.
    Protocol(ProtocolEvent<Gossip>),
    /// Sync with a peer completed.
    PeerSynced(PeerId),
    /// Request fullfilled with a successful clone.
    RequestCloned(RadUrl),
    /// Request is being cloned from a peer.
    RequestCloning(RadUrl),
    /// Request for the URN was created and is pending submission to the network.
    RequestCreated(RadUrn),
    /// Request for the URN was submitted to the network.
    RequestQueried(RadUrn),
    /// Waiting room interval ticked.
    RequestTick,
    /// The request for [`RadUrn`] timed out.
    RequestTimedOut(RadUrn),
    /// The [`Status`] of the peer changed.
    StatusChanged(Status, Status),
}

impl MaybeFrom<&Input> for Event {
    fn maybe_from(input: &Input) -> Option<Self> {
        match input {
            Input::Announce(input::Announce::Succeeded(updates)) => {
                Some(Self::Announced(updates.clone()))
            },
            Input::Peer(event) => match event {
                PeerEvent::GossipFetch(FetchInfo {
                    provider,
                    gossip,
                    result,
                }) => Some(Self::GossipFetched {
                    provider: *provider,
                    gossip: gossip.clone(),
                    result: result.clone(),
                }),
            },
            Input::PeerSync(input::Sync::Succeeded(peer_id)) => Some(Self::PeerSynced(*peer_id)),
            Input::Protocol(protocol_event) => Some(Self::Protocol(protocol_event.clone())),
            Input::Request(input::Request::Cloned(url)) => Some(Self::RequestCloned(url.clone())),
            Input::Request(input::Request::Cloning(url)) => Some(Self::RequestCloning(url.clone())),
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
    /// Phase where the local peer tries get up-to-date.
    #[serde(rename_all = "camelCase")]
    Syncing {
        /// Number of completed syncs.
        synced: usize,
        /// Number of synchronisation underway.
        syncs: usize,
    },
    /// The local peer is operational and is able to interact with the peers it has connected to.
    #[serde(rename_all = "camelCase")]
    Online {
        /// Number of connected peers.
        connected: usize,
    },
}

/// State kept for a running local peer.
pub struct RunState {
    /// Confiugration to change how input [`Input`]s are interpreted.
    config: Config,
    /// Tracking remote peers that have an active connection.
    ///
    /// As a peer known by [`PeerId`] can be connected multiple times, e.g. when opening a git
    /// connection to clone and fetch, tracking the connection count per peer is paramount to not
    /// falsely end up in an unconnected state despite the fact the protocol is connected, alive
    /// and kicking. The following scenario led to an offline state when a `HashSet` was used in
    /// the past:
    ///
    /// `Connected(Peer1) -> Connected(Peer1) -> Disconnecting(Peer1)`
    //
    // FIXME(xla): Use a `Option<NonEmpty>` here to express the invariance.
    connected_peers: HashMap<PeerId, usize>,
    /// Current internal status.
    pub status: Status,
    /// Timestamp of last status change.
    status_since: SystemTime,
    /// Current set of requests.
    waiting_room: WaitingRoom<SystemTime, Duration>,
}

impl RunState {
    /// Constructs a new state.
    #[cfg(test)]
    fn construct(
        config: Config,
        connected_peers: HashMap<PeerId, usize>,
        status: Status,
        status_since: SystemTime,
    ) -> Self {
        Self {
            config,
            connected_peers,
            status,
            status_since,
            waiting_room: WaitingRoom::new(waiting_room::Config::default()),
        }
    }

    /// Creates a new `RunState` initialising it with the provided `config` and `waiting_room`.
    pub fn new(config: Config, waiting_room: WaitingRoom<SystemTime, Duration>) -> Self {
        Self {
            config,
            connected_peers: HashMap::new(),
            status: Status::Stopped,
            status_since: SystemTime::now(),
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
            Input::Peer(peer_event) => Self::handle_peer_event(peer_event),
            Input::Protocol(protocol_event) => self.handle_protocol(protocol_event),
            Input::PeerSync(peer_sync_input) => self.handle_peer_sync(&peer_sync_input),
            Input::Request(request_input) => self.handle_request(request_input),
            Input::Timeout(timeout_input) => self.handle_timeout(timeout_input),
        };

        log::trace!("TRANSITION END: {:?} {:?}", self.status, cmds);

        cmds
    }

    /// Handle [`input::Announce`]s.
    fn handle_announce(&mut self, input: input::Announce) -> Vec<Command> {
        match (&self.status, input) {
            // Announce new updates while the peer is online.
            (
                Status::Online { .. } | Status::Started { .. } | Status::Syncing { .. },
                input::Announce::Tick,
            ) => vec![Command::Announce],
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
            input::Control::Status(sender) => vec![Command::Control(command::Control::Respond(
                control::Response::CurrentStatus(sender, self.status.clone()),
            ))],
        }
    }

    fn handle_peer_event(event: PeerEvent) -> Vec<Command> {
        match event {
            PeerEvent::GossipFetch(FetchInfo {
                result: PutResult::Applied(Gossip { urn, .. }),
                ..
            }) => vec![Command::Include(urn)],
            PeerEvent::GossipFetch(_) => vec![],
        }
    }

    /// Handle [`input::Sync`]s.
    fn handle_peer_sync(&mut self, input: &input::Sync) -> Vec<Command> {
        if let Status::Syncing { synced, syncs } = self.status {
            match input {
                input::Sync::Started(_peer_id) => {
                    self.status = Status::Syncing {
                        synced,
                        syncs: syncs + 1,
                    };
                },
                input::Sync::Failed(_peer_id) | input::Sync::Succeeded(_peer_id) => {
                    self.status = if synced + 1 >= self.config.sync.max_peers {
                        Status::Online {
                            connected: self.connected_peers.len(),
                        }
                    } else {
                        Status::Syncing {
                            synced: synced + 1,
                            syncs: syncs - 1,
                        }
                    };
                },
            }
        }

        vec![]
    }

    /// Handle [`ProtocolEvent`]s.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_protocol(&mut self, event: ProtocolEvent<Gossip>) -> Vec<Command> {
        match (&self.status, event) {
            // Go from [`Status::Stopped`] to [`Status::Started`] once we are listening.
            (Status::Stopped { .. }, ProtocolEvent::Listening(_addr)) => {
                self.status = Status::Started;
                self.status_since = SystemTime::now();

                vec![]
            },
            (state, ProtocolEvent::Connected(peer_id)) => {
                if let Some(counter) = self.connected_peers.get_mut(&peer_id) {
                    *counter += 1;
                } else {
                    self.connected_peers.insert(peer_id, 1);
                }

                match state {
                    Status::Offline => {
                        self.status = Status::Online {
                            connected: self.connected_peers.len(),
                        };

                        vec![]
                    },
                    Status::Started => {
                        // Sync with first incoming peer.
                        //
                        // In case the peer is configured to sync on startup we start syncing,
                        // otherwise we go online straight away.
                        // TODO(xla): Also issue sync if we come online after a certain period of
                        // being disconnected from any peer.
                        if self.config.sync.on_startup {
                            self.status = Status::Syncing {
                                synced: 0,
                                syncs: 0,
                            };
                            self.status_since = SystemTime::now();

                            vec![
                                Command::SyncPeer(peer_id),
                                Command::StartSyncTimeout(self.config.sync.period),
                            ]
                        } else {
                            self.status = Status::Online {
                                connected: self.connected_peers.len(),
                            };
                            self.status_since = SystemTime::now();

                            vec![]
                        }
                    },
                    // Issue syncs until we reach maximum amount of peers to sync with.
                    Status::Syncing { syncs, .. } if *syncs < self.config.sync.max_peers => {
                        vec![Command::SyncPeer(peer_id)]
                    },
                    // Update status with its connected peers.
                    Status::Online { .. } => {
                        self.status = Status::Online {
                            connected: self.connected_peers.len(),
                        };
                        vec![]
                    },
                    // Noop
                    Status::Stopped | Status::Syncing { .. } => vec![],
                }
            },
            // Remove peer that just disconnected.
            (_, ProtocolEvent::Disconnecting(peer_id)) => {
                if let Some(counter) = self.connected_peers.get_mut(&peer_id) {
                    *counter -= 1;

                    if *counter == 0 {
                        self.connected_peers.remove(&peer_id);
                    }
                } else {
                    log::error!("The impossible has happened, somehow we disconnected from '{}' without already being connected to them", peer_id);
                    return vec![];
                }

                // Go offline if we have no more connected peers left.
                if self.connected_peers.is_empty() {
                    self.status = Status::Offline;
                    self.status_since = SystemTime::now();
                }

                vec![]
            },
            // Found URN.
            (
                _,
                ProtocolEvent::Gossip(Info::Has(Has {
                    provider,
                    val: Gossip { urn, .. },
                })),
            ) => {
                // This message is uninteresting to the waiting room
                if !self.waiting_room.has(&urn) {
                    return vec![];
                }

                match self.waiting_room.found(
                    RadUrl {
                        urn: urn.clone(),
                        authority: provider.peer_id,
                    },
                    SystemTime::now(),
                ) {
                    Err(err) => {
                        log::warn!("waiting room error: {:?}", err);

                        match err {
                            waiting_room::Error::TimeOut { .. } => {
                                vec![Command::Request(command::Request::TimedOut(urn))]
                            },
                            _ => vec![],
                        }
                    },
                    Ok(_) => vec![],
                }
            },
            _ => vec![],
        }
    }

    /// Handle [`input::Request`]s.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_request(&mut self, input: input::Request) -> Vec<Command> {
        match (&self.status, input) {
            // Check for new query and clone requests.
            (Status::Online { .. } | Status::Syncing { .. }, input::Request::Tick) => {
                let mut cmds = Vec::with_capacity(2);

                if let Some(urn) = self.waiting_room.next_query(SystemTime::now()) {
                    cmds.push(Command::Request(command::Request::Query(urn)));
                    cmds.push(Command::PersistWaitingRoom(self.waiting_room.clone()));
                }
                if let Some(url) = self.waiting_room.next_clone() {
                    cmds.push(Command::Request(command::Request::Clone(url)));
                    cmds.push(Command::PersistWaitingRoom(self.waiting_room.clone()));
                }
                cmds
            },
            // FIXME(xla): Come up with a strategy for the results returned by the waiting room.
            (_, input::Request::Cloning(url)) => self
                .waiting_room
                .cloning(url.clone(), SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(url.urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (_, input::Request::Cloned(url)) => self
                .waiting_room
                .cloned(&url, SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(url.urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (_, input::Request::Queried(urn)) => self
                .waiting_room
                .queried(&urn, SystemTime::now())
                .map_or_else(
                    |error| Self::handle_waiting_room_timeout(urn, &error),
                    |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                ),
            (_, input::Request::Failed { url, reason }) => {
                log::warn!("Cloning failed with: {}", reason);
                let urn = url.urn.clone();
                self.waiting_room
                    .cloning_failed(url, SystemTime::now())
                    .map_or_else(
                        |error| Self::handle_waiting_room_timeout(urn, &error),
                        |_| vec![Command::PersistWaitingRoom(self.waiting_room.clone())],
                    )
            },
            _ => vec![],
        }
    }

    /// Handle [`waiting_room::Error`]s.
    fn handle_waiting_room_timeout(urn: RadUrn, error: &waiting_room::Error) -> Vec<Command> {
        log::warn!("WaitingRoom::Error : {}", error);
        match error {
            waiting_room::Error::TimeOut { .. } => {
                vec![Command::Request(command::Request::TimedOut(urn))]
            },
            _ => vec![],
        }
    }

    /// Handle [`input::Timeout`]s.
    fn handle_timeout(&mut self, input: input::Timeout) -> Vec<Command> {
        match (&self.status, input) {
            // Go online if we exceed the sync period.
            (Status::Syncing { .. }, input::Timeout::SyncPeriod) => {
                self.status = Status::Online {
                    connected: self.connected_peers.len(),
                };
                self.status_since = SystemTime::now();

                vec![]
            },
            _ => vec![],
        }
    }
}

#[allow(clippy::needless_update, clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        iter::FromIterator,
        net::{IpAddr, SocketAddr},
        time::{Duration, SystemTime},
    };

    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;
    use tokio::sync::oneshot;

    use librad::{
        keys::SecretKey,
        net::{gossip, peer::Gossip, protocol::ProtocolEvent},
        peer::PeerId,
        uri::{RadUrl, RadUrn},
    };

    use super::{command, config, input, Command, Config, Input, RunState, Status};

    #[test]
    fn transition_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped;
        let status_since = SystemTime::now();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);

        let cmds = state.transition(Input::Protocol(ProtocolEvent::Listening(addr)));
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Started {..});

        Ok(())
    }

    #[test]
    fn transition_to_online_if_sync_is_disabled() {
        let status = Status::Started;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(
            Config {
                sync: config::Sync {
                    on_startup: false,
                    ..config::Sync::default()
                },
                ..Config::default()
            },
            HashMap::new(),
            status,
            status_since,
        );

        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Online {..});
    }

    #[test]
    fn transition_to_online_after_sync_max_peers() {
        let status = Status::Syncing {
            synced: config::DEFAULT_SYNC_MAX_PEERS - 1,
            syncs: 1,
        };
        let status_since = SystemTime::now();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);

        let _cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::PeerSync(input::Sync::Succeeded(peer_id)))
        };
        assert_matches!(state.status, Status::Online {..});
    }

    #[test]
    fn transition_to_online_after_sync_period() {
        let status = Status::Syncing {
            synced: 0,
            syncs: 3,
        };
        let status_since = SystemTime::now();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);

        let _cmds = state.transition(Input::Timeout(input::Timeout::SyncPeriod));
        assert_matches!(state.status, Status::Online {..});
    }

    #[test]
    fn transition_to_offline_when_last_peer_disconnects() {
        let peer_id = PeerId::from(SecretKey::new());
        let status = Status::Online { connected: 0 };
        let status_since = SystemTime::now();
        let mut state = RunState::construct(
            Config::default(),
            HashMap::from_iter(vec![(peer_id, 1)]),
            status,
            status_since,
        );

        let _cmds = state.transition(Input::Protocol(ProtocolEvent::Disconnecting(peer_id)));
        assert_matches!(state.status, Status::Offline);
    }

    #[test]
    fn issue_sync_command_until_max_peers() {
        let max_peers = 13;
        let status = Status::Started;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(
            Config {
                sync: config::Sync {
                    max_peers,
                    on_startup: true,
                    ..config::Sync::default()
                },
                ..Config::default()
            },
            HashMap::new(),
            status,
            status_since,
        );

        for _i in 0..(max_peers - 1) {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);

            // Expect to sync with the first connected peer.
            let cmds = state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)));
            assert!(!cmds.is_empty(), "expected command");
            assert_matches!(cmds.first().unwrap(), Command::SyncPeer(sync_id) => {
                assert_eq!(*sync_id, peer_id);
            });
            let _cmds = state.transition(Input::PeerSync(input::Sync::Started(peer_id)));
            assert_matches!(state.status, Status::Syncing{ syncs: syncing_peers, .. } => {
                assert_eq!(syncing_peers, 1);
            });
            let _cmds = state.transition(Input::PeerSync(input::Sync::Succeeded(peer_id)));
        }

        // Issue last sync.
        {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            let cmds = state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)));

            assert!(!cmds.is_empty(), "expected command");
            assert_matches!(cmds.first().unwrap(), Command::SyncPeer{..});

            let _cmds = state.transition(Input::PeerSync(input::Sync::Started(peer_id)));
            let _cmds = state.transition(Input::PeerSync(input::Sync::Succeeded(peer_id)));
        };

        // Expect to be online at this point.
        assert_matches!(state.status, Status::Online {..});

        // No more syncs should be expected after the maximum of peers have connected.
        let cmd = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(cmd.is_empty(), "should not emit any more commands");
    }

    #[test]
    fn issue_sync_timeout_when_transitioning_to_syncing() {
        let sync_period = Duration::from_secs(60 * 10);
        let status = Status::Started;
        let status_since = SystemTime::now();
        let mut state = RunState::construct(
            Config {
                sync: config::Sync {
                    on_startup: true,
                    period: sync_period,
                    ..config::Sync::default()
                },
                ..Config::default()
            },
            HashMap::new(),
            status,
            status_since,
        );

        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert_matches!(cmds.get(1), Some(Command::StartSyncTimeout(period)) => {
            assert_eq!(*period, sync_period);
        });
    }

    #[test]
    fn issue_announce_while_online() {
        let status = Status::Online { connected: 0 };
        let status_since = SystemTime::now();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);
        let cmds = state.transition(Input::Announce(input::Announce::Tick));

        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::Announce);

        let status = Status::Offline;
        let status_since = SystemTime::now();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);
        let cmds = state.transition(Input::Announce(input::Announce::Tick));

        assert!(cmds.is_empty(), "expected no command");
    }

    #[test]
    fn issue_query_when_requested_and_online() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let urn: RadUrn =
            "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c".parse()?;

        let status = Status::Online { connected: 1 };
        let status_since = SystemTime::now();
        let (response_sender, _) = oneshot::channel();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);
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
    fn issue_query_when_requested_and_syncing() -> Result<(), Box<dyn std::error::Error + 'static>>
    {
        let urn: RadUrn =
            "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c".parse()?;

        let status = Status::Syncing {
            synced: 0,
            syncs: 1,
        };
        let status_since = SystemTime::now();
        let (response_sender, _) = oneshot::channel();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);
        state.transition(Input::Control(input::Control::CreateRequest(
            urn.clone(),
            SystemTime::now(),
            response_sender,
        )));

        let cmds = state.transition(Input::Request(input::Request::Tick));
        let cmd = cmds.first().unwrap();
        assert_matches!(cmd, Command::Request(command::Request::Query(have)) => {
            assert_eq!(*have, urn);
        });

        Ok(())
    }

    #[test]
    fn issue_clone_when_found() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let urn: RadUrn =
            "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c".parse()?;
        let peer_id = PeerId::from(SecretKey::new());
        let url = RadUrl {
            urn: urn.clone(),
            authority: peer_id,
        };

        let status = Status::Online { connected: 0 };
        let status_since = SystemTime::now();
        let (response_sender, _) = oneshot::channel();
        let mut state =
            RunState::construct(Config::default(), HashMap::new(), status, status_since);

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
        assert!(state
            .transition(Input::Protocol(ProtocolEvent::Gossip(gossip::Info::Has(
                gossip::Has {
                    provider: gossip::types::PeerInfo {
                        peer_id,
                        advertised_info: gossip::types::PeerAdvertisement {
                            capabilities: HashSet::new(),
                            listen_addr: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 11)),
                            listen_port: 12345,
                        },
                        seen_addrs: HashSet::new(),
                    },
                    val: Gossip {
                        urn,
                        origin: None,
                        rev: None
                    },
                },
            ))))
            .is_empty());

        let cmds = state.transition(Input::Request(input::Request::Tick));
        assert_matches!(
            cmds.first().unwrap(),
            Command::Request(command::Request::Clone(have)) => {
                assert_eq!(*have, url);
            }
        );

        Ok(())
    }
}
