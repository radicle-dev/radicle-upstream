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
};

use crate::state::Lock;

mod announcement;
pub use announcement::Announcement;

/// Upper bound of messages stored in receiver channels.
const RECEIVER_CAPACITY: usize = 128;

/// Duration we delay until we go online regardless if and how many syncs have succeeded.
// TODO(xla): Review duration.
// TODO(xla): Make configurable as part of peer configuration.
const SYNC_PERIOD: Duration = Duration::from_secs(10);

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to build and announce state updates.
    #[error(transparent)]
    Announcement(#[from] announcement::Error),
}

/// Significant events that occur during [`Peer`] lifetime.
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    /// Gossiped a list of updates of new heads in our [`crate::state::State`]`.
    Announced(announcement::Updates),
    /// Received a low-level protocol event.
    Protocol(ProtocolEvent<Gossip>),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Announced(updates) => write!(f, "announcements = {}", updates.len()),
            Self::Protocol(event) => write!(f, "protocol = {:?}", event),
        }
    }
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Peer [`librad::net::peer::RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state that is passed to subroutines.
    state: Lock,
    /// On-disk storage  for caching.
    store: kv::Store,
    /// Handle used to broadcast [`Event`].
    subscriber: broadcast::Sender<Event>,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: Lock, store: kv::Store) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        Self {
            run_loop,
            state,
            store,
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
    pub async fn run(self) -> Result<(), Error> {
        // Subscribe to protocol events.
        let protocol_subscriber = {
            let state = self.state.lock().await;
            let protocol = state.api.protocol();
            protocol.subscribe().await
        };
        tokio::pin!(protocol_subscriber);

        // Start announcement timer.
        let mut announce_timer = interval(std::time::Duration::from_secs(1));

        let (mut timeout_sender, mut timeout_receiver) =
            mpsc::channel::<Timeout>(RECEIVER_CAPACITY);

        // Advance the librad protocol.
        tokio::spawn(self.run_loop);

        let mut state = State::default();

        loop {
            let maybe_input = tokio::select! {
                _ = announce_timer.tick() => {
                    Some(Input::AnnouncementTick)
                    // Self::announce(self.state.clone(), &self.store).await.map(Event::Announced)
                },
                Some(event) = protocol_subscriber.next() => Some(Input::Protocol(event)),
                Some(timeout) = timeout_receiver.recv() => Some(Input::Timeout(timeout)),
                else => break,
            };

            let maybe_cmd = maybe_input.and_then(|input| state.transition(input));

            if let Some(cmd) = maybe_cmd {
                let event = match cmd {
                    Command::Announce => {
                        let updates = Self::announce(self.state.clone(), &self.store).await?;
                        Event::Announced(updates)
                    }
                    Command::Sync(_peer_id) => {
                        tokio::spawn(async move {
                            tokio::time::delay_for(SYNC_PERIOD).await;
                            timeout_sender
                                .send(Timeout::SyncPeriod)
                                .await
                                .expect("unable to send SyncPeriod timeout");
                        });
                        // TODO(xla): Initiate sync for peer_id.
                        todo!()
                    }
                };

                log::debug!("{:?}", event);

                // Send will error if there are no active receivers. This case is expected and
                // should not crash the run loop.
                self.subscriber.send(event).ok();
            }
        }

        Ok(())
    }

    /// Announcement subroutine.
    async fn announce(state: Lock, store: &kv::Store) -> Result<announcement::Updates, Error> {
        let old = announcement::load(store)?;
        let new = announcement::build(state.clone()).await?;
        let updates = announcement::diff(&old, &new);

        announcement::announce(state, updates.iter()).await;

        if !updates.is_empty() {
            announcement::save(store, updates.clone()).map_err(Error::from)?;
        }

        Ok(updates)
    }
}

enum Status {
    Stopped(Instant),
    Started(Instant),
    Offline(Instant),
    Syncing(Instant, usize),
    Online(Instant),
}

#[allow(clippy::large_enum_variant)]
enum Input {
    AnnouncementTick,
    Protocol(ProtocolEvent<Gossip>),
    Timeout(Timeout),
}

#[derive(Debug, PartialEq)]
enum Command {
    Announce,
    Sync(PeerId),
}

#[derive(Debug)]
enum Timeout {
    SyncPeriod,
}

struct State {
    connected_peers: HashSet<PeerId>,
    status: Status,
}

impl State {
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
    fn transition(&mut self, input: Input) -> Option<Command> {
        match (&self.status, input) {
            // Go from [`Input::Stopped`] to [`Input::Offline`] once we are listening.
            (Status::Stopped(_since), Input::Protocol(ProtocolEvent::Listening(_addr))) => {
                self.status = Status::Started(Instant::now());

                None
            }
            // Sync with first incoming peer.
            (Status::Started(_since), Input::Protocol(ProtocolEvent::Connected(ref peer_id))) => {
                self.connected_peers.insert(peer_id.clone());
                self.status = Status::Syncing(Instant::now(), 1);

                Some(Command::Sync(peer_id.clone()))
            }
            // TODO(xla): Also issue sync if we come online after a certain period of being
            // disconnected from any peers.
            // Announce new updates while the peer is online.
            (Status::Online(_since), Input::AnnouncementTick) => Some(Command::Announce),
            // Issue more syncs if we connect to new peers while syncing.
            (
                Status::Syncing(since, syncs),
                Input::Protocol(ProtocolEvent::Connected(ref peer_id)),
            ) => {
                self.status = Status::Syncing(*since, syncs + 1);

                Some(Command::Sync(peer_id.clone()))
            }
            // Go online if we exceed the sync period.
            (Status::Syncing(_since, _syncs), Input::Timeout(Timeout::SyncPeriod)) => {
                self.status = Status::Online(Instant::now());

                None
            }
            // Go offline if we have no more connected peers left.
            (_, Input::Protocol(ProtocolEvent::Disconnecting(peer_id)))
                if self.connected_peers.len() == 1 =>
            {
                self.connected_peers.remove(&peer_id);
                self.status = Status::Offline(Instant::now());

                None
            }
            // Remove peer that just disconnected.
            (_, Input::Protocol(ProtocolEvent::Disconnecting(peer_id))) => {
                self.connected_peers.remove(&peer_id);

                None
            }
            _ => None,
        }
    }
}

impl Default for State {
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

    use pretty_assertions::assert_eq;

    use librad::keys::SecretKey;
    use librad::net::protocol::ProtocolEvent;
    use librad::peer::PeerId;

    use super::{Command, Input, State, Status, Timeout};

    #[test]
    fn transitions_to_started_on_listen() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:12345".parse::<SocketAddr>()?;

        let status = Status::Stopped(Instant::now());
        let mut state = State::new(HashSet::new(), status);

        let cmd = state.transition(Input::Protocol(ProtocolEvent::Listening(addr)));
        assert_eq!(cmd, None);
        assert!(matches!(state.status, Status::Started(_)));

        Ok(())
    }

    #[test]
    fn sync_on_startup() -> Result<(), Box<dyn std::error::Error>> {
        let key = SecretKey::new();
        let peer_id = PeerId::from(key);

        let status = Status::Started(Instant::now());
        let mut state = State::new(HashSet::new(), status);

        // We expect to sync with the first connected peer.
        let cmd = state
            .transition(Input::Protocol(ProtocolEvent::Connected(peer_id.clone())))
            .expect("expected command");
        assert_eq!(cmd, Command::Sync(peer_id));
        assert!(matches!(state.status, Status::Syncing(_, 1)));

        Ok(())
    }

    #[test]
    fn transitions_to_online_after_sync_period() {
        let status = Status::Syncing(Instant::now(), 3);
        let mut state = State::new(HashSet::new(), status);

        let _cmd = state.transition(Input::Timeout(Timeout::SyncPeriod));
        assert!(matches!(state.status, Status::Online(_)));
    }

    #[test]
    fn announce_when_online() {
        let status = Status::Online(Instant::now());
        let mut state = State::new(HashSet::new(), status);
        let cmd = state
            .transition(Input::AnnouncementTick)
            .expect("expected command");

        assert_eq!(cmd, Command::Announce);
    }
}
