// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Utilities for changing the service environment used in [`crate::process`].

use anyhow::Context as _;
use futures::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, Notify};

use crate::keystore;

/// Persistent environment with depedencies for running the API and coco peer services.
pub struct Environment {
    /// Secret key for the coco peer.
    ///
    /// If this is `None` coco is not started.
    pub key: Option<link_crypto::SecretKey>,
    /// If set, we use a temporary directory for on-disk persistence.
    pub temp_dir: Option<tempfile::TempDir>,
    /// Paths & profile id for on-disk persistence.
    pub coco_profile: librad::profile::Profile,
    /// A reference to the key store.
    pub keystore: Arc<dyn crate::keystore::Keystore + Send + Sync>,
    /// If true, we are running the service in test mode.
    pub test_mode: bool,
}

/// Configuration for initializing [`Environment`].
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    /// If `true`, then [`Environment::temp_dir`] is set for temporary on-disk persistence and
    /// [`Environment::test_mode`] is set to `true`.
    pub test_mode: bool,

    /// If `true`, then fast but unsafe encryption parameters are used for the keystore.
    pub unsafe_fast_keystore: bool,

    /// Path to the secret key for the identity. Uses `LNK_HOME` if not provided.
    pub identity_key: Option<std::path::PathBuf>,
}

/// Error returned when creating a new [`Environment`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to create temporary directory
    #[error("failed to create temporary directory")]
    TempDir(
        #[source]
        #[from]
        std::io::Error,
    ),
    #[error(transparent)]
    Profile(#[from] librad::profile::Error),
}

impl Environment {
    /// Create a new initial environment.
    fn new(config: &EnvironmentConfig) -> Result<Self, Error> {
        let (temp_dir, coco_profile) = if config.test_mode {
            let temp_dir = tempfile::tempdir()?;
            let coco_profile = librad::profile::Profile::from_root(temp_dir.path(), None)?;
            (Some(temp_dir), coco_profile)
        } else {
            let coco_profile = librad::profile::Profile::load()?;
            (None, coco_profile)
        };

        let key_file = if let Some(identity_key) = config.identity_key.clone() {
            identity_key
        } else {
            coco_profile.paths().keys_dir().join("librad.key")
        };

        let keystore: Arc<dyn keystore::Keystore + Send + Sync> = if config.unsafe_fast_keystore {
            Arc::new(keystore::unsafe_fast_file(key_file))
        } else {
            Arc::new(keystore::file(key_file))
        };

        Ok(Self {
            key: None,
            temp_dir,
            coco_profile,
            keystore,
            test_mode: config.test_mode,
        })
    }
}

/// Manages changes to [`Environment`].
pub struct Manager {
    /// Notifier to restart the services
    reload_notify: Arc<Notify>,
    /// Sender side of the [`Message`] channel
    message_sender: mpsc::Sender<Message>,
    /// Receiver side of the [`Message`] channel
    message_receiver: mpsc::Receiver<Message>,
    /// The current environemtn of the services
    environment: Environment,
    environment_config: EnvironmentConfig,
}

impl Manager {
    /// Create a new manager.
    ///
    /// If `test_mode` is `true` then `Environment::temp_dir` is set for temporary on-disk
    /// persistence.
    pub fn new(environment_config: EnvironmentConfig) -> Result<Self, Error> {
        let environment = Environment::new(&environment_config)?;
        let (message_sender, message_receiver) = mpsc::channel(10);
        Ok(Self {
            reload_notify: Arc::new(Notify::new()),
            message_sender,
            message_receiver,
            environment,
            environment_config,
        })
    }

    /// Get a handle to send updates to [`Manager`].
    pub fn handle(&self) -> Handle {
        Handle {
            reload_notify: self.reload_notify.clone(),
            message_sender: self.message_sender.clone(),
        }
    }

    /// Get the current environment. If `None` is returned this indicates that the service was
    /// asked to shut down.
    pub fn environment(&mut self) -> Result<Option<&Environment>, Error> {
        while let Some(Some(message)) = self.message_receiver.recv().now_or_never() {
            match message {
                Message::Reset => self.environment = Environment::new(&self.environment_config)?,
                Message::SetSecretKey(key) => self.environment.key = Some(key),
                Message::Seal => self.environment.key = None,
                Message::Shutdown => return Ok(None),
            }
        }

        Ok(Some(&self.environment))
    }

    /// Returns a future that becomes ready when the service needs to restart because the
    /// environment has changed.
    pub fn notified_restart(&mut self) -> impl Future<Output = ()> + Send + 'static {
        let reload_notify = self.reload_notify.clone();
        async move { reload_notify.notified().await }
    }

    /// Unseal the keystore with the given passphrase. Afterwards the `Environment` returned by
    /// [`Self::environment`] contains the secret key.
    pub fn unseal_keystore(
        &mut self,
        passphrase: radicle_keystore::pinentry::SecUtf8,
    ) -> Result<(), anyhow::Error> {
        let env = self
            .environment()
            .context("failed to load environment")?
            .ok_or_else(|| anyhow::anyhow!("service has been shut down"))?;
        let key = env
            .keystore
            .get(passphrase)
            .context("failed to get key with passphrase")?;
        self.handle().set_secret_key(key);
        Ok(())
    }
}

/// Messages that are sent from [`Handle`] to [`Manager`] to change the service environment.
#[allow(clippy::large_enum_variant)]
enum Message {
    /// Reset the service to the initial environment and delete all persisted state
    Reset,
    /// Unseal the key store with the given secret key
    SetSecretKey(link_crypto::SecretKey),
    /// Seal the key store and reload the services
    Seal,
    /// Shutdown the service and exit the process
    Shutdown,
}

/// A handle to communicate with [`Manager`].
#[derive(Clone)]
pub struct Handle {
    /// Notifier to restart the services
    reload_notify: Arc<Notify>,
    /// Sender side of the [`Message`] channel
    message_sender: mpsc::Sender<Message>,
}

impl Handle {
    /// Reset the service to the initial configuration and delete all persisted state
    pub fn reset(&mut self) {
        self.send_message(Message::Reset)
    }

    /// Unseal the key store with the given secret key
    pub fn set_secret_key(&mut self, key: link_crypto::SecretKey) {
        self.send_message(Message::SetSecretKey(key))
    }

    /// Seal the key store and reload the services
    pub fn seal(&mut self) {
        self.send_message(Message::Seal)
    }

    /// Shutdown the service and exit the process
    pub fn shutdown(&mut self) {
        self.send_message(Message::Shutdown)
    }

    /// Send [`Message`] to [`Manager`]
    fn send_message(&mut self, message: Message) {
        #![allow(clippy::panic)]
        match self.message_sender.try_send(message) {
            Ok(()) => {},
            Err(err) => match err {
                mpsc::error::TrySendError::Full(_) => {
                    // In practice we can’t send more than one update message at a time.
                    panic!("service::Manager message queue is full")
                },
                mpsc::error::TrySendError::Closed(_) => {
                    // The manager must not be dropped before all handles are dropped.
                    panic!("service::Manager meesage queue is closed")
                },
            },
        }
        self.reload_notify.notify_waiters()
    }
}
