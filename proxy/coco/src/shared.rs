use std::{ops::Deref, sync::Arc};

use tokio::sync::RwLock;

pub struct Shared<T> {
    pub value: Arc<RwLock<T>>,
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
