// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Machinery to advance the underlying network protocol and manage auxiliary
//! tasks ensuring prorper state updates.

use std::{io, net::SocketAddr, vec};

use futures::{
    future::{Either, FutureExt as _, TryFutureExt as _},
    stream::StreamExt as _,
    Future,
};
use tokio::{
    sync::{broadcast, mpsc, watch},
    task::JoinError,
};

use crate::daemon::state;
use librad::{net, Signer};

pub mod announcement;
pub use announcement::Announcement;

mod control;
pub use control::Control;

pub mod gossip;

pub mod include;

mod run_state;
pub use run_state::{config as run_config, Config as RunConfig, Event, Status, WaitingRoomEvent};

mod subroutines;
use subroutines::Subroutines;

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

    /// Encountered an I/O error, for example fetching the peer's listen
    /// addresses.
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

    /// Peer initialisation error.
    #[error(transparent)]
    Init(#[from] net::peer::error::Init),
}

/// Local peer to participate in the radicle code-collaboration network.
pub struct Peer<S, D> {
    /// The API for interacting with the protocol and storage.
    pub peer: net::peer::Peer<S>,

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

impl<S, D> Peer<S, D>
where
    S: Clone + Signer,
    D: net::discovery::Discovery<Addr = SocketAddr> + Clone + Send + Sync + 'static,
{
    /// Constructs a new [`Peer`].
    ///
    /// To kick-off the peer's subroutines be sure to use
    /// [`start`][`Self::start`].
    ///
    /// # Errors
    ///
    /// If the underlying [`librad::net::peer::Peer`] fails to initialise.
    #[must_use = "give a peer some love"]
    pub fn new(
        config: net::peer::Config<S>,
        disco: D,
        store: kv::Store,
        run_config: RunConfig,
    ) -> Result<Self, Error>
    where
        S: Clone + Signer,
    {
        let peer = librad::net::peer::Peer::new(config)?;
        Ok(Self::with_peer(peer, disco, store, run_config))
    }

    /// Construct a new [`Peer`] using an existing [`net::peer::Peer`].
    ///
    /// To kick-off the peer's subroutines be sure to use
    /// [`start`][`Self::start`].
    #[must_use = "give a peer some love"]
    pub fn with_peer(
        peer: net::peer::Peer<S>,
        disco: D,
        store: kv::Store,
        run_config: RunConfig,
    ) -> Self {
        let (subscriber, _receiver) = broadcast::channel(RECEIVER_CAPACITY);
        let (control_sender, control_receiver) = mpsc::channel(RECEIVER_CAPACITY);
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
    /// NB(xla): A caller must call this before the run loop is started, as that
    /// consumes the peer. There is also a configured [`RECEIVER_CAPACITY`],
    /// which prevents unbounded queues fron filling up.
    #[must_use = "eat your events"]
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.subscriber.subscribe()
    }

    /// Returns a future that runs the peer and a handle that shuts the peer
    /// down when it is dropped.
    ///
    /// The function only returns when an error occurs or the [`Shutdown`] value
    /// is dropped. becomes ready.
    ///
    /// The future returned by this function must be run to completion for the
    /// daemon to shut down properly.
    ///
    /// # Errors
    /// * Failed to accept peer connections
    /// * A subroutine panicked or was cancelled
    ///
    /// # Panics
    /// * If the subroutine is gone when the protocol network is still setting up shop.
    pub fn start(self) -> (Shutdown, impl Future<Output = Result<(), Error>>) {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
        let shutdown = Shutdown(shutdown_tx.clone());

        // We move all code inside the future so that this function can be called
        // without a runtime being present yet.
        let run = async move {
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
            let subroutines = Subroutines::new(
                peer.clone(),
                addrs_rx,
                store,
                &run_config,
                protocol_events,
                subscriber,
                control_receiver,
            )
            .run()
            .fuse()
            .map_err(Error::Join);

            let protocol = async move {
                loop {
                    match peer.bind().await {
                        Ok(bound) => {
                            addrs_tx
                                .send(bound.listen_addrs())
                                .expect("subroutines is gone");
                            let (stop_accepting, run) = bound.accept(disco.clone().discover());
                            let shutdown_recv = shutdown_rx.recv();
                            futures::pin_mut!(shutdown_recv);
                            futures::pin_mut!(run);
                            let result = match futures::future::select(shutdown_recv, run).await {
                                Either::Left((_, run)) => {
                                    stop_accepting();
                                    run.await
                                },
                                Either::Right((run_result, _)) => run_result,
                            };
                            match result {
                                Err(net::protocol::io::error::Accept::Done) => {
                                    tracing::info!("network endpoint shut down");
                                    return Ok(());
                                },
                                Err(error) => {
                                    tracing::error!(?error, "accept error");
                                },
                                Ok(never) => unreachable!("absurd: {}", never),
                            };
                        },
                        Err(e) => {
                            tracing::error!(error = ?e, "bound error");
                            return Err(Error::Bootstrap(e));
                        },
                    }
                }
            }
            .fuse();

            futures::pin_mut!(subroutines);
            futures::pin_mut!(protocol);
            match futures::future::select(subroutines, protocol).await {
                Either::Left((result, protocol)) => {
                    let _result = shutdown_tx.send(());

                    protocol.await?;
                    result
                },
                Either::Right((result, _subroutines)) => result,
            }
        };

        (shutdown, run)
    }
}

/// Shutdown handle returned by [`Peer::start`].
///
/// If this value is dropped, the peer will shutdown.
#[derive(Debug)]
pub struct Shutdown(mpsc::Sender<()>);

impl Drop for Shutdown {
    fn drop(&mut self) {
        // If this errors, the peer has already shut down.
        let _ = self.0.try_send(());
    }
}
