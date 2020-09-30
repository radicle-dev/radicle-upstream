//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::time::{Duration, Instant};

use futures::StreamExt as _;
use tokio::{
    sync::{broadcast, mpsc},
    time::interval,
};

use librad::{
    net::peer::RunLoop,
    peer::PeerId,
    uri::{RadUrl, RadUrn},
};

use crate::{
    request::waiting_room::{self, WaitingRoom},
    shared::Shared,
    state::Lock,
};

mod announcement;
pub use announcement::Announcement;

mod request;

mod run_state;
pub use run_state::{
    AnnounceConfig, AnnounceEvent, Config as RunConfig, Event, RequestCommand, RequestEvent,
    SyncConfig, SyncEvent, TimeoutEvent,
};
use run_state::{Command, RunState};

mod sync;

/// Upper bound of messages stored in receiver channels.
const RECEIVER_CAPACITY: usize = 128;

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
    pub async fn run<W>(
        self,
        who: &str,
        run_config: RunConfig,
        state: Lock,
        store: kv::Store,
        waiting_room: W,
    ) where
        W: Into<Shared<WaitingRoom<Instant, Duration>>> + Send + Sync,
    {
        let waiting_room = waiting_room.into();
        // Subscribe to protocol events.
        let protocol_subscriber = {
            let state = state.lock().await;
            let protocol = state.api.protocol();
            protocol.subscribe().await
        };
        tokio::pin!(protocol_subscriber);

        // Start announcement timer.
        // TODO(xla): Find a more structured approach to manage timings.
        let mut announce_timer = interval(run_config.announce.interval);

        let (announce_sender, mut announcements) =
            mpsc::channel::<AnnounceEvent>(RECEIVER_CAPACITY);
        let (peer_sync_sender, mut peer_syncs) = mpsc::channel::<SyncEvent>(RECEIVER_CAPACITY);
        let (timeout_sender, mut timeouts) = mpsc::channel::<TimeoutEvent>(RECEIVER_CAPACITY);
        let (request_sender, mut requests) = mpsc::channel::<RequestEvent>(RECEIVER_CAPACITY);

        let request_queries = waiting_room::stream::Queries::new(who, waiting_room.clone().value);
        tokio::pin!(request_queries);
        let request_clones = waiting_room::stream::Clones::new(waiting_room.clone().value);
        tokio::pin!(request_clones);

        // Advance the librad protocol.
        tokio::spawn(self.run_loop);

        let mut run_state = RunState::from(run_config);
        loop {
            let event = tokio::select! {
                _ = announce_timer.tick() => Event::Announce(AnnounceEvent::Tick),
                Some(announce_event) = announcements.recv() => Event::Announce(announce_event),
                Some(protocol_event) = protocol_subscriber.next() => Event::Protocol(protocol_event),
                Some(sync_event) = peer_syncs.recv() => Event::PeerSync(sync_event),
                Some(timeout_event) = timeouts.recv() => Event::Timeout(timeout_event),
                Some(urn) = request_queries.next() => Event::Request(RequestEvent::Query(urn)),
                Some(url) = request_clones.next() => Event::Request(RequestEvent::Clone(url)),
                Some(request_event) = requests.next() => Event::Request(request_event),
                else => {
                    break
                },
            };

            // Send will error if there are no active receivers. This case is expected and
            // should not terminate the run loop.
            self.subscriber.send(event.clone()).ok();
            log::debug!("{:?}", event);

            for cmd in run_state.transition(event) {
                match cmd {
                    Command::Announce => {
                        Self::announce(state.clone(), store.clone(), announce_sender.clone());
                    },
                    Command::Request(RequestCommand::Query(urn)) => {
                        log::debug!("Querying for '{}'", urn);
                        Self::query(urn, state.clone(), waiting_room.clone());
                    },
                    Command::Request(RequestCommand::Found(url)) => {
                        log::debug!("Found '{}'", url);
                        Self::found(url, waiting_room.clone());
                    },
                    Command::Request(RequestCommand::Clone(url)) => {
                        log::debug!("Cloning '{}'", url);
                        Self::clone(
                            url,
                            state.clone(),
                            waiting_room.clone(),
                            request_sender.clone(),
                        );
                    },
                    Command::SyncPeer(peer_id) => {
                        Self::sync(state.clone(), peer_sync_sender.clone(), peer_id.clone()).await;
                    },
                    Command::StartSyncTimeout(sync_period) => {
                        Self::start_sync_timeout(sync_period, timeout_sender.clone())
                    },
                };
            }
        }
    }

    /// Query subroutine.
    fn query(urn: RadUrn, state: Lock, waiting_room: Shared<WaitingRoom<Instant, Duration>>) {
        tokio::spawn(async move {
            request::query(urn.clone(), state.clone(), waiting_room.clone())
                .await
                .unwrap_or_else(|err| {
                    log::warn!(
                        "an error occurred for the command 'Query' for the URN '{}':\n{}",
                        urn,
                        err
                    );
                });
        });
    }

    /// Found subroutine.
    fn found(url: RadUrl, waiting_room: Shared<WaitingRoom<Instant, Duration>>) {
        tokio::spawn(async move {
            request::found(url.clone(), waiting_room.clone())
                .await
                .unwrap_or_else(|err| {
                    log::warn!(
                        "an error occurred for the command 'Found' for the URL '{}':\n{}",
                        url,
                        err
                    );
                });
        });
    }

    /// Clone subroutine.
    fn clone(
        url: RadUrl,
        state: Lock,
        waiting_room: Shared<WaitingRoom<Instant, Duration>>,
        mut sender: mpsc::Sender<RequestEvent>,
    ) {
        tokio::spawn(async move {
            match request::clone(url.clone(), state.clone(), waiting_room.clone()).await {
                Ok(()) => sender.send(RequestEvent::Cloned(url)).await.ok(),
                Err(err) => {
                    log::warn!(
                        "an error occurred for the command 'Clone' for the URL '{}':\n{}",
                        url,
                        err
                    );
                    sender
                        .send(RequestEvent::Failed {
                            url,
                            reason: err.to_string(),
                        })
                        .await
                        .ok()
                },
            }
        });
    }

    /// Announcement subroutine.
    fn announce(state: Lock, store: kv::Store, mut sender: mpsc::Sender<AnnounceEvent>) {
        tokio::spawn(async move {
            match announcement::run(state, &store).await {
                Ok(updates) => sender.send(AnnounceEvent::Succeeded(updates)).await.ok(),
                Err(err) => {
                    log::error!("announce error: {:?}", err);
                    sender.send(AnnounceEvent::Failed).await.ok()
                },
            };
        });
    }

    /// Peer syncing subroutine.
    async fn sync(state: Lock, mut sender: mpsc::Sender<SyncEvent>, peer_id: PeerId) {
        sender.send(SyncEvent::Started(peer_id.clone())).await.ok();

        tokio::spawn(async move {
            match sync::sync(state.clone(), peer_id.clone()).await {
                Ok(_) => sender
                    .send(SyncEvent::Succeeded(peer_id.clone()))
                    .await
                    .ok(),
                Err(err) => {
                    log::error!("sync error for {}: {:?}", peer_id, err);
                    sender.send(SyncEvent::Failed(peer_id.clone())).await.ok()
                },
            };
        });
    }

    /// Sync timeout subroutine.
    fn start_sync_timeout(sync_period: Duration, sender: mpsc::Sender<TimeoutEvent>) {
        tokio::spawn(async move {
            tokio::time::delay_for(sync_period).await;
            sender.clone().send(TimeoutEvent::SyncPeriod).await.ok();
        });
    }
}
