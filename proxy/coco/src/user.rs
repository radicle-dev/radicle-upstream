//! Commonly used vesions of [`librad::meta::user::User`].

use librad::{
    git::{identities::person, storage::Storage},
    identities::{Person, Urn, VerifiedPerson},
};

use crate::state;

/// Export a verified [`user::User`] type.
pub type User = VerifiedPerson;

/// Verify a user using a fake resolver that resolves the user to itself.
///
/// # Errors
///
/// If any of the verification steps fail
pub fn verify<C>(storage: &Storage, urn: &Urn) -> Result<Option<User>, state::Error<C>>
where
    C: std::fmt::Debug,
{
    Ok(person::verify(storage, urn)?)
}
