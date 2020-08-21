//! Proxy library errors usable for caller control flow and additional context for API responses.

use std::time::SystemTimeError;

use librad::meta::common::url;

use crate::keystore;

/// All error variants the API will return.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Peer(#[from] coco::peer::Error),

    /// Failure when interacting with [`crate::keystore`].
    #[error(transparent)]
    Keystorage(#[from] keystore::Error),

    /// Common I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Url parse error.
    #[error(transparent)]
    Url(#[from] url::ParseError),

    /// Issues when access persistent storage.
    #[error(transparent)]
    Store(#[from] kv::Error),

    /// Errors from handling time.
    #[error(transparent)]
    Time(#[from] SystemTimeError),
}
