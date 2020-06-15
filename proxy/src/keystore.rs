//! Storage of secret keys.

use secstr::SecUtf8;

use librad::keys;
use librad::paths;
use radicle_keystore::{crypto::Pwhash, file, FileStorage, Keystore, SecretKeyExt};

type Store = FileStorage<
    Pwhash<SecUtf8>,
    keys::PublicKey,
    keys::SecretKey,
    <keys::SecretKey as SecretKeyExt>::Metadata,
>;

/// Storage of [`keys::SecretKey`], for putting and retrieval of the key.
pub struct CocoStore {
    store: Store,
}

impl CocoStore {
    /// Create a new `CocoStore` for storing your [`keys::SecretKey`].
    #[must_use = "must use CocoStore to put/get a key"]
    pub fn new(paths: &paths::Paths, pw: SecUtf8) -> Result<CocoStore, <Store as Keystore>::Error> {
        let path = paths.keys_dir();
        let file_path = path.join("librad.key");
        let crypto = Pwhash::new(pw.clone());
        Ok(CocoStore {
            store: FileStorage::new(&file_path, crypto),
        })
    }

    /// Attempt to get a [`keys::SecretKey`], otherwise we create one and store it.
    ///
    /// # Errors
    ///
    /// Fails with [`file::Error`].
    pub fn get_key_or_create(&mut self) -> Result<keys::SecretKey, <Store as Keystore>::Error> {
        match self.store.get_key() {
            Ok(keypair) => Ok(keypair.secret_key),
            Err(file::Error::NoSuchKey) => {
                let key = keys::SecretKey::new();
                self.store.put_key(key.clone())?;
                Ok(key)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CocoStore;
    use librad::paths;
    use secstr::SecUtf8;

    #[test]
    fn can_create_key() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(temp_dir.path())?;
        let pw = SecUtf8::from("asdf");
        let mut store = CocoStore::new(&paths, pw)?;

        match store.get_key_or_create() {
            Ok(_) => {}
            Err(err) => assert!(false, "could not get or create a key: {:?}", err),
        }

        Ok(())
    }
}
