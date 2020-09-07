//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use futures::StreamExt as _;

use librad::net::peer::RunLoop;

use crate::error;
use crate::state::Lock;

mod announcement;
pub use announcement::{Announcement, Store as AnnouncementStore};

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer<A>
where
    A: AnnouncementStore,
{
    announcement_store: A,
    /// Peer [`RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state access.
    state: Lock,
}

impl<A> Peer<A>
where
    A: AnnouncementStore,
{
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: Lock, announcement_store: A) -> Self {
        Self {
            announcement_store,
            run_loop,
            state,
        }
    }

    /// Start up the internal machinery to advance the underlying protocol, react to significant
    /// events and keep auxiliary tasks running.
    ///
    /// # Errors
    ///
    /// * if one of the handlers of the select loop fails
    pub async fn run(self) -> Result<(), error::Error> {
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
            let res: Result<(), error::Error> = tokio::select! {
                _ = announce_timer.tick() => {
                    let old = self.announcement_store.load().map_err(Error::from)?;
                    let new = announcement::build(self.state.clone()).await?;
                    let updates = announcement::diff(&old, &new);

                    announcement::announce(self.state.clone(), updates.iter()).await;
                    log::info!("announcements = {}", updates.len());

                    self.announcement_store.save(updates).map_err(Error::from)?;

                    Ok(())
                },
                Some(event) = protocol_subscriber.next() => {
                    log::info!("protocol.event = {:?}", event);
                    Ok(())
                },
                else => break,
            };

            // Propagate if one of the select failed.
            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }
}
