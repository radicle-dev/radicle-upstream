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

/// Get the seed nodes (see [`settings::CoCo::seeds`]) from the session settings current session.
///
/// Returns `None` if no session exists.
///
/// # Errors
///
/// Errors if we cannot read data from the store.
pub async fn seeds(store: &kv::Store) -> Result<Option<Vec<String>>, error::Error> {
    Ok(get_current(store)?.map(|session| session.settings.coco.seeds))
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

/// Initialize the current session with the given identity and the default settings.
///
/// # Errors
///
/// * Errors when we cannot write to the store.
pub fn initialize(
    store: &kv::Store,
    identity: identity::Identity,
) -> Result<Session, error::Error> {
    let session = Session {
        identity,
        settings: settings::Settings::default(),
    };

    set_current(store, session.clone())?;
    Ok(session)
}

/// Enriched [`crate::context::Context`] for HTTP handlers that require a session to be present.
pub struct Context {
    /// Handle to inspect state and perform actions on the currently running local [`coco::Peer`].
    pub peer_control: coco::PeerControl,
    /// [`coco::State`] to operate on the local monorepo.
    pub state: coco::State,
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
    /// The owner that [`Context::state`] was initialized with.
    pub owner: coco::user::User,
    /// The current session
    pub session: Session,
}

impl Context {
    /// Stores the [`settings::Settings`] in the current session.
    ///
    /// # Errors
    ///
    /// Errors if the session cannot be saved in the store.
    pub fn set_settings(&self, settings: settings::Settings) -> Result<(), error::Error> {
        let mut session = self.session.clone();
        session.settings = settings;

        set_current(&self.store, session)
    }
}

/// Initialize a session for tests.
///
/// Creates an owner identity for the session using `owner_handle` and stores the current session.
///
/// Panics if anything goes wrong.
#[cfg(test)]
pub async fn initialize_test(ctx: &crate::context::Context, owner_handle: &str) -> Context {
    let crate::context::Context {
        peer_control,
        state,
        store,
    } = ctx.clone();
    let owner = state
        .init_owner(owner_handle)
        .await
        .expect("cannot init owner identity");
    let identity = (state.peer_id(), owner.clone()).into();
    let session = initialize(&store, identity).expect("failed to initialize session");

    Context {
        peer_control,
        state,
        store,
        owner,
        session,
    }
}

/// Stores the session as the current session
fn set_current(store: &kv::Store, sess: Session) -> Result<(), error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .set(KEY_CURRENT, kv::Json(sess))?)
}
