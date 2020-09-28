//! See [`Shared`].

use std::{ops::Deref, sync::Arc};

use tokio::sync::RwLock;

/// A value that's wrapped in an `Arc` and `RwLock`. It makes it easy to have an automatic
/// conversion of `T` into a `Shared<T>` via `From` and `Into`.
///
/// It also implements `Deref` so the wrapping should be transparent when making use of it.
pub struct Shared<T> {
    /// Shared value in the `Arc` and `RwLock`.
    pub value: Arc<RwLock<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T> From<T> for Shared<T> {
    fn from(other: T) -> Self {
        Self {
            value: Arc::from(RwLock::from(other)),
        }
    }
}

impl<T> From<Arc<RwLock<T>>> for Shared<T> {
    fn from(other: Arc<RwLock<T>>) -> Self {
        Self { value: other }
    }
}

impl<T> Deref for Shared<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
