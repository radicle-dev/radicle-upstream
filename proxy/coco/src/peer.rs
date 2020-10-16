//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::{future::FutureExt as _, stream::StreamExt as _};
use tokio::sync::{broadcast, mpsc, Barrier};

use librad::net::peer::RunLoop;

use crate::{
    spawn_abortable::{self, SpawnAbortable},
    state::{self, State},
};

mod announcement;
pub use announcement::Announcement;

mod control;
pub use control::Control;

pub mod gossip;

mod run_state;
pub use run_state::{AnnounceConfig, Config as RunConfig, Event, Status, SyncConfig};

mod subroutines;
use subroutines::Subroutines;

mod sync;

/// Upper bound of messages stored in receiver channels.
pub const RECEIVER_CAPACITY: usize = 128;

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to build and announce state updates.
    #[error(transparent)]
    Announcement(#[from] announcement::Error),

    /// There was an error in a spawned task.
    #[error("the running peer was either cancelled, or one of its tasks panicked")]
    Spawn(#[source] spawn_abortable::Error),

    /// There was an error when interacting with [`State`].
    #[error(transparent)]
    State(#[from] state::Error),
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer {
    /// Peer [`librad::net::peer::RunLoop`] to advance the network protocol.
    run_loop: RunLoop,
    /// Underlying state that is passed to subroutines.
    state: State,
    /// On-disk storage for caching.
    store: kv::Store,
    /// Handle used to broadcast [`Event`]s.
    subscriber: broadcast::Sender<Event>,
    /// Subroutine config.
    run_config: RunConfig,
    /// Receiving end of requests fired from control handles.
    control_receiver: mpsc::Receiver<control::Request>,
    /// Sending end for control handles to send requests to the running peer.
    control_sender: mpsc::Sender<control::Request>,
}

impl Peer {
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(run_loop: RunLoop, state: State, store: kv::Store, run_config: RunConfig) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        let (control_sender, control_receiver) = mpsc::channel(RECEIVER_CAPACITY);
        Self {
            run_loop,
            state,
            store,
            subscriber,
            run_config,
            control_receiver,
            control_sender,
        }
    }

    /// Acquire a handle to inspect state and perform actions on a running peer.
    #[must_use = "take control"]
    pub fn control(&self) -> Control {
        Control::new(self.control_sender.clone())
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
    /// The returned [`Running`] future has similar semantics to [`tokio::task::JoinHandle`]:
    /// internally, all tasks are spawned immediately, and polling the future is only necessary
    /// to get notified of errors. Unlike [`tokio::task::JoinHandle`], however, [`Running`] does
    /// not detach the tasks. That is, if and when [`Running`] is dropped, all tasks are
    /// cancelled.
    pub fn into_running(self) -> Running {
        let Self {
            run_loop,
            state,
            store,
            subscriber,
            run_config,
            control_receiver,
            ..
        } = self;

        // Rendezvous on a barrier to let the subroutines subscribe for protocol
        // events before it actually starts. As `Protocol::subscribe` is async,
        // we would otherwise need to make `into_running` async as well, which
        // yields the weird requirement to double `.await` it.
        let barrier = Arc::new(Barrier::new(2));
        let subroutines = {
            let barrier = barrier.clone();
            SpawnAbortable::new(async move {
                let protocol_events = state.api.protocol().subscribe().await.boxed();
                barrier.wait().await;
                Subroutines::new(
                    state,
                    store,
                    run_config,
                    protocol_events,
                    subscriber,
                    control_receiver,
                )
                .await
            })
        };
        let protocol = SpawnAbortable::new(async move {
            barrier.wait().await;
            run_loop.await
        });

        Running {
            protocol,
            subroutines,
        }
    }
}

/// Future returned by [`Peer::into_running`].
#[must_use = "to the sig hup, don't stop, just drop"]
pub struct Running {
    /// Join and abort handles for the protocol run loop.
    protocol: SpawnAbortable<()>,
    /// The [`Subroutines`] associated with this [`Peer`] instance.
    subroutines: SpawnAbortable<Result<(), spawn_abortable::Error>>,
}

impl Drop for Running {
    fn drop(&mut self) {
        log::trace!("`peer::Running` is being dropped");
        self.protocol.abort();
        self.subroutines.abort();
    }
}

impl Future for Running {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let err = match self.protocol.poll_unpin(cx) {
            Poll::Ready(val) => match val {
                Err(e) => Some(Error::Spawn(e)),
                Ok(()) => None,
            },
            Poll::Pending => None,
        };

        if let Some(err) = err {
            log::trace!("run loop error: {:?}", err);
            return Poll::Ready(Err(err));
        }

        match self.subroutines.poll_unpin(cx) {
            Poll::Ready(val) => {
                let val = match val {
                    Err(e) | Ok(Err(e)) => Err(Error::Spawn(e)),
                    Ok(Ok(())) => Ok(()),
                };
                Poll::Ready(val)
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
