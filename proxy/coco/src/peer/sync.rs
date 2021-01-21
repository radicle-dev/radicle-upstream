//! Perform full state syncs with remote peers.

use librad::{identities::generic::Identity, peer::PeerId};

use crate::state::State;

use super::{include, Error};

/// Initiaites a fetch for all locally tracked projects from the given [`PeerId`].
pub async fn sync(state: &State, remote_peer: PeerId) -> Result<(), Error> {
    log::debug!("Starting sync from {}", remote_peer);

    let urns = state
        .list_projects()
        .await?
        .iter()
        .map(Identity::urn)
        .collect::<Vec<_>>();

    for urn in urns {
        log::debug!("Starting fetch of {} from {}", urn, remote_peer);
        match state.fetch(urn.clone(), remote_peer, vec![], None).await {
            Ok(result) => {
                log::debug!(
                    "Finished fetch of {} from {} with the result {:?}",
                    urn,
                    remote_peer,
                    result
                );
                include::update(state.clone(), urn).await;
            },
            Err(e) => log::debug!("Fetch of {} from {} errored: {}", urn, remote_peer, e),
        }
    }

    Ok(())
}
