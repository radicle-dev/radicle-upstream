//! Container to bundle and associate information around an identity.

use radicle_registry_client::UserId;

use crate::error;

/// The users personal identifying metadata and keys.
#[derive(Clone)]
pub struct Identity {
    /// The librad id.
    pub id: String,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Bundle of user provided data.
    pub metadata: Metadata,
    /// Indicator if the identity is registered on the Registry.
    pub registered: Option<UserId>,
}

/// User maintained information for an identity, which can evolve over time.
#[derive(Clone)]
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
    Ok(Identity {
        id: "123abcd.git".into(),
        shareable_entity_identifier: format!("{}@123abcd.git", handle),
        metadata: Metadata {
            handle,
            display_name,
            avatar_url,
        },
        registered: None,
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
    }))
}
