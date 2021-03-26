//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use serde::{Deserialize, Serialize};

use crate::{error, identity};

pub mod settings;

/// Name for the storage bucket used for all session data.
const BUCKET_NAME: &str = "session";
/// Name of the item used for the currently active session.
const KEY_CURRENT: &str = "current";

/// Container for all local state.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The currently used [`identity::Identity`].
    pub identity: identity::Identity,
    /// User controlled parameters to control the behaviour and state of the application.
    pub settings: settings::Settings,
}

/// Get the seed nodes (see [`settings::CoCo::seeds`]) from the session settings current session.
///
/// If there is no session yet, returns the seeds from the default value of [`settings::CoCo`].
///
/// # Errors
///
/// Errors if we cannot read data from the store.
pub async fn seeds(
    store: &kv::Store,
    default_seeds: &[String],
) -> Result<Vec<String>, error::Error> {
    let settings = get_current(store)?.map(|session| session.settings);

    match settings {
        Some(settings) => Ok(settings.coco.seeds),
        None => Ok(default_seeds.to_vec()),
    }
}

/// Get the current session if present
///
/// # Errors
///
/// Errors if we cannot read data from the store.
pub fn get_current(store: &kv::Store) -> Result<Option<Session>, error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .get(KEY_CURRENT)?
        .map(kv::Codec::to_inner))
}

/// Initialize the current session with the given identity, default settings and default seeds.
///
/// # Errors
///
/// * Errors when we cannot write to the store.
pub fn initialize(
    store: &kv::Store,
    identity: identity::Identity,
    default_seeds: &[String],
) -> Result<Session, error::Error> {
    let mut session = Session {
        identity,
        settings: settings::Settings::default(),
    };

    session.settings.coco.seeds = default_seeds.to_owned();

    set_current(store, session.clone())?;
    Ok(session)
}

/// Update the session settings. Does nothing if there is no session yet.
///
/// # Errors
///
/// Errors when we cannot access the store.
pub fn set_settings(store: &kv::Store, settings: settings::Settings) -> Result<(), error::Error> {
    if let Some(mut session) = get_current(store)? {
        session.settings = settings;

        set_current(store, session)?
    }
    Ok(())
}

/// Initialize a session for tests.
///
/// Creates an owner identity for the session using `owner_handle` and stores the current session.
///
/// Panics if anything goes wrong.
#[cfg(test)]
pub async fn initialize_test(ctx: &crate::context::Unsealed, owner_handle: &str) -> Session {
    let owner = coco::state::init_owner(
        &ctx.peer,
        coco::identities::payload::Person {
            name: owner_handle.into(),
        },
    )
    .await
    .expect("cannot init owner identity");
    let identity = (ctx.peer.peer_id(), owner.into_inner().into_inner()).into();
    initialize(&ctx.store, identity, &ctx.default_seeds).expect("failed to initialize session")
}

/// Stores the session as the current session
fn set_current(store: &kv::Store, sess: Session) -> Result<(), error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .set(KEY_CURRENT, kv::Json(sess))?)
}
