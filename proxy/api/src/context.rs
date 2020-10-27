//! Datastructure and machinery to safely share the common dependencies across components.

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::service;
use coco::PeerControl;

#[cfg(test)]
use coco::{keystore, signer, RunConfig};

/// Container to pass down dependencies into HTTP filter chains.
#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Context {
    /// Coco peer API is sealed and unavailable
    Sealed(Sealed),
    /// Coco peer API is unsealed and available
    Unsealed(Unsealed),
}

impl Context {
    /// Returns `true` if the stack is set up in test mode.
    pub fn test(&self) -> bool {
        match self {
            Self::Sealed(sealed) => sealed.test,
            Self::Unsealed(unsealed) => unsealed.test,
        }
    }

    /// Returns the [`kv::Store`] for persistent storage.
    pub fn store(&self) -> &kv::Store {
        match self {
            Self::Sealed(sealed) => &sealed.store,
            Self::Unsealed(unsealed) => &unsealed.store,
        }
    }

    pub fn auth_cookie(&self) -> Arc<RwLock<Option<String>>> {
        match self {
            Self::Sealed(sealed) => sealed.auth_cookie.clone(),
            Self::Unsealed(unsealed) => unsealed.auth_cookie.clone(),
        }
    }

    pub fn service_handle(&mut self) -> &mut service::Handle {
        match self {
            Context::Sealed(sealed) => &mut sealed.service_handle,
            Context::Unsealed(unsealed) => &mut unsealed.service_handle,
        }
    }
}

impl From<Unsealed> for Context {
    fn from(unsealed: Unsealed) -> Self {
        Self::Unsealed(unsealed)
    }
}

impl From<Sealed> for Context {
    fn from(sealed: Sealed) -> Self {
        Self::Sealed(sealed)
    }
}

/// Context for HTTP requests with access to coco peer APIs.
#[derive(Clone)]
pub struct Unsealed {
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
    /// Handle to inspect state and perform actions on the currently running local [`coco::Peer`].
    pub peer_control: PeerControl,
    /// [`coco::State`] to operate on the local monorepo.
    pub state: coco::State,
    /// Flag to control if the stack is set up in test mode.
    pub test: bool,
    pub service_handle: service::Handle,

    /// Cookie set on unsealing the key store.
    pub auth_cookie: Arc<RwLock<Option<String>>>,
}

/// Context for HTTP request if the coco peer APIs have not been initialized yet.
#[derive(Clone)]
pub struct Sealed {
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
    /// Flag to control if the stack is set up in test mode.
    pub test: bool,
    pub paths: coco::Paths,
    pub service_handle: service::Handle,
    /// Cookie set on unsealing the key store.
    pub auth_cookie: Arc<RwLock<Option<String>>>,
}

impl Unsealed {
    /// Initialises a new [`ContextUnselaed`] the given temporary directory.
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
            state,
            store,
            test: false,
            service_handle: service::Handle::dummy(),
            auth_cookie: Arc::new(RwLock::new(None)),
        })
    }
}
