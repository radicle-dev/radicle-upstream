//! Container to bundle and associate information around an identity.

/// The users personal identifying metadata and keys.
pub struct Identity {
    /// The librad id.
    pub id: String,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Bundle of user provided data.
    pub metadata: Metadata,
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
