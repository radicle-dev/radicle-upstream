//! Datastructure and machinery to safely share the common dependencies across components.

use coco::{signer, PeerControl};

#[cfg(test)]
use coco::{keystore, RunConfig};

/// Container to pass down dependencies into HTTP filter chains.
#[derive(Clone)]
pub struct Context {
    /// Handle to inspect state and perform actions on the currently running local [`coco::Peer`].
    pub peer_control: PeerControl,
    /// [`coco::State`] to operate on the local monorepo.
    pub state: coco::State,
    /// [`coco::signer::BoxedSigner`] for write operations on the monorepo.
    pub signer: Option<signer::BoxedSigner>,
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

            (peer.control(), state)
        };

        Ok(Self {
            peer_control,
            state,
            signer: Some(signer),
            store,
        })
    }
}
