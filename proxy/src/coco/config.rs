//! Configuration for [`proxy::coco`].

use secstr::SecUtf8;
use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;

use crate::coco;
use crate::error;
use crate::keystore;

/// Path configuration
pub enum PathsConfig {
    /// Select the default [`paths::Paths`] for configuration.
    Default,
    /// Use [`paths::Paths::from_root`] for configuration.
    FromRoot(std::path::PathBuf),
}

impl PathsConfig {
    /// Get the [`paths::Paths`] for this configuration.
    pub fn to_paths(&self) -> Result<paths::Paths, error::Error> {
        match self {
            Self::Default => Ok(paths::Paths::new()?),
            Self::FromRoot(path) => Ok(paths::Paths::from_root(path)?),
        }
    }
}

impl TryFrom<PathsConfig> for paths::Paths {
    type Error = error::Error;

    fn try_from(config: PathsConfig) -> Result<Self, Self::Error> {
        config.to_paths()
    }
}

/// Configure a [`super::Peer`].
pub async fn configure(
    path: Option<std::path::PathBuf>,
    pw: SecUtf8,
) -> Result<coco::Peer, error::Error> {
    let paths_config = path
        .map(PathsConfig::FromRoot)
        .unwrap_or(PathsConfig::Default);
    let paths = paths::Paths::try_from(paths_config)?;
    let mut store = keystore::CocoStore::new(&paths, pw)?;
    let key = store.get_key_or_create()?;

    // TODO(finto): There should be a coco::config module that knows how to parse the
    // configs/parameters to give us back a `PeerConfig`

    // TODO(finto): Should be read from config file
    let gossip_params = Default::default();
    // TODO(finto): Read from config or passed as param
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    // TODO(finto): could we initialise with known seeds from a cache?
    let seeds: Vec<(peer::PeerId, SocketAddr)> = vec![];
    let disco = discovery::Static::new(seeds);
    // TODO(finto): read in from config or passed as param
    let config = net::peer::PeerConfig {
        key,
        paths,
        listen_addr,
        gossip_params,
        disco,
    };

    Ok(coco::Peer::new(config).await?)
}
