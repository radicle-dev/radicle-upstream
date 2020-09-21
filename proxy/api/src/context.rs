//! Datastructure and machinery to safely share the common dependencies across components.

use std::sync::Arc;

use tokio::sync::RwLock;

use coco::{keystore, signer};

/// Wrapper around the thread-safe handle on [`Context`].
pub type Ctx = Arc<RwLock<Context>>;

impl From<Context> for Ctx {
    fn from(ctx: Context) -> Self {
        Arc::new(RwLock::new(ctx))
    }
}

/// Container to pass down dependencies into HTTP filter chains.
pub struct Context {
    /// [`coco::State`] to operate on the local monorepo.
    pub state: coco::Lock,
    /// [`coco::signer::BoxedSigner`] for write operations on the monorepo.
    pub signer: signer::BoxedSigner,
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
        let paths = coco::Paths::from_root(tmp_dir.path())?;

        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut keystore = keystore::Keystorage::new(&paths, pw);
        let key = keystore.init()?;
        let signer = signer::BoxedSigner::from(signer::SomeSigner {
            signer: key.clone(),
        });

        let (_peer, state) = {
            let config = coco::config::default(key, tmp_dir.path())?;
            coco::into_peer_state(config, signer.clone()).await?
        };

        Ok(Arc::new(RwLock::new(Self {
            state,
            signer,
            store,
        })))
    }
}

/// Resets the peer and keystore within the `Ctx`.
///
/// # Errors
///
///   * If we could not create a new temporary path.
///   * If we could not initialise the key.
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

    let paths = coco::Paths::from_root(tmp_path.clone())?;

    let pw = keystore::SecUtf8::from("radicle-upstream");
    let mut new_keystore = keystore::Keystorage::new(&paths, pw);
    let key = new_keystore.init()?;
    let signer = signer::BoxedSigner::from(key.clone());

    let (_new_peer, new_state) = {
        let config = coco::config::configure(paths, key, *coco::config::LOCALHOST_ANY, vec![]);
        coco::into_peer_state(config, signer.clone()).await?
    };

    let mut ctx = ctx.write().await;
    ctx.state = new_state;
    ctx.signer = signer;
    ctx.store = kv::Store::new(kv::Config::new(tmp_path.join("store")))?;

    Ok(())
}
