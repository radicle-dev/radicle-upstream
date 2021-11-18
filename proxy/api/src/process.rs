// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Provides [`run`] to run the proxy process.
// Otherwise clippy complains about FromArgs
#![allow(clippy::default_trait_access)]

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use futures::prelude::*;
use tokio::sync::{broadcast, watch, RwLock};

use crate::{cli::Args, config, context, git_helper, http, notification, service, session};

/// Run the proxy process
///
/// # Errors
///
/// Errors when the setup or any of the services fatally fails.
pub async fn run(args: Args) -> Result<(), anyhow::Error> {
    setup_logging(&args);

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    if !args.skip_remote_helper_install {
        git_helper::setup(&proxy_path, &bin_dir)?;
    }

    let mut service_manager = service::Manager::new(service::EnvironmentConfig {
        test_mode: args.test,
        insecure_http_api: args.insecure_http_api,
        unsafe_fast_keystore: args.unsafe_fast_keystore,
    })?;

    if let Some(passphrase) = &args.key_passphrase {
        service_manager.unseal_keystore(radicle_keystore::pinentry::SecUtf8::from(
            passphrase.clone(),
        ))?;
    }

    #[cfg(unix)]
    {
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
    }

    let auth_token = Arc::new(RwLock::new(None));
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

        run_session(
            service_handle,
            environment,
            auth_token.clone(),
            notified_restart,
            args.clone(),
        )
        .await?;
        tracing::info!("reloading");
    }

    Ok(())
}

async fn run_session(
    service_handle: service::Handle,
    environment: &service::Environment,
    auth_token: Arc<RwLock<Option<String>>>,
    restart_signal: impl Future<Output = ()> + Send + Sync + 'static,
    args: Args,
) -> Result<(), anyhow::Error> {
    // Required for `tokio::select`. We can’t put it on the element directly, though.
    #![allow(clippy::mut_mut)]

    let store_path = if let Some(temp_dir) = &environment.temp_dir {
        temp_dir.path().join("store")
    } else {
        config::store_dir(environment.coco_profile.id())
    };

    let store = kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?;
    let paths = environment.coco_profile.paths();

    let sealed = context::Sealed {
        store: store.clone(),
        insecure_http_api: environment.insecure_http_api,
        test: environment.test_mode,
        default_seeds: args.default_seeds,
        seeds: args.seeds,
        service_handle,
        auth_token,
        keystore: environment.keystore.clone(),
        paths: paths.clone(),
        shutdown: Arc::new(tokio::sync::Notify::new()),
    };

    let (peer_events_sender, _) = tokio::sync::broadcast::channel(32);

    let mut shutdown_runner = crate::shutdown_runner::ShutdownRunner::new();

    // Trigger a shutdown when the restart signal resolves
    shutdown_runner.add_without_shutdown(restart_signal.map(Ok));

    let ctx = if let Some(key) = environment.key.clone() {
        let signer = link_crypto::BoxedSigner::new(link_crypto::SomeSigner { signer: key });

        let config = radicle_daemon::config::configure(paths.clone(), signer, args.peer_listen);

        let disco = if let Some(ref seeds) = sealed.seeds {
            let seeds = radicle_daemon::seed::resolve(seeds)
                .await
                .context("failed to parse and resolve seeds")?;
            let (_, seeds_receiver) = watch::channel(seeds);
            radicle_daemon::config::StreamDiscovery::new(seeds_receiver)
        } else {
            let (watch_seeds_run, disco) = watch_seeds_discovery(store.clone()).await;
            shutdown_runner.add_without_shutdown(watch_seeds_run.map(Ok).boxed());
            disco
        };

        let peer = radicle_daemon::Peer::new(
            config,
            disco,
            store.clone(),
            radicle_daemon::RunConfig::default(),
        )?;

        let peer_control = peer.control();
        let ctx = context::Context::Unsealed(context::Unsealed {
            peer_control,
            peer: peer.peer.clone(),
            rest: sealed,
        });

        shutdown_runner.add_without_shutdown(
            send_peer_events(peer.subscribe(), peer_events_sender.clone())
                .map(Ok)
                .boxed(),
        );

        shutdown_runner.add_with_shutdown(move |shutdown_signal| {
            let (peer_shutdown, peer_run) = peer.start();
            let peer_run = peer_run.fuse();
            let mut shutdown_signal = shutdown_signal.fuse();
            async move {
                futures::pin_mut!(peer_run);
                futures::select! {
                    _ = shutdown_signal => {
                        drop(peer_shutdown);
                        peer_run.await
                    }
                    result = peer_run => {
                        result
                    }
                }
            }
            .map_err(|e| anyhow::Error::new(e).context("peer failed"))
            .boxed()
        });

        ctx
    } else {
        context::Context::Sealed(sealed)
    };

    shutdown_runner.add_with_shutdown({
        let ctx = ctx.clone();
        let peer_events_sender = peer_events_sender;
        let http_listen_addr = args.http_listen;
        move |shutdown_signal| {
            serve(ctx, peer_events_sender, http_listen_addr, shutdown_signal)
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

/// Get and resolve seed settings from the session store.
async fn session_seeds(
    store: &kv::Store,
) -> Result<Vec<radicle_daemon::seed::Seed>, anyhow::Error> {
    let seeds = session::seeds(store)?.unwrap_or_default();
    Ok(radicle_daemon::seed::resolve(&seeds)
        .await
        .unwrap_or_else(|err| {
            tracing::error!(?seeds, ?err, "Error parsing seed list");
            vec![]
        }))
}

async fn send_peer_events(
    mut peer_events: broadcast::Receiver<radicle_daemon::PeerEvent>,
    peer_events_sender: broadcast::Sender<notification::Notification>,
) {
    loop {
        match peer_events.recv().await {
            Ok(event) => {
                if let radicle_daemon::peer::Event::WaitingRoomTransition(ref transition) = event {
                    tracing::debug!(event = ?transition.event, "waiting room transition")
                }

                if let radicle_daemon::peer::Event::GossipFetched { gossip, result, .. } = &event {
                    tracing::debug!(?gossip, ?result, "gossip received")
                }

                if let Some(notification) = notification::from_peer_event(event) {
                    let _result = peer_events_sender.send(notification).err();
                }
            },
            Err(err) => {
                tracing::error!(?err, "Failed to receive peer event");
                return;
            },
        }
    }
}

/// Create a [`radicle_daemon::config::StreamDiscovery`] that emits new peer addresses whenever the
/// seed configuration changes in `store`.
///
/// The returned task is the future that needs to be run to watch the seeds.
async fn watch_seeds_discovery(
    store: kv::Store,
) -> (
    impl Future<Output = ()> + Send + 'static,
    radicle_daemon::config::StreamDiscovery,
) {
    let mut last_seeds = session_seeds(&store)
        .await
        .expect("Failed to read session store");

    let (seeds_sender, seeds_receiver) = watch::channel(last_seeds.clone());

    let run = async move {
        let mut timer = tokio::time::interval(Duration::from_millis(400));
        loop {
            let _timestamp = timer.tick().await;

            let seeds = session_seeds(&store)
                .await
                .expect("Failed to read session store");

            if seeds == last_seeds {
                continue;
            }

            if seeds_sender.send(seeds.clone()).is_err() {
                break;
            }

            last_seeds = seeds;
        }
    };

    (
        run,
        radicle_daemon::config::StreamDiscovery::new(seeds_receiver),
    )
}

fn serve(
    ctx: context::Context,
    peer_events_sender: broadcast::Sender<notification::Notification>,
    listen_addr: std::net::SocketAddr,
    restart_signal: impl Future<Output = ()> + Send + 'static,
) -> impl Future<Output = anyhow::Result<()>> {
    let ctx_shutdown = match &ctx {
        context::Context::Sealed(sealed) => sealed.shutdown.clone(),
        context::Context::Unsealed(unsealed) => unsealed.rest.shutdown.clone(),
    };
    let api = http::api(ctx, peer_events_sender);
    async move {
        let (_, server) = warp::serve(api).try_bind_with_graceful_shutdown(listen_addr, {
            async move {
                restart_signal.await;
                ctx_shutdown.notify_waiters()
            }
        })?;
        server.await;
        Ok(())
    }
}

fn setup_logging(args: &Args) {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let env_filter = if let Ok(value) = std::env::var("RUST_LOG") {
        tracing_subscriber::EnvFilter::new(value)
    } else {
        let mut env_filter = tracing_subscriber::EnvFilter::default();

        let mut directives = vec!["info", "quinn=warn"];

        if args.dev_log {
            directives.extend([
                "api=debug",
                "radicle_daemon=debug",
                "librad=debug",
                // Silence some noisy debug statements
                "librad::git::refs=info",
                "librad::git::include=info",
                "librad::git::identities::person=info",
                "librad::git::identities::local=info",
                "librad::net::protocol::membership::periodic=info",
                "librad::git::tracking=info",
            ])
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
