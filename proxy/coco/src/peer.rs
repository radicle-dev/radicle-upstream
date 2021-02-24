//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{
    future::Future,
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::FutureExt as _, stream::StreamExt as _};
use tokio::{
    sync::{broadcast, mpsc},
    task::{JoinError, JoinHandle},
};

use librad::{net, net::protocol, signer::BoxedSigner};
use crate::{seed::Seed, state};

mod announcement;
pub use announcement::Announcement;

mod control;
pub use control::Control;

pub mod gossip;

pub mod include;

mod run_state;
pub use run_state::{config as run_config, Config as RunConfig, Event, Status};

mod subroutines;
use subroutines::Subroutines;

mod sync;

mod waiting_room;

/// Upper bound of messages stored in receiver channels.
pub const RECEIVER_CAPACITY: usize = 128;

/// Peer operation errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to build and announce state updates.
    #[error(transparent)]
    Announcement(#[from] announcement::Error),

    /// Peer bootstrap error.
    #[error(transparent)]
    Bootstrap(#[from] net::protocol::error::Bootstrap),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("the running peer was either cancelled, or one of its tasks panicked")]
    Join(#[source] JoinError),

    #[error(transparent)]
    Protocol(#[from] net::quic::Error),

    #[error(transparent)]
    State(#[from] state::error::Error),
}

/// Constructs a [`Peer`] and [`State`] pair from a [`net::peer::PeerConfig`].
///
/// # Errors
///
/// * peer construction from config fails.
/// * accept on the peer fails.
pub async fn bootstrap<D>(
    config: net::peer::Config<BoxedSigner>,
    disco: D,
    store: kv::Store,
    run_config: RunConfig,
) -> Result<Peer<D>, Error>
where
    D: net::discovery::Discovery<Addr = SocketAddr> + Send + 'static,
    <D as net::discovery::Discovery>::Stream: 'static,
{
    let peer = librad::net::peer::Peer::new(config);
    let bound = peer.bind().await?;

    let peer = Peer::new(peer, bound, disco, store, run_config)?;

    Ok(peer)
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer<D> {
    pub peer: net::peer::Peer<BoxedSigner>,
    pub listen_addrs: Vec<SocketAddr>,
    bound: protocol::Bound<net::peer::PeerStorage>,
    disco: D,

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

impl<D> From<&Peer<D>> for Seed {
    fn from(peer: &Peer<D>) -> Self {
        Self {
            peer_id: peer.peer.peer_id(),
            addrs: peer.listen_addrs.clone(),
        }
    }
}

impl<D> Peer<D>
where
    D: net::discovery::Discovery<Addr = SocketAddr> + Send + 'static,
{
    /// Constructs a new [`Peer`].
    #[must_use = "give a peer some love"]
    pub fn new(
        peer: net::peer::Peer<BoxedSigner>,
        bound: protocol::Bound<net::peer::PeerStorage>,
        disco: D,
        store: kv::Store,
        run_config: RunConfig,
    ) -> Result<Self, Error> {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        let (control_sender, control_receiver) = mpsc::channel(RECEIVER_CAPACITY);
        Ok(Self {
            peer,
            listen_addrs: bound.listen_addrs()?,
            bound,
            disco,
            store,
            subscriber,
            run_config,
            control_receiver,
            control_sender,
        })
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
            peer,
            bound,
            disco,
            store,
            subscriber,
            run_config,
            control_receiver,
            ..
        } = self;

        let subroutines = tokio::spawn(async move {
            let protocol_events = peer.subscribe().boxed();
            Subroutines::new(
                peer.clone(),
                store,
                run_config,
                protocol_events,
                subscriber,
                control_receiver,
            )
            .await
        });
        let protocol = tokio::spawn(net::protocol::accept(bound, disco.discover()));

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
    protocol: JoinHandle<Result<!, net::quic::Error>>,
    /// The [`Subroutines`] associated with this [`Peer`] instance.
    subroutines: JoinHandle<Result<(), JoinError>>,
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
                Err(e) => Some(Error::Join(e)),
                Ok(res) => Some(Error::Protocol(res.err().unwrap())),
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
                    Err(e) | Ok(Err(e)) => Err(Error::Join(e)),
                    Ok(Ok(())) => Ok(()),
                };
                Poll::Ready(val)
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
