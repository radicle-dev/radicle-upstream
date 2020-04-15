//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use crate::error;
use crate::identity;

/// Container for all local state.
pub struct Session {
    /// The currently used [`identity::Identity`].
    pub identity: Option<identity::Identity>,
}

/// Resets the session state.
///
/// # Errors
///
/// Errors if the state on disk can't be accessed.
pub fn clear(store: &kv::Store) -> Result<(), error::Error> {
    let bucket = store
        .bucket::<kv::Raw, String>(Some("session"))
        .expect("unable to get session bucket");

    bucket.clear().expect("unable to clear session bucket");

    Ok(())
}

/// Reads the current session.
///
/// # Errors
///
/// Errors if access to the session state fails, or associated data like the [`identity::Identity`]
/// can't be found.
pub fn get(store: &kv::Store) -> Result<Session, error::Error> {
    let bucket = store
        .bucket::<kv::Raw, String>(Some("session"))
        .expect("unable to get session bucket");

    let identity = bucket
        .get("identity")
        .expect("unable to fetch identity")
        .and_then(|id| identity::get(id.as_ref()).expect("unable to retrieve identity"));

    Ok(Session { identity })
}

/// Stores the Session in its entirety.
///
/// # Errors
///
/// Errors if access to the session state fails.
pub fn set(store: &kv::Store, sess: Session) -> Result<(), error::Error> {
    let bucket = store
        .bucket::<kv::Raw, String>(Some("session"))
        .expect("unable to get session bucket");

    if let Some(identity) = sess.identity {
        bucket
            .set("identity", identity.id)
            .expect("unable to save identity");
    }

    Ok(())
}
