//! State machine to manage the current mode of operation during peer lifecycle.

use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use tokio::sync::oneshot;

use librad::{
    net::{
        gossip::{Has, Info},
        peer::Gossip,
        protocol::ProtocolEvent,
    },
    peer::PeerId,
    uri::{RadUrl, RadUrn},
};

use crate::{
    convert::MaybeFrom,
    peer::{announcement, control},
    request::{
        waiting_room::{self, WaitingRoom},
        SomeRequest,
    },
};

/// Default time to wait between announcement subroutine runs.
const DEFAULT_ANNOUNCE_INTERVAL: Duration = std::time::Duration::from_secs(60);

/// Default number of peers a full sync is attempting with up on startup.
/// TODO(xla): Revise number.
const DEFAULT_SYNC_MAX_PEERS: usize = 5;

/// Default Duration until the local peer goes online regardless if and how many syncs have
/// succeeded.
// TODO(xla): Review duration.
const DEFAULT_SYNC_PERIOD: Duration = Duration::from_secs(5);

/// Default period at which we query the waiting room.
const DEFAULT_WAITING_ROOM_INTERVAL: Duration = Duration::from_millis(500);

/// Default period to consider until a query has timed out.
const DEFAULT_WAITING_ROOM_TIMEOUT: Duration = Duration::from_secs(10);

/// Instructions to issue side-effectful operations which are the results from state transitions.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Command {
    /// Start the announcement subroutine.
    Announce,
    /// Answer control requests.
    Control(ControlCommand),
    /// Fulfill request commands.
    Request(RequestCommand),
    /// Initiate a full sync with `PeerId`.
    SyncPeer(PeerId),
    /// Start sync timeout.
    StartSyncTimeout(Duration),
}

/// Reactions for incoming control requests.
#[derive(Debug)]
pub enum ControlCommand {
    /// Send a response corresponding to a control request.
    Respond(control::Response),
}

/// Commands issued when requesting an identity from the network.
#[derive(Debug, PartialEq)]
pub enum RequestCommand {
    /// Tell the subroutine to attempt a clone from the given `RadUrl`.
    Clone(RadUrl),
    /// Tell the subroutine that we should query for the given `RadUrn` on the network.
    Query(RadUrn),
    /// The request for [`RadUrn`] timed out.
    TimedOut(RadUrn),
}

impl From<RequestCommand> for Command {
    fn from(other: RequestCommand) -> Self {
        Self::Request(other)
    }
}

/// Events external subscribers can observe for internal peer operations.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Event {
    /// Announcement subroutine completed and emitted the enclosed updates.
    Announced(announcement::Updates),
    /// An event from the underlying coco network stack.
    /// FIXME(xla): Align variant naming to indicate observed occurrences.
    Protocol(ProtocolEvent<Gossip>),
    /// Sync with a peer completed.
    PeerSynced(PeerId),
    /// Request fullfilled with a successful clone.
    RequestCloned(RadUrl),
    /// Requested urn was queried on the network.
    RequestQueried(RadUrn),
    /// Waiting room interval ticked.
    RequestTick,
    /// The request for [`RadUrn`] timed out.
    RequestTimedOut(RadUrn),
    StatusChanged(Status, Status),
}

impl MaybeFrom<&Input> for Event {
    fn maybe_from(input: &Input) -> Option<Self> {
        match input {
            Input::Announce(AnnounceInput::Succeeded(updates)) => {
                Some(Self::Announced(updates.clone()))
            },
            Input::PeerSync(SyncInput::Succeeded(peer_id)) => Some(Self::PeerSynced(*peer_id)),
            Input::Protocol(event) => Some(Self::Protocol(event.clone())),
            Input::Request(RequestInput::Cloned(url)) => Some(Self::RequestCloned(url.clone())),
            Input::Request(RequestInput::Queried(urn)) => Some(Self::RequestQueried(urn.clone())),
            Input::Request(RequestInput::Tick) => Some(Self::RequestTick),
            Input::Request(RequestInput::TimedOut(urn)) => Some(Self::RequestTimedOut(urn.clone())),
            _ => None,
        }
    }
}

/// Significant events that occur during [`Peer`] lifetime.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Input {
    /// Announcement subroutine lifecycle events.
    Announce(AnnounceInput),
    Control(ControlInput),
    /// Inputs from the underlying coco protocol.
    Protocol(ProtocolEvent<Gossip>),
    /// Lifecycle events during peer sync operations.
    PeerSync(SyncInput),
    /// Request subroutine events that wish to attempt to fetch an identity from the network.
    Request(RequestInput),
    /// Scheduled timeouts which can occur.
    Timeout(TimeoutInput),
}

/// Announcement subroutine lifecycle events.
#[derive(Clone, Debug)]
pub enum AnnounceInput {
    /// Operation failed.
    Failed,
    /// Operation succeeded and emitted the enclosed list of updates.
    Succeeded(announcement::Updates),
    /// The ticker duration has elapsed.
    Tick,
}

#[derive(Debug)]
pub enum ControlInput {
    Status(oneshot::Sender<Status>),
}

/// Request even that wishes to fetch an identity from the network.
#[derive(Debug)]
pub enum RequestInput {
    /// Started cloning the requested urn from a peer.
    Cloning(RadUrl),
    /// Succeeded cloning from the `RadUrl`.
    Cloned(RadUrl),
    /// Failed to clone from the `RadUrl`.
    Failed {
        /// The URL that we were attempting the clone from.
        url: RadUrl,
        /// The reason the clone failed.
        reason: String,
    },
    /// Query the network for the `RadUrn`.
    Queried(RadUrn),
    /// A urn was requested.
    Requested(
        RadUrn,
        Instant,
        Option<oneshot::Sender<Option<SomeRequest<Instant>>>>,
    ),
    /// [`WaitingRoom`] query interval.
    Tick,
    /// The request for [`RadUrn`] timed out.
    TimedOut(RadUrn),
}

/// Lifecycle events during peer sync operations.
#[derive(Debug)]
pub enum SyncInput {
    /// A sync has been initiated for `PeerId`.
    Started(PeerId),
    /// A sync has failed for `PeerId`.
    Failed(PeerId),
    /// A sync has succeeded for `PeerId`.
    Succeeded(PeerId),
}

/// Scheduled timeouts which can occur.
#[derive(Debug)]
pub enum TimeoutInput {
    /// Grace period is over signaling that we should go offline, no matter how many syncs have
    /// succeeded.
    SyncPeriod,
}

/// The current status of the local peer and its relation to the network.
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    /// Nothing is setup, not even a socket to listen on.
    Stopped(Instant),
    /// Local peer is listening on a socket but has not connected to any peers yet.
    Started(Instant),
    /// The local peer lost its connections to all its peers.
    Offline(Instant),
    /// Phase where the local peer tries get up-to-date.
    Syncing(Instant, usize),
    /// The local peer is operational and is able to interact with the peers it has connected to.
    Online(Instant),
}

/// Set of knobs to change the behaviour of the [`RunState`].
#[derive(Default)]
pub struct Config {
    /// Set of knobs to alter announce behaviour.
    pub announce: AnnounceConfig,
    /// Set of knobs to alter sync behaviour.
    pub sync: SyncConfig,
    /// Set of knobs to alter [`WaitingRoom`] behaviour.
    pub waiting_room: WaitingRoomConfig,
}

/// Set of knobs to alter announce behaviour.
pub struct AnnounceConfig {
    /// Determines how often the announcement subroutine should be run.
    pub interval: Duration,
}

impl Default for AnnounceConfig {
    fn default() -> Self {
        Self {
            interval: DEFAULT_ANNOUNCE_INTERVAL,
        }
    }
}

/// Set of knobs to alter sync behaviour.
pub struct SyncConfig {
    /// Number of peers that a full sync is attempted with upon startup.
    pub max_peers: usize,
    /// Enables the syncing stage when coming online.
    pub on_startup: bool,
    /// Duration until the local peer goes online regardless if and how many syncs have succeeded.
    pub period: Duration,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            max_peers: DEFAULT_SYNC_MAX_PEERS,
            on_startup: false,
            period: DEFAULT_SYNC_PERIOD,
        }
    }
}

/// Set of knobs to alter the [`WaitingRoom`] behvaviour.
pub struct WaitingRoomConfig {
    /// Interval at which to query the [`WaitingRoom`] for ready requests.
    pub interval: Duration,
    /// Period to consider until a query has timed out.
    pub timeout_period: Duration,
}

impl Default for WaitingRoomConfig {
    fn default() -> Self {
        Self {
            timeout_period: DEFAULT_WAITING_ROOM_TIMEOUT,
            interval: DEFAULT_WAITING_ROOM_INTERVAL,
        }
    }
}

/// State kept for a running local peer.
pub struct RunState {
    /// Confiugration to change how input [`Input`]s are interpreted.
    config: Config,
    /// Tracking remote peers that have an active connection.
    connected_peers: HashSet<PeerId>,
    /// Current internal status.
    pub status: Status,
    /// Current set of requests.
    waiting_room: WaitingRoom<Instant, Duration>,
}

impl From<Config> for RunState {
    fn from(config: Config) -> Self {
        let waiting_room_config = waiting_room::Config {
            delta: config.waiting_room.timeout_period,
            ..waiting_room::Config::default()
        };

        Self {
            config,
            connected_peers: HashSet::new(),
            status: Status::Stopped(Instant::now()),
            waiting_room: WaitingRoom::new(waiting_room_config),
        }
    }
}

impl RunState {
    /// Constructs a new state.
    #[cfg(test)]
    fn new(config: Config, connected_peers: HashSet<PeerId>, status: Status) -> Self {
        Self {
            config,
            connected_peers,
            status,
            waiting_room: WaitingRoom::new(waiting_room::Config::default()),
        }
    }

    /// Applies the `input` and based on the current state, transforms to the new state and in some
    /// cases produes commands which should be executed in the appropriate subroutines.
    pub fn transition(&mut self, input: Input) -> Vec<Command> {
        log::trace!("TRANSITION START: {:?} {:?}", input, self.status);

        let cmds = match input {
            Input::Announce(announce_input) => self.handle_announce(announce_input),
            Input::Control(control_input) => self.handle_control(control_input),
            Input::Protocol(protocol_event) => self.handle_protocol(protocol_event),
            Input::PeerSync(_peer_sync_input) => vec![],
            Input::Request(request_input) => self.handle_request(request_input),
            Input::Timeout(timeout_input) => self.handle_timeout(timeout_input),
        };

        log::trace!("TRANSITION END: {:?} {:?}", self.status, cmds);

        cmds
    }

    /// Handle [`AnnunceInput`]s.
    fn handle_announce(&mut self, input: AnnounceInput) -> Vec<Command> {
        match (&self.status, input) {
            // Announce new updates while the peer is online.
            (
                Status::Online(_) | Status::Started(_) | Status::Syncing(_, _),
                AnnounceInput::Tick,
            ) => vec![Command::Announce],
            _ => vec![],
        }
    }

    fn handle_control(&self, input: ControlInput) -> Vec<Command> {
        match input {
            ControlInput::Status(sender) => vec![Command::Control(ControlCommand::Respond(
                control::Response::CurrentStatus(sender, self.status.clone()),
            ))],
        }
    }

    /// Handle [`ProtolEvent`]s.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_protocol(&mut self, event: ProtocolEvent<Gossip>) -> Vec<Command> {
        match (&self.status, event) {
            // Go from [`Status::Stopped`] to [`Status::Started`] once we are listening.
            (Status::Stopped(_since), ProtocolEvent::Listening(_addr)) => {
                self.status = Status::Started(Instant::now());

                vec![]
            },
            // Sync with first incoming peer.
            //
            // In case the peer is configured to sync on startup we start syncing, otherwise we go
            // online straight away.
            // TODO(xla): Also issue sync if we come online after a certain period of being
            // disconnected from any peer.
            (Status::Started(_since), ProtocolEvent::Connected(ref peer_id)) => {
                self.connected_peers.insert(*peer_id);

                if self.config.sync.on_startup {
                    self.status = Status::Syncing(Instant::now(), 1);

                    vec![
                        Command::SyncPeer(*peer_id),
                        Command::StartSyncTimeout(self.config.sync.period),
                    ]
                } else {
                    self.status = Status::Online(Instant::now());

                    vec![]
                }
            },
            // Sync until configured maximum of peers is reached.
            (Status::Syncing(since, syncs), ProtocolEvent::Connected(peer_id))
                if *syncs < self.config.sync.max_peers =>
            {
                self.connected_peers.insert(peer_id);
                if syncs + 1 == self.config.sync.max_peers {
                    self.status = Status::Online(Instant::now());
                } else {
                    self.status = Status::Syncing(*since, syncs + 1);
                }

                vec![Command::SyncPeer(peer_id)]
            }
            // Remove peer that just disconnected.
            (_, ProtocolEvent::Disconnecting(peer_id)) => {
                self.connected_peers.remove(&peer_id);

                // Go offline if we have no more connected peers left.
                if self.connected_peers.is_empty() {
                    self.status = Status::Offline(Instant::now());
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
                match self.waiting_room.found(
                    RadUrl {
                        urn: urn.clone(),
                        authority: provider.peer_id,
                    },
                    Instant::now(),
                ) {
                    Err(err) => {
                        log::warn!("waiting room error: {:?}", err);

                        match err {
                            waiting_room::Error::TimeOut { .. } => {
                                vec![Command::Request(RequestCommand::TimedOut(urn))]
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

    /// Handle [`RequestInput`]s.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn handle_request(&mut self, input: RequestInput) -> Vec<Command> {
        match (&self.status, input) {
            // Check for new querie and clone requests.
            (Status::Online(_) | Status::Syncing(_, _), RequestInput::Tick) => {
                let mut cmds = Vec::with_capacity(2);

                if let Some(urn) = self.waiting_room.next_query(Instant::now()) {
                    cmds.push(Command::Request(RequestCommand::Query(urn)));
                }
                if let Some(url) = self.waiting_room.next_clone() {
                    cmds.push(Command::Request(RequestCommand::Clone(url)));
                }
                cmds
            },
            // FIXME(xla): Come up with a strategy for the results returned by the waiting room.
            (_, RequestInput::Cloning(url)) => {
                match self.waiting_room.cloning(url.clone(), Instant::now()) {
                    Err(err) => {
                        log::warn!("waiting room error: {:?}", err);

                        match err {
                            waiting_room::Error::TimeOut { .. } => {
                                vec![Command::Request(RequestCommand::TimedOut(url.urn))]
                            },
                            _ => vec![],
                        }
                    },
                    Ok(_) => vec![],
                }
            },
            (_, RequestInput::Cloned(url)) => {
                match self.waiting_room.cloned(&url, Instant::now()) {
                    Err(err) => {
                        log::warn!("waiting room error: {:?}", err);

                        match err {
                            waiting_room::Error::TimeOut { .. } => {
                                vec![Command::Request(RequestCommand::TimedOut(url.urn))]
                            },
                            _ => vec![],
                        }
                    },
                    Ok(_) => vec![],
                }
            },
            (_, RequestInput::Queried(urn)) => {
                match self.waiting_room.queried(&urn, Instant::now()) {
                    Err(err) => {
                        log::warn!("waiting room error: {:?}", err);

                        match err {
                            waiting_room::Error::TimeOut { .. } => {
                                vec![Command::Request(RequestCommand::TimedOut(urn))]
                            },
                            _ => vec![],
                        }
                    },
                    Ok(_) => vec![],
                }
            },
            (_, RequestInput::Requested(urn, time, maybe_sender)) => {
                let maybe_request = self.waiting_room.request(&urn, time);

                if let Some(sender) = maybe_sender {
                    vec![Command::Control(ControlCommand::Respond(
                        control::Response::Urn(sender, maybe_request),
                    ))]
                } else {
                    vec![]
                }
            },
            _ => vec![],
        }
    }

    /// Handle [`Timeout`]s.
    fn handle_timeout(&mut self, input: TimeoutInput) -> Vec<Command> {
        match (&self.status, input) {
            // Go online if we exceed the sync period.
            (Status::Syncing(_since, _syncs), TimeoutInput::SyncPeriod) => {
                self.status = Status::Online(Instant::now());

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
        collections::HashSet,
        iter::FromIterator,
        net::{IpAddr, SocketAddr},
        time::{Duration, Instant},
    };

    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use librad::{
        keys::SecretKey,
        net::{gossip, peer::Gossip, protocol::ProtocolEvent},
        peer::PeerId,
        uri::{RadUrl, RadUrn},
    };

    use super::{
        AnnounceInput, Command, Config, Input, RequestCommand, RequestInput, RunState, Status,
        SyncConfig, TimeoutInput, DEFAULT_SYNC_MAX_PEERS,
    };

    #[test]
    fn transition_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);

        let cmds = state.transition(Input::Protocol(ProtocolEvent::Listening(addr)));
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Started(_));

        Ok(())
    }

    #[test]
    fn transition_to_online_if_sync_is_disabled() {
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(
            Config {
                sync: SyncConfig {
                    on_startup: false,
                    ..SyncConfig::default()
                },
                ..Config::default()
            },
            HashSet::new(),
            status,
        );

        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn transition_to_online_after_sync_max_peers() {
        let status = Status::Syncing(Instant::now(), DEFAULT_SYNC_MAX_PEERS - 1);
        let mut state = RunState::new(Config::default(), HashSet::new(), status);

        let _cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn transition_to_online_after_sync_period() {
        let status = Status::Syncing(Instant::now(), 3);
        let mut state = RunState::new(Config::default(), HashSet::new(), status);

        let _cmds = state.transition(Input::Timeout(TimeoutInput::SyncPeriod));
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn transition_to_offline_when_last_peer_disconnects() {
        let peer_id = PeerId::from(SecretKey::new());
        let status = Status::Online(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::from_iter(vec![peer_id]), status);

        let _cmds = state.transition(Input::Protocol(ProtocolEvent::Disconnecting(peer_id)));
        assert_matches!(state.status, Status::Offline(_));
    }

    #[test]
    fn issue_sync_command_until_max_peers() {
        let max_peers = 13;
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(
            Config {
                sync: SyncConfig {
                    max_peers,
                    on_startup: true,
                    ..SyncConfig::default()
                },
                ..Config::default()
            },
            HashSet::new(),
            status,
        );

        for i in 0..(max_peers - 1) {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);

            // Expect to sync with the first connected peer.
            let cmds = state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)));
            assert!(!cmds.is_empty(), "expected command");
            assert_matches!(cmds.first().unwrap(), Command::SyncPeer(sync_id) => {
                assert_eq!(*sync_id, peer_id);
            });
            assert_matches!(state.status, Status::Syncing(_, syncing_peers) => {
                assert_eq!(i + 1, syncing_peers);
            });
        }

        // Issue last sync.
        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Input::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::SyncPeer{..});
        // Expect to be online at this point.
        assert_matches!(state.status, Status::Online(_));

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
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(
            Config {
                sync: SyncConfig {
                    on_startup: true,
                    period: sync_period,
                    ..SyncConfig::default()
                },
                ..Config::default()
            },
            HashSet::new(),
            status,
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
        let status = Status::Online(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Announce(AnnounceInput::Tick));

        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::Announce);

        let status = Status::Offline(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Announce(AnnounceInput::Tick));

        assert!(cmds.is_empty(), "expected no command");
    }
    #[test]
    fn issue_query_when_requested() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let urn: RadUrn =
            "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c".parse()?;

        let status = Status::Stopped(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Request(RequestInput::Requested(
            urn.clone(),
            Instant::now(),
            None,
        )));
        assert_matches!(cmds.first(), None);

        let status = Status::Started(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Request(RequestInput::Requested(
            urn.clone(),
            Instant::now(),
            None,
        )));
        assert_matches!(cmds.first(), None);

        let status = Status::Offline(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Request(RequestInput::Requested(
            urn.clone(),
            Instant::now(),
            None,
        )));
        assert_matches!(cmds.first(), None);

        let status = Status::Syncing(Instant::now(), 1);
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Request(RequestInput::Requested(
            urn.clone(),
            Instant::now(),
            None,
        )));
        assert!(cmds.is_empty());

        let cmds = state.transition(Input::Request(RequestInput::Tick));
        let cmd = cmds.first().unwrap();
        assert_matches!(cmd, Command::Request(RequestCommand::Query(have)) => {
            assert_eq!(*have, urn);
        });

        let status = Status::Online(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);
        let cmds = state.transition(Input::Request(RequestInput::Requested(
            urn.clone(),
            Instant::now(),
            None,
        )));
        assert!(cmds.is_empty());

        let cmds = state.transition(Input::Request(RequestInput::Tick));
        assert_matches!(
            cmds.first().unwrap(),
            Command::Request(RequestCommand::Query(have)) => {
                assert_eq!(*have, urn);
            }
        );

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

        let status = Status::Online(Instant::now());
        let mut state = RunState::new(Config::default(), HashSet::new(), status);

        assert!(state
            .transition(Input::Request(RequestInput::Requested(
                urn.clone(),
                Instant::now(),
                None
            )))
            .is_empty());
        assert!(state
            .transition(Input::Request(RequestInput::Queried(urn.clone())))
            .is_empty());
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

        let cmds = state.transition(Input::Request(RequestInput::Tick));
        assert_matches!(
            cmds.first().unwrap(),
            Command::Request(RequestCommand::Clone(have)) => {
                assert_eq!(*have, url);
            }
        );

        Ok(())
    }
}
