//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::fmt;

use futures::StreamExt as _;
use tokio::sync::broadcast;

use librad::net::peer::Gossip;
use librad::net::peer::RunLoop;
use librad::net::protocol;

use crate::state::Lock;

mod announcement;
pub use announcement::Announcement;

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Broadcast(broadcast::SendError<Event>),

    #[error(transparent)]
    Announcement(#[from] announcement::Error),
}

#[derive(Clone)]
pub enum Event {
    Announced(usize),
    Protocol(protocol::ProtocolEvent<Gossip>),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Announced(updates) => write!(f, "announcements = {}", updates),
            Self::Protocol(event) => write!(f, "protocol = {:?}", event),
        }
    }
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Peer [`RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state access.
    state: Lock,
    store: kv::Store,
    subscriber: broadcast::Sender<Event>,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: Lock, store: kv::Store) -> Self {
        let (subscriber, _receiver) = broadcast::channel(32);
        Self {
            run_loop,
            state,
            store,
            subscriber,
        }
    }

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
        let mut announce_timer = tokio::time::interval(std::time::Duration::from_secs(10));

        // Advance the librad protocol.
        tokio::spawn(self.run_loop);

        loop {
            let res = tokio::select! {
                _ = announce_timer.tick() => {
                    let updates = Self::announce(self.state.clone(), &self.store).await?;
                    Ok(Event::Announced(updates))
                },
                Some(event) = protocol_subscriber.next() => {
                    Ok(Event::Protocol(event))
                },
                else => break,
            };

            match res {
                // Propagate if one of the select failed.
                Err(err) => return Err(err),
                Ok(event) => {
                    log::info!("{:?}", event);

                    self.subscriber.send(event).map_err(Error::Broadcast)?;
                }
            }
        }

        Ok(())
    }

    async fn announce(state: Lock, store: &kv::Store) -> Result<usize, Error> {
        let updates = {
            let old = announcement::load(store)?;
            let new = announcement::build(state.clone()).await?;
            announcement::diff(&old, &new)
        };

        announcement::announce(state, updates.iter()).await;
        announcement::save(&store, updates.clone()).map_err(Error::from)?;

        Ok(updates.len())
    }
}
