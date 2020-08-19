//! Seed nodes.
use std::net::SocketAddr;

use librad::peer;

/// A seed-related error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Seed input is invalid.
    #[error("the seed '{0}' is invalid: {:1}")]
    InvalidSeed(String, Option<librad::peer::conversion::Error>),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Resolve seed identifiers into `(PeerId, SocketAddr)` pairs.
///
/// The expected format is `<peer-id>@<host>:<port>`
///
/// # Errors
///
/// If any of the supplied seeds cannot be parsed, an error is returned.
#[allow(clippy::indexing_slicing)]
pub async fn resolve<T: AsRef<str> + Send + Sync>(
    seeds: &[T],
) -> Result<Vec<(peer::PeerId, SocketAddr)>, Error> {
    let mut resolved = Vec::with_capacity(seeds.len());

    for seed in seeds.iter() {
        let seed = seed.as_ref();

        if let Some(ix) = seed.chars().position(|c| c == '@') {
            let (peer_id, rest) = seed.split_at(ix);
            let host = &rest[1..]; // Skip '@'

            if let Some(addr) = tokio::net::lookup_host(host).await?.next() {
                let peer_id = peer::PeerId::from_default_encoding(peer_id)
                    .map_err(|err| Error::InvalidSeed(seed.to_string(), Some(err)))?;

                resolved.push((peer_id, addr));
            }
        } else {
            return Err(Error::InvalidSeed(seed.to_string(), None));
        }
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use std::net;

    #[tokio::test]
    async fn test_resolve_seeds() {
        let seeds = super::resolve(&[
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
    }
}
