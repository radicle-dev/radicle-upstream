//! Storage of secret keys.

use std::convert::Infallible;

use librad::keys;
use librad::paths;
pub use radicle_keystore::pinentry::SecUtf8;
use radicle_keystore::{
    crypto::{Pwhash, SecretBoxError},
    file, FileStorage, Keystore, SecretKeyExt,
};

/// File path to librad key
const LIBRAD_KEY: &str = "librad.key";

/// Storage for putting and getting the necessary cryptographic keys.
pub struct Keystorage {
    /// Store for `librad`.
    librad_store: LibradStore,
}

impl Keystorage {
    /// Create a new `Keystorage`.
    #[must_use = "must use CocoStore to put/get a key"]
    pub fn new(paths: &paths::Paths, pw: SecUtf8) -> Self {
        let path = paths.keys_dir();
        let librad_path = path.join(LIBRAD_KEY);
        Self {
            librad_store: FileStorage::new(&librad_path, Pwhash::new(pw.clone())),
        }
    }

    /// Fetch the [`keys::SecretKey`]
    ///
    /// # Errors
    ///
    /// Fails with [`LibradError`]
    pub fn get_librad_key(&self) -> Result<keys::SecretKey, Error> {
        Ok(self.librad_store.get_key().map(|pair| pair.secret_key)?)
    }

    /// Attempt to get a [`keys::SecretKey`], otherwise we create one and store it.
    ///
    /// # Errors
    ///
    /// Fails with [`LibradError`]
    pub fn init_librad_key(&mut self) -> Result<keys::SecretKey, Error> {
        match self.librad_store.get_key() {
            Ok(keypair) => Ok(keypair.secret_key),
            Err(file::Error::NoSuchKey) => {
                let key = keys::SecretKey::new();
                self.librad_store.put_key(key.clone())?;
                Ok(key)
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Synonym for an error when interacting with a store for [`librad::keys`].
type LibradError = file::Error<SecretBoxError<Infallible>, keys::IntoSecretKeyError>;
/// Synonym for storing keys related to `librad`.
type LibradStore = FileStorage<
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
    Librad(#[from] LibradError),
}

#[cfg(test)]
mod tests {
    use super::Keystorage;
    use librad::paths;
    use radicle_keystore::pinentry::SecUtf8;

    #[allow(clippy::panic)]
    #[test]
    fn can_create_librad_key() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(temp_dir.path())?;
        let pw = SecUtf8::from("asdf");
        let mut store = Keystorage::new(&paths, pw);

        let key = store.init_librad_key().expect("could not create key:");

        assert!(
            key == store.get_librad_key()?,
            "the stored key was not equal to the one retrieved"
        );

        Ok(())
    }

    #[test]
    fn can_create_registry_key() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(temp_dir.path())?;
        let pw = SecUtf8::from("asdf");
        let mut store = Keystorage::new(&paths, pw);

        let _key = store
            .init_registry_key()
            .expect("could not get or create a key:");

        // N.B. We'd like to test that the get does actually get the right key back, buuut we can't
        // compare them OH WELL.

        Ok(())
    }
}
