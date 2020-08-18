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
    Ok(configure(paths, key, vec![]))
}

/// Configure a [`net::peer::PeerConfig`].
#[must_use]
pub fn configure(
    paths: paths::Paths,
    key: keys::SecretKey,
    seeds: Vec<(peer::PeerId, SocketAddr)>,
) -> net::peer::PeerConfig<Disco, keys::SecretKey> {
    // TODO(finto): There should be a coco::config module that knows how to parse the
    // configs/parameters to give us back a `PeerConfig`

    // TODO(finto): Should be read from config file
    let gossip_params = net::gossip::MembershipParams::default();
    // TODO(finto): Read from config or passed as param
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let disco = discovery::Static::new(seeds);

    // TODO(finto): read in from config or passed as param
    net::peer::PeerConfig {
        signer: key,
        paths,
        listen_addr,
        gossip_params,
        disco,
    }
}

/// Resolve seed identifiers into `(PeerId, SocketAddr)` pairs.
///
/// The expected format is `<peer-id>@<host>:<port>`
///
/// # Errors
///
/// If any of the supplied seeds cannot be parsed, an error is returned.
pub async fn resolve_seeds<T: AsRef<str> + Send + Sync>(
    seeds: &[T],
) -> Result<Vec<(peer::PeerId, SocketAddr)>, error::Error> {
    let mut resolved = Vec::with_capacity(seeds.len());

    for seed in seeds.iter() {
        let seed = seed.as_ref();

        if let Some(ix) = seed.chars().position(|c| c == '@') {
            let (peer_id, rest) = seed.split_at(ix);
            #[allow(clippy::indexing_slicing)]
            let host = &rest[1..]; // Skip '@'

            if let Some(addr) = tokio::net::lookup_host(host).await?.next() {
                let peer_id = peer::PeerId::from_default_encoding(peer_id)
                    .map_err(|err| error::Error::InvalidSeed(seed.to_string(), Some(err)))?;

                resolved.push((peer_id, addr));
            }
        } else {
            return Err(error::Error::InvalidSeed(seed.to_string(), None));
        }
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use std::net;

    #[tokio::test]
    async fn test_resolve_seeds() {
        let seeds = super::resolve_seeds(&[
            "hydsst3z3d5bc6pxq4gz1g4cu6sgbx38czwf3bmmk3ouz4ibjbbtds@localhost:9999",
        ])
        .await
        .expect("a valid seed doesn't return an error");

        let expected: net::SocketAddr = ([127, 0, 0, 1], 9999).into();

        assert!(
            matches!(seeds.first(), Some((_, addr)) if *addr == expected),
            "{:?}",
            seeds
        );

        super::resolve_seeds(&[String::from("hydsst3obtds@localhost:9999")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve_seeds(&[String::from("localhost:9999")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve_seeds(&[String::from("hydsst3obtds@localhost")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve_seeds(&[String::from("hydsst3obtds")])
            .await
            .expect_err("an invalid seed returns an error");
    }
}
