//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{
    convert::From,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use futures::{
    future::{self, FutureExt as _},
    stream::StreamExt as _,
};
use tokio::{
    sync::{broadcast, mpsc},
    task::{JoinError, JoinHandle},
    time::interval,
};

use librad::{net::peer::RunLoop, peer::PeerId};

use crate::state::State;

mod announcement;
pub use announcement::Announcement;

mod run_state;
pub use run_state::{
    AnnounceConfig, AnnounceEvent, Config as RunConfig, Event, SyncConfig, SyncEvent, TimeoutEvent,
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

    /// The future was aborted.
    #[error("the running peer was aborted")]
    Aborted(#[source] future::Aborted),

    /// There was an error in a spawned task.
    #[error("the running peer was either cancelled, or one of its tasks panicked")]
    Join(#[source] JoinError),

    /// Stop-gap until we get rid of crate level errors.
    // TODO(xla): Remove once we transitioned to per module errors.
    #[error(transparent)]
    Crate(#[from] crate::error::Error),
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Peer [`librad::net::peer::RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state that is passed to subroutines.
    state: State,
    /// On-disk storage for caching.
    store: kv::Store,
    /// Handle used to broadcast [`Event`].
    subscriber: broadcast::Sender<Event>,
    /// Subroutine config.
    run_config: RunConfig,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: State, store: kv::Store, run_config: RunConfig) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        Self {
            run_loop,
            state,
            store,
            subscriber,
            run_config,
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
    /// Calling this method spawns tasks onto the async executor, so the returned future has
    /// similar semantics to [`JoinHandle`]. Unlike [`JoinHandle`], however, dropping the
    /// [`Running`] future will cancel all its tasks. It is advisable to poll the [`Running`] future
    /// in order to be able to get notified of errors -- in which case, however, not much can be
    /// done except for creating a new [`Peer`] and put it into running state.
    pub fn into_running(self) -> Running {
        self.into()
    }
}

/// [`JoinHandle`] for the protocol run loop task.
type JoinProtocol = JoinHandle<Result<(), future::Aborted>>;

/// [`JoinHandle`] for the subroutines task.
type JoinSubroutines = JoinHandle<Result<Result<(), Error>, future::Aborted>>;

/// Future returned by [`Peer::into_running`].
#[must_use = "to the sig hup, don't stup, don't drop"]
pub struct Running {
    /// Join and abort handles for the protocol run loop.
    protocol: (JoinProtocol, future::AbortHandle),
    /// Join and abort handles for the subroutines task.
    subroutines: (JoinSubroutines, future::AbortHandle),
}

impl Running {
    /// Abort the tasks of this future.
    ///
    /// The next call to [`Future::poll`] will return an [`Error::Aborted`].
    pub fn abort(&mut self) {
        self.protocol.1.abort();
        self.subroutines.1.abort();
    }
}

impl From<Peer> for Running {
    fn from(peer: Peer) -> Self {
        let Peer {
            run_loop,
            state,
            store,
            subscriber,
            run_config,
        } = peer;

        // Note: this must be spawned first to not loose the race for the first
        // `protocol_events`.
        let subroutines = {
            let fut = async move {
                let mut protocol_events = state.api.protocol().subscribe().await;
                let mut announce_timer = interval(run_config.announce.interval);

                let (mut announce_sender, mut announcements) =
                    mpsc::channel::<AnnounceEvent>(RECEIVER_CAPACITY);
                let (mut peer_sync_sender, mut peer_syncs) =
                    mpsc::channel::<SyncEvent>(RECEIVER_CAPACITY);
                let (timeout_sender, mut timeouts) =
                    mpsc::channel::<TimeoutEvent>(RECEIVER_CAPACITY);

                let mut run_state = RunState::from(run_config);

                let announcer_notifier = Arc::new(tokio::sync::Notify::new());
                let announcer_handle = {
                    let (abort_handle, abort_registration) = future::AbortHandle::new_pair();
                    let state = state.clone();
                    let store = store.clone();
                    let notifier = announcer_notifier.clone();
                    tokio::spawn(future::Abortable::new(
                        async move {
                            loop {
                                log::trace!("announcer: waiting to get notified");
                                notifier.notified().await;
                                log::trace!("announcer: notified");
                                announce(&state, &store, &mut announce_sender).await
                            }
                        },
                        abort_registration,
                    ));

                    abort_handle
                };

                let (mut peer_sync_queue, mut peer_sync_receiver) =
                    mpsc::channel::<PeerId>(RECEIVER_CAPACITY);
                let peer_sync_handle = {
                    let (abort_handle, abort_registration) = future::AbortHandle::new_pair();
                    let state = state.clone();
                    tokio::spawn(future::Abortable::new(
                        async move {
                            while let Some(peer_id) = peer_sync_receiver.recv().await {
                                log::trace!("syncer: received peer id {}", peer_id);
                                sync(&state, peer_id, &mut peer_sync_sender).await;
                            }
                        },
                        abort_registration,
                    ));

                    abort_handle
                };

                loop {
                    let event = tokio::select! {
                        _ = announce_timer.tick() => Event::Announce(AnnounceEvent::Tick),
                        Some(announce_event) = announcements.recv() => Event::Announce(announce_event),
                        Some(protocol_event) = protocol_events.next() => Event::Protocol(protocol_event),
                        Some(sync_event) = peer_syncs.recv() => Event::PeerSync(sync_event),
                        Some(timeout_event) = timeouts.recv() => {
                            peer_sync_handle.abort();
                            Event::Timeout(timeout_event)
                        },
                        else => {
                            break
                        },
                    };

                    // Send will error if there are no active receivers. This
                    // case is expected and should not terminate the run loop.
                    subscriber.send(event.clone()).ok();
                    log::debug!("{:?}", event);

                    for cmd in run_state.transition(event) {
                        match cmd {
                            Command::Announce => announcer_notifier.notify(),
                            Command::SyncPeer(peer_id) => {
                                let _ = peer_sync_queue.send(peer_id).await;
                            },
                            Command::StartSyncTimeout(sync_period) => {
                                tokio::spawn(start_sync_timeout(
                                    sync_period,
                                    timeout_sender.clone(),
                                ));
                            },
                        };
                    }
                }

                announcer_handle.abort();
                peer_sync_handle.abort();

                Ok(())
            };

            let (handle, registration) = future::AbortHandle::new_pair();
            let join = tokio::spawn(future::Abortable::new(fut, registration));

            (join, handle)
        };

        let protocol = {
            let (handle, registration) = future::AbortHandle::new_pair();
            let fut = future::Abortable::new(run_loop, registration);
            let join = tokio::spawn(fut);

            (join, handle)
        };

        Self {
            protocol,
            subroutines,
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        log::debug!("dropping `Peer`");
        self.abort()
    }
}

impl Future for Running {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let err = match self.protocol.0.poll_unpin(cx) {
            Poll::Ready(val) => match val {
                Err(e) => Some(Error::Join(e)),
                Ok(Err(e)) => Some(Error::Aborted(e)),
                Ok(Ok(())) => None,
            },
            Poll::Pending => None,
        };

        if let Some(err) = err {
            return Poll::Ready(Err(err));
        }

        match self.subroutines.0.poll_unpin(cx) {
            Poll::Ready(val) => match val {
                // Jeez...
                Err(e) => Poll::Ready(Err(Error::Join(e))),
                Ok(Err(e)) => Poll::Ready(Err(Error::Aborted(e))),
                Ok(Ok(Err(e))) => Poll::Ready(Err(e)),
                Ok(Ok(Ok(()))) => Poll::Ready(Ok(())),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Announcement subroutine.
async fn announce(state: &State, store: &kv::Store, sender: &mut mpsc::Sender<AnnounceEvent>) {
    match announcement::run(state, store).await {
        Ok(updates) => sender.send(AnnounceEvent::Succeeded(updates)).await.ok(),
        Err(err) => {
            log::error!("announce error: {:?}", err);
            sender.send(AnnounceEvent::Failed).await.ok()
        },
    };
}

/// Peer syncing subroutine.
async fn sync(state: &State, peer_id: PeerId, sender: &mut mpsc::Sender<SyncEvent>) {
    sender.send(SyncEvent::Started(peer_id.clone())).await.ok();
    match sync::sync(state, peer_id.clone()).await {
        Ok(_) => sender
            .send(SyncEvent::Succeeded(peer_id.clone()))
            .await
            .ok(),
        Err(err) => {
            log::error!("sync error for {}: {:?}", peer_id, err);
            sender.send(SyncEvent::Failed(peer_id.clone())).await.ok()
        },
    };
}

/// Sync timeout subroutine.
async fn start_sync_timeout(sync_period: Duration, mut sender: mpsc::Sender<TimeoutEvent>) {
    tokio::time::delay_for(sync_period).await;
    sender.send(TimeoutEvent::SyncPeriod).await.ok();
}
