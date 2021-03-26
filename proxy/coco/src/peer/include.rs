//! Handling of include files

use librad::{git::Urn, net::peer::Peer, signer::BoxedSigner};

use crate::state;

/// Update the include file for the given `RadUrn` and log the result.
pub async fn update(peer: Peer<BoxedSigner>, urn: Urn) {
    match state::update_include(&peer, urn.clone()).await {
        Ok(path) => log::debug!("Updated include file @ {}", path.display()),
        Err(err) => log::debug!("Failed to update include file for `{}`: {}", urn, err),
    }
}
