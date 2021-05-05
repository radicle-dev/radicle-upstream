//! Utilities for changing the service environment used in [`crate::process`].

use futures::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, Notify};

use radicle_daemon::{keys, keystore, profile};

/// Persistent environment with depedencies for running the API and coco peer services.
pub struct Environment {
    /// Secret key for the coco peer.
    ///
    /// If this is `None` coco is not started.
    pub key: Option<keys::SecretKey>,
    /// If set, we use a temporary directory for on-disk persistence.
    pub temp_dir: Option<tempfile::TempDir>,
    /// Paths & profile id for on-disk persistence.
    pub coco_profile: profile::Profile,
    /// A reference to the key store.
    pub keystore: Arc<dyn keystore::Keystore + Send + Sync>,
    /// If true we are running the service in test mode.
    pub test_mode: bool,
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
    Profile(#[from] profile::Error),
}

impl Environment {
    /// Create a new initial environment.
    ///
    /// If `test_mode` is `true` then `Environment::temp_dir` is set for temporary on-disk
    /// persistence.
    fn new(test_mode: bool) -> Result<Self, Error> {
        if test_mode {
            let temp_dir = tempfile::tempdir()?;
            let coco_profile = profile::Profile::from_root(temp_dir.path(), None)?;
            let keystore = Arc::new(keystore::memory());
            Ok(Self {
                key: None,
                temp_dir: Some(temp_dir),
                coco_profile,
                keystore,
                test_mode,
            })
        } else {
            let coco_profile = profile::Profile::load()?;
            let keystore = Arc::new(keystore::file(coco_profile.paths().clone()));
            Ok(Self {
                key: None,
                temp_dir: None,
                coco_profile,
                keystore,
                test_mode,
            })
        }
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
}

impl Manager {
    /// Create a new manager.
    ///
    /// If `test_mode` is `true` then `Environment::temp_dir` is set for temporary on-disk
    /// persistence.
    pub fn new(test_mode: bool) -> Result<Self, Error> {
        let environment = Environment::new(test_mode)?;
        let (message_sender, message_receiver) = mpsc::channel(10);
        Ok(Self {
            reload_notify: Arc::new(Notify::new()),
            message_sender,
            message_receiver,
            environment,
        })
    }

    /// Get a handle to send updates to [`Manager`].
    pub fn handle(&self) -> Handle {
        Handle {
            reload_notify: self.reload_notify.clone(),
            message_sender: self.message_sender.clone(),
        }
    }

    /// Get the current environment
    pub fn environment(&mut self) -> Result<&Environment, Error> {
        while let Some(Some(message)) = self.message_receiver.recv().now_or_never() {
            match message {
                Message::Reset => {
                    let test_mode = self.environment.test_mode;
                    self.environment = Environment::new(test_mode)?
                },
                Message::SetSecretKey(key) => self.environment.key = Some(key),
                Message::Seal => self.environment.key = None,
            }
        }

        Ok(&self.environment)
    }

    /// Returns a future that becomes ready when the service needs to restart because the
    /// environment has changed.
    pub fn notified_restart(&mut self) -> impl Future<Output = ()> + Send + 'static {
        let reload_notify = Arc::new(Notify::new());
        self.reload_notify = reload_notify.clone();
        async move { reload_notify.notified().await }
    }
}

/// Messages that are sent from [`Handle`] to [`Manager`] to change the service environment.
#[allow(clippy::clippy::large_enum_variant)]
enum Message {
    /// Reset the service to the initial environment and delete all persisted state
    Reset,
    /// Unseal the key store with the given secret key
    SetSecretKey(keys::SecretKey),
    /// Seal the key store and reload the services
    Seal,
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
    pub fn set_secret_key(&mut self, key: keys::SecretKey) {
        self.send_message(Message::SetSecretKey(key))
    }

    /// Seal the key store and reload the services
    pub fn seal(&mut self) {
        self.send_message(Message::Seal)
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
        self.reload_notify.notify_one();
    }

    /// Create a handle where none of the methods have any effect.
    #[cfg(test)]
    pub fn dummy() -> Self {
        let (message_sender, mut message_receiver) = mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                if message_receiver.recv().await.is_none() {
                    break;
                }
            }
        });
        Self {
            reload_notify: Arc::new(Notify::new()),
            message_sender,
        }
    }
}
