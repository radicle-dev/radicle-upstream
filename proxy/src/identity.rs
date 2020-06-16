//! Container to bundle and associate information around an identity.

use serde::{Deserialize, Serialize};

use crate::avatar;
use crate::error;
use crate::registry;
use std::convert::TryFrom;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// The librad id.
    pub id: String,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
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
    handle: String,
) -> Result<Identity, error::Error> {
    let id = format!("{}@123abcd.git", handle);
    Ok(Identity {
        id: id.clone(),
        shareable_entity_identifier: id.clone(),
        metadata: Metadata {
            handle,
        },
        registered: None,
        avatar_fallback: avatar::Avatar::from(&id, avatar::Usage::Identity),
    })
}

/// Retrieve an identity by id.
///
/// # Errors
///
/// Errors if access to coco state on the filesystem fails, or the id is malformed.
pub fn get(id: &str) -> Result<Option<Identity>, error::Error> {
    Ok(Some(Identity {
        id: id.to_string(),
        shareable_entity_identifier: format!("cloudhead@{}", id),
        metadata: Metadata {
            handle: "cloudhead".into(),
        },
        registered: registry::Id::try_from("cloudhead").ok(),
        avatar_fallback: avatar::Avatar::from(id, avatar::Usage::Identity),
    }))
}
