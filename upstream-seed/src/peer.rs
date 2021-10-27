// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use std::{net::SocketAddr, time::Duration};

use anyhow::Context as _;
use futures::stream::StreamExt as _;

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

    /// Run the peer. This function runs indefinitely until a fatal error occurs.
    pub async fn run(self, bootstrap: Vec<(PeerId, SocketAddr)>) {
        let disco = discovery::Static::resolve(bootstrap).unwrap();

        loop {
            match self.librad_peer.bind().await {
                Ok(bound) => {
                    let (_kill, run) = bound.accept(disco.clone().discover());

                    if let Err(err) = run.await {
                        tracing::error!(?err, "failed to accept on bound librad peer");
                    }
                }
                Err(err) => {
                    tracing::error!(?err, "failed to bind librad peer");
                    tokio::time::sleep(Duration::from_secs(2)).await
                }
            }
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

    async fn get_project(&self, urn: Urn) -> anyhow::Result<Option<link_identities::git::Project>> {
        Ok(self
            .librad_peer
            .using_storage(move |storage| identities::project::get(&storage, &urn))
            .await??)
    }
}
