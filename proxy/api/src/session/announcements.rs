//! Caching of [`coco::announcement;:Announcement`]s.

use std::collections::HashSet;

use kv::Codec as _;

use coco::announcement;

use crate::error;

/// Name for the bucket used in [`kv::Store`].
const BUCKET_NAME: &str = "announcements";
/// Key for the single value used as cache.
const KEY_NAME: &str = "latest";

/// Load the cached announcements from the [`kv::Store`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the access of the key in the [`kv::Bucket`] fails
pub fn load(store: &kv::Store) -> Result<HashSet<announcement::Announcement>, error::Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<HashSet<announcement::Announcement>>>(Some(BUCKET_NAME))?;
    let value = bucket
        .get(KEY_NAME)?
        .map_or(HashSet::new(), kv::Json::to_inner);

    Ok(value)
}

/// Update the cache with the latest announcements.
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the storage of the new updates fails
#[allow(clippy::implicit_hasher)]
pub fn save(
    store: &kv::Store,
    updates: HashSet<announcement::Announcement>,
) -> Result<(), error::Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<HashSet<announcement::Announcement>>>(Some(BUCKET_NAME))?;
    bucket
        .set(KEY_NAME, kv::Json(updates))
        .map_err(error::Error::from)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use coco::{oid, uri};

    use crate::error;

    #[test]
    fn save_and_load() -> Result<(), error::Error> {
        let updates: HashSet<_> = vec![
            (
                uri::RadUrn {
                    id: coco::Hash::hash(b"project0"),
                    proto: uri::Protocol::Git,
                    path: "cloudhead/new-language"
                        .parse::<uri::Path>()
                        .map_err(coco::Error::from)?,
                },
                "7dec3269".parse::<oid::Oid>().map_err(coco::Error::from)?,
            ),
            (
                uri::RadUrn {
                    id: coco::Hash::hash(b"project0"),
                    proto: uri::Protocol::Git,
                    path: "fintohaps/notations"
                        .parse::<uri::Path>()
                        .map_err(coco::Error::from)?,
                },
                "b4d3276d".parse::<oid::Oid>().map_err(coco::Error::from)?,
            ),
            (
                uri::RadUrn {
                    id: coco::Hash::hash(b"project0"),
                    proto: uri::Protocol::Git,
                    path: "kalt/loops"
                        .parse::<uri::Path>()
                        .map_err(coco::Error::from)?,
                },
                "2206e5dc".parse::<oid::Oid>().map_err(coco::Error::from)?,
            ),
            (
                uri::RadUrn {
                    id: coco::Hash::hash(b"project1"),
                    proto: uri::Protocol::Git,
                    path: "backport".parse::<uri::Path>().map_err(coco::Error::from)?,
                },
                "869e5740".parse::<oid::Oid>().map_err(coco::Error::from)?,
            ),
        ]
        .iter()
        .cloned()
        .collect();
        let dir = tempfile::tempdir()?;
        let store = kv::Store::new(kv::Config::new(dir.path().join("store")))?;

        super::save(&store, updates.clone())?;

        assert_eq!(super::load(&store)?, updates);

        Ok(())
    }
}
