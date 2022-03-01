// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;
use futures::prelude::*;

#[derive(Clone)]
pub struct Peer {
    paths: librad::paths::Paths,
    daemon_control: crate::daemon::PeerControl,
    librad_peer: librad::net::peer::Peer<link_crypto::BoxedSigner>,
    events: async_broadcast::InactiveReceiver<crate::daemon::PeerEvent>,
}

impl Peer {
    /// Get a reference to the peer's paths.
    pub fn paths(&self) -> &librad::paths::Paths {
        &self.paths
    }

    pub fn daemon_control(&mut self) -> &mut crate::daemon::PeerControl {
        &mut self.daemon_control
    }

    pub fn librad_peer(&self) -> &librad::net::peer::Peer<link_crypto::BoxedSigner> {
        &self.librad_peer
    }

    /// Stream that emits [`crate::daemon::PeerEvent`] and stops when the peer is shutdown.
    pub fn events(&self) -> async_broadcast::Receiver<crate::daemon::PeerEvent> {
        self.events.activate_cloned()
    }
}

pub struct Config {
    pub signer: link_crypto::BoxedSigner,
    pub paths: librad::paths::Paths,
    pub listen: std::net::SocketAddr,
    pub discovery: crate::daemon::config::StreamDiscovery,
    pub store: kv::Store,
}

pub struct Runner {
    daemon_peer:
        crate::daemon::Peer<link_crypto::BoxedSigner, crate::daemon::config::StreamDiscovery>,
}

impl Runner {
    /// Run the peer. Stops the peer when `shutdown_signal` is ready and then returns.
    pub async fn run(
        self,
        shutdown_signal: future::BoxFuture<'static, ()>,
    ) -> Result<(), crate::daemon::peer::Error> {
        let (peer_shutdown, peer_run) = self.daemon_peer.start();
        let mut shutdown_signal = shutdown_signal.fuse();
        let peer_run = peer_run.fuse();
        futures::pin_mut!(peer_run);
        futures::select! {
            _ = shutdown_signal => {
                drop(peer_shutdown);
                peer_run.await
            }
            result = peer_run => {
                result
            }
        }
    }
}

pub fn create(config: Config) -> anyhow::Result<(Peer, Runner)> {
    let daemon_config =
        crate::daemon::config::configure(config.paths.clone(), config.signer, config.listen);
    let daemon_peer = crate::daemon::Peer::new(
        daemon_config,
        config.discovery,
        config.store,
        crate::daemon::RunConfig::default(),
    )
    .context("failed to initialize crate::daemon peer")?;

    let daemon_control = daemon_peer.control();
    let librad_peer = daemon_peer.peer.clone();

    let (peer_events_tx, peer_events) = async_broadcast::broadcast(32);
    tokio::task::spawn(forward_broadcast(daemon_peer.subscribe(), peer_events_tx));

    let peer = Peer {
        paths: config.paths,
        daemon_control,
        librad_peer,
        events: peer_events.deactivate(),
    };

    let runner = Runner { daemon_peer };

    Ok((peer, runner))
}

/// Forward messages from a `tokio` broadcast receiver to an `async_broadcast` sender with message
/// overflow enabled.
///
/// The future is done and stops forwarding when either channel is closed.
async fn forward_broadcast<T: Clone>(
    mut tokio_receiver: tokio::sync::broadcast::Receiver<T>,
    mut async_sender: async_broadcast::Sender<T>,
) {
    async_sender.set_overflow(true);
    loop {
        use tokio::sync::broadcast::error::RecvError;
        match tokio_receiver.recv().await {
            Ok(item) => {
                if let Err(err) = async_sender.try_broadcast(item) {
                    match err {
                        async_broadcast::TrySendError::Full(_) => {
                            panic!("broadcast channel in overflow mode cannot be full")
                        },
                        async_broadcast::TrySendError::Closed(_) => {
                            break;
                        },
                        async_broadcast::TrySendError::Inactive(_) => {},
                    }
                }
            },
            Err(err) => match err {
                RecvError::Closed => {
                    break;
                },
                RecvError::Lagged(_) => {},
            },
        }
    }
}
