//! Container to bundle and associate information around an identity.

use crate::avatar;
use crate::error;
use crate::registry;

/// The users personal identifying metadata and keys.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Metadata {
    /// Similar to a nickname, the users chosen short identifier.
    pub handle: String,
    /// A longer name to display, e.g.: full name.
    pub display_name: Option<String>,
    /// Url of an image the user wants to present alongside this [`Identity`].
    pub avatar_url: Option<String>,
}

/// Creates a new identity.
///
/// # Errors
pub fn create(
    handle: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
) -> Result<Identity, error::Error> {
    let id = "123abcd.git";
    Ok(Identity {
        id: id.into(),
        shareable_entity_identifier: format!("{}@123abcd.git", handle),
        metadata: Metadata {
            handle,
            display_name,
            avatar_url,
        },
        registered: None,
        avatar_fallback: avatar::Avatar::from(id, avatar::Usage::Identity),
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
            display_name: Some("Alexis Sellier".into()),
            avatar_url: Some("https://avatars1.githubusercontent.com/u/40774".into()),
        },
        registered: None,
        avatar_fallback: avatar::Avatar::from(id, avatar::Usage::Identity),
    }))
}
