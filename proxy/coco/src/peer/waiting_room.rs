//! Persist the `WaitingRoom` to a k/v store.

use std::time::{Duration, SystemTime};

use kv::Codec as _;

use crate::request::waiting_room::WaitingRoom;

/// Name for the bucket used in [`kv::Store`].
const BUCKET_NAME: &str = "waiting_room";

/// Key for the single value used as cache.
const KEY_NAME: &str = "latest";

/// Announcement errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failures from [`kv`].
    #[error(transparent)]
    Kv(#[from] kv::Error),
}

/// Load the cached [`WaitingRoom`] from the [`kv::Store`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the access of the key in the [`kv::Bucket`] fails
pub fn load(store: &kv::Store) -> Result<Option<WaitingRoom<SystemTime, Duration>>, Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<WaitingRoom<SystemTime, Duration>>>(Some(BUCKET_NAME))?;
    Ok(bucket.get(KEY_NAME)?.map(kv::Json::to_inner))
}

/// Update the cache with the latest [`WaitingRoom`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the storage of the new updates fails
#[allow(clippy::implicit_hasher)]
pub fn save(
    store: &kv::Store,
    waiting_room: WaitingRoom<SystemTime, Duration>,
) -> Result<(), Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<WaitingRoom<SystemTime, Duration>>>(Some(BUCKET_NAME))?;
    bucket
        .set(KEY_NAME, kv::Json(waiting_room))
        .map_err(Error::from)
}
