// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Provides [`run`] to run the proxy process.
use std::sync::Arc;

use futures::prelude::*;

use crate::{cli::Args, config, context, service};

/// Run the proxy process
///
/// # Errors
///
/// Errors when the setup or any of the services fatally fails.
pub async fn run(args: Args) -> Result<(), anyhow::Error> {
    setup_logging(&args);

    if !args.skip_identity_check && !args.test {
        loop {
            match lnk_profile::get(None, None) {
                Ok(Some(_)) => break,
                Ok(None) | Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                },
            };
        }
    }

    let mut service_manager = service::Manager::new(service::EnvironmentConfig {
        test_mode: args.test,
        unsafe_fast_keystore: args.unsafe_fast_keystore,
        identity_key: args.identity_key.clone(),
    })?;

    if let Some(passphrase) = &args.key_passphrase {
        service_manager.unseal_keystore(radicle_keystore::pinentry::SecUtf8::from(
            passphrase.clone(),
        ))?;
    }

    #[cfg(unix)]
    install_signal_handlers(&service_manager)?;

    loop {
        let notified_restart = service_manager.notified_restart();
        let service_handle = service_manager.handle();
        let maybe_environment = service_manager.environment()?;
        let environment = if let Some(environment) = maybe_environment {
            environment
        } else {
            tracing::info!("process has shut down");
            break;
        };

        run_session(service_handle, environment, notified_restart, args.clone()).await?;
        tracing::info!("reloading");
    }

    Ok(())
}

async fn ssh_agent_signer(
    paths: &librad::paths::Paths,
) -> Result<Option<link_crypto::BoxedSigner>, anyhow::Error> {
    let storage = match librad::git::storage::ReadOnly::open(paths) {
        Ok(storage) => storage,
        // Don't throw if the monorepo hasn't been initialised yet, like it is the case before the
        // user has onboarded.
        Err(_) => return Ok(None),
    };
    let peer_id = storage.peer_id();
    let pk = (*peer_id.as_public_key()).into();
    let agent = radicle_keystore::sign::SshAgent::new(pk);
    let keys = radicle_keystore::sign::ssh::list_keys::<tokio::net::UnixStream>(&agent).await?;
    if keys.contains(&pk) {
        let signer = agent.connect::<tokio::net::UnixStream>().await?;
        Ok(Some(
            link_crypto::SomeSigner {
                signer: Arc::new(signer),
            }
            .into(),
        ))
    } else {
        Ok(None)
    }
}

async fn add_key_to_ssh_agent(paths: &librad::paths::Paths, key: link_crypto::SecretKey) {
    let storage = match librad::git::storage::ReadOnly::open(paths) {
        Ok(storage) => storage,
        // Don't throw if the monorepo hasn't been initialised yet, like it is the case before the
        // user has onboarded.
        Err(_) => return,
    };
    let peer_id = storage.peer_id();
    let pk = (*peer_id.as_public_key()).into();
    let agent = radicle_keystore::sign::SshAgent::new(pk);

    if (radicle_keystore::sign::ssh::add_key::<tokio::net::UnixStream>(
        &agent,
        key.into(),
        &Vec::new(),
    )
    .await)
        .is_err()
    {
        tracing::warn!("could not add ssh key, is ssh-agent running?");
    }
}

async fn run_session(
    service_handle: service::Handle,
    environment: &service::Environment,
    restart_signal: impl Future<Output = ()> + Send + Sync + 'static,
    args: Args,
) -> Result<(), anyhow::Error> {
    let store_path = if let Some(temp_dir) = &environment.temp_dir {
        temp_dir.path().join("store")
    } else {
        config::store_dir(
            environment.coco_profile.id(),
            std::env::var_os("LNK_HOME")
                .as_ref()
                .map(std::path::Path::new),
        )
    };

    let store = kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?;
    let paths = environment.coco_profile.paths();

    let sealed = context::Sealed {
        store: store.clone(),
        test: environment.test_mode,
        service_handle,
        keystore: environment.keystore.clone(),
        paths: paths.clone(),
        shutdown: Arc::new(tokio::sync::Notify::new()),
    };

    let mut shutdown_runner = crate::shutdown_runner::ShutdownRunner::new();

    // Trigger a shutdown when the restart signal resolves
    shutdown_runner.add_without_shutdown(restart_signal.map(Ok));

    let maybe_signer = if let Some(key) = environment.key.clone() {
        let signer = Some(link_crypto::BoxedSigner::new(link_crypto::SomeSigner {
            signer: key.clone(),
        }));

        add_key_to_ssh_agent(paths, key).await;

        signer
    } else {
        ssh_agent_signer(paths).await.unwrap_or_else(|_| {
            tracing::warn!("could not lookup ssh key, is ssh-agent running?");
            None
        })
    };

    let ctx = if let Some(signer) = maybe_signer {
        let (peer, peer_runner) = crate::peer::create(crate::peer::Config {
            paths: paths.clone(),
            signer,
            store: store.clone(),
            discovery: crate::daemon::config::NoDiscovery::new(),
            listen: args.peer_listen,
        })?;

        let (git_fetch, git_fetch_runner) = crate::git_fetch::create(
            peer.clone(),
            args.git_seeds.unwrap_or_default(),
            std::time::Duration::from_secs(args.git_fetch_interval),
            &store,
        )
        .await?;

        let (watch_monorepo, watch_monorepo_runner) = crate::watch_monorepo::create(peer.clone());

        tokio::task::spawn(log_daemon_peer_events(peer.events()));
        tokio::task::spawn(handle_monorepo_events(
            watch_monorepo.updates(),
            git_fetch.clone(),
        ));

        shutdown_runner
            .add_with_shutdown(|shutdown| git_fetch_runner.run(shutdown).map(Ok).boxed());

        shutdown_runner.add_without_shutdown(watch_monorepo_runner.run().map(Ok).boxed());

        shutdown_runner.add_with_shutdown(|shutdown| {
            peer_runner
                .run(shutdown)
                .map_err(|err| anyhow::Error::new(err).context("failed to run peer"))
                .boxed()
        });

        context::Context::Unsealed(context::Unsealed {
            peer,
            rest: sealed,
            git_fetch,
            watch_monorepo,
        })
    } else {
        context::Context::Sealed(sealed)
    };

    shutdown_runner.add_with_shutdown({
        let ctx = ctx.clone();
        let http_listen_addr = args.http_listen;
        move |shutdown_signal| {
            crate::http_next::serve(ctx, http_listen_addr, shutdown_signal)
                .map_err(|e| e.context("server failed"))
                .boxed()
        }
    });

    let results = shutdown_runner.run().await;

    for result in results {
        result?
    }

    Ok(())
}

async fn log_daemon_peer_events(events: impl Stream<Item = crate::daemon::peer::Event>) {
    events
        .for_each(|event| {
            match event {
                crate::daemon::peer::Event::WaitingRoomTransition(ref transition) => {
                    tracing::debug!(event = ?transition.event, "waiting room transition")
                },

                crate::daemon::peer::Event::GossipFetched {
                    gossip,
                    result,
                    provider,
                } => {
                    use librad::net::protocol::broadcast::PutResult;
                    let result = match result {
                        PutResult::Applied(_) => "Applied".to_string(),
                        result => format!("{:?}", result),
                    };
                    tracing::debug!(
                        provider_id = %provider.peer_id,
                        provider_seen_addrs = ?provider.seen_addrs.clone().into_inner(),
                        urn = %gossip.urn,
                        rev = ?gossip.rev,
                        origin = ?gossip.origin,
                        result = %result,
                        "storage put"
                    )
                },
                _ => {},
            };
            future::ready(())
        })
        .await;
}

// Trigger a `git_fetch` whenever a project is cloned via `rad clone` to set the project's seed URL
// in the KV store.
async fn handle_monorepo_events(
    events: impl Stream<Item = link_identities::Urn<link_identities::Revision>>,
    git_fetch_handle: crate::git_fetch::Handle,
) {
    events
        .for_each(|event| async {
            if let Some(path) = event.path {
                if path == librad::reflike!("refs/rad/id") {
                    git_fetch_handle.add(event.id).await;
                }
            }
        })
        .await;
}

/// Install signal handlers.
///
/// On `SIGHUP` the service is restarted. On `SIGTERM` or `SIGINT` the service is asked to
/// shutdown. If `SIGTERM` or `SIGINT` is received a second time after more than five seconds, the
/// process is exited immediately.
fn install_signal_handlers(service_manager: &service::Manager) -> Result<(), anyhow::Error> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sighup = signal(SignalKind::hangup())?;

    let mut handle = service_manager.handle();
    tokio::spawn(async move {
        loop {
            if sighup.recv().await.is_some() {
                tracing::info!("SIGHUP received, reloading...");
                handle.reset();
            } else {
                break;
            }
        }
    });

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    let mut handle = service_manager.handle();
    tokio::spawn(async move {
        futures::future::select(Box::pin(sigterm.recv()), Box::pin(sigint.recv())).await;
        tracing::info!(
            "shutting down. send SIGINT or SIGTERM again in 5 seconds to force shutdown"
        );
        handle.shutdown();
        let grace_period_start = std::time::Instant::now();
        let grace_period = std::time::Duration::from_secs(5);
        loop {
            futures::future::select(Box::pin(sigterm.recv()), Box::pin(sigint.recv())).await;
            let now = std::time::Instant::now();
            if now - grace_period_start > grace_period {
                std::process::exit(5);
            }
        }
    });

    Ok(())
}

fn setup_logging(args: &Args) {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let env_filter = if let Ok(value) = std::env::var("RUST_LOG") {
        tracing_subscriber::EnvFilter::new(value)
    } else {
        let mut env_filter = tracing_subscriber::EnvFilter::default();

        let mut directives = vec![
            "info",
            "quinn=warn",
            // Silence some noisy debug statements.
            "librad::net::protocol::io::streams=warn",
            "librad::net::protocol::io::recv::git=warn",
        ];

        if args.dev_log {
            directives.extend(["upstream_proxy=debug", "crate::daemon=debug"])
        }

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
        _ => {
            if args.dev_log {
                builder.pretty().init()
            } else {
                builder.init()
            }
        },
    };
}
