//! Container to bundle and associate information around an identity.

use serde::{Deserialize, Serialize};

use coco::signer;

use crate::{avatar, error};

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

impl<S> From<(coco::PeerId, coco::MetaUser<S>)> for Identity {
    fn from((peer_id, user): (coco::PeerId, coco::MetaUser<S>)) -> Self {
        let urn = user.urn();
        Self {
            peer_id: peer_id.clone(),
            urn: urn.clone(),
            shareable_entity_identifier: coco::Identifier {
                handle: user.name().to_string(),
                peer_id,
            },
            metadata: Metadata {
                handle: user.name().to_string(),
            },
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
pub fn create(
    state: &coco::State,
    key: &signer::BoxedSigner,
    handle: &str,
) -> Result<Identity, error::Error> {
    let user = state.init_owner(key, handle)?;
    Ok((state.peer_id(), user).into())
}

/// Retrieve an identity by id. We assume the `Identity` is owned by this peer.
///
/// # Errors
///
/// Errors if access to coco state on the filesystem fails, or the id is malformed.
pub fn get(state: &coco::State, id: &coco::Urn) -> Result<Identity, error::Error> {
    let user = state.get_user(id)?;
    Ok((state.peer_id(), user).into())
}

/// Retrieve the list of identities known to the session user.
///
/// # Errors
pub fn list(state: &coco::State) -> Result<Vec<Identity>, error::Error> {
    let mut users = vec![];
    for project in state.list_projects()? {
        let project_urn = project.urn();
        for peer in state.tracked(&project_urn)? {
            let user = peer.into();
            if !users.contains(&user) {
                users.push(user)
            }
        }
    }
    Ok(users)
}
