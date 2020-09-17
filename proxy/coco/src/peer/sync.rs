//! Perform full state syncs with remote peers.

use librad::{peer::PeerId, uri::RadUrl};

use crate::state::Lock;

use super::Error;

/// Initiaites a fetch for all locally tracked projects from the given [`PeerId`].
pub async fn sync(state: Lock, peer_id: PeerId) -> Result<(), Error> {
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
        let state = state.clone();
        let state = state.lock_owned().await;
        tokio::task::spawn_blocking(move || state.fetch(url, vec![]))
            .await
            .expect("join thread failed")?;
    }

    Ok(())
}
