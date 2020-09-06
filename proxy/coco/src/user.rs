//! Commonly used vesions of [`librad::meta::user::User`].

use librad::meta::entity;
use librad::meta::user;
use librad::uri::RadUrn;

use crate::error::Error;

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// Verify a user using a fake resolver that resolves the user to itself.
///
/// TODO(finto): Should not live here permanently, because resolvers should solve this verification.
///
/// # Errors
///
/// If any of the verification steps fail
pub fn verify(user: user::User<entity::Draft>) -> Result<User, Error> {
    let fake_resolver = FakeUserResolver(user.clone());
    let verified_user = user.check_history_status(&fake_resolver, &fake_resolver)?;
    Ok(verified_user)
}

/// Acting as a fake resolver where a User resolves to itself.
/// This allows us to check the history status of a single User.
/// TODO(finto): Remove this once Resolvers are complete.
struct FakeUserResolver(user::User<entity::Draft>);

impl entity::Resolver<user::User<entity::Draft>> for FakeUserResolver {
    fn resolve(&self, _uri: &RadUrn) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }

    fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }
}
