//! Abort semantics for spawned tasks.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::future::{self, FutureExt as _};
use tokio::task::{JoinError, JoinHandle};

/// [`SpawnAbortable`] errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The spawned task either panicked, or was cancelled by the runtime.
    #[error(transparent)]
    Join(#[from] JoinError),

    /// The spawned task was aborted by calling [`SpawnAbortable::abort`].
    #[error(transparent)]
    Abort(#[from] future::Aborted),
}

/// A spawned task which can also be aborted by the user.
///
/// Stop-gap until we can abort [`JoinHandle`]s directly:
/// tokio-rs@cbb14a7bb9a13363e1abee8caff2bad1f996c263
#[allow(clippy::missing_docs_in_private_items)]
pub struct SpawnAbortable<T> {
    join_handle: JoinHandle<Result<T, future::Aborted>>,
    abort_handle: future::AbortHandle,
}

impl<T> SpawnAbortable<T> {
    /// Create a new [`SpawnAbortable`] from a [`Future`].
    ///
    /// The supplied [`Future`] will be spawned onto the async executor **immediately**!
    pub fn new<Fut>(fut: Fut) -> Self
    where
        Fut: Future<Output = T> + Send + 'static,
        Fut::Output: Send + 'static,
    {
        let (abort_handle, abort_reg) = future::AbortHandle::new_pair();
        let join_handle = tokio::spawn(future::Abortable::new(fut, abort_reg));

        Self {
            join_handle,
            abort_handle,
        }
    }

    /// Abort this future.
    ///
    /// Subsequent polls will return `SpawnAbortableError::Abort`.
    pub fn abort(&mut self) {
        self.abort_handle.abort()
    }
}

impl<T> Drop for SpawnAbortable<T> {
    fn drop(&mut self) {
        self.abort()
    }
}

impl<T> Future for SpawnAbortable<T> {
    type Output = Result<T, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.join_handle.poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(val) => {
                let val = match val {
                    Err(join) => Err(join.into()),
                    Ok(Err(abort)) => Err(abort.into()),
                    Ok(Ok(t)) => Ok(t),
                };
                Poll::Ready(val)
            },
        }
    }
}
