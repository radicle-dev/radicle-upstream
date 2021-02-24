//! Container to bundle and associate information around an identity.

use serde::{Deserialize, Serialize};

use radicle_avatar as avatar;

use coco::signer::BoxedSigner;

use crate::error;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The Peer Id for the user.
    pub peer_id: coco::PeerId,
    /// The coco URN.
    pub urn: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: coco::Identifier,
    /// Bundle of user provided data.
    pub metadata: Metadata,
    /// Generated fallback avatar to be used if actual avatar url is missing or can't be loaded.
    pub avatar_fallback: avatar::Avatar,
}

impl From<(coco::PeerId, coco::Person)> for Identity {
    fn from((peer_id, user): (coco::PeerId, coco::Person)) -> Self {
        let urn = user.urn();
        let handle = user.subject().name.to_string();
        Self {
            peer_id,
            urn: urn.clone(),
            shareable_entity_identifier: coco::Identifier {
                handle: handle.clone(),
                peer_id,
            },
            metadata: Metadata { handle },
            avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
        }
    }
}

/// User maintained information for an identity, which can evolve over time.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Similar to a nickname, the users chosen short identifier.
    pub handle: String,
}

/// Creates a new identity.
///
/// # Errors
pub async fn create(
    peer: &coco::net::peer::Peer<BoxedSigner>,
    handle: &str,
) -> Result<Identity, error::Error> {
    let user = coco::state::init_owner(peer, handle.to_string()).await?;
    Ok((peer.peer_id(), user.into_inner().into_inner()).into())
}

/// Retrieve an identity by id. We assume the `Identity` is owned by this peer.
///
/// # Errors
///
/// Errors if access to coco state on the filesystem fails, or the id is malformed.
pub async fn get(
    peer: &coco::net::peer::Peer<BoxedSigner>,
    id: coco::Urn,
) -> Result<Option<Identity>, error::Error> {
    match coco::state::get_user(peer, id).await? {
        Some(user) => Ok(Some(
            (peer.peer_id(), user.into_inner().into_inner()).into(),
        )),
        None => Ok(None),
    }
}

// TODO(finto): Check if this is used and if so, express more elegantly after
// radicle-dev/radicle-link#374.
/// Retrieve the list of identities known to the session user.
///
/// # Errors
///
///  * If we cannot get the list of projects
///  * If we cannot get the tracked peers for a given project
pub async fn list(
    peer: &coco::net::peer::Peer<BoxedSigner>,
) -> Result<Vec<Identity>, error::Error> {
    let mut users = vec![];
    for project in coco::state::list_projects(peer).await? {
        let project_urn = project.urn();
        for peer in coco::state::tracked(peer, project_urn)
            .await?
            .into_iter()
            .filter_map(coco::project::Peer::replicated_remote)
        {
            let user = peer.into();
            if !users.contains(&user) {
                users.push(user)
            }
        }
    }
    Ok(users)
}
