//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

use futures::StreamExt as _;
use tokio::sync::broadcast;

use librad::net::peer::Gossip;
use librad::net::peer::RunLoop;
use librad::net::protocol::ProtocolEvent;
use librad::peer::PeerId;

use crate::state::Lock;

mod announcement;
pub use announcement::Announcement;

/// Upper bound of messages stored in receiver channels.
const RECEIVER_CAPACITY: usize = 128;

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
        let mut announce_timer = tokio::time::interval(std::time::Duration::from_secs(1));

        // Advance the librad protocol.
        tokio::spawn(self.run_loop);

        let mut state = State::default();

        loop {
            let maybe_input = tokio::select! {
                _ = announce_timer.tick() => {
                    Some(Input::AnnouncementTick)
                    // Self::announce(self.state.clone(), &self.store).await.map(Event::Announced)
                },
                Some(event) = protocol_subscriber.next() => match event {
                    ProtocolEvent::Connected(peer_id) => Some(Input::Connected(peer_id)),
                    _ => None,
                },

                    // None
                    // Ok(Event::Protocol(event))
                // },
                else => break,
            };

            let maybe_cmd = maybe_input.and_then(|input| state.transition(input));

            if let Some(cmd) = maybe_cmd {
                let event = match cmd {
                    Command::Announce => {
                        let updates = Self::announce(self.state.clone(), &self.store).await?;
                        Event::Announced(updates)
                    }
                    _ => todo!(),
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
    Offline(Option<Instant>),
    Syncing(Instant, usize),
    Online(Instant),
}

enum Input {
    AnnouncementTick,
    Connected(PeerId),
    Disconnected(PeerId),
    SyncTimeout,
}

#[derive(Debug, PartialEq)]
enum Command {
    Announce,
    Sync(PeerId),
}

struct State {
    connected_peers: HashSet<PeerId>,
    status: Status,
}

impl State {
    fn new(connected_peers: HashSet<PeerId>, status: Status) -> Self {
        Self {
            connected_peers,
            status,
        }
    }

    fn transition(&mut self, input: Input) -> Option<Command> {
        match (&self.status, input) {
            // First connection after startup, which we know from the recorded `since` being
            // `None`.
            (Status::Offline(None), Input::Connected(peer_id)) => {
                self.connected_peers.insert(peer_id.clone());
                self.status = Status::Syncing(Instant::now(), 1);

                Some(Command::Sync(peer_id))
            }
            // Go offline if we have no more connected peers left. We produce no output.
            (_, Input::Disconnected(peer_id)) if self.connected_peers.len() == 1 => {
                self.connected_peers.remove(&peer_id);
                self.status = Status::Offline(Some(Instant::now()));

                None
            }
            // Remove peer that just disconnected.
            (_, Input::Disconnected(peer_id)) => {
                self.connected_peers.remove(&peer_id);

                None
            }
            // Announce new updates while the peer is online.
            (Status::Online(_since), Input::AnnouncementTick) => Some(Command::Announce),
            _ => None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            connected_peers: HashSet::new(),
            status: Status::Offline(None),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::time::Instant;

    use librad::keys::SecretKey;
    use librad::peer::PeerId;

    use super::{Command, Input, State, Status};

    #[test]
    fn sync_on_startup() {
        let key = SecretKey::new();
        let peer_id = PeerId::from(key);

        // Startup can be inferred by the `Offline` state which doesn't have a recorded timestamp.
        let status = Status::Offline(None);
        let mut state = State::new(HashSet::new(), status);
        let cmd = state
            .transition(Input::Connected(peer_id.clone()))
            .expect("expected command");

        assert_eq!(cmd, Command::Sync(peer_id));
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
