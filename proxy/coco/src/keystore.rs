//! Storage of secret keys.

use std::convert::Infallible;

use librad::keys;
use librad::paths;
pub use radicle_keystore::pinentry::SecUtf8;
use radicle_keystore::{
    crypto::{Pwhash, SecretBoxError},
    file, FileStorage, Keystore, SecretKeyExt,
};

/// Last component of the file path to the key.
const KEY_PATH: &str = "librad.key";

/// Storage for putting and getting the necessary cryptographic keys.
pub struct Keystorage {
    /// Store to sign operations on the monorepo.
    store: Store,
}

impl Keystorage {
    /// Create a new `Keystorage`.
    #[must_use = "must use CocoStore to put/get a key"]
    pub fn new(paths: &paths::Paths, pw: SecUtf8) -> Self {
        let key_path = paths.keys_dir().join(KEY_PATH);
        Self {
            store: FileStorage::new(&key_path, Pwhash::new(pw)),
        }
    }

    /// Fetch the [`keys::SecretKey`]
    ///
    /// # Errors
    ///
    /// Fails with [`StoreError`]
    pub fn get(&self) -> Result<keys::SecretKey, Error> {
        Ok(self.store.get_key().map(|pair| pair.secret_key)?)
    }

    /// Attempt to get a [`keys::SecretKey`], otherwise we create one and store it.
    ///
    /// # Errors
    ///
    /// Fails with [`StoreError`]
    pub fn init(&mut self) -> Result<keys::SecretKey, Error> {
        match self.store.get_key() {
            Ok(keypair) => Ok(keypair.secret_key),
            Err(file::Error::NoSuchKey) => {
                let key = keys::SecretKey::new();
                self.store.put_key(key.clone())?;
                Ok(key)
            },
            Err(err) => Err(err.into()),
        }
    }
}

/// Synonym for an error when interacting with a store for [`librad::keys`].
type StoreError = file::Error<SecretBoxError<Infallible>, keys::IntoSecretKeyError>;
/// Synonym for storing the key.
type Store = FileStorage<
    Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

/// The [`Keystorage`] can result in two kinds of errors depending on what storage you're using.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that occurred when interacting with the `librad.key`.
    #[error(transparent)]
    Librad(#[from] StoreError),
}

#[cfg(test)]
mod tests {
    use super::Keystorage;
    use librad::paths;
    use radicle_keystore::pinentry::SecUtf8;

    #[allow(clippy::panic)]
    #[test]
    fn can_create_key() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(temp_dir.path())?;
        let pw = SecUtf8::from("asdf");
        let mut store = Keystorage::new(&paths, pw);

        let key = store.init().expect("could not create key:");

        assert!(
            key == store.get()?,
            "the stored key was not equal to the one retrieved"
        );

        Ok(())
    }
}
