//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use serde::{Deserialize, Serialize};

use crate::error;
use crate::identity;

pub mod announcements;
pub mod settings;

/// Name for the storage bucket used for all session data.
const BUCKET_NAME: &str = "session";
/// Name of the item used for the currently active session.
const KEY_CURRENT: &str = "current";

/// Container for all local state.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The currently used [`identity::Identity`].
    pub identity: Option<identity::Identity>,
    /// User controlled parameters to control the behaviour and state of the application.
    pub settings: settings::Settings,
}

/// Resets the session state.
///
/// # Errors
///
/// Errors if the state on disk can't be accessed.
pub fn clear_current(store: &kv::Store) -> Result<(), error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .remove(KEY_CURRENT)?)
}

/// Read the current settings.
///
/// # Errors
///
/// Errors if access to the setttings fails.
pub async fn settings(store: &kv::Store) -> Result<settings::Settings, error::Error> {
    let session = get(store, KEY_CURRENT)?;
    Ok(session.settings)
}

/// Reads the current session.
///
/// # Errors
///
/// Errors if access to the session state fails, or associated data like the [`identity::Identity`]
/// can't be found.
pub async fn current(state: &coco::State, store: &kv::Store) -> Result<Session, error::Error> {
    let mut session = get(store, KEY_CURRENT)?;

    if let Some(id) = session.identity.clone() {
        identity::get(state, &id.urn)?;
        session.identity = Some(id);
    }

    Ok(session)
}

/// Stores the [`identity::Identity`] in the current session.
///
/// # Errors
///
/// Errors if access to the session state fails, or associated data like the [`identity::Identity`]
/// can't be found.
pub fn set_identity(store: &kv::Store, id: identity::Identity) -> Result<(), error::Error> {
    let mut sess = get(store, KEY_CURRENT)?;
    sess.identity = Some(id);

    set(store, KEY_CURRENT, sess)
}

/// Stores the [`settings::Settings`] in the current session.
///
/// # Errors
///
/// Errors if access to the session state fails.
pub fn set_settings(store: &kv::Store, settings: settings::Settings) -> Result<(), error::Error> {
    let mut sess = get(store, KEY_CURRENT)?;
    sess.settings = settings;

    set(store, KEY_CURRENT, sess)
}

/// Fetches the session for the given item key.
fn get(store: &kv::Store, key: &str) -> Result<Session, error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .get(key)?
        .map(kv::Codec::to_inner)
        .unwrap_or_default())
}

/// Stores the session for the given item key.
fn set(store: &kv::Store, key: &str, sess: Session) -> Result<(), error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .set(key, kv::Json(sess))?)
}
