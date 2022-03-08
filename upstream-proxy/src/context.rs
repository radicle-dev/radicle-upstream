// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Datastructure and machinery to safely share the common dependencies across components.

use std::sync::Arc;

use data_encoding::HEXLOWER;
use futures::prelude::*;
use rand::Rng as _;
use tokio::sync::RwLock;

use crate::{keystore, service};

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
    pub const fn test(&self) -> bool {
        match self {
            Self::Sealed(sealed) => sealed.test,
            Self::Unsealed(unsealed) => unsealed.rest.test,
        }
    }

    /// Returns `true` if the HTTP api will accept any request without checking the auth token.
    pub const fn insecure_http_api(&self) -> bool {
        match self {
            Self::Sealed(sealed) => sealed.insecure_http_api,
            Self::Unsealed(unsealed) => unsealed.rest.insecure_http_api,
        }
    }

    /// Returns a mutable reference to the authentication cookie value.
    pub fn auth_token(&self) -> Arc<RwLock<Option<String>>> {
        match self {
            Self::Sealed(sealed) => sealed.auth_token.clone(),
            Self::Unsealed(unsealed) => unsealed.rest.auth_token.clone(),
        }
    }

    /// Returns a handle to control the service configuration
    pub fn service_handle(&mut self) -> &mut service::Handle {
        match self {
            Self::Sealed(sealed) => &mut sealed.service_handle,
            Self::Unsealed(unsealed) => &mut unsealed.rest.service_handle,
        }
    }

    pub fn read_only_storage(&self) -> Result<librad::git::storage::ReadOnly, crate::error::Error> {
        let paths = match self {
            Self::Sealed(sealed) => &sealed.paths,
            Self::Unsealed(unsealed) => &unsealed.rest.paths,
        };
        let storage = librad::git::storage::ReadOnly::open(paths)?;
        Ok(storage)
    }

    /// Unseal the key store and restart the coco service with the obtained key. Returns the auth
    /// token required to access the keystore.
    ///
    /// # Errors
    ///
    /// * Errors if the passphrase is wrong.
    /// * Errors if backend fails to retrieve the data.
    /// * Errors if there is no key in the storage yet.
    pub async fn unseal_keystore(
        &mut self,
        passphrase: keystore::SecUtf8,
    ) -> Result<String, crate::error::Error> {
        let keystore = self.keystore();
        let key = tokio::task::spawn_blocking(move || keystore.get(passphrase))
            .await
            .expect("Task to unseal key was aborted")?;
        self.service_handle().set_secret_key(key);
        let auth_token = self.reset_auth_token().await;
        Ok(auth_token)
    }

    /// Create a key and store it encrypted with the given passphrase. Then restart the coco
    /// service to use the new key. Returns the auth token required to access the keystore.
    ///
    /// # Errors
    ///
    /// Errors when the storage backend fails to persist the key or a key already exists.
    pub async fn create_key(
        &mut self,
        passphrase: keystore::SecUtf8,
    ) -> Result<String, crate::error::Error> {
        let keystore = self.keystore();
        let key = tokio::task::spawn_blocking(move || keystore.create_key(passphrase))
            .await
            .expect("Task to create key was aborted")?;
        self.service_handle().set_secret_key(key);
        let auth_token = self.reset_auth_token().await;
        Ok(auth_token)
    }

    fn keystore(&self) -> Arc<dyn keystore::Keystore + Sync + Send> {
        match self {
            Self::Sealed(sealed) => sealed.keystore.clone(),
            Self::Unsealed(unsealed) => unsealed.rest.keystore.clone(),
        }
    }

    /// Generate a new authentication token and store it.
    async fn reset_auth_token(&self) -> String {
        let new_token_data = rand::thread_rng().gen::<[u8; 32]>();
        let new_token = HEXLOWER.encode(&new_token_data);
        let auth_token_lock = self.auth_token();
        let mut auth_token = auth_token_lock.write().await;
        *auth_token = Some(new_token.clone());
        new_token
    }

    /// Returns `true` if `token` matches the stored authentication token.
    pub async fn check_auth_token(&self, token: String) -> bool {
        Some(token) == *self.auth_token().read().await
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
    pub peer: crate::peer::Peer,
    pub rest: Sealed,
}

/// Context for HTTP request if the coco peer APIs have not been initialized yet.
#[derive(Clone)]
pub struct Sealed {
    /// [`kv::Store`] used for session state and cache.
    pub store: kv::Store,
    /// Flag to control if the stack is set up in test mode.
    pub test: bool,
    /// If `true`, the HTTP api will accept any request without checking the auth token.
    pub insecure_http_api: bool,
    /// Default seeds that will be written to the settings kv store.
    pub default_seeds: Vec<String>,
    pub seeds: Option<Vec<String>>,
    /// Handle to control the service configuration.
    pub service_handle: service::Handle,
    /// Cookie set on unsealing the key store.
    pub auth_token: Arc<RwLock<Option<String>>>,
    /// Reference to the key store.
    pub keystore: Arc<dyn keystore::Keystore + Send + Sync>,
    pub paths: librad::paths::Paths,
    /// Receives a notification when the server is asked to shut down
    pub shutdown: Arc<tokio::sync::Notify>,
}

impl Unsealed {
    /// Return a stream that emits peer events.
    ///
    /// The stream ends when API server is shut down.
    pub fn peer_events(&self) -> impl Stream<Item = crate::daemon::PeerEvent> + Send + 'static {
        let shutdown = self.rest.shutdown.clone();
        self.peer
            .events()
            .take_until(async move { shutdown.notified().await })
    }

    /// Initialises a new [`Unsealed`] context with the store and coco state in the given temporary
    /// directory.
    ///
    /// # Errors
    ///
    /// * coco key creation fails
    /// * creation of the [`kv::Store`] fails
    #[cfg(test)]
    pub fn tmp(
        tmp_dir: &tempfile::TempDir,
    ) -> Result<(Self, impl Future<Output = ()>), crate::error::Error> {
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        let key = link_crypto::SecretKey::new();
        let paths = librad::paths::Paths::from_root(tmp_dir.path())?;

        let seeds_watch = tokio::sync::watch::channel(vec![]).1;

        let (peer, peer_runner) = crate::peer::create(crate::peer::Config {
            paths: paths.clone(),
            key,
            store: store.clone(),
            discovery: crate::daemon::config::StreamDiscovery::new(seeds_watch),
            listen: "127.0.0.1:0".parse().expect("invalid IP address"),
        })
        .unwrap();

        let run_handle = async move {
            let (shutdown_tx, shutdown_rx) = futures::channel::oneshot::channel::<()>();
            let run = tokio::task::spawn(peer_runner.run(shutdown_rx.map(|_| ()).boxed()));
            if let Err(err) = run.await {
                tracing::error!(?err, "peer run error");
            }
            drop(shutdown_tx);
        };

        Ok((
            Self {
                peer,
                rest: Sealed {
                    store,
                    test: false,
                    insecure_http_api: true,
                    default_seeds: vec![],
                    seeds: None,
                    service_handle: service::Handle::dummy(),
                    auth_token: Arc::new(RwLock::new(None)),
                    keystore: Arc::new(keystore::memory()),
                    paths,
                    shutdown: Arc::new(tokio::sync::Notify::new()),
                },
            },
            run_handle,
        ))
    }
}
