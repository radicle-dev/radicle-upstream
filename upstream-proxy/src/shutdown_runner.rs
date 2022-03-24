// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use futures::{
    future::{BoxFuture, Either},
    prelude::*,
};

/// Poll a collection of futures until one of them resolves—then shut down all other futures and
/// poll them to completion
///
/// ```no_compile
/// # tokio_test::block_on(async {
/// let shutdown_runner = ShutdownRunner::new();
/// shutdown_runner.add_without_shutdown({
///   futures::future::ok(());
/// });
/// shutdown_runner.add_task_shutdown(|shutdown| async move {
///   shutdown.await;
///   println!("shut down");
///   Ok(());
/// });
/// let results = shutdown_runner.run().await;
/// # })
/// ```
pub struct ShutdownRunner<Error> {
    futures: Vec<BoxFuture<'static, Result<(), Error>>>,
    semaphore: std::sync::Arc<tokio::sync::Semaphore>,
}

impl<Error: Send + 'static> ShutdownRunner<Error> {
    pub fn new() -> Self {
        Self {
            futures: vec![],
            semaphore: std::sync::Arc::new(tokio::sync::Semaphore::new(0)),
        }
    }

    /// Add a new future to be run.
    ///
    /// If another future shuts down the runner, it stops polling the given future and drops it.
    ///
    /// When the given future resolves, the runner will shut down.
    pub fn add_without_shutdown(
        &mut self,
        fut: impl Future<Output = Result<(), Error>> + Send + 'static,
    ) {
        self.add_with_shutdown(move |shutdown_signal| {
            futures::future::select(Box::pin(fut), shutdown_signal)
                .map(|output| match output {
                    Either::Left((result, _)) => result,
                    Either::Right(_) => Ok(()),
                })
                .boxed()
        });
    }

    /// Add a new future to be run and do cleanup when the runner is being shut down.
    ///
    ///
    /// The future passed to the closure resolves when the runner will shut down
    ///
    /// Callers need to ensure that the returned future eventually resolves after the `shutdown`
    /// parameter resolves.
    pub fn add_with_shutdown(
        &mut self,
        f: impl FnOnce(BoxFuture<'static, ()>) -> BoxFuture<'static, Result<(), Error>>,
    ) {
        self.futures.push({
            let semaphore = self.semaphore.clone();
            let shutdown_signal = self.shutdown_signal();
            let fut = f(shutdown_signal);
            async move {
                let result = fut.await;
                semaphore.add_permits(10_000);
                result
            }
            .boxed()
        })
    }

    /// Run all added futures and return the list of their results.
    pub async fn run(self) -> Vec<Result<(), Error>> {
        futures::future::join_all(self.futures).await
    }

    fn shutdown_signal(&self) -> BoxFuture<'static, ()> {
        let semaphore = self.semaphore.clone();
        async move {
            let _result = semaphore.acquire().await;
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn shutdown_cleanup() {
        let mut shutdown_runner = ShutdownRunner::<String>::new();
        shutdown_runner.add_without_shutdown(futures::future::ok(()));
        shutdown_runner.add_with_shutdown(|shutdown| {
            async move {
                shutdown.await;
                Err("shutdown".to_string())
            }
            .boxed()
        });
        let results = shutdown_runner.run().await;
        assert_eq!(results, vec![Ok(()), Err("shutdown".to_string())]);
    }

    #[tokio::test]
    async fn shutdown_abort() {
        let mut shutdown_runner = ShutdownRunner::<String>::new();
        shutdown_runner.add_without_shutdown(futures::future::pending());
        shutdown_runner.add_with_shutdown(|_| futures::future::err("shutdown".to_string()).boxed());
        let results = shutdown_runner.run().await;
        assert_eq!(results, vec![Ok(()), Err("shutdown".to_string())]);
    }

    #[tokio::test]
    async fn keep_running() {
        let mut shutdown_runner = ShutdownRunner::<String>::new();
        shutdown_runner.add_without_shutdown(futures::future::pending());
        shutdown_runner.add_with_shutdown(|shutdown| {
            async move {
                shutdown.await;
                Ok(())
            }
            .boxed()
        });
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(40), shutdown_runner.run()).await;
        assert!(result.is_err());
    }
}
