use librad::peer::PeerId;
use librad::uri::RadUrl;

use crate::state::Lock;

use super::Error;

pub async fn sync(state: Lock, peer_id: PeerId) -> Result<(), Error> {
    let state = state.lock().await;
    let urls = state
        .list_projects()
        .map_err(Error::from)?
        .iter()
        .map(|project| RadUrl {
            authority: peer_id.clone(),
            urn: project.urn(),
        })
        .collect::<Vec<RadUrl>>();

    for url in urls {
        state.fetch(url, vec![])?;
    }

    Ok(())
}
