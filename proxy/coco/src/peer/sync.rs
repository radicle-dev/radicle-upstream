//! Perform full state syncs with remote peers.

use librad::{identities::generic::Identity, net::peer::Peer, peer::PeerId, signer::BoxedSigner};

use crate::state;

use super::{include, Error};

/// Initiaites a fetch for all locally tracked projects from the given [`PeerId`].
pub async fn sync(peer: &Peer<BoxedSigner>, remote_peer: PeerId) -> Result<(), Error> {
    log::debug!("Starting sync from {}", remote_peer);

    let urns = state::list_projects(peer)
        .await?
        .iter()
        .map(Identity::urn)
        .collect::<Vec<_>>();

    for urn in urns {
        log::debug!("Starting fetch of {} from {}", urn, remote_peer);
        match state::fetch(peer, urn.clone(), remote_peer, vec![], None).await {
            Ok(result) => {
                log::debug!(
                    "Finished fetch of {} from {} with the result {:?}",
                    urn,
                    remote_peer,
                    result.updated_tips
                );
                include::update(peer.clone(), urn).await;
            },
            Err(e) => log::debug!("Fetch of {} from {} errored: {}", urn, remote_peer, e),
        }
    }

    Ok(())
}
