// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#![warn(
    clippy::all,
    clippy::cargo,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::multiple_crate_versions)]

use anyhow::Context;
use futures::prelude::*;

use librad::profile::Profile;
use link_identities::git::Urn;

mod cli;
mod peer;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    init_logging();

    let args: crate::cli::Args = structopt::StructOpt::from_args();

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

    let profile = Profile::from_root(&options.rad_home, None)
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
    task_runner.add_vital(
        track_projects(
            peer.clone(),
            task_runner.shutdown_triggered(),
            options.project,
        )
        .map(Ok),
    );

    task_runner.add_vital({
        let shutdown_signal = task_runner.shutdown_triggered();
        async move { peer.run(bootstrap_addrs, shutdown_signal).await }
    });

    match task_runner.run().await {
        Ok(_) => {}
        Err(errs) => {
            for err in errs {
                tracing::error!(?err, "task failed")
            }
        }
    }

    Ok(())
}

async fn track_projects(
    client: peer::Peer,
    shutdown: impl Future<Output = ()>,
    projects: Vec<Urn>,
) {
    let (delay_queue, projects_rx) = futures_delay_queue::delay_queue();
    for project in projects {
        delay_queue.insert(project, std::time::Duration::new(0, 0));
    }

    let projects_rx = projects_rx.into_stream().take_until(shutdown);
    futures::pin_mut!(projects_rx);

    let retry_delay = std::time::Duration::from_secs(1);

    while let Some(project) = projects_rx.next().await {
        tracing::info!(%project, "trying to track project");
        match client.track_project(project.clone()).await {
            Ok(found) => {
                if found {
                    tracing::info!(%project, "project tracked");
                } else {
                    tracing::info!(%project, "project not found");
                    delay_queue.insert(project, retry_delay);
                }
            }
            Err(err) => {
                tracing::warn!(?err, %project, "project tracking failed");
                delay_queue.insert(project, retry_delay);
            }
        }
    }

    tracing::debug!("track_projects done");
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
            "Shutting down. Send SIGINT or SIGTERM again within the next 10 seconds to force a shutdown."
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

fn load_or_create_secret_key(path: &std::path::Path) -> anyhow::Result<librad::SecretKey> {
    use librad::keystore::SecretKeyExt as _;
    use std::io::Write as _;
    use std::os::unix::prelude::PermissionsExt as _;

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

fn init_logging() {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let env_filter = if let Ok(value) = std::env::var("RUST_LOG") {
        tracing_subscriber::EnvFilter::new(value)
    } else {
        let directives = [
            "info",
            "quinn=warn",
            "upstream_seed=debug",
            "librad=debug",
            // Silence some noisy debug statements.
            "librad::git::refs=info",
            "librad::git::include=info",
            "librad::git::identities::person=info",
            "librad::git::identities::local=info",
            "librad::net::quic::connection::tracking",
            "librad::net::protocol::membership::periodic=info",
            "librad::net::protocol::accept=info",
            "librad::git::tracking=info",
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

    match std::env::var("TRACING_FMT").as_deref() {
        Ok("pretty") => builder.pretty().init(),
        Ok("compact") => builder.compact().init(),
        Ok("json") => builder.json().init(),
        _ => builder.pretty().init(),
    };
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
    /// When the futures resolves, the shutdown is triggered. The task runner only completes once
    /// `fut` resolves.
    ///
    /// The caller needs to ensure that `fut` eventually resolves when a shutdown is triggered.
    pub fn add_vital(&mut self, fut: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
        self.futures.push(self.shutdown.wrap_vital(fut).boxed());
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
        let mut task_runner = TaskRunner::new();

        task_runner.add_vital(
            task_runner
                .shutdown_triggered()
                .map(|_| Err(anyhow::anyhow!("foo"))),
        );
        task_runner.add_vital(future::err(anyhow::anyhow!("bar")));

        let errors = task_runner.run().await.unwrap_err();
        let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(error_messages, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[tokio::test]
    async fn task_runner_panic() {
        let mut task_runner = TaskRunner::new();
        task_runner.add_vital(
            task_runner
                .shutdown_triggered()
                .map(|_| Err(anyhow::anyhow!("foo"))),
        );
        task_runner.add_vital(async move { panic!("panic") });

        let errors = task_runner.run().await.unwrap_err();
        let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(error_messages, vec!["foo".to_string(), "panic".to_string()]);
    }
}
