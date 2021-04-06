//! Container to bundle and associate information around an identity.

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use radicle_avatar as avatar;

use coco::{
    identities::payload::{ExtError, Person, PersonPayload},
    signer::BoxedSigner,
};

use crate::{
    error,
    ethereum::{address::Address, claim_ext::V1 as EthereumClaimExtV1},
    identifier::Identifier,
};

use std::convert::TryFrom;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The Peer Id for the user.
    pub peer_id: coco::PeerId,
    /// The coco URN.
    pub urn: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: Identifier,
    /// Bundle of user provided data.
    pub metadata: Metadata,
    /// Generated fallback avatar to be used if actual avatar url is missing or can't be loaded.
    pub avatar_fallback: avatar::Avatar,
}

impl From<(coco::PeerId, coco::Person)> for Identity {
    fn from((peer_id, user): (coco::PeerId, coco::Person)) -> Self {
        let urn = user.urn();
        let handle = user.subject().name.to_string();
        let ethereum = match user.payload().get_ext::<EthereumClaimExtV1>() {
            Ok(ext_opt) => ext_opt.map(Ethereum::from),
            Err(err) => {
                log::warn!("Ethereum claim of user {} is malformed: {}", urn, err);
                // Ignore the malformed extension payload, the identity itself is still valid
                None
            },
        };
        Self {
            peer_id,
            urn: urn.clone(),
            shareable_entity_identifier: Identifier {
                handle: handle.clone(),
                peer_id,
            },
            metadata: Metadata { handle, ethereum },
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
    /// The user's Ethereum address.
    pub ethereum: Option<Ethereum>,
}

impl TryFrom<Metadata> for PersonPayload {
    type Error = ExtError;

    fn try_from(metadata: Metadata) -> Result<Self, Self::Error> {
        let person = Person {
            name: metadata.handle.into(),
        };
        let mut payload = Self::new(person);
        if let Some(ethereum) = metadata.ethereum {
            payload.set_ext(EthereumClaimExtV1::from(ethereum))?;
        };
        Ok(payload)
    }
}

/// The user's Ethereum address claim.
/// Meaningful only if confirmed on Ethereum. See [the RFC](docs/ethereum_attestation.md).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ethereum {
    /// The Ethereum address claimed by the user.
    pub address: Address,
    /// The timestamp before which the address claim is valid
    pub expiration: DateTime<Utc>,
}

impl From<EthereumClaimExtV1> for Ethereum {
    fn from(ethereum: EthereumClaimExtV1) -> Self {
        Self {
            address: ethereum.address,
            expiration: ethereum.expiration,
        }
    }
}

impl From<Ethereum> for EthereumClaimExtV1 {
    fn from(ethereum: Ethereum) -> Self {
        Self {
            address: ethereum.address,
            expiration: ethereum.expiration,
        }
    }
}

/// Creates a new identity.
///
/// # Errors
pub async fn create(
    peer: &coco::net::peer::Peer<BoxedSigner>,
    metadata: Metadata,
) -> Result<Identity, error::Error> {
    let user = coco::state::init_owner(peer, metadata).await?;
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
