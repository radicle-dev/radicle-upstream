//! Configuration for [`crate::coco`].

use std::convert::TryFrom;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use librad::keys;
use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;

use crate::coco::seed;
use crate::error;

lazy_static! {
    /// Localhost binding to any available port, i.e. `127.0.0.1:0`.
    pub static ref LOCALHOST_ANY: SocketAddr =
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
}

/// The environment variable that points to where librad data lives.
pub const RAD_HOME: &str = "RAD_HOME";

/// The default name for a user's remote, which is `"rad"`.
pub const RAD_REMOTE: &str = "rad";

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
pub type Disco = discovery::Static<
    std::iter::Map<std::vec::IntoIter<seed::Seed>, fn(seed::Seed) -> (peer::PeerId, SocketAddr)>,
    SocketAddr,
>;

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
    key: keys::SecretKey,
    path: impl AsRef<std::path::Path>,
) -> Result<net::peer::PeerConfig<Disco, keys::SecretKey>, error::Error> {
    let paths = paths::Paths::from_root(path)?;
    Ok(configure(paths, key, *LOCALHOST_ANY, vec![]))
}

/// Configure a [`net::peer::PeerConfig`].
#[must_use]
#[allow(clippy::as_conversions)]
pub fn configure(
    paths: paths::Paths,
    key: keys::SecretKey,
    listen_addr: SocketAddr,
    seeds: Vec<seed::Seed>,
) -> net::peer::PeerConfig<Disco, keys::SecretKey> {
    let gossip_params = net::gossip::MembershipParams::default();
    let disco = discovery::Static::new(
        seeds
            .into_iter()
            .map(seed::Seed::into as fn(seed::Seed) -> (peer::PeerId, SocketAddr)),
    );

    net::peer::PeerConfig {
        signer: key,
        paths,
        listen_addr,
        gossip_params,
        disco,
    }
}
