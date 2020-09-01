//! Datastructure and machinery to safely share the common dependencies across components.

use std::sync::Arc;

use tokio::sync::RwLock;

use librad::paths;

use crate::keystore;

/// Wrapper around the thread-safe handle on [`Context`].
pub type Ctx = Arc<RwLock<Context>>;

impl From<Context> for Ctx {
    fn from(ctx: Context) -> Self {
        Arc::new(RwLock::new(ctx))
    }
}

/// Container to pass down dependencies into HTTP filter chains.
pub struct Context {
    /// [`coco::Api`] to operate on the local monorepo.
    pub peer_api: coco::Api,
    /// Storage to manage keys.
    pub keystore: keystore::Keystorage,
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
}

impl Context {
    /// Initialises a new [`Ctx`] the given temporary directory.
    ///
    /// # Errors
    ///
    /// * coco key creation fails
    /// * creation of the [`kv::Store`] fails
    #[cfg(test)]
    pub async fn tmp(tmp_dir: &tempfile::TempDir) -> Result<Ctx, crate::error::Error> {
        let paths = librad::paths::Paths::from_root(tmp_dir.path())?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut keystore = keystore::Keystorage::new(&paths, pw);
        let key = keystore.init_librad_key()?;

        let peer_api = {
            let config = coco::config::default(key, tmp_dir.path())?;
            coco::Api::new(config).await?
        };

        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        Ok(Arc::new(RwLock::new(Self {
            keystore,
            peer_api,
            store,
        })))
    }
}

/// Resets the peer and keystore within the `Ctx`.
///
/// # Errors
///
///   * If we could not get the librad path.
///   * If we could not initialise the librad key.
///   * If we could not construct the peer API.
///
/// # Panics
///
///   * If we could not get the temporary directory.
pub async fn reset_ctx_peer(ctx: Ctx) -> Result<(), crate::error::Error> {
    // TmpDir deletes the temporary directory once it DROPS.
    // This means our new directory goes missing, and future calls will fail.
    // The Peer creates the directory again.
    //
    // N.B. this may gather lot's of tmp files on your system. We're sorry.
    let tmp_path = {
        let temp_dir = tempfile::tempdir()?;
        log::debug!("New temporary path is: {:?}", temp_dir.path());
        std::env::set_var("RAD_HOME", temp_dir.path());
        temp_dir.path().to_path_buf()
    };

    let paths = paths::Paths::from_root(tmp_path)?;

    let pw = keystore::SecUtf8::from("radicle-upstream");
    let mut new_keystore = keystore::Keystorage::new(&paths, pw);
    let key = new_keystore.init_librad_key()?;

    let config = coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, vec![]);
    let new_peer_api = coco::Api::new(config).await?;

    let mut ctx = ctx.write().await;
    ctx.peer_api = new_peer_api;
    ctx.keystore = new_keystore;

    Ok(())
}
