//! Perform full state syncs with remote peers.

use librad::{peer::PeerId, uri::RadUrl};

use crate::state::Lock;

use super::Error;

/// Initiaites a fetch for all locally tracked projects from the given [`PeerId`].
pub async fn sync(state: Lock, peer_id: PeerId) -> Result<(), Error> {
    log::debug!("Starting sync from {}", peer_id);

    let urls = state
        .lock()
        .await
        .list_projects()
        .map_err(Error::from)?
        .iter()
        .map(|project| RadUrl {
            authority: peer_id.clone(),
            urn: project.urn(),
        })
        .collect::<Vec<RadUrl>>();

    for url in urls {
        log::debug!("Starting fetch of {} from {}", url.clone(), peer_id);
        let state = state.clone();
        let state = state.lock_owned().await;
        let task_url = url.clone();
        tokio::task::spawn_blocking(move || state.fetch(task_url, vec![]))
            .await
            .expect("join thread failed")?;
        log::debug!("Finished fetch of {} from {}", url, peer_id);
    }

    Ok(())
}
