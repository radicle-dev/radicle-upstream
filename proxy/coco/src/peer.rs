//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{
    convert::From,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::{
    future::{self, FutureExt as _},
    stream::StreamExt as _,
};
use tokio::{
    sync::broadcast,
    task::{JoinError, JoinHandle},
};

use librad::net::{
    peer::{Gossip, RunLoop},
    protocol,
};

use crate::state::State;

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

    /// The future was aborted.
    #[error("the running peer was aborted")]
    Aborted(#[source] future::Aborted),

    /// There was an error in a spawned task.
    #[error("the running peer was either cancelled, or one of its tasks panicked")]
    JoinError(#[source] JoinError),
}

/// Significant events that occur during [`Peer`] lifetime.
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    /// Gossiped a list of updates of new heads in our [`crate::state::State`]`.
    Announced(announcement::Updates),
    /// Received a low-level protocol event.
    Protocol(protocol::ProtocolEvent<Gossip>),
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
    state: State,
    /// On-disk storage  for caching.
    store: kv::Store,
    /// Handle used to broadcast [`Event`].
    subscriber: broadcast::Sender<Event>,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: State, store: kv::Store) -> Self {
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
    /// Calling this method spawns tasks onto the async executor, so the returned future has
    /// similar semantics to [`JoinHandle`]. Unlike [`JoinHandle`], however, dropping the
    /// [`Running`] future will cancel all its tasks. It is advisable to poll the [`Running`] future
    /// in order to be able to get notified of errors -- in which case, however, not much can be
    /// done except for creating a new [`Peer`] and put it into running state.
    pub fn into_running(self) -> Running {
        let Self {
            run_loop,
            state,
            store,
            subscriber,
        } = self;

        let protocol = {
            let (handle, reg) = future::AbortHandle::new_pair();
            let fut = future::Abortable::new(run_loop, reg);
            let join = tokio::spawn(fut);

            (join, handle)
        };

        let announce = {
            let (handle, reg) = future::AbortHandle::new_pair();
            let fut = async move {
                let mut protocol_events = state.api.protocol().subscribe().await;
                let mut timer = tokio::time::interval(Duration::from_secs(1));

                loop {
                    let res = tokio::select! {
                        _ = timer.tick() => {
                            Self::announce(state.clone(), &store).await.map(Event::Announced)
                        },
                        Some(event) = protocol_events.next() => {
                            Ok(Event::Protocol(event))
                        },
                        else => break,
                    };

                    match res {
                        // Propagate if one of the select failed.
                        Err(err) => return Err(err),
                        Ok(event) => {
                            log::info!("{:?}", event);

                            // Send will error if there are no active receivers.
                            // This case is expected and should not crash the
                            // run loop.
                            subscriber.send(event).ok();
                        },
                    }
                }

                Ok(())
            };
            let join = tokio::spawn(future::Abortable::new(fut, reg));

            (join, handle)
        };

        Running { protocol, announce }
    }

    /// Announcement subroutine.
    async fn announce(state: State, store: &kv::Store) -> Result<announcement::Updates, Error> {
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

/// Future returned by [`Peer::into_running`].
#[must_use = "to the sig hup, don't stup, don't drop"]
pub struct Running {
    protocol: (JoinHandle<Result<(), future::Aborted>>, future::AbortHandle),
    announce: (
        JoinHandle<Result<Result<(), Error>, future::Aborted>>,
        future::AbortHandle,
    ),
}

impl Running {
    /// Abort the tasks of this future.
    ///
    /// The next call to [`Future::poll`] will return an [`Error::Aborted`].
    pub fn abort(&mut self) {
        self.protocol.1.abort();
        self.announce.1.abort();
    }
}

impl From<Peer> for Running {
    fn from(peer: Peer) -> Self {
        peer.into_running()
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        self.abort()
    }
}

impl Future for Running {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let err = match self.protocol.0.poll_unpin(cx) {
            Poll::Ready(val) => match val {
                Err(e) => Some(Error::JoinError(e)),
                Ok(Err(e)) => Some(Error::Aborted(e)),
                Ok(Ok(())) => None,
            },
            Poll::Pending => None,
        };

        if let Some(err) = err {
            return Poll::Ready(Err(err));
        }

        match self.announce.0.poll_unpin(cx) {
            Poll::Ready(val) => match val {
                Err(e) => Poll::Ready(Err(Error::JoinError(e))),
                Ok(Err(e)) => Poll::Ready(Err(Error::Aborted(e))),
                Ok(Ok(Err(e))) => Poll::Ready(Err(e)),
                Ok(Ok(Ok(()))) => Poll::Ready(Ok(())),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
