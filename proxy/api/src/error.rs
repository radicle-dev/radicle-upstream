//! Proxy library errors usable for caller control flow and additional context for API responses.

use std::io;

/// All error variants the API will return.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Keystore error.
    #[error(transparent)]
    Keystore(#[from] coco::keystore::Error),

    /// Error interacting with [`coco::state::State`].
    #[error(transparent)]
    State(#[from] coco::state::Error),

    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Issues when access persistent storage.
    #[error(transparent)]
    Store(#[from] kv::Error),
}
