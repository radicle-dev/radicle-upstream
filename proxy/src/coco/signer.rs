//! Signing and key handling.

use std::convert::Infallible;

use librad::keys;
use librad::paths;
use librad::signer;
use radicle_keystore::crypto;
use radicle_keystore::file;
use radicle_keystore::sign::{self, Signer as _};
use radicle_keystore::{FileStorage, Keystore, SecretKeyExt};

pub use librad::keys::SignError;
pub use radicle_keystore::pinentry::SecUtf8;

/// Filename of the key on disk.
const KEY: &str = "librad.key";

/// Blanket trait to use as our generic [`signer::Signer`].
pub trait Signer: Clone + keys::AsPKCS8 + signer::Signer {}

impl<T: Clone + keys::AsPKCS8 + signer::Signer> Signer for T {}

/// Utility trait to manage state for an exisitng [`Signer`].
pub trait Reset: Signer {
    /// Reset the [`Signer`] state under the given path.
    ///
    /// # Errors
    ///
    /// * if the write of the newly generated key fails
    fn reset(&mut self, paths: &paths::Paths, passphrase: SecUtf8) -> Result<(), Error>;
}

/// Synonym for an error when interacting with a store for [`librad::keys`].
pub type Error = file::Error<crypto::SecretBoxError<Infallible>, keys::IntoSecretKeyError>;

/// Parameterised [`FileStorage`] used in Store implementation.
type Storage = FileStorage<
    crypto::Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

/// [`FileStorage`] backed [`Signer`] implementation.
#[derive(Clone)]
pub struct Store {
    /// Cached public key.
    public_key: sign::PublicKey,
    /// The underlying [`FileStorage`].
    storage: Storage,
}

impl Store {
    /// Sets up a new [`Store`] with a fresh key.
    ///
    /// # Errors
    ///
    /// * if the file system errors for any other reason than [`file::Error::NoSuchKey`]
    /// * if the write of the generated key fails
    pub fn init(paths: &paths::Paths, passphrase: SecUtf8) -> Result<Self, Error> {
        let path = paths.keys_dir();
        let key_path = path.join(KEY);
        let mut storage = FileStorage::new(&key_path, crypto::Pwhash::new(passphrase));

        let key = match storage.get_key() {
            Ok(key) => Ok(key.secret_key),
            Err(err) => match err {
                file::Error::NoSuchKey => {
                    let key = keys::SecretKey::new();
                    storage.put_key(key.clone())?;
                    Ok(key)
                },
                file::Error::KeyExists
                | file::Error::Crypto(_)
                | file::Error::Conversion(_)
                | file::Error::Serde(_)
                | file::Error::Io(_) => Err(err),
            },
        }?;

        Ok(Self {
            public_key: key.public_key(),
            storage,
        })
    }
}

// TODO(xla): That we need this trait on our Signer was a stop-gap solution and should be properly
// addressed with TLS resolution, see https://github.com/radicle-dev/radicle-link/issues/195
impl keys::AsPKCS8 for Store {
    fn as_pkcs8(&self) -> Vec<u8> {
        self.storage
            .get_key()
            .map(|pair| pair.secret_key)
            .expect("unable to read key")
            .as_pkcs8()
    }
}

impl Reset for Store {
    fn reset(&mut self, paths: &paths::Paths, passphrase: SecUtf8) -> Result<(), Error> {
        let path = paths.keys_dir();
        let key_path = path.join(KEY);
        let mut storage = FileStorage::new(&key_path, crypto::Pwhash::new(passphrase));
        let key = keys::SecretKey::new();
        storage.put_key(key.clone())?;

        self.public_key = key.public_key();
        self.storage = storage;

        Ok(())
    }
}

#[allow(clippy::unreachable)]
#[async_trait::async_trait]
impl sign::Signer for Store {
    type Error = Error;

    fn public_key(&self) -> sign::PublicKey {
        self.public_key
    }

    async fn sign(&self, data: &[u8]) -> Result<sign::Signature, Self::Error> {
        let pair = self.storage.get_key()?;

        match sign::Signer::sign(&pair.secret_key, data).await {
            Ok(signature) => Ok(signature),
            Err(_infallible) => unreachable!(),
        }
    }
}
