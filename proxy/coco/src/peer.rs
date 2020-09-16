//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::time::Duration;
use std::{collections::HashSet, fmt, time::Instant};

use futures::StreamExt as _;
use tokio::sync::{broadcast, mpsc};
use tokio::time::interval;

use librad::{
    net::{
        peer::{Gossip, RunLoop},
        protocol::ProtocolEvent,
    },
    peer::PeerId,
    uri::RadUrl,
};

use crate::state::Lock;

mod announcement;
pub use announcement::Announcement;

/// Upper bound of messages stored in receiver channels.
const RECEIVER_CAPACITY: usize = 128;

/// Duration we delay until we go online regardless if and how many syncs have succeeded.
// TODO(xla): Review duration.
// TODO(xla): Make configurable as part of peer configuration.
const SYNC_PERIOD: Duration = Duration::from_secs(5);

/// Number of peers a full sync is attempting with up on startup.
/// TODO(xla): Revise number.
/// TODO(xla): Make configurable as part of peer configuration.
const SYNC_MAX_PEERS: usize = 5;

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to build and announce state updates.
    #[error(transparent)]
    Announcement(#[from] announcement::Error),
    /// Stop-gap until we get rid of crate level errors.
    // TODO(xla): Remove once we transitioned to per module errors.
    #[error(transparent)]
    Crate(#[from] crate::error::Error),
}

/// Significant events that occur during [`Peer`] lifetime.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Event {
    Announce(AnnounceEvent),
    Protocol(ProtocolEvent<Gossip>),
    PeerSync(SyncEvent),
    Timeout(TimeoutEvent),
}

#[derive(Clone, Debug)]
pub enum AnnounceEvent {
    Failed,
    Succeeded(announcement::Updates),
    Tick,
}

#[derive(Clone, Debug)]
pub enum SyncEvent {
    Started(PeerId),
    Failed(PeerId),
    Succeeded(PeerId),
}

#[derive(Clone, Debug)]
pub enum TimeoutEvent {
    SyncPeriod,
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Peer [`librad::net::peer::RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Handle used to broadcast [`Event`].
    subscriber: broadcast::Sender<Event>,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        Self {
            run_loop,
            subscriber,
        }
    }

    /// Subscribe to peer events.
    ///
    /// NB(xla): A caller must call this before the run loop is started, as that consumes the peer.
    /// There is also a configured [`RECEIVER_CAPACITY`], which prevents unbounded queues fron
    /// filling up.
    #[must_use = "eat your events"]
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.subscriber.subscribe()
    }

    /// Start up the internal machinery to advance the underlying protocol, react to significant
    /// events and keep auxiliary tasks running.
    ///
    /// # Errors
    ///
    /// * if one of the handlers of the select loop fails
    pub async fn run(self, state: Lock, store: kv::Store, name: &str) -> Result<(), Error> {
        let peer_id = state.lock().await.peer_id();
        // Subscribe to protocol events.
        let protocol_subscriber = {
            let state = state.lock().await;
            let protocol = state.api.protocol();
            protocol.subscribe().await
        };
        tokio::pin!(protocol_subscriber);

        // Start announcement timer.
        // TODO(xla): Find a more structured approach to manage timings.
        let mut announce_timer = interval(std::time::Duration::from_secs(1));

        let (announce_sender, mut announcements) =
            mpsc::channel::<AnnounceEvent>(RECEIVER_CAPACITY);
        let (peer_sync_sender, mut peer_syncs) = mpsc::channel::<SyncEvent>(RECEIVER_CAPACITY);
        let (timeout_sender, mut timeouts) = mpsc::channel::<TimeoutEvent>(RECEIVER_CAPACITY);

        // Advance the librad protocol.
        tokio::spawn(self.run_loop);

        let mut run_state = RunState::default();
        loop {
            log::warn!("LOOP: {} - {}", name, peer_id);
            let event = tokio::select! {
                _ = announce_timer.tick() => Event::Announce(AnnounceEvent::Tick),
                Some(announce_event) = announcements.recv() => Event::Announce(announce_event),
                Some(protocol_event) = protocol_subscriber.next() => Event::Protocol(protocol_event),
                Some(sync_event) = peer_syncs.recv() => Event::PeerSync(sync_event),
                Some(timeout_event) = timeouts.recv() => Event::Timeout(timeout_event),
                else => {
                    log::debug!("BREAK");
                    break
                },
            };

            // Send will error if there are no active receivers. This case is expected and
            // ssh the run loop.
            self.subscriber.send(event.clone()).ok();
            log::debug!("{} - {:?}", name, event);

            let maybe_cmd = run_state.transition(event);

            if let Some(cmd) = maybe_cmd {
                match cmd {
                    Command::Announce => {
                        let mut sender = announce_sender.clone();
                        let announce_state = state.clone();
                        let store = store.clone();

                        tokio::spawn(async move {
                            match Self::announce(announce_state, store).await {
                                Ok(updates) => {
                                    sender.send(AnnounceEvent::Succeeded(updates)).await.ok()
                                }
                                Err(_err) => sender.send(AnnounceEvent::Failed).await.ok(),
                            }
                        });
                    }
                    Command::SyncPeer(peer_id) => {
                        let mut sync_tx = peer_sync_sender.clone();
                        let mut timeout_tx = timeout_sender.clone();
                        let peer = peer_id.clone();
                        let sync_state = state.clone();

                        // TODO(xla): Find a more structured approach for timeout management.
                        tokio::spawn(async move {
                            tokio::time::delay_for(SYNC_PERIOD).await;
                            timeout_tx.send(TimeoutEvent::SyncPeriod).await.ok();
                        });
                        tokio::spawn(async move {
                            sync_tx.send(SyncEvent::Started(peer.clone())).await.ok();

                            match Self::sync(sync_state, peer.clone()).await {
                                Ok(_) => sync_tx.send(SyncEvent::Succeeded(peer)).await.ok(),
                                Err(_) => sync_tx.send(SyncEvent::Failed(peer)).await.ok(),
                            }
                        });
                    }
                };
            }
        }

        Ok(())
    }

    /// Announcement subroutine.
    async fn announce(state: Lock, store: kv::Store) -> Result<announcement::Updates, Error> {
        log::debug!("GOT TO ANNOUNCE");
        let old = announcement::load(store.clone())?;
        let new = announcement::build(state.clone()).await?;
        let updates = announcement::diff(&old, &new);

        announcement::announce(state, updates.iter()).await;

        if !updates.is_empty() {
            announcement::save(store.clone(), updates.clone()).map_err(Error::from)?;
        }

        Ok(updates)
    }

    /// Sync subroutine.
    async fn sync(state: Lock, peer_id: PeerId) -> Result<(), Error> {
        let state = state.lock().await;
        let urls = state
            .list_projects()
            .map_err(Error::from)?
            .iter()
            .map(|project| RadUrl {
                authority: peer_id.clone(),
                urn: project.urn(),
            })
            .collect::<Vec<RadUrl>>();

        for url in urls {
            state.fetch(url, vec![])?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Status {
    Stopped(Instant),
    Started(Instant),
    Offline(Instant),
    Syncing(Instant, usize),
    Online(Instant),
}

#[derive(Debug, PartialEq)]
enum Command {
    Announce,
    SyncPeer(PeerId),
}

struct RunState {
    connected_peers: HashSet<PeerId>,
    status: Status,
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
    fn transition(&mut self, event: Event) -> Option<Command> {
        match (&self.status, event) {
            // Go from [`Input::Stopped`] to [`Input::Offline`] once we are listening.
            (Status::Stopped(_since), Event::Protocol(ProtocolEvent::Listening(_addr))) => {
                self.status = Status::Started(Instant::now());

                None
            }
            // Sync with first incoming peer.
            (Status::Started(_since), Event::Protocol(ProtocolEvent::Connected(ref peer_id))) => {
                self.connected_peers.insert(peer_id.clone());
                self.status = Status::Syncing(Instant::now(), 1);

                Some(Command::SyncPeer(peer_id.clone()))
            }
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

                Some(Command::SyncPeer(peer_id))
            }
            // TODO(xla): Also issue sync if we come online after a certain period of being
            // disconnected from any peer.
            // Issue more syncs if we connect to new peers while syncing.
            (
                Status::Syncing(since, syncs),
                Event::Protocol(ProtocolEvent::Connected(ref peer_id)),
            ) => {
                self.status = Status::Syncing(*since, syncs + 1);

                Some(Command::SyncPeer(peer_id.clone()))
            }
            // Go online if we exceed the sync period.
            (Status::Syncing(_since, _syncs), Event::Timeout(TimeoutEvent::SyncPeriod)) => {
                self.status = Status::Online(Instant::now());

                None
            }
            // Go offline if we have no more connected peers left.
            (_, Event::Protocol(ProtocolEvent::Disconnecting(peer_id)))
                if self.connected_peers.len() == 1 =>
            {
                self.connected_peers.remove(&peer_id);
                self.status = Status::Offline(Instant::now());

                None
            }
            // Remove peer that just disconnected.
            (_, Event::Protocol(ProtocolEvent::Disconnecting(peer_id))) => {
                self.connected_peers.remove(&peer_id);

                None
            }
            // Announce new updates while the peer is online.
            (
                Status::Online(_) | Status::Started(_) | Status::Syncing(_, _),
                Event::Announce(AnnounceEvent::Tick),
            ) => Some(Command::Announce),
            _ => None,
        }
    }
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            connected_peers: HashSet::new(),
            status: Status::Stopped(Instant::now()),
        }
    }
}

#[allow(clippy::panic)]
#[cfg(test)]
mod test {
    use std::net::SocketAddr;
    use std::{collections::HashSet, time::Instant};

    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use librad::keys::SecretKey;
    use librad::net::protocol::ProtocolEvent;
    use librad::peer::PeerId;

    use super::{AnnounceEvent, Command, Event, RunState, Status, TimeoutEvent, SYNC_MAX_PEERS};

    #[test]
    fn transition_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);

        let cmd = state.transition(Event::Protocol(ProtocolEvent::Listening(addr)));
        assert_eq!(cmd, None);
        assert_matches!(state.status, Status::Started(_));

        Ok(())
    }

    #[test]
    fn transition_to_online_after_sync_max_peers() {
        let status = Status::Syncing(Instant::now(), SYNC_MAX_PEERS - 1);
        let mut state = RunState::new(HashSet::new(), status);

        let _cmd = {
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

        let _cmd = state.transition(Event::Timeout(TimeoutEvent::SyncPeriod));
        assert_matches!(state.status, Status::Online(_));
    }

    #[test]
    fn issue_sync_command_until_max_peers() -> Result<(), Box<dyn std::error::Error>> {
        let status = Status::Started(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);

        for i in 0..(SYNC_MAX_PEERS - 1) {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);

            // Expect to sync with the first connected peer.
            let cmd = state
                .transition(Event::Protocol(ProtocolEvent::Connected(peer_id.clone())))
                .expect("expected command");
            assert_matches!(cmd, Command::SyncPeer(sync_id) => {
                assert_eq!(sync_id, peer_id);
            });
            assert_matches!(state.status, Status::Syncing(_, syncing_peers) => {
                assert_eq!(i + 1, syncing_peers);
            });
        }

        // Issue last sync.
        let cmd = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state
                .transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
                .expect("expected command")
        };
        assert_matches!(cmd, Command::SyncPeer(_));
        // Expect to be online at this point.
        assert_matches!(state.status, Status::Online(_));

        // No more syncs should be expected after the maximum of peers have connected.
        let cmd = {
            let key = SecretKey::new();
            let peer_id = PeerId::from(key);
            state.transition(Event::Protocol(ProtocolEvent::Connected(peer_id)))
        };
        assert_matches!(cmd, None);

        Ok(())
    }

    #[test]
    fn issue_announce_while_online() {
        let status = Status::Online(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);
        let cmd = state
            .transition(Event::Announce(AnnounceEvent::Tick))
            .expect("expected command");

        assert_matches!(cmd, Command::Announce);

        let status = Status::Offline(Instant::now());
        let mut state = RunState::new(HashSet::new(), status);
        let cmd = state.transition(Event::Announce(AnnounceEvent::Tick));

        assert_matches!(cmd, None);
    }
}
