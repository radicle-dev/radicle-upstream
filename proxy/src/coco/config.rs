//! Configuration for [`crate::coco`].

use std::convert::TryFrom;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use librad::keys;
use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;

use crate::error;

/// Path configuration
pub enum Paths {
    /// Select the default [`paths::Paths`] for configuration.
    Default,
    /// Use [`paths::Paths::from_root`] for configuration.
    FromRoot(std::path::PathBuf),
}

impl Default for Paths {
    fn default() -> Self {
        Self::Default
    }
}

impl TryFrom<Paths> for paths::Paths {
    type Error = error::Error;

    fn try_from(config: Paths) -> Result<Self, Self::Error> {
        match config {
            Paths::Default => Ok(Self::new()?),
            Paths::FromRoot(path) => Ok(Self::from_root(path)?),
        }
    }
}

/// Short-hand type for [`discovery::Static`] over a vector of [`peer::PeerId`]s and
/// [`SocketAddr`].
pub type Disco = discovery::Static<std::vec::IntoIter<(peer::PeerId, SocketAddr)>, SocketAddr>;

/// Configure a [`net::peer::PeerConfig`].
#[must_use]
pub fn configure(paths: paths::Paths, key: keys::SecretKey) -> net::peer::PeerConfig<Disco> {
    // TODO(finto): There should be a coco::config module that knows how to parse the
    // configs/parameters to give us back a `PeerConfig`

    // TODO(finto): Should be read from config file
    let gossip_params = net::gossip::MembershipParams::default();
    // TODO(finto): Read from config or passed as param
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    // TODO(finto): could we initialise with known seeds from a cache?
    let seeds: Vec<(peer::PeerId, SocketAddr)> = vec![];
    let disco = discovery::Static::new(seeds);
    // TODO(finto): read in from config or passed as param
    net::peer::PeerConfig {
        key,
        paths,
        listen_addr,
        gossip_params,
        disco,
    }
}
