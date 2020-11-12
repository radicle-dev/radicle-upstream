//! Persist the `WaitingRoom` to a k/v store.

use std::time::{Duration, Instant};

use kv::Codec as _;

use crate::request::waiting_room::WaitingRoom;

pub use config::Config;

/// Name for the bucket used in [`kv::Store`].
const BUCKET_NAME: &str = "waiting_room";

/// Key for the single value used as cache.
const KEY_NAME: &str = "latest";

/// Key for the waiting room configuration
const KEY_CONFIG: &str = "config";

/// Default period at which we query the waiting room.
const DEFAULT_WAITING_ROOM_INTERVAL: Duration = Duration::from_millis(500);

/// Default period to consider until a query has timed out.
const DEFAULT_WAITING_ROOM_TIMEOUT: Duration = Duration::from_secs(10);

/// Announcement errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failures from [`kv`].
    #[error(transparent)]
    Kv(#[from] kv::Error),
}

/// Configuration for the `WaitingRoom` setup.
pub mod config {
    use std::time::Duration;

    use kv::Codec as _;
    use serde::{Deserialize, Serialize};

    use crate::request::waiting_room;

    use super::{
        Error, BUCKET_NAME, DEFAULT_WAITING_ROOM_INTERVAL, DEFAULT_WAITING_ROOM_TIMEOUT, KEY_CONFIG,
    };

    /// Set of knobs to alter the [`waiting_room::WaitingRoom`] behvaviour.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Config {
        /// Interval at which to query the [`waiting_room::WaitingRoom`] for ready requests.
        pub interval: Duration,
        /// Period to consider until a query has timed out.
        pub timeout_period: Duration,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                timeout_period: DEFAULT_WAITING_ROOM_TIMEOUT,
                interval: DEFAULT_WAITING_ROOM_INTERVAL,
            }
        }
    }

    impl From<Config> for waiting_room::Config<Duration> {
        fn from(config: Config) -> Self {
            Self {
                delta: config.timeout_period,
                ..Self::default()
            }
        }
    }

    /// Load the cached [`Config`] from the [`kv::Store`].
    ///
    /// # Errors
    ///
    /// * if the [`kv::Bucket`] can't be accessed
    /// * if the access of the key in the [`kv::Bucket`] fails
    pub fn load(store: &kv::Store) -> Result<Config, Error> {
        let bucket = store.bucket::<&'static str, kv::Json<Config>>(Some(BUCKET_NAME))?;
        let value = bucket
            .get(KEY_CONFIG)?
            .map_or(Config::default(), kv::Json::to_inner);

        Ok(value)
    }

    /// Update the cache with the latest [`Config`].
    ///
    /// # Errors
    ///
    /// * if the [`kv::Bucket`] can't be accessed
    /// * if the storage of the new updates fails
    #[allow(clippy::implicit_hasher)]
    pub fn save(store: &kv::Store, config: Config) -> Result<(), Error> {
        let bucket = store.bucket::<&'static str, kv::Json<Config>>(Some(BUCKET_NAME))?;
        bucket
            .set(KEY_CONFIG, kv::Json(config))
            .map_err(Error::from)
    }
}

/// Load the cached [`WaitingRoom`] from the [`kv::Store`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the access of the key in the [`kv::Bucket`] fails
pub fn load(store: &kv::Store) -> Result<Option<WaitingRoom<Instant, Duration>>, Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<WaitingRoom<Instant, Duration>>>(Some(BUCKET_NAME))?;
    Ok(bucket.get(KEY_NAME)?.map(kv::Json::to_inner))
}

/// Update the cache with the latest [`WaitingRoom`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the storage of the new updates fails
#[allow(clippy::implicit_hasher)]
pub fn save(store: &kv::Store, waiting_room: WaitingRoom<Instant, Duration>) -> Result<(), Error> {
    let bucket = store
        .bucket::<&'static str, kv::Json<WaitingRoom<Instant, Duration>>>(Some(BUCKET_NAME))?;
    bucket
        .set(KEY_NAME, kv::Json(waiting_room))
        .map_err(Error::from)
}
