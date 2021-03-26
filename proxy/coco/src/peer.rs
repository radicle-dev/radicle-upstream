//! Machinery to advance the underlying network protocol and manage auxiliary tasks ensuring
//! prorper state updates.

use std::{io, net::SocketAddr, vec};

use futures::{
    future::{FutureExt as _, TryFutureExt as _},
    stream::StreamExt as _,
};
use tokio::{
    sync::{broadcast, mpsc, watch},
    task::JoinError,
};

use crate::state;
use librad::{net, signer::BoxedSigner};

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

    /// Encountered an I/O error, for example fetching the peer's listen addresses.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// The joining of a thread failed.
    #[error("the running peer was either cancelled, or one of its tasks panicked")]
    Join(#[source] JoinError),

    /// The protocol encountered an error.
    #[error(transparent)]
    Protocol(#[from] net::quic::Error),

    /// An interaction with the underlying storage failed.
    #[error(transparent)]
    State(#[from] state::error::Error),
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer<D> {
    /// The API for interacting with the protocol and storage.
    pub peer: net::peer::Peer<BoxedSigner>,

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

impl<D> Peer<D>
where
    D: net::discovery::Discovery<Addr = SocketAddr> + Clone + Send + Sync + 'static,
{
    /// Constructs a new [`Peer`].
    ///
    /// # Errors
    ///
    /// Failed to get the listener addresses for the peer.
    #[must_use = "give a peer some love"]
    pub fn new(
        config: net::peer::Config<BoxedSigner>,
        disco: D,
        store: kv::Store,
        run_config: RunConfig,
    ) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        let (control_sender, control_receiver) = mpsc::channel(RECEIVER_CAPACITY);
        let peer = librad::net::peer::Peer::new(config);
        Self {
            peer,
            disco,
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
    /// # Errors
    /// * Failed to accept peer connections
    /// * A subroutine panicked or was cancelled
    pub async fn run(self) -> Result<(), Error> {
        #![allow(clippy::mut_mut)]

        let Self {
            peer,
            disco,
            store,
            subscriber,
            run_config,
            control_receiver,
            ..
        } = self;
        let (addrs_tx, addrs_rx) = watch::channel(vec![]);

        let protocol_events = peer.subscribe().boxed();
        let mut subroutines = Subroutines::new(
            peer.clone(),
            addrs_rx,
            store,
            &run_config,
            protocol_events,
            subscriber,
            control_receiver,
        )
        .map_err(Error::Join);

        let protocol = async move {
            loop {
                match peer.bind().await {
                    Ok(bound) => {
                        match bound.listen_addrs() {
                            Ok(listen_addrs) => {
                                addrs_tx.send(listen_addrs).expect("subroutines is gone");
                            },
                            Err(e) => {
                                return Err(Error::Io(e));
                            },
                        }

                        if let Err(e) = net::protocol::accept(bound, disco.clone().discover()).await
                        {
                            log::error!("accept error: {}", e);
                        }
                    },
                    Err(e) => {
                        log::error!("bound error: {}", e);
                        return Err(Error::Bootstrap(e));
                    },
                }
            }
        }
        .fuse();
        futures::pin_mut!(protocol);

        futures::select! {
            res = protocol => res?,
            res = subroutines => res?
        };
        Ok(())
    }
}
