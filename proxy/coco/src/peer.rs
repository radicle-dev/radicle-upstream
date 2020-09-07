//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use futures::stream::BoxStream;
use futures::{Stream, StreamExt as _};

use librad::net::peer::{PeerEvent, RunLoop};

use crate::state::Lock;

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Stream of [`librad::net::peer::PeerEvent`]s we store to be consumed in our main run loop.
type ApiSubscriber = BoxStream<'static, PeerEvent>;

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Stream of peer events.
    api_subscriber: ApiSubscriber,
    /// Peer [`RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state access.
    state: Lock,
}

impl Peer {
    /// Constructs a new [`Peer`].
    ///
    /// As the [`librad::net::peer::PeerApi`] is not `Send` we can't get its subscriber directly
    /// from it as we would cross await point boundaries. Therefore we expect the caller to set up
    /// the future which we can await to get the actual subscription in form of a stream out of it.
    pub async fn new<F>(run_loop: RunLoop, api_subscriber: F, state: Lock) -> Self
    where
        F: std::future::Future + Send,
        F::Output: Stream<Item = PeerEvent> + Send + 'static,
    {
        Self {
            api_subscriber: Box::pin(api_subscriber.await),
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
    pub async fn run(self) -> Result<(), Error> {
        // Subscribe to API events.
        let api_subscriber = self.api_subscriber;
        tokio::pin!(api_subscriber);

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
            let res: Result<(), Error> = tokio::select! {
                _ = announce_timer.tick() => {
                    Ok(())
                },
                Some(event) = api_subscriber.next() => {
                    log::info!("peer.event = {:?}", event);
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
