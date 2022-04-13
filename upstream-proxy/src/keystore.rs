// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Copyright © 2019-2020 The Radicle Foundation <hello@radicle.foundation>
//
// This file is part of radicle-link, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Storage of secret keys.
//!
//! This module provides the [`Keystore`] trait and the [`file()`] function to construct specific
//! [`Keystore`] implementations.

use std::{convert::Infallible, path::PathBuf};

pub use radicle_keystore::pinentry::SecUtf8;
use radicle_keystore::{
    crypto::{self, Pwhash, SecretBoxError},
    file, Keystore as _, SecretKeyExt,
};

/// Storage for one secret key.
pub trait Keystore {
    /// Create a key and store it encrypted with the given passphrase.
    ///
    /// # Errors
    ///
    /// Errors when the storage backend fails to persist the key or a key
    /// already exists.
    fn create_key(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error>;

    /// Get the secret from the storage.
    ///
    /// # Errors
    ///
    /// * Errors if the passphrase is wrong.
    /// * Errors if backend fails to retrieve the data.
    /// * Errors if there is no key in the storage yet.
    fn get(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error>;

    /// Returns true if the keystore has a key.
    fn has_key(&self) -> bool;
}

/// Create a [`Keystore`] that is backed by an encrypted file on disk.
#[must_use]
pub fn file(path: PathBuf) -> FileStore {
    FileStore {
        path,
        kdf_params: *crypto::KDF_PARAMS_PROD,
    }
}

/// Create a [`Keystore`] that is backed by an encrypted file on disk and uses weak (but fast)
/// encrpytion parameters.
#[must_use]
pub fn unsafe_fast_file(path: PathBuf) -> FileStore {
    FileStore {
        path,
        kdf_params: *crypto::KDF_PARAMS_TEST,
    }
}

/// File-backed [`Keystore`]
pub struct FileStore {
    /// Determines the location of the key file when a key is loaded or written.
    path: PathBuf,
    kdf_params: crypto::KdfParams,
}

/// Concrete type of the [`FileStorage`] in use.
type FileStorage = radicle_keystore::FileStorage<
    Pwhash<SecUtf8>,
    link_crypto::PublicKey,
    link_crypto::SecretKey,
    <link_crypto::SecretKey as SecretKeyExt>::Metadata,
>;

impl FileStore {
    /// Get the [`FileStorage`] backend for this key store.
    fn store(&self, passphrase: SecUtf8) -> FileStorage {
        let crypto = Pwhash::new(passphrase, self.kdf_params);
        FileStorage::new(&self.path, crypto)
    }

    /// Create a key and store it encrypted with the given passphrase.
    ///
    /// If `seed_data` is given the secret key is deterministically derived from the hash of the
    /// seed data. Otherwise, a random key is generated.
    ///
    /// # Errors
    ///
    /// Errors when the storage backend fails to persist the key or a key
    /// already exists.
    pub fn create_key_with_seed(
        &self,
        passphrase: SecUtf8,
        seed_data: Option<&[u8]>,
    ) -> Result<link_crypto::SecretKey, Error> {
        let mut store = self.store(passphrase);
        match store.get_key() {
            Ok(_keypair) => Err(FileError::KeyExists(store.key_file_path().to_owned()).into()),
            Err(FileError::NoSuchKey(_)) => {
                let key = match seed_data {
                    Some(seed_data) => {
                        use sha2::Digest;
                        let seed = sha2::Sha256::digest(seed_data);
                        link_crypto::SecretKey::from_seed(seed.into())
                    },
                    None => link_crypto::SecretKey::new(),
                };
                store.put_key(key.clone())?;
                Ok(key)
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Keystore for FileStore {
    fn create_key(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error> {
        self.create_key_with_seed(passphrase, None)
    }

    fn get(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error> {
        let key_pair = self.store(passphrase).get_key()?;
        Ok(key_pair.secret_key)
    }

    fn has_key(&self) -> bool {
        self.path.exists()
    }
}

/// Insecure in-memory [`Keystore`]
#[cfg(test)]
struct MemoryStore {
    /// Secret key and passphrase if present
    key_and_passphrase: std::sync::Mutex<Option<(link_crypto::SecretKey, SecUtf8)>>,
}

#[cfg(test)]
impl Keystore for MemoryStore {
    fn create_key(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error> {
        let mut key_and_passphrase = self
            .key_and_passphrase
            .lock()
            .expect("Failed to access memory key");
        if key_and_passphrase.is_some() {
            return Err(FileError::KeyExists(PathBuf::new()).into());
        }

        let key = link_crypto::SecretKey::new();
        *key_and_passphrase = Some((key.clone(), passphrase));
        Ok(key)
    }

    fn get(&self, passphrase: SecUtf8) -> Result<link_crypto::SecretKey, Error> {
        if let Some((key, stored_passphrase)) = &*self
            .key_and_passphrase
            .lock()
            .expect("Failed to access memory key")
        {
            if *stored_passphrase == passphrase {
                Ok(key.clone())
            } else {
                Err(FileError::Crypto(SecretBoxError::InvalidKey).into())
            }
        } else {
            Err(FileError::NoSuchKey(PathBuf::new()).into())
        }
    }

    fn has_key(&self) -> bool {
        self.key_and_passphrase
            .lock()
            .expect("failed to lock")
            .is_some()
    }
}

/// Error type for the [`FileStorage`] backend.
type FileError = file::Error<SecretBoxError<Infallible>, link_crypto::IntoSecretKeyError>;

/// Errors that occur when creating or unsealing keys.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    /// The inner error
    inner: FileError,
}

impl Error {
    /// Returns `true` if the error indicates that an invalid passphrase was
    /// used to decrypt the secret key.
    #[must_use]
    pub const fn is_invalid_passphrase(&self) -> bool {
        matches!(self.inner, FileError::Crypto(SecretBoxError::InvalidKey))
    }

    /// Returns `true` if the error indicates that a key already exists in the
    /// store.
    #[must_use]
    pub const fn is_key_exists(&self) -> bool {
        matches!(self.inner, FileError::KeyExists(_))
    }
}
