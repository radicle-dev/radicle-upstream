//! Management of local session state like the currently used identity, wallet related data and
//! configuration of all sorts.

use serde::{Deserialize, Serialize};

use crate::coco;
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
    /// Permissions of the user to control actions.
    pub permissions: Permissions,
    /// User controlled parameters to control the behaviour and state of the application.
    pub settings: settings::Settings,
    /// Transaction deposits.
    pub transaction_deposits: registry::Deposits,
    /// Minimum transaction fee.
    pub minimum_transaction_fee: registry::Balance,
}

/// Set of permitted actions the user can perform.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    /// Permission to register a user handle
    pub register_handle: bool,
    /// Permission to register an org
    pub register_org: bool,
    /// Permission to register a project
    pub register_project: bool,
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
pub async fn current<R>(
    api: &coco::Api,
    registry: &R,
    store: &kv::Store,
) -> Result<Session, error::Error>
where
    R: registry::Client,
{
    let mut session = get(store, KEY_CURRENT)?;
    session.transaction_deposits = registry::get_deposits();
    session.minimum_transaction_fee = registry::MINIMUM_FEE;

    // Reset the permissions
    session.permissions = Permissions::default();

    if let Some(id) = session.identity.clone() {
        identity::get(api, &id.urn)?;
    }

    if let Some(mut id) = session.identity.clone() {
        if let Some(handle) = id.registered.clone() {
            if registry.get_user(handle.clone()).await?.is_some() {
                session.orgs = registry.list_orgs(handle).await?;
                session.permissions.register_org = true;
                let projects = api.list_projects()?;
                if !projects.is_empty() {
                    session.permissions.register_project = true;
                }
            } else {
                id.registered = None;
                session.identity = Some(id);
                session.permissions.register_handle = true;
            }
        } else {
            session.permissions.register_handle = true;
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

/// User controlled parameters for application appearance, behaviour and state.
pub mod settings {
    use serde::{Deserialize, Serialize};

    /// User controlled parameters for application appearance, behaviour and state.
    #[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Settings {
        /// Currently set appearance parameters.
        pub appearance: Appearance,
        /// Currently set registry parameters.
        pub registry: Registry,
        /// User-determined p2p parameters.
        pub coco: CoCo,
    }

    /// Knobs for the look and feel.
    #[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Appearance {
        /// Currently active color scheme.
        pub theme: Theme,
    }

    /// Color schemes available.
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Theme {
        /// A dark theme.
        Dark,
        /// A light theme.
        Light,
    }

    impl Default for Theme {
        fn default() -> Self {
            Self::Light
        }
    }

    /// Registry parameters.
    #[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Registry {
        /// Currently configured network.
        pub network: Network,
    }

    /// Known networks the application can connect to.
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Network {
        /// In-memory registry, which only lives as long as the app does.
        Emulator,
        /// The friends-n-family network. For the loved ones.
        FFnet,
        /// Test network.
        Testnet,
    }

    impl Default for Network {
        fn default() -> Self {
            Self::Emulator
        }
    }

    /// `CoCo` config parameters subject to user preferences
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct CoCo {
        /// Sources of feeds
        pub seeds: Vec<String>,
    }

    // TODO(sos): should these come  from librad?
    impl Default for CoCo {
        fn default() -> Self {
            Self {
                seeds: ["seed.radicle.xyz", "194.134.54.13"]
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect(),
            }
        }
    }
}
