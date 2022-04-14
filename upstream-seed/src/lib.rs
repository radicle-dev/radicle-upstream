// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::unwrap_used,
    unused_import_braces,
    unused_qualifications
)]

use std::{collections::HashSet, iter::FromIterator as _};

use anyhow::Context;
use futures::prelude::*;

use librad::profile::Profile;
use link_identities::git::Urn;

mod cli;
mod peer;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let args = cli::from_args();

    init_logging(args.log_json);

    if let Err(err) = run(args).await {
        tracing::error!(?err, "fatal error");
        std::process::exit(1);
    }

    std::process::exit(0);
}

pub async fn run(options: cli::Args) -> anyhow::Result<()> {
    let git_version = std::process::Command::new("git")
        .arg("version")
        .output()
        .context("failed to run git")?
        .stdout;
    tracing::info!(
        git_version = std::str::from_utf8(&git_version)
            .expect("invalid git output")
            .trim()
    );

    let profile = Profile::from_root(&options.lnk_home, None)
        .context("failed to initialize Radicle profile")?;
    let rad_paths = profile.paths().clone();

    let key_path = if let Some(key_path) = options.identity_key {
        key_path
    } else {
        rad_paths.keys_dir().join("identity.key")
    };

    let key = load_or_create_secret_key(&key_path)?;

    let peer_id = librad::PeerId::from(&key);
    let bootstrap_addrs = options.bootstrap.clone().unwrap_or_default();
    tracing::info!(?peer_id, ?bootstrap_addrs);

    let shutdown_signal = install_signal_handler().context("failed to install signal handler")?;
    let peer = peer::Peer::new(peer::Config {
        rad_paths,
        key,
        listen: options.listen,
    });

    let mut task_runner = TaskRunner::new();
    task_runner.add_vital(shutdown_signal.map(Ok));

    task_runner.add_cancel(log_events(peer.clone()).map(Ok));

    task_runner.add_cancel({
        peer.clone()
            .connected_peers()
            .for_each(|peers| {
                tracing::info!(?peers, "p2p connections changed");
                futures::future::ready(())
            })
            .map(Ok)
    });

    task_runner.add_cancel({
        peer.clone()
            .membership()
            .for_each(|membership_info| {
                tracing::info!(active = ?membership_info.active, passive = ?membership_info.passive, "gossip membership changed");
                futures::future::ready(())
            })
            .map(Ok)
    });

    task_runner.add_cancel(announce(peer.clone()).map(Ok));

    task_runner.add_vital(
        fetch_from_connected(
            task_runner.shutdown_triggered(),
            peer.clone(),
            options.project.clone(),
        )
        .map(Ok),
    );

    task_runner.add_vital({
        let shutdown_signal = task_runner.shutdown_triggered();
        async move { peer.run(bootstrap_addrs, shutdown_signal).await }
    });

    match task_runner.run().await {
        Ok(_) => {},
        Err(errs) => {
            for err in errs {
                tracing::error!(?err, "task failed")
            }
        },
    }

    Ok(())
}

/// Announce all projects ([`crate::peer::Peer::announce_all_projects`]) when the membership of the
/// gossip network changes.
async fn announce(peer: crate::peer::Peer) {
    let mut prev_active = HashSet::new();
    let mut membership = peer.membership().boxed();
    while let Some(membership) = membership.next().await {
        let active = HashSet::from_iter(membership.active);
        let mut added = active.difference(&prev_active);
        if added.next().is_some() {
            let result = peer.announce_all_projects().await;
            if let Err(err) = result {
                tracing::error!(?err, "failed to announce all projects");
            }
        }

        prev_active = active;
    }
}

/// Try to fetch all projects whenever a new peer connects.
async fn fetch_from_connected(
    shutdown_signal: impl Future<Output = ()> + Send,
    peer: crate::peer::Peer,
    projects: Vec<Urn>,
) {
    let mut new_connections = peer.new_connections().take_until(shutdown_signal).boxed();
    while let Some(new_connections) = new_connections.next().await {
        futures::stream::iter(&projects)
            .for_each(|project| {
                let new_connections = Clone::clone(&new_connections);
                let peer = &peer;
                async move {
                    for peer_id in new_connections {
                        let result = peer
                            .fetch_identity_from_peer(project.clone(), peer_id, None)
                            .await;
                        if let Err(err) = result {
                            tracing::error!(
                                %peer_id,
                                %project,
                                ?err,
                                "failed to track project from peer"
                            )
                        }
                    }
                }
            })
            .await
    }
}

/// Install signal handlers for SIGINT or SIGTERM and return when one of these signals is received.
///
/// Also starts a background task that exits the process if any of the signals is received after a
/// grace period of ten seconds after the first signal.
fn install_signal_handler() -> anyhow::Result<impl Future<Output = ()>> {
    use tokio::signal::unix::{signal, SignalKind};
    const GRACE_PERIOD: std::time::Duration = std::time::Duration::from_secs(10);

    let (shutdown_tx, shutdown_rx) = futures::channel::oneshot::channel();
    let mut sig_int =
        signal(SignalKind::interrupt()).context("failed to install signal handler")?;
    let mut sig_term =
        signal(SignalKind::terminate()).context("failed to install signal handler")?;

    tokio::spawn(async move {
        futures::future::select(sig_term.recv().boxed(), sig_int.recv().boxed()).await;
        let _ = shutdown_tx.send(());

        tracing::info!(
            "Shutting down. Send SIGINT or SIGTERM again after 10 seconds to force a shutdown."
        );
        let grace_period_end = std::time::Instant::now() + GRACE_PERIOD;
        loop {
            futures::future::select(sig_term.recv().boxed(), sig_int.recv().boxed()).await;
            if std::time::Instant::now() > grace_period_end {
                std::process::exit(5);
            }
        }
    });

    Ok(shutdown_rx.map(|_| ()))
}

async fn log_events(peer: crate::peer::Peer) {
    peer.events()
        .for_each(|event| {
            if let librad::net::peer::ProtocolEvent::Gossip(gossip) = event {
                match *gossip {
                    librad::net::peer::event::upstream::Gossip::Put {
                        provider,
                        payload,
                        result,
                    } => {
                        use librad::net::protocol::broadcast::PutResult;
                        let result = match result {
                            PutResult::Applied(_) => "Applied".to_string(),
                            result => format!("{:?}", result),
                        };
                        tracing::debug!(
                            provider_id = %provider.peer_id,
                            provider_seen_addrs = ?provider.seen_addrs.clone().into_inner(),
                            urn = %payload.urn,
                            rev = ?payload.rev,
                            origin = ?payload.origin,
                            result = %result,
                            "storage put"
                        )
                    },
                }
            };
            future::ready(())
        })
        .await
}

fn load_or_create_secret_key(path: &std::path::Path) -> anyhow::Result<librad::SecretKey> {
    use librad::keystore::SecretKeyExt as _;
    use std::{io::Write as _, os::unix::prelude::PermissionsExt as _};

    if let Some(keys_dir) = path.parent() {
        std::fs::create_dir_all(keys_dir)?;
    }

    if path.exists() {
        let contents = std::fs::read(path)?;
        let secret_key = (librad::SecretKey::from_bytes_and_meta(contents.into(), &()))?;
        Ok(secret_key)
    } else {
        let mut file = std::fs::File::create(path)?;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
        let secret_key = librad::SecretKey::new();
        file.write_all(secret_key.as_ref())?;
        Ok(secret_key)
    }
}

fn init_logging(log_json: bool) {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LIB_BACKTRACE", "0");
    }

    let env_filter = if let Ok(value) = std::env::var("RUST_LOG") {
        tracing_subscriber::EnvFilter::new(value)
    } else {
        let directives = [
            "info",
            "upstream_seed=debug",
            // Silence some noisy debug statements.
            "librad::net::protocol::io::streams=warn",
            "librad::net::protocol::io::recv::git=warn",
        ];

        let mut env_filter = tracing_subscriber::EnvFilter::default();

        for directive in directives {
            env_filter = env_filter.add_directive(directive.parse().expect("invalid log directive"))
        }
        env_filter
    };

    let builder = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(env_filter);

    if log_json {
        builder.json().init();
    } else {
        builder.pretty().init();
    }
}

/// Run [`Future`]s as tasks until a shutdown condition is triggered and collect their result.
struct TaskRunner {
    futures: Vec<future::BoxFuture<'static, anyhow::Result<()>>>,
    shutdown: async_shutdown::Shutdown,
}

impl TaskRunner {
    pub fn new() -> Self {
        Self {
            futures: vec![],
            shutdown: async_shutdown::Shutdown::new(),
        }
    }

    /// Returns when a shutdown is triggered.
    pub fn shutdown_triggered(&self) -> impl Future<Output = ()> + Send + Unpin + 'static {
        self.shutdown.wait_shutdown_triggered()
    }

    /// Add a vital future.
    ///
    /// When the future resolves, a shutdown is triggered. The task runner only completes once
    /// `fut` resolves.
    ///
    /// The caller needs to ensure that `fut` eventually resolves when a shutdown is triggered.
    pub fn add_vital(&mut self, fut: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
        self.futures.push(self.shutdown.wrap_vital(fut).boxed());
    }

    /// Add a future that is dropped when shutdown is triggered.
    ///
    /// If the future resolves with [`Ok`], no shutdown is triggered. Otherwise, a shutdown is
    /// triggered and the error is added to the error list returned by [`TaskRunner::run`].
    pub fn add_cancel(&mut self, fut: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
        let vital_token = self.shutdown.vital_token();
        self.futures.push(
            self.shutdown
                .wrap_cancel(fut)
                .map(move |maybe_result| {
                    let result = maybe_result.unwrap_or(Ok(()));
                    if result.is_ok() {
                        vital_token.forget();
                    }
                    result
                })
                .boxed(),
        );
    }

    /// Run all added futures as tasks and collect any errors.
    pub async fn run(self) -> Result<(), Vec<anyhow::Error>> {
        let tasks = self.futures.into_iter().map(tokio::spawn);
        let results = future::join_all(tasks).await;
        let errors = results
            .into_iter()
            .filter_map(|res| match res {
                Ok(Ok(_)) => None,
                Ok(Err(err)) => Some(err),
                Err(join_err) => Some(anyhow::Error::new(join_err)),
            })
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn task_runner() {
        let (dropped_tx, dropped_rx) = futures::channel::oneshot::channel::<()>();
        let mut task_runner = TaskRunner::new();

        task_runner.add_cancel(async move {
            future::pending::<()>().await;
            drop(dropped_tx);
            Ok(())
        });
        task_runner.add_vital(
            task_runner
                .shutdown_triggered()
                .map(|_| Err(anyhow::anyhow!("foo"))),
        );
        task_runner.add_vital(future::err(anyhow::anyhow!("bar")));

        let errors = task_runner.run().await.unwrap_err();
        let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(error_messages, vec!["foo".to_string(), "bar".to_string()]);

        assert!(dropped_rx.await.is_err());
    }

    #[tokio::test]
    async fn task_runner_panic() {
        let (dropped_tx, dropped_rx) = futures::channel::oneshot::channel::<()>();
        let mut task_runner = TaskRunner::new();

        task_runner.add_cancel(async move {
            future::pending::<()>().await;
            drop(dropped_tx);
            Ok(())
        });

        task_runner.add_vital(
            task_runner
                .shutdown_triggered()
                .map(|_| Err(anyhow::anyhow!("foo"))),
        );

        task_runner.add_vital(async move { panic!("panic") });

        let errors = task_runner.run().await.unwrap_err();
        let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(error_messages, vec!["foo".to_string(), "panic".to_string()]);

        assert!(dropped_rx.await.is_err());
    }

    #[tokio::test]
    async fn task_runner_cancel_ok_doesnt_trigger_shutdown() {
        let mut task_runner = TaskRunner::new();

        task_runner.add_cancel(future::ok(()));
        task_runner.add_cancel(future::pending());

        let result =
            tokio::time::timeout(std::time::Duration::from_millis(10), task_runner.run()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn task_runner_cancel_err_triggers_shutdown() {
        let mut task_runner = TaskRunner::new();

        task_runner.add_cancel(future::err(anyhow::anyhow!("foo")));
        task_runner.add_cancel(future::pending());

        let errors = task_runner.run().await.unwrap_err();
        let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(error_messages, vec!["foo".to_string()]);
    }
}
