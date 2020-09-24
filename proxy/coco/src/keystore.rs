//! Storage of secret keys.

use std::convert::Infallible;

use librad::{keys, paths};
pub use radicle_keystore::pinentry::SecUtf8;
use radicle_keystore::{
    crypto::{self, Pwhash, SecretBoxError},
    file, memory, FileStorage, Keystore, MemoryStorage, SecretKeyExt,
};

/// File name component of the file path to the key.
const KEY_PATH: &str = "librad.key";

/// Storage for putting and getting the necessary cryptographic keys.
///
/// The type parameter `S` determines the concrete storage backend in use.
pub struct Keystorage<S> {
    /// Store to sign operations on the monorepo.
    store: S,
}

/// Concrete type of the [`FileStorage`] in use.
type File = FileStorage<
    Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

/// Concrete type of the [`MemoryStorage`] in use.
type Memory = MemoryStorage<
    Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

impl Keystorage<File> {
    /// Create a file-backed keystore, suitable for production use.
    #[must_use = "must use CocoStore to put/get a key"]
    pub fn file(paths: &paths::Paths, pw: SecUtf8) -> Self {
        let key_path = paths.keys_dir().join(KEY_PATH);
        let crypto = Pwhash::new(pw, *crypto::KDF_PARAMS_PROD);
        Self {
            store: FileStorage::new(&key_path, crypto),
        }
    }

    /// Fetch the [`keys::SecretKey`]
    ///
    /// # Errors
    ///
    /// If no key is stored in this store (i.e. [`Self::init`] was not called), decryption fails,
    /// or some IO error occurred.
    pub fn get(&self) -> Result<keys::SecretKey, Error> {
        Ok(self.store.get_key().map(|pair| pair.secret_key)?)
    }

    /// Attempt to get a [`keys::SecretKey`], otherwise we create one and store it.
    ///
    /// # Errors
    ///
    /// Fails with [`FileError`]
    pub fn init(&mut self) -> Result<keys::SecretKey, Error> {
        match self.store.get_key() {
            Ok(keypair) => Ok(keypair.secret_key),
            Err(file::Error::NoSuchKey) => {
                let key = keys::SecretKey::new();
                self.store.put_key(key)?;
                Ok(key)
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl Keystorage<Memory> {
    /// Create an in-memory keystore, suitable for testing.
    ///
    /// A fresh [`keys::SecretKey`] will be generated every time this variant is
    /// instantiated.
    ///
    /// # Note
    ///
    /// This is not feature-gated behind `#[cfg(test)]`, because the sibling `api` crate needs to
    /// be able to access it. Use with extreme caution, and only from `#[cfg(test)]` code!
    ///
    /// # Errors
    ///
    /// If sealing the generated key fails.
    pub fn memory(pw: SecUtf8) -> Result<Self, Error> {
        let mut store = MemoryStorage::new(Pwhash::new(pw, *crypto::KDF_PARAMS_TEST));
        let key = keys::SecretKey::new();
        store.put_key(key)?;

        Ok(Self { store })
    }

    /// Fetch the [`keys::SecretKey`]
    #[must_use = "clippy insists this must be used :shrug:"]
    pub fn get(&self) -> keys::SecretKey {
        self.store
            .get_key()
            .map(|pair| pair.secret_key)
            .expect("constructor ensures a key is present")
    }
}

/// Synonym for an error when interacting with a file-backed store for [`librad::keys`].
type FileError = file::Error<SecretBoxError<Infallible>, keys::IntoSecretKeyError>;

/// Synonym for an error when interacting with a memory store for [`librad::keys`].
type MemoryError = memory::Error<SecretBoxError<Infallible>, keys::IntoSecretKeyError>;

/// The [`Keystorage`] can result in two kinds of errors depending on what storage you're using.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that occurred when interacting with the `librad.key`.
    #[error(transparent)]
    File(#[from] FileError),

    /// Errors that occurred when using the in-memory backend.
    #[error(transparent)]
    Mem(#[from] MemoryError),
}

#[cfg(test)]
mod tests {
    use super::Keystorage;
    use radicle_keystore::pinentry::SecUtf8;

    #[allow(clippy::panic)]
    #[test]
    fn can_create_key() {
        let pw = SecUtf8::from("asdf");
        let store = Keystorage::memory(pw).expect("could not create keystorage");
        let key = store.get();

        assert!(
            key.as_ref() == store.get().as_ref(),
            "the stored key was not equal to the one retrieved"
        )
    }
}
