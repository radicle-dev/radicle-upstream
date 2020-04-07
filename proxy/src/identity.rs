//! Container to bundle and associate information around an identity.

use radicle_registry_client::UserId;

use crate::error;

/// The users personal identifying metadata and keys.
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
    store: &kv::Store,
    handle: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
) -> Result<Identity, error::Error> {
    let bucket = store
        .bucket::<kv::Raw, String>(Some("session"))
        .expect("unable to get bucket");

    if let Some(id) = bucket.get("identity").expect("unable to fetch identity") {
        return Err(error::Error::IdentityExists(id));
    }

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
