//! Seed nodes.
use std::{io, net::SocketAddr};

use librad::peer;

/// Errors that occur when resolving seed addresses.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Seed DNS failed to resolve to an address.
    #[error("the seed '{0}' failed to resolve to an address")]
    DnsLookupFailed(String),

    /// Seed input is invalid.
    #[error("the seed '{0}' is invalid: {:1}")]
    InvalidSeed(String, Option<librad::peer::conversion::Error>),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// A peer used to seed our client.
#[derive(Debug, Clone, PartialEq)]
pub struct Seed {
    /// The seed peer id.
    pub peer_id: peer::PeerId,
    /// The seed address.
    pub addrs: Vec<SocketAddr>,
}

impl From<Seed> for (peer::PeerId, Vec<SocketAddr>) {
    fn from(seed: Seed) -> Self {
        (seed.peer_id, seed.addrs)
    }
}

impl Seed {
    /// Create a seed from a string.
    ///
    /// # Errors
    ///
    /// If the supplied seed cannot be parsed or resolved, an error is returned.
    #[allow(clippy::indexing_slicing)]
    pub async fn from_str(seed: &str) -> Result<Self, Error> {
        if let Some(ix) = seed.chars().position(|c| c == '@') {
            let (peer_id, rest) = seed.split_at(ix);
            let host = &rest[1..]; // Skip '@'

            if let Some(addr) = tokio::net::lookup_host(host).await?.next() {
                let peer_id = peer::PeerId::from_default_encoding(peer_id)
                    .map_err(|err| Error::InvalidSeed(seed.to_string(), Some(err)))?;

                Ok(Self {
                    peer_id,
                    addrs: vec![addr],
                })
            } else {
                Err(Error::DnsLookupFailed(seed.to_string()))
            }
        } else {
            Err(Error::InvalidSeed(seed.to_string(), None))
        }
    }
}

/// Resolve seed identifiers into `(PeerId, SocketAddr)` pairs.
///
/// The expected format is `<peer-id>@<host>:<port>`
///
/// # Errors
///
/// If any of the supplied seeds cannot be parsed or resolved, an error is returned.
pub async fn resolve<T: AsRef<str> + Send + Sync>(seeds: &[T]) -> Result<Vec<Seed>, Error> {
    let mut resolved = Vec::with_capacity(seeds.len());

    for seed in seeds.iter() {
        let seed = seed.as_ref();
        resolved.push(Seed::from_str(seed).await?);
    }

    Ok(resolved)
}

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::net;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_resolve_seeds() -> Result<(), super::Error> {
        let seeds = super::resolve(&[
            "hydsst3z3d5bc6pxq4gz1g4cu6sgbx38czwf3bmmk3ouz4ibjbbtds@localhost:9999",
        ])
        .await?;

        assert!(!seeds.is_empty(), "seeds should not be empty");

        if let Some(super::Seed { addrs, .. }) = seeds.first() {
            let addr = addrs.first().unwrap();
            let expected: net::SocketAddr = match *addr {
                net::SocketAddr::V4(_addr) => ([127, 0, 0, 1], 9999).into(),
                net::SocketAddr::V6(_addr) => "[::1]:9999".parse().expect("valid ivp6 address"),
            };

            assert_eq!(expected, *addr);
        }

        super::resolve(&[String::from("hydsst3obtds@localhost:9999")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve(&[String::from("localhost:9999")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve(&[String::from("hydsst3obtds@localhost")])
            .await
            .expect_err("an invalid seed returns an error");
        super::resolve(&[String::from("hydsst3obtds")])
            .await
            .expect_err("an invalid seed returns an error");

        Ok(())
    }
}
