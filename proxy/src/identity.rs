//! Container to bundle and associate information around an identity.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use librad::keys;
use librad::uri::RadUrn;

use crate::avatar;
use crate::coco;
use crate::error;
use crate::registry;

pub use shared_identifier::SharedIdentifier;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The librad id.
    pub id: RadUrn,
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
pub async fn create(
    peer: Arc<Mutex<coco::PeerApi>>,
    key: keys::SecretKey,
    handle: String,
) -> Result<Identity, error::Error> {
    let user = coco::init_owner(peer, key, &handle).await?;

    let id = user.urn();
    let shareable_entity_identifier = user.into();
    Ok(Identity {
        id: id.clone(),
        shareable_entity_identifier,
        metadata: Metadata { handle },
        registered: None,
        avatar_fallback: avatar::Avatar::from(&id.to_string(), avatar::Usage::Identity),
    })
}

/// Retrieve an identity by id.
///
/// # Errors
///
/// Errors if access to coco state on the filesystem fails, or the id is malformed.
pub fn get(peer: &coco::PeerApi, id: &RadUrn) -> Result<Identity, error::Error> {
    let user = coco::get_user(peer, id)?;
    Ok(Identity {
        id: id.clone(),
        shareable_entity_identifier: SharedIdentifier {
            handle: user.name().to_string(),
            urn: id.clone(),
        },
        metadata: Metadata {
            handle: user.name().to_string(),
        },
        registered: None,
        avatar_fallback: avatar::Avatar::from(&id.to_string(), avatar::Usage::Identity),
    })
}

/// A `SharedIdentifier` is the combination of a user handle and the [`RadUrn`] that identifies the
/// user.
pub mod shared_identifier {

    use std::{fmt, str::FromStr};

    use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

    use librad::meta::user;
    use librad::uri::{rad_urn, RadUrn};

    /// Errors captured when parsing a shareable identifier of the form `<handle>@<urn>`.
    #[derive(Debug, thiserror::Error)]
    pub enum ParseError {
        /// Could not parse the URN portion of the identifier.
        #[error(transparent)]
        Urn(#[from] rad_urn::ParseError),
        /// The identifier contained more than one '@' symbol.
        #[error("shared identifier contains more than one '@' symbol")]
        AtSplitError,
        /// The handle portion of the identifier was missing.
        #[error("shared identifier is missing the handle to the left of the '@' symbol")]
        MissingHandle,
        /// The urn portion of the identifier was missing.
        #[error("shared identifier is missing the URN to the right of the '@' symbol")]
        MissingUrn,
    }

    /// The combination of a handle and a urn give user's a structure for sharing their identities.
    #[derive(Clone, Debug)]
    pub struct SharedIdentifier {
        /// The user's chosen handle.
        pub handle: String,
        /// The unique identifier of the user.
        pub urn: RadUrn,
    }

    impl<ST> From<user::User<ST>> for SharedIdentifier {
        fn from(user: user::User<ST>) -> Self {
            Self {
                handle: user.name().to_string(),
                urn: user.urn(),
            }
        }
    }

    impl FromStr for SharedIdentifier {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut sub = s.split('@');
            let handle = sub.next();
            let urn = sub.next();

            if sub.count() != 0 {
                return Err(ParseError::AtSplitError);
            }

            let handle = handle.ok_or(ParseError::MissingHandle)?.to_string();
            let urn = urn
                .ok_or(ParseError::MissingUrn)
                .and_then(|urn| Ok(urn.parse()?))?;

            Ok(Self { handle, urn })
        }
    }

    impl fmt::Display for SharedIdentifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}@{}", self.handle, self.urn)
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
