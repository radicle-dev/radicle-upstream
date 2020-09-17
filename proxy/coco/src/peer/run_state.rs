//! State machine to manage the current mode of operation during peer lifecycle.

use std::{collections::HashSet, time::Instant};

use librad::{
    net::{peer::Gossip, protocol::ProtocolEvent},
    peer::PeerId,
};

use crate::peer::announcement;

/// Number of peers a full sync is attempting with up on startup.
/// TODO(xla): Revise number.
/// TODO(xla): Make configurable as part of peer configuration.
const SYNC_MAX_PEERS: usize = 5;

/// Instructions to issue side-effectful operations which are the results from state transitions.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Start the announcement subroutine.
    Announce,
    /// Initiate a full sync with `PeerId`.
    SyncPeer(PeerId),
    /// Start sync timeout.
    StartSyncTimeout,
}

/// Significant events that occur during [`Peer`] lifetime.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Event {
    /// Announcement subroutine lifecycle events.
    Announce(AnnounceEvent),
    /// Events from the underlying coco protocol.
    Protocol(ProtocolEvent<Gossip>),
    /// Lifecycle events during peer sync operations.
    PeerSync(SyncEvent),
    /// Scheduled timeouts which can occur.
    Timeout(TimeoutEvent),
}

/// Announcement subroutine lifecycle events.
#[derive(Clone, Debug)]
pub enum AnnounceEvent {
    /// Operation failed.
    Failed,
    /// Operation succeeded and emitted the enclosed list of updates.
    Succeeded(announcement::Updates),
    /// The ticker duration has elapsed.
    Tick,
}

/// Lifecycle events during peer sync operations.
#[derive(Clone, Debug)]
pub enum SyncEvent {
    /// A sync has been initiated for `PeerId`.
    Started(PeerId),
    /// A sync has failed for `PeerId`.
    Failed(PeerId),
    /// A sync has succeeded for `PeerId`.
    Succeeded(PeerId),
}

/// Scheduled timeouts which can occur.
#[derive(Clone, Debug)]
pub enum TimeoutEvent {
    /// Grace period is over signaling that we should go offline, no matter how many syncs have
    /// succeeded.
    SyncPeriod,
}

/// Internal state representation the current mode of operation for the local peer.
#[derive(Debug)]
enum Status {
    /// Nothing is setup, not even a socket to listen on.
    Stopped(Instant),
    /// Local peer is listening on a socket, but no connected peers yet.
    Started(Instant),
    /// The loccal peer lost connections to all peers.
    Offline(Instant),
    /// Phase where the local peer tries get up-to-date.
    Syncing(Instant, usize),
    /// Normal operation with other other peers being connected.
    Online(Instant),
}

/// State kept for a running local peer.
pub struct RunState {
    /// Track peers that we have a connection to.
    connected_peers: HashSet<PeerId>,
    /// Current internal status.
    status: Status,
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            connected_peers: HashSet::new(),
            status: Status::Stopped(Instant::now()),
        }
    }
}

impl RunState {
    /// Constructs a new state.
    #[cfg(test)]
    const fn new(connected_peers: HashSet<PeerId>, status: Status) -> Self {
        Self {
            connected_peers,
            status,
        }
    }

    /// Applies the `input` and based on the current state transforms to the new state and in some
    /// cases produes commands which should be executed in the appropriate sub-routines.
    pub fn transition(&mut self, event: Event) -> Vec<Command> {
        match (&self.status, event) {
            // Go from [`Input::Stopped`] to [`Input::Offline`] once we are listening.
            (Status::Stopped(_since), Event::Protocol(ProtocolEvent::Listening(_addr))) => {
                self.status = Status::Started(Instant::now());

                vec![]
            },
            // Sync with first incoming peer.
            (Status::Started(_since), Event::Protocol(ProtocolEvent::Connected(ref peer_id))) => {
                self.connected_peers.insert(peer_id.clone());
                self.status = Status::Syncing(Instant::now(), 1);

                vec![
                    Command::SyncPeer(peer_id.clone()),
                    Command::StartSyncTimeout,
                ]
            },
            // Sync until configured maximum of peers is reached.
            (Status::Syncing(since, syncs), Event::Protocol(ProtocolEvent::Connected(peer_id)))
                if *syncs < SYNC_MAX_PEERS =>
            {
                self.connected_peers.insert(peer_id.clone());
                if syncs + 1 == SYNC_MAX_PEERS {
                    self.status = Status::Online(Instant::now());
                } else {
                    self.status = Status::Syncing(*since, syncs + 1);
                }

                vec![Command::SyncPeer(peer_id)]
            }
            // TODO(xla): Also issue sync if we come online after a certain period of being
            // disconnected from any peer.
            // Issue more syncs if we connect to new peers while syncing.
            (
                Status::Syncing(since, syncs),
                Event::Protocol(ProtocolEvent::Connected(ref peer_id)),
            ) => {
                self.status = Status::Syncing(*since, syncs + 1);

                vec![Command::SyncPeer(peer_id.clone())]
            },
            // Go online if we exceed the sync period.
            (Status::Syncing(_since, _syncs), Event::Timeout(TimeoutEvent::SyncPeriod)) => {
                self.status = Status::Online(Instant::now());

                vec![]
            },
            // Go offline if we have no more connected peers left.
            (_, Event::Protocol(ProtocolEvent::Disconnecting(peer_id)))
                if self.connected_peers.len() == 1 =>
            {
                self.connected_peers.remove(&peer_id);
                self.status = Status::Offline(Instant::now());

                vec![]
            }
            // Remove peer that just disconnected.
            (_, Event::Protocol(ProtocolEvent::Disconnecting(peer_id))) => {
                self.connected_peers.remove(&peer_id);

                vec![]
            },
            // Announce new updates while the peer is online.
            (
                Status::Online(_) | Status::Started(_) | Status::Syncing(_, _),
                Event::Announce(AnnounceEvent::Tick),
            ) => vec![Command::Announce],
            _ => vec![],
        }
    }
}

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{collections::HashSet, net::SocketAddr, time::Instant};

    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use librad::{keys::SecretKey, net::protocol::ProtocolEvent, peer::PeerId};

    use super::{AnnounceEvent, Command, Event, RunState, Status, TimeoutEvent, SYNC_MAX_PEERS};

    #[test]
    fn transition_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);

        let cmds = state.transition(Event::Protocol(ProtocolEvent::Listening(addr)));
        assert!(cmds.is_empty());
        assert_matches!(state.status, Status::Started(_));

        Ok(())
    }

    #[test]
    fn transition_to_online_after_sync_max_peers() {
        let status = Status::Syncing(Instant::now(), SYNC_MAX_PEERS - 1);
        let mut state = RunState::new(HashSet::new(), status);

        let _cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn transition_to_online_after_sync_period() {
        let status = Status::Syncing(Instant::now(), 3);
        let mut state = RunState::new(HashSet::new(), status);

        let _cmds = state.transition(Event::Timeout(TimeoutEvent::SyncPeriod));
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn issue_sync_command_until_max_peers() {
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);

        for i in 0..(SYNC_MAX_PEERS - 1) {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);

            // Expect to sync with the first connected peer.
            let cmds = state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id.clone())));
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
            state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::SyncPeer{..});
        // Expect to be online at this point.
        assert_matches!(state.status, Status::Online(_));

        // No more syncs should be expected after the maximum of peers have connected.
        let cmd = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert!(cmd.is_empty(), "should not emit any more commands");
    }

    #[test]
    fn issue_sync_timeout_when_transitioning_to_syncing() {
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);

        let cmds = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert_matches!(cmds.get(1), Some(Command::StartSyncTimeout));
    }

    #[test]
    fn issue_announce_while_online() {
        let status = Status::Online(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);
        let cmds = state.transition(Event::Announce(AnnounceEvent::Tick));

        assert!(!cmds.is_empty(), "expected command");
        assert_matches!(cmds.first().unwrap(), Command::Announce);

        let status = Status::Offline(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);
        let cmds = state.transition(Event::Announce(AnnounceEvent::Tick));

        assert!(cmds.is_empty(), "expected no command");
    }
}
