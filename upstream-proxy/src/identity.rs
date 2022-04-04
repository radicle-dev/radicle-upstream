// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Container to bundle and associate information around an identity.

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use link_crypto::{BoxedSigner, PeerId};
use link_identities::{
    git::Urn,
    payload::{self, ExtError, PersonPayload},
    Person as LinkPerson,
};

use crate::{
    error,
    ethereum::{address::Address, claim_ext::V1 as EthereumClaimExtV1},
};

use std::convert::TryFrom;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The Peer ID for the user.
    pub peer_id: PeerId,
    /// The coco URN.
    pub urn: Urn,
    /// Bundle of user provided data.
    pub metadata: Metadata,
}

impl From<(PeerId, LinkPerson)> for Identity {
    fn from((peer_id, user): (PeerId, LinkPerson)) -> Self {
        let identity = Person::from(user);
        Self {
            peer_id,
            urn: identity.urn,
            metadata: identity.metadata,
        }
    }
}

/// The remote users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    /// The coco URN.
    pub urn: Urn,
    /// Bundle of user provided data.
    pub metadata: Metadata,
    /// The user's PeerIds.
    pub peer_ids: Vec<PeerId>,
}

impl From<LinkPerson> for Person {
    fn from(person: LinkPerson) -> Self {
        let urn = person.urn();
        let handle = person.subject().name.to_string();
        let peer_ids = person
            .delegations()
            .iter()
            .map(|pk| PeerId::from(*pk))
            .collect::<Vec<PeerId>>();
        let ethereum = match person.payload().get_ext::<EthereumClaimExtV1>() {
            Ok(ext_opt) => ext_opt.map(Ethereum::from),
            Err(err) => {
                tracing::warn!(%urn, ?err, "Ethereum claim of user is malformed");
                // Ignore the malformed extension payload, the identity itself is still valid
                None
            },
        };
        Self {
            urn,
            peer_ids,
            metadata: Metadata { handle, ethereum },
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

// Update the given payload using the properties from the given metadata.
pub fn update_payload(
    mut payload: PersonPayload,
    metadata: Metadata,
) -> Result<PersonPayload, ExtError> {
    payload.subject = payload::Person {
        name: metadata.handle.into(),
    };
    let ethereum_claim = metadata.ethereum.map(EthereumClaimExtV1::from);
    payload.with_ext(ethereum_claim)
}

impl TryFrom<Metadata> for PersonPayload {
    type Error = ExtError;

    fn try_from(metadata: Metadata) -> Result<Self, Self::Error> {
        let person = payload::Person {
            name: metadata.handle.into(),
        };
        let mut payload = Self::new(person);
        let ethereum_claim = metadata.ethereum.map(EthereumClaimExtV1::from);
        payload.set_ext(ethereum_claim)?;

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
    peer: &crate::daemon::net::peer::Peer<BoxedSigner>,
    metadata: Metadata,
) -> Result<Identity, error::Error> {
    let user = crate::daemon::state::init_owner(peer, metadata).await?;
    Ok((peer.peer_id(), user.into_inner().into_inner()).into())
}

/// Updates the new identity metadata.
///
/// # Errors
pub async fn update(
    peer: &crate::daemon::net::peer::Peer<BoxedSigner>,
    metadata: Metadata,
) -> Result<Identity, error::Error> {
    let current_payload = crate::daemon::state::default_owner(peer)
        .await?
        .ok_or(crate::daemon::state::Error::MissingOwner)?
        .payload()
        .clone();
    let new_payload =
        update_payload(current_payload, metadata).map_err(crate::daemon::state::Error::from)?;
    crate::daemon::state::update_owner_payload(peer, new_payload).await?;
    let user = crate::daemon::state::default_owner(peer)
        .await?
        .ok_or(crate::daemon::state::Error::MissingOwner)?;
    Ok((peer.peer_id(), user.into_inner().into_inner()).into())
}
