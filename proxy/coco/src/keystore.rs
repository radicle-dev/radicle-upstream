//! Storage of secret keys.
//!
//! This module provides the [`KeyStore`] trait and the [`file()`] and [`memory()`] functions to
//! construct specific [`KeyStore`] implementations.

use std::convert::Infallible;

use librad::{keys, paths};
pub use radicle_keystore::pinentry::SecUtf8;
use radicle_keystore::{
    crypto::{self, Pwhash, SecretBoxError},
    file, Keystore, SecretKeyExt,
};

/// Storage for one secret key.
pub trait KeyStore {
    /// Create a key and store it encrypted with the given passphrase.
    ///
    /// # Errors
    ///
    /// Errors when the storage backend fails to persist the key or a key already exists.
    fn create_key(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error>;

    /// Get the secret from the storage.
    ///
    /// # Errors
    ///
    /// * Errors if the password is wrong.
    /// * Errors if backend fails to retrieve the data.
    /// * Errors if there is no key in the storage yet.
    fn get(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error>;
}

/// File name component of the file path to the key.
const KEY_PATH: &str = "librad.key";

/// Create a [`KeyStore`] that is backed by an encrypted file on disk.
///
/// The key file is named `librad.key` and located under in the `paths` key directory.
#[must_use]
pub fn file(paths: paths::Paths) -> impl KeyStore + Send + Sync {
    FileStore { paths }
}

/// File-backed [`KeyStore`]
struct FileStore {
    /// Determines the location of the key file when a key is loaded or written.
    paths: paths::Paths,
}

/// Concrete type of the [`FileStorage`] in use.
type FileStorage = radicle_keystore::FileStorage<
    Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

impl FileStore {
    /// Get the [`FileStorage`] backend for this key store.
    fn store(&self, passphrase: SecUtf8) -> FileStorage {
        let key_path = self.paths.keys_dir().join(KEY_PATH);
        let crypto = Pwhash::new(passphrase, *crypto::KDF_PARAMS_PROD);
        FileStorage::new(&key_path, crypto)
    }
}

impl KeyStore for FileStore {
    fn create_key(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error> {
        let mut store = self.store(passphrase);
        match store.get_key() {
            Ok(_keypair) => Err(FileError::KeyExists.into()),
            Err(FileError::NoSuchKey) => {
                let key = keys::SecretKey::new();
                store.put_key(key)?;
                Ok(key)
            },
            Err(err) => Err(err.into()),
        }
    }

    fn get(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error> {
        let key_pair = self.store(passphrase).get_key()?;
        Ok(key_pair.secret_key)
    }
}

/// Create an insecure in-memory [`KeyStore`].
#[must_use]
pub fn memory() -> impl KeyStore + Send + Sync {
    MemoryStore {
        key_and_passphrase: std::sync::Mutex::new(None),
    }
}

/// Insecure in-memory [`KeyStore`]
struct MemoryStore {
    /// Secret key and passphrase if present
    key_and_passphrase: std::sync::Mutex<Option<(keys::SecretKey, SecUtf8)>>,
}

impl KeyStore for MemoryStore {
    fn create_key(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error> {
        let mut key_and_passphrase = self
            .key_and_passphrase
            .lock()
            .expect("Failed to access memory key");
        if key_and_passphrase.is_some() {
            return Err(FileError::KeyExists.into());
        }

        let key = keys::SecretKey::new();
        *key_and_passphrase = Some((key, passphrase));
        Ok(key)
    }

    fn get(&self, passphrase: SecUtf8) -> Result<keys::SecretKey, Error> {
        if let Some((key, stored_passphrase)) = &*self
            .key_and_passphrase
            .lock()
            .expect("Failed to access memory key")
        {
            if *stored_passphrase == passphrase {
                Ok(*key)
            } else {
                Err(FileError::Crypto(SecretBoxError::InvalidKey).into())
            }
        } else {
            Err(FileError::NoSuchKey.into())
        }
    }
}

/// Error type for the [`FileStorage`] backend.
type FileError = file::Error<SecretBoxError<Infallible>, keys::IntoSecretKeyError>;

/// Errors that occur when creating or unsealing keys.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    /// The inner error
    inner: FileError,
}

impl Error {
    /// Returns `true` if the error indicates that an invalid password was used to decrypt the
    /// secret key.
    #[must_use]
    pub const fn is_invalid_password(&self) -> bool {
        #[allow(clippy::wildcard_enum_match_arm)]
        matches!(self.inner, FileError::Crypto(SecretBoxError::InvalidKey))
    }

    /// Returns `true` if the error indicates that a key already exists in the store.
    #[must_use]
    pub const fn is_key_exists(&self) -> bool {
        #[allow(clippy::wildcard_enum_match_arm)]
        matches!(self.inner, FileError::KeyExists)
    }
}
