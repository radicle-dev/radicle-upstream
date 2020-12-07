//! Handling of include files

use librad::identities::Urn;

use crate::state::State;

/// Update the include file for the given `RadUrn` and log the result.
pub async fn update(state: State, urn: Urn) {
    match state.update_include(urn.clone()).await {
        Ok(path) => log::debug!("Updated include file @ {}", path.display()),
        Err(err) => log::debug!("Failed to update include file for `{}`: {}", urn, err),
    }
}
