//! Container to bundle and associate information around an identity.

use serde::{Deserialize, Serialize};

use librad::keys;
use librad::peer;

use crate::avatar;
use crate::coco;
use crate::error;
use crate::registry;

pub use shared_identifier::SharedIdentifier;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The Peer Id for the user.
    pub peer_id: peer::PeerId,
    /// The librad id.
    pub urn: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: SharedIdentifier,
    /// Bundle of user provided data.
    pub metadata: Metadata,
    /// Indicator if the identity is registered on the Registry.
    pub registered: Option<registry::Id>,
    /// Generated fallback avatar to be used if actual avatar url is missing or can't be loaded.
    pub avatar_fallback: avatar::Avatar,
}

/// User maintained information for an identity, which can evolve over time.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Similar to a nickname, the users chosen short identifier.
    pub handle: String,
}

/// Creates a new identity.
///
/// # Errors
pub fn create(
    peer: &coco::PeerApi,
    key: keys::SecretKey,
    handle: String,
) -> Result<Identity, error::Error> {
    let user = coco::init_owner(peer, key, &handle)?;
    let peer_id = peer.peer_id().clone();

    let urn = user.urn();
    let shareable_entity_identifier = (peer_id.clone(), user).into();
    Ok(Identity {
        peer_id,
        urn: urn.clone(),
        shareable_entity_identifier,
        metadata: Metadata { handle },
        registered: None,
        avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
    })
}

/// Retrieve an identity by id. We assume the `Identity` is owned by this peer.
///
/// # Errors
///
/// Errors if access to coco state on the filesystem fails, or the id is malformed.
pub fn get(peer: &coco::PeerApi, id: &coco::Urn) -> Result<Identity, error::Error> {
    let user = coco::get_user(peer, id)?;
    let handle = user.name().to_string();
    let urn = user.urn();
    let peer_id = peer.peer_id().clone();
    let shareable_entity_identifier = (peer_id.clone(), user).into();
    Ok(Identity {
        peer_id,
        urn: urn.clone(),
        shareable_entity_identifier,
        metadata: Metadata { handle },
        registered: None,
        avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
    })
}

/// A `SharedIdentifier` is the combination of a user handle and the [`coco::Urn`] that identifies
/// the user.
pub mod shared_identifier {
    use std::{fmt, str::FromStr};

    use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

    use librad::meta::user;
    use librad::peer;

    /// Errors captured when parsing a shareable identifier of the form `<handle>@<urn>`.
    #[derive(Debug, thiserror::Error)]
    pub enum ParseError {
        /// Could not parse the URN portion of the identifier.
        #[error(transparent)]
        Peer(#[from] peer::conversion::Error),
        /// The identifier contained more than one '@' symbol.
        #[error("shared identifier contains more than one '@' symbol")]
        AtSplitError,
        /// The handle portion of the identifier was missing.
        #[error("shared identifier is missing the handle to the left of the '@' symbol")]
        MissingHandle,
        /// The urn portion of the identifier was missing.
        #[error("shared identifier is missing the URN to the right of the '@' symbol")]
        MissingPeerId,
    }

    /// The combination of a handle and a urn give user's a structure for sharing their identities.
    #[derive(Clone, Debug)]
    pub struct SharedIdentifier {
        /// The user's chosen handle.
        pub handle: String,
        /// The unique identifier of the user.
        pub peer_id: peer::PeerId,
    }

    impl<ST> From<(peer::PeerId, user::User<ST>)> for SharedIdentifier {
        fn from((peer_id, user): (peer::PeerId, user::User<ST>)) -> Self {
            Self {
                handle: user.name().to_string(),
                peer_id,
            }
        }
    }

    impl FromStr for SharedIdentifier {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut sub = s.split('@');
            let handle = sub.next();
            let peer_id = sub.next();

            if sub.count() != 0 {
                return Err(ParseError::AtSplitError);
            }

            let handle = handle.ok_or(ParseError::MissingHandle)?.to_string();
            let peer_id = peer_id
                .ok_or(ParseError::MissingPeerId)
                .and_then(|peer_id| Ok(peer_id.parse()?))?;

            Ok(Self { handle, peer_id })
        }
    }

    impl fmt::Display for SharedIdentifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}@{}", self.handle, self.peer_id)
        }
    }

    impl Serialize for SharedIdentifier {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for SharedIdentifier {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            /// A phantom Visitor for serde to deserialize.
            struct IdVisitor;

            impl<'de> Visitor<'de> for IdVisitor {
                type Value = SharedIdentifier;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "a shared identifier of the form <handle>@<urn>")
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    s.parse().map_err(serde::de::Error::custom)
                }
            }

            deserializer.deserialize_str(IdVisitor)
        }
    }
}
