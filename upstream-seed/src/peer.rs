// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use std::{collections::HashSet, iter::FromIterator, net::SocketAddr, time::Duration};

use anyhow::Context as _;
use futures::prelude::*;

use librad::{
    git::{identities, replication, storage::fetcher, tracking},
    net::{
        discovery::{self, Discovery as _},
        protocol, Network,
    },
    paths::Paths,
    PeerId,
};
use link_identities::git::Urn;

type LibradPeer = librad::net::peer::Peer<librad::SecretKey>;
type PeerInfo = librad::net::peer::PeerInfo<SocketAddr>;

/// Configuration for creating a new [`Peer`].
#[derive(Clone)]
pub struct Config {
    pub rad_paths: Paths,
    pub key: librad::SecretKey,
    pub listen: SocketAddr,
}

const PROVIDER_QUERY_TIMEOUT: Duration = Duration::from_secs(60);

/// Wrapper around [`librad::net::peer::Peer`] that provides seed specific functionality.
#[derive(Clone)]
pub struct Peer {
    librad_peer: LibradPeer,
}

impl Peer {
    /// Create a new client.
    pub fn new(config: Config) -> Self {
        let storage = librad::net::peer::config::Storage {
            protocol: librad::net::peer::config::ProtocolStorage {
                fetch_slot_wait_timeout: Default::default(),
                pool_size: 4,
            },
            user: librad::net::peer::config::UserStorage { pool_size: 4 },
        };

        let peer_config = librad::net::peer::Config {
            signer: config.key,
            protocol: protocol::Config {
                paths: config.rad_paths,
                listen_addr: config.listen,
                advertised_addrs: None, // TODO: Should we use this?
                membership: Default::default(),
                network: Network::Main,
                replication: replication::Config::default(),
                fetch: Default::default(),
                rate_limits: Default::default(),
            },
            storage,
        };
        let librad_peer = LibradPeer::new(peer_config).expect("failed to create peer");

        Self { librad_peer }
    }

    /// Run the peer by listening for incoming connections.
    ///
    /// Returns when `shutdown_signal` resolves or an error occurs.
    pub async fn run(
        &self,
        bootstrap: Vec<(PeerId, SocketAddr)>,
        shutdown_signal: impl Future<Output = ()>,
    ) -> anyhow::Result<()> {
        let librad_peer = self.librad_peer.clone();
        let static_discovery = discovery::Static::resolve(bootstrap)
            .context("failed to resolve bootstrap addresses")?;
        let shutdown_signal = shutdown_signal.shared();

        let bound = librad_peer
            .bind()
            .await
            .context("failed to bind librad peer")?;
        tracing::info!(addrs = ?bound.listen_addrs(), "peer bound");

        let (stop_accepting, listen) = bound.accept(static_discovery.clone().discover());
        let result = match future::select(shutdown_signal.clone(), listen.boxed()).await {
            future::Either::Left((_, listen)) => {
                stop_accepting();
                listen.await
            }
            future::Either::Right((listen_result, _)) => listen_result,
        };

        match result {
            // We called `stop_accepting`.
            Err(librad::net::protocol::io::error::Accept::Done) => {
                tracing::info!("peer stopped listening");
                Ok(())
            }
            Err(err) => Err(err).context("peer listening failed"),
            Ok(never) => never,
        }
    }

    /// Try to track and replicate the project by issuing a Want query to the network.
    ///
    /// Return `true` if we were able to find a peer and fetch the project from it or if the
    /// project has already been replicated. Returns `false` if no peer providing the project was
    /// found before the deadline.
    pub async fn track_project(&self, urn: Urn) -> anyhow::Result<bool> {
        if let Some(_project) = self
            .get_project(urn.clone())
            .await
            .context("failed to get project")?
        {
            return Ok(true);
        }

        let mut peers = self
            .librad_peer
            .providers(urn.clone(), PROVIDER_QUERY_TIMEOUT / 2);

        while let Some(peer_info) = peers.next().await {
            let peer_id = peer_info.peer_id;
            match self.track_project_from_peer(urn.clone(), peer_info).await {
                Ok(_) => return Ok(true),
                Err(err) => {
                    tracing::error!(%urn, %peer_id, ?err, "tracking failed")
                }
            }
        }

        Ok(false)
    }

    pub async fn track_project_from_peer(
        &self,
        urn: Urn,
        peer_info: PeerInfo,
    ) -> anyhow::Result<()> {
        self.librad_peer
            .using_storage({
                let urn = urn.clone();
                let peer_id = peer_info.peer_id;
                move |storage| tracking::track(storage, &urn, peer_id)
            })
            .await??;

        let cfg = self.librad_peer.protocol_config().replication;
        self.librad_peer
            .using_storage({
                let seen_addrs = peer_info.seen_addrs.to_vec();
                let peer_id = peer_info.peer_id;
                move |storage| -> anyhow::Result<()> {
                    let fetcher = fetcher::PeerToPeer::new(urn.clone(), peer_id, seen_addrs)
                        .build(storage)
                        .context("failed to build fetcher")?
                        .context("failed to build inner fetcher")?;

                    replication::replicate(storage, fetcher, cfg, None)
                        .context("librad replication failed")?;
                    Ok(())
                }
            })
            .await??;

        Ok(())
    }

    /// Returns stream that emits an item whenever the membership of the gossip layer changes.
    ///
    /// The stream never ends.
    pub fn membership(&self) -> impl Stream<Item = librad::net::peer::MembershipInfo> + 'static {
        self.events().filter_map({
            let librad_peer = self.librad_peer.clone();

            move |event| match event {
                librad::net::peer::ProtocolEvent::Membership(_) => {
                    let librad_peer = librad_peer.clone();
                    async move { Some(librad_peer.membership().await) }.left_future()
                }
                _ => futures::future::ready(None).right_future(),
            }
        })
    }

    /// Returns stream that emits the set of connected peers whenever it changes.
    ///
    /// The stream never ends.
    pub fn connected_peers(&self) -> impl Stream<Item = HashSet<PeerId>> + 'static {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        tokio_stream::wrappers::IntervalStream::new(interval)
            .then({
                let librad_peer = self.librad_peer.clone();

                move |_| {
                    let librad_peer = librad_peer.clone();
                    async move { HashSet::from_iter(librad_peer.connected_peers().await) }
                }
            })
            .filter_map({
                let mut prev = HashSet::new();
                move |connected| {
                    futures::future::ready(if connected == prev {
                        None
                    } else {
                        prev = connected.clone();
                        Some(connected)
                    })
                }
            })
    }

    async fn get_project(&self, urn: Urn) -> anyhow::Result<Option<link_identities::git::Project>> {
        Ok(self
            .librad_peer
            .using_storage(move |storage| identities::project::get(&storage, &urn))
            .await??)
    }

    /// Stream of events from [`LibradPeer`].
    ///
    /// It’s not guaranteed that all peer events are delivered to the stream. If items from the
    /// stream are not processed in time events may be skipped.
    ///
    /// The stream will never end.
    fn events(
        &self,
    ) -> impl Stream<Item = librad::net::peer::ProtocolEvent> + Unpin + Send + 'static {
        self.librad_peer
            .subscribe()
            .scan((), |(), res| async move {
                use tokio::sync::broadcast::error::RecvError;
                match res {
                    Ok(item) => Some(Some(item)),
                    Err(err) => match err {
                        RecvError::Closed => None,
                        RecvError::Lagged(_) => {
                            tracing::warn!("skipped peer events");
                            Some(None)
                        }
                    },
                }
            })
            .filter_map(futures::future::ready)
            .boxed()
    }
}
