//! Crate configuration.

use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use async_stream::stream;
use futures::stream::BoxStream;
use tokio::sync::watch;

use librad::{
    net,
    net::discovery,
    paths,
    peer::PeerId,
    signer::{BoxedSigner, Signer},
};

use crate::seed;

lazy_static::lazy_static! {
    /// Localhost binding to any available port, i.e. `127.0.0.1:0`.
    pub static ref LOCALHOST_ANY: SocketAddr =
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
}

/// The environment variable that points to where librad data lives.
pub const RAD_HOME: &str = "RAD_HOME";

/// The default name for a user's remote, which is `"rad"`.
pub const RAD_REMOTE: &str = "rad";

/// Provide the default config.
///
/// Address: 127.0.0.1:0
/// No seeds.
/// Default gossip parameters.
///
/// # Errors
///
/// Results in an error if the [`paths::Paths`] could not be created.
pub fn default(
    signer: BoxedSigner,
    path: impl AsRef<std::path::Path>,
) -> Result<net::peer::Config<BoxedSigner>, io::Error> {
    let paths = paths::Paths::from_root(path)?;
    Ok(configure(paths, signer, *LOCALHOST_ANY))
}

/// Configure a [`net::peer::Config`].
#[allow(clippy::as_conversions)]
#[must_use]
pub fn configure<S>(paths: paths::Paths, signer: S, listen_addr: SocketAddr) -> net::peer::Config<S>
where
    S: Signer + Clone + Send + Sync + 'static,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    net::peer::Config {
        signer,
        protocol: net::protocol::Config {
            paths,
            listen_addr,
            membership: net::protocol::membership::Params::default(),
            network: net::Network::default(),
            replication: librad::git::replication::Config::default(),
        },
        storage_pools: net::peer::PoolSizes::default(),
    }
}

/// Builds a static discovery over the list of given `seeds`.
#[allow(clippy::as_conversions)]
#[must_use]
pub fn static_seed_discovery(seeds: &[seed::Seed]) -> discovery::Static {
    discovery::Static::resolve(
        seeds
            .iter()
            .map(|seed| (seed.peer_id, seed.addrs.as_slice())),
    )
    .expect("unable to resolve seeds")
}

/// Stream based discovery based on a watch.
#[derive(Clone)]
pub struct StreamDiscovery {
    /// Stream of new sets of seeds coming from configuration changes.
    seeds_receiver: watch::Receiver<Vec<seed::Seed>>,
}

impl StreamDiscovery {
    /// Returns a new streaming discovery.
    #[must_use]
    pub fn new(seeds_receiver: watch::Receiver<Vec<seed::Seed>>) -> Self {
        Self { seeds_receiver }
    }
}

impl discovery::Discovery for StreamDiscovery {
    type Addr = SocketAddr;
    type Stream = BoxStream<'static, (PeerId, Vec<SocketAddr>)>;

    fn discover(mut self) -> Self::Stream {
        let updates = stream! {
            loop {
                let seeds = self.seeds_receiver.borrow().clone();
                for seed in seeds {
                    yield seed.into();
                }
                self.seeds_receiver.changed().await.unwrap();
            }
        };

        Box::pin(updates)
    }
}
