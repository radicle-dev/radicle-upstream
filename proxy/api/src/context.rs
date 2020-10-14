//! Datastructure and machinery to safely share the common dependencies across components.

use coco::PeerControl;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(test)]
use coco::{keystore, signer, RunConfig};

/// Container to pass down dependencies into HTTP filter chains.
#[derive(Clone)]
pub struct Context {
    /// Handle to inspect state and perform actions on the currently running local [`coco::Peer`].
    pub peer_control: PeerControl,
    /// [`coco::State`] to operate on the local monorepo.
    state: Arc<RwLock<Option<coco::State>>>,
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
}

impl Context {
    /// Initialises a new [`Context`] the given temporary directory.
    ///
    /// # Errors
    ///
    /// * coco key creation fails
    /// * creation of the [`kv::Store`] fails
    #[cfg(test)]
    pub async fn tmp(tmp_dir: &tempfile::TempDir) -> Result<Self, crate::error::Error> {
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let key = keystore::Keystorage::memory(pw)?.get();
        let signer = signer::BoxedSigner::from(signer::SomeSigner { signer: key });

        let (peer_control, state) = {
            let config = coco::config::default(key, tmp_dir.path())?;
            let (peer, state) =
                coco::into_peer_state(config, signer.clone(), store.clone(), RunConfig::default())
                    .await?;

            let peer_control = peer.control();
            tokio::spawn(peer.into_running());

            (peer_control, state)
        };

        Ok(Self {
            peer_control,
            state: Arc::new(RwLock::new(Some(state))),
            store,
        })
    }

    /// Create a new [`Context`] using the given parameters.
    #[must_use]
    pub fn new(peer_control: PeerControl, state: coco::State, store: kv::Store) -> Self {
        Self {
            peer_control,
            state: Arc::new(RwLock::new(Some(state))),
            store,
        }
    }

    /// Get a [`coco::State`] handle to interact with the code collaboration system.
    ///
    /// # Errors
    ///
    /// None currently. Future changes will add errors.
    pub async fn coco_state(&self) -> Result<coco::State, crate::error::Error> {
        if let Some(ref state) = *self.state.read().await {
            Ok(state.clone())
        } else {
            unreachable!("Context::state is always set on construction")
        }
    }
}
