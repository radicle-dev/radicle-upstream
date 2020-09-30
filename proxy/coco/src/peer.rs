//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{
    convert::From,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
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

use librad::{
    net::peer::RunLoop,
    peer::PeerId,
    uri::{RadUrl, RadUrn},
};

use crate::{
    request::waiting_room::{self, WaitingRoom},
    shared::Shared,
    state::State,
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

/// The period at which we ping the `select!` loop.
const PING_PERIOD: Duration = Duration::from_millis(500);

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
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    /// Handle used to broadcast [`Event`].
    subscriber: broadcast::Sender<Event>,
    /// Subroutine config.
    run_config: RunConfig,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(
        run_loop: RunLoop,
        state: State,
        store: kv::Store,
        waiting_room: Shared<WaitingRoom<Instant, Duration>>,
        run_config: RunConfig,
    ) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        Self {
            run_loop,
            state,
            store,
            waiting_room,
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
type JoinSubroutines = JoinHandle<Result<(), Error>>;

/// Future returned by [`Peer::into_running`].
#[must_use = "to the sig hup, don't stup, don't drop"]
pub struct Running {
    /// Join and abort handles for the protocol run loop.
    protocol: (JoinProtocol, future::AbortHandle),
    /// Join and abort handles for the subroutines task.
    subroutines: (JoinSubroutines, Arc<tokio::sync::Notify>),
}

impl Running {
    /// Abort the tasks of this future.
    ///
    /// The next call to [`Future::poll`] will return an [`Error::Aborted`].
    pub fn abort(&mut self) {
        self.protocol.1.abort();
        self.subroutines.1.notify();
    }
}

impl From<Peer> for Running {
    fn from(peer: Peer) -> Self {
        let Peer {
            run_loop,
            state,
            store,
            waiting_room,
            subscriber,
            run_config,
        } = peer;

        // Note: this must be spawned first to not lose the race for the first
        // protocol event
        let subroutines = {
            let stop = Arc::new(tokio::sync::Notify::new());
            let fut = subroutines(
                state,
                store,
                waiting_room,
                run_config,
                subscriber,
                stop.clone(),
            );
            let join = tokio::spawn(fut);

            (join, stop)
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
                Err(e) => Poll::Ready(Err(Error::Join(e))),
                Ok(Err(e)) => Poll::Ready(Err(e)),
                Ok(Ok(())) => Poll::Ready(Ok(())),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Schedule subroutines.
async fn subroutines(
    state: State,
    store: kv::Store,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    run_config: RunConfig,
    subscriber: broadcast::Sender<Event>,
    stop: Arc<tokio::sync::Notify>,
) -> Result<(), Error> {
    let mut protocol_events = state.api.protocol().subscribe().await;
    let mut announce_timer = interval(run_config.announce.interval);
    let mut ping = interval(PING_PERIOD);

    let (announce_sender, mut announcements) = mpsc::channel::<AnnounceEvent>(RECEIVER_CAPACITY);
    let (peer_sync_sender, mut peer_syncs) = mpsc::channel::<SyncEvent>(RECEIVER_CAPACITY);
    let (timeout_sender, mut timeouts) = mpsc::channel::<TimeoutEvent>(RECEIVER_CAPACITY);
    let (request_sender, mut requests) = mpsc::channel::<RequestEvent>(RECEIVER_CAPACITY);

    let request_queries = waiting_room::stream::Queries::new(waiting_room.clone().value);
    tokio::pin!(request_queries);
    let request_clones = waiting_room::stream::Clones::new(waiting_room.clone().value);
    tokio::pin!(request_clones);

    let mut run_state = RunState::from(run_config);
    loop {
        let event = tokio::select! {
            _ = ping.tick() => Event::Ping,
            _ = announce_timer.tick() => Event::Announce(AnnounceEvent::Tick),
            Some(announce_event) = announcements.recv() => Event::Announce(announce_event),
            Some(protocol_event) = protocol_events.next() => Event::Protocol(protocol_event),
            Some(sync_event) = peer_syncs.recv() => Event::PeerSync(sync_event),
            Some(timeout_event) = timeouts.recv() => Event::Timeout(timeout_event),
            Some(urn) = request_queries.next() => Event::Request(RequestEvent::Query(urn)),
            Some(url) = request_clones.next() => Event::Request(RequestEvent::Clone(url)),
            Some(request_event) = requests.next() => Event::Request(request_event),
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
                Command::Request(RequestCommand::Query(urn)) => {
                    let stop = stop.clone();
                    query(
                        urn,
                        state.clone(),
                        waiting_room.clone(),
                        async move { stop.notified().await }.boxed(),
                    );
                },
                Command::Request(RequestCommand::Found(url)) => {
                    let stop = stop.clone();
                    found(
                        url,
                        waiting_room.clone(),
                        async move { stop.notified().await }.boxed(),
                    );
                },
                Command::Request(RequestCommand::Clone(url)) => {
                    let stop = stop.clone();
                    clone(
                        url,
                        state.clone(),
                        waiting_room.clone(),
                        request_sender.clone(),
                        async move { stop.notified().await }.boxed(),
                    );
                },
                Command::Announce => {
                    let stop = stop.clone();
                    announce(
                        state.clone(),
                        store.clone(),
                        announce_sender.clone(),
                        async move { stop.notified().await }.boxed(),
                    )
                },
                Command::SyncPeer(peer_id) => {
                    // Cancel when either the parent shuts down, or the timeout
                    // task fired
                    let stopper = {
                        let on_stop = {
                            let stop = stop.clone();
                            async move { stop.notified().await }
                        };
                        let on_timeout = {
                            let events = subscriber.subscribe();
                            async move {
                                tokio::stream::StreamExt::skip_while(events, |e| match e {
                                    Ok(Event::Timeout(TimeoutEvent::SyncPeriod)) | Err(_) => false,
                                    _ => true,
                                })
                                .map(|_| ())
                                .next()
                                .await
                            }
                        };

                        future::select(on_stop.boxed(), on_timeout.boxed()).map(|_| ())
                    };

                    sync(
                        state.clone(),
                        peer_id.clone(),
                        peer_sync_sender.clone(),
                        stopper,
                    );
                },
                Command::StartSyncTimeout(sync_period) => {
                    start_sync_timeout(sync_period, timeout_sender.clone());
                },
            };
        }
    }

    Ok(())
}

/// Announcement subroutine.
fn announce(
    state: State,
    store: kv::Store,
    mut sender: mpsc::Sender<AnnounceEvent>,
    stop: impl Future<Output = ()> + Send + Unpin + 'static,
) {
    tokio::spawn(async move {
        let go = async {
            match announcement::run(&state, &store).await {
                Ok(updates) => {
                    sender.send(AnnounceEvent::Succeeded(updates)).await.ok();
                },
                Err(err) => {
                    log::error!("announce error: {:?}", err);
                    sender.send(AnnounceEvent::Failed).await.ok();
                },
            }
        };
        tokio::pin!(go);

        future::select(stop, go).map(|_| ()).await;
    });
}

/// Peer syncing subroutine.
fn sync(
    state: State,
    peer_id: PeerId,
    mut sender: mpsc::Sender<SyncEvent>,
    stop: impl Future<Output = ()> + Send + Unpin + 'static,
) {
    tokio::spawn(async move {
        sender.send(SyncEvent::Started(peer_id.clone())).await.ok();

        let go = async {
            match sync::sync(&state, peer_id.clone()).await {
                Ok(_) => {
                    sender
                        .send(SyncEvent::Succeeded(peer_id.clone()))
                        .await
                        .ok();
                },
                Err(err) => {
                    log::error!("sync error for {}: {:?}", peer_id, err);
                    sender.send(SyncEvent::Failed(peer_id.clone())).await.ok();
                },
            }
        };
        tokio::pin!(go);

        future::select(stop, go).map(|_| ()).await;
    });
}

/// Sync timeout subroutine.
fn start_sync_timeout(sync_period: Duration, mut sender: mpsc::Sender<TimeoutEvent>) {
    tokio::spawn(async move {
        tokio::time::delay_for(sync_period).await;
        sender.send(TimeoutEvent::SyncPeriod).await.ok();
    });
}

/// Query subroutine.
fn query(
    urn: RadUrn,
    state: State,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    stop: impl Future<Output = ()> + Send + Unpin + 'static,
) {
    tokio::spawn(async move {
        let go = async {
            request::query(urn.clone(), state.clone(), waiting_room.clone())
                .await
                .unwrap_or_else(|err| {
                    log::warn!(
                        "an error occurred for the command 'Query' for the URN '{}':\n{}",
                        urn,
                        err
                    );
                });
        };
        tokio::pin!(go);

        future::select(stop, go).map(|_| ()).await;
    });
}

/// Found subroutine.
fn found(
    url: RadUrl,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    stop: impl Future<Output = ()> + Send + Unpin + 'static,
) {
    tokio::spawn(async move {
        let go = async {
            request::found(url.clone(), waiting_room.clone())
                .await
                .unwrap_or_else(|err| {
                    log::warn!(
                        "an error occurred for the command 'Found' for the URL '{}':\n{}",
                        url,
                        err
                    );
                });
        };
        tokio::pin!(go);

        future::select(stop, go).map(|_| ()).await;
    });
}

/// Clone subroutine.
fn clone(
    url: RadUrl,
    state: State,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    mut sender: mpsc::Sender<RequestEvent>,
    stop: impl Future<Output = ()> + Send + Unpin + 'static,
) {
    tokio::spawn(async move {
        let go = async {
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
        };
        tokio::pin!(go);

        future::select(stop, go).map(|_| ()).await;
    });
}
