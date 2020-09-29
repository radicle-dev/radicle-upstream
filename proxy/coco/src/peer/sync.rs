//! Perform full state syncs with remote peers.

use librad::{peer::PeerId, uri::RadUrl};

use crate::state::State;

use super::Error;

/// Initiaites a fetch for all locally tracked projects from the given [`PeerId`].
pub async fn sync(state: &State, peer_id: PeerId) -> Result<(), Error> {
    log::debug!("Starting sync from {}", peer_id);

    let urls = state
        .list_projects()
        .await?
        .iter()
        .map(|project| RadUrl {
            authority: peer_id.clone(),
            urn: project.urn(),
        })
        .collect::<Vec<RadUrl>>();

    for url in urls {
        log::debug!("Starting fetch of {} from {}", url.clone(), peer_id);
        state.fetch(url.clone(), vec![]).await?;
        log::debug!("Finished fetch of {} from {}", url, peer_id);
    }

    Ok(())
}
