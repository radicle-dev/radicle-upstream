//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use serde::{Deserialize, Serialize};

use crate::error;
use crate::identity;
use crate::registry;

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
    /// List of the orgs of the user associated with the current identity.
    pub orgs: Vec<registry::Org>,
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

/// Reads the current session.
///
/// # Errors
///
/// Errors if access to the session state fails, or associated data like the [`identity::Identity`]
/// can't be found.
pub async fn current<R: registry::Client>(
    store: &kv::Store,
    registry: R,
) -> Result<Session, error::Error> {
    let mut session = get(store, KEY_CURRENT)?;

    if let Some(mut id) = session.identity.clone() {
        if let Some(handle) = id.registered.clone() {
            if registry.get_user(handle.clone()).await?.is_some() {
                session.orgs = registry.list_orgs(handle).await?;
            } else {
                id.registered = None;
                session.identity = Some(id);
            }
        }
    }

    Ok(session)
}

/// Stores the [`registry::Id`] for the registered user handle in the current session.
///
/// # Errors
///
/// Errors if access to the session state fails.
pub fn set_handle(store: &kv::Store, handle: registry::Id) -> Result<(), error::Error> {
    let sess = get(store, KEY_CURRENT)?;
    let registered = sess.identity.map(|mut id| {
        id.registered = Some(handle);
        id
    });

    if let Some(id) = registered {
        set_identity(store, id)
    } else {
        Ok(())
    }
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
