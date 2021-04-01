//! Provides [`run`] to run the proxy process.
// Otherwise clippy complains about FromArgs
#![allow(clippy::default_trait_access)]

use std::{future::Future, net, sync::Arc, time::Duration};

use argh::FromArgs;
use futures::prelude::*;
use thiserror::Error;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::{watch, RwLock},
};

use radicle_daemon::{
    convert::MaybeFrom as _, peer::run_config, seed, signer, Peer, PeerStatus, RunConfig,
};

use crate::{config, context, git_helper, http, notification, service, session};

/// Flags accepted by the proxy binary.
#[derive(Clone, FromArgs)]
pub struct Args {
    /// put proxy in test mode to use certain fixtures
    #[argh(switch)]
    pub test: bool,
    /// run HTTP API on a specified address:port (default: 127.0.0.1:17246)
    #[argh(
        option,
        default = "std::net::SocketAddr::from(([127, 0, 0, 1], 17246))"
    )]
    pub http_listen: net::SocketAddr,
    /// run the peer on a specified address:port (default: 0.0.0.0:0)
    #[argh(option, default = "std::net::SocketAddr::from(([0, 0, 0, 0], 0))")]
    pub peer_listen: net::SocketAddr,
    /// add one or more default seed addresses to initialise the settings store (default: none)
    #[argh(option, long = "default-seed")]
    pub default_seeds: Vec<String>,
}

/// Data required to run the peer and the API
struct Rigging {
    /// The context provided to the API
    ctx: context::Context,
    /// The [`Peer`] to run
    peer: Option<Peer<radicle_daemon::config::StreamDiscovery>>,
    /// Channel to receive updates to the seed nodes from the API
    seeds_sender: Option<watch::Sender<Vec<seed::Seed>>>,
}

/// Run the proxy process
///
/// # Errors
///
/// Errors when the setup or any of the services fatally fails.
pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    git_helper::setup(&proxy_path, &bin_dir)?;

    let mut service_manager = service::Manager::new(args.test)?;
    let mut sighup = signal(SignalKind::hangup())?;

    let mut handle = service_manager.handle();
    tokio::spawn(async move {
        loop {
            if sighup.recv().await.is_some() {
                log::info!("SIGHUP received, reloading...");
                handle.reset();
            } else {
                break;
            }
        }
    });

    let auth_token = Arc::new(RwLock::new(None));
    loop {
        let notified_restart = service_manager.notified_restart();
        let service_handle = service_manager.handle();
        let environment = service_manager.environment()?;
        let rigging = rig(
            service_handle,
            environment,
            auth_token.clone(),
            args.clone(),
        )
        .await?;
        let result = run_rigging(rigging, notified_restart).await;
        match result {
            // We've been shut down, ignore
            Err(RunError::Peer(radicle_daemon::peer::Error::Join(_))) | Ok(()) => {
                log::debug!("aborted")
            },
            // Actual error, abort the process
            Err(e) => return Err(e.into()),
        };
    }
}

/// Error running either the peer, the event tasks or the API.
#[derive(Debug, Error)]
enum RunError {
    /// The peer errored
    #[error(transparent)]
    Peer(#[from] radicle_daemon::peer::Error),

    /// Warp errored
    #[error(transparent)]
    Warp(#[from] warp::Error),
}

/// Run the API and peer.
///
/// Returns when either the peer or the API stops.
///
/// # Errors
///
/// Errors when either the peer or the API error.
async fn run_rigging(
    rigging: Rigging,
    restart_signal: impl Future<Output = ()> + Send + 'static,
) -> Result<(), RunError> {
    // Required for `tokio::select`. We can’t put it on the element directly, though.
    #![allow(clippy::unreachable)]
    let Rigging {
        ctx,
        peer,
        seeds_sender,
    } = rigging;

    let subscriptions = notification::Subscriptions::default();
    let peer_subscriptions = subscriptions.clone();
    let server_ctx = ctx.clone();

    let server = async move {
        log::info!("starting API");
        let api = http::api(server_ctx.clone(), subscriptions.clone());
        let (_, server) = warp::serve(api).try_bind_with_graceful_shutdown(
            server_ctx.http_listen(),
            async move {
                restart_signal.await;
                subscriptions.clear().await;
            },
        )?;

        server.await;
        Ok(())
    };

    if let Some(peer) = peer {
        let mut tasks = vec![server.boxed()];

        if let Some(seeds_sender) = seeds_sender {
            let mut peer_control = peer.control();
            let seeds_store = ctx.store().clone();
            let seeds_event_task = async move {
                let mut last_seeds = session_seeds(&seeds_store, ctx.default_seeds())
                    .await
                    .expect("Failed to read session store");
                let mut timer = tokio::time::interval(Duration::from_secs(5));

                loop {
                    let _timestamp = timer.tick().await;

                    let seeds = session_seeds(&seeds_store, ctx.default_seeds())
                        .await
                        .expect("Failed to read session store");

                    let current_status = peer_control.current_status().await;

                    if seeds == last_seeds && current_status != PeerStatus::Offline {
                        continue;
                    }

                    if seeds_sender.send(seeds.clone()).is_err() {
                        break;
                    }

                    last_seeds = seeds;
                }
            };
            tasks.push(seeds_event_task.map(Ok).boxed());
        }
        let peer_event_task = {
            let mut peer_events = peer.subscribe();

            async move {
                loop {
                    match peer_events.recv().await {
                        Ok(event) => {
                            if let Some(notification) =
                                notification::Notification::maybe_from(event)
                            {
                                peer_subscriptions.broadcast(notification).await
                            }
                        },
                        Err(err) => {
                            log::error!("Failed to receive peer event: {}", err);
                            return;
                        },
                    }
                }
            }
        };
        tasks.push(peer_event_task.map(Ok).boxed());

        let peer = async move {
            log::info!("starting peer");
            peer.run().await
        };

        tasks.push(peer.map_err(RunError::from).boxed());

        let (result, _, _) = futures::future::select_all(tasks).await;
        result
    } else {
        server.await
    }
}

/// Create [`Rigging`] to run the peer and API.
async fn rig(
    service_handle: service::Handle,
    environment: &service::Environment,
    auth_token: Arc<RwLock<Option<String>>>,
    args: Args,
) -> Result<Rigging, Box<dyn std::error::Error>> {
    let store_path = if let Some(temp_dir) = &environment.temp_dir {
        temp_dir.path().join("store")
    } else {
        config::store_dir(environment.coco_profile.id())
    };

    let store = kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?;

    if let Some(key) = environment.key.clone() {
        let signer = signer::BoxedSigner::new(signer::SomeSigner { signer: key });

        let seeds = session_seeds(&store, &args.default_seeds).await?;
        let (seeds_sender, seeds_receiver) = watch::channel(seeds);

        let config = radicle_daemon::config::configure(
            environment.coco_profile.paths().clone(),
            signer.clone(),
            args.peer_listen,
        );
        let disco = radicle_daemon::config::StreamDiscovery::new(seeds_receiver);

        let peer = radicle_daemon::Peer::new(config, disco, store.clone(), coco_run_config());

        let peer_control = peer.control();
        let ctx = context::Context::Unsealed(context::Unsealed {
            peer_control,
            peer: peer.peer.clone(),
            store,
            test: environment.test_mode,
            http_listen: args.http_listen,
            default_seeds: args.default_seeds,
            service_handle: service_handle.clone(),
            auth_token,
            keystore: environment.keystore.clone(),
        });

        Ok(Rigging {
            ctx,
            peer: Some(peer),
            seeds_sender: Some(seeds_sender),
        })
    } else {
        let ctx = context::Context::Sealed(context::Sealed {
            store,
            test: environment.test_mode,
            http_listen: args.http_listen,
            default_seeds: args.default_seeds,
            service_handle,
            auth_token,
            keystore: environment.keystore.clone(),
        });
        Ok(Rigging {
            ctx,
            peer: None,
            seeds_sender: None,
        })
    }
}

/// Get and resolve seed settings from the session store.
async fn session_seeds(
    store: &kv::Store,
    default_seeds: &[String],
) -> Result<Vec<radicle_daemon::seed::Seed>, Box<dyn std::error::Error>> {
    let seeds = session::seeds(store, default_seeds).await?;
    Ok(seed::resolve(&seeds).await.unwrap_or_else(|err| {
        log::error!("Error parsing seed list {:?}: {}", seeds, err);
        vec![]
    }))
}

/// [`RunConfig`] for the coco peer.
fn coco_run_config() -> RunConfig {
    RunConfig {
        sync: run_config::Sync {
            interval: Duration::from_secs(5),
        },
        ..RunConfig::default()
    }
}
