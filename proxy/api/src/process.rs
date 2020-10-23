//! Provides [`run`] to run the proxy process.
use std::{convert::TryFrom, time::Duration};

use tempfile::TempDir;
use thiserror::Error;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::{mpsc, watch},
};

use coco::{convert::MaybeFrom as _, keystore, seed, signer, Peer, RunConfig, SyncConfig};

use crate::{config, context, http, notification, session};

/// Flags accepted by the proxy binary.
#[derive(Clone, Copy)]
pub struct Args {
    /// Put proxy in test mode to use certain fixtures.
    pub test: bool,
}

/// Data required to run the peer and the API
struct Rigging {
    /// Optional temporary directory to use for storage
    temp: Option<TempDir>,
    /// The context provided to the API
    ctx: context::Context,
    /// The [`Peer`] to run
    peer: Peer,
    /// Channel to receive updates to the seed nodes from the API
    seeds_sender: Option<watch::Sender<Vec<seed::Seed>>>,
    /// Subscriptions for [`Peer`] events passed to the API
    subscriptions: notification::Subscriptions,
}

/// Run the proxy process
///
/// # Errors
///
/// Errors when the setup or any of the services fatally fails.
pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Required for `tokio::select`. We can’t put it on the element directly, though.
    #![allow(clippy::unreachable)]

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    coco::git_helper::setup(&proxy_path, &bin_dir)?;

    let mut sighup = signal(SignalKind::hangup())?;
    loop {
        let rigging = rig(args).await?;
        let (mut tx, rx) = mpsc::channel(1);
        let runner = run_rigging(rigging, (tx.clone(), rx));

        tokio::select! {
            r = runner => match r {
                // We've been shut down, ignore
                Err(RunError::Peer(coco::peer::Error::Spawn(_))) | Ok(()) => {
                    log::debug!("aborted")
                },
                // Actual error, abort the process
                Err(e) => return Err(e.into()),
            },

            Some(()) = sighup.recv() => {
                log::info!("SIGHUP received, reloading...");
                tx.send(()).await.ok();
            }
        }

        // Give sled some time to clean up if we're in persistent mode
        if !args.test {
            tokio::time::delay_for(Duration::from_millis(200)).await
        }
    }
}

/// Error running either the peer or the API.
#[derive(Debug, Error)]
enum RunError {
    /// The peer errored
    #[error(transparent)]
    Peer(#[from] coco::peer::Error),

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
    (killswitch, mut poisonpill): (mpsc::Sender<()>, mpsc::Receiver<()>),
) -> Result<(), RunError> {
    // Required for `tokio::select`. We can’t put it on the element directly, though.
    #![allow(clippy::unreachable)]
    let Rigging {
        temp: _dont_drop_me,
        ctx,
        peer,
        seeds_sender,
        subscriptions,
    } = rigging;

    if let Some(seeds_sender) = seeds_sender {
        let seeds_store = ctx.store.clone();
        tokio::spawn(async move {
            let mut last_seeds: Vec<seed::Seed> = vec![];
            let mut timer = tokio::time::interval(Duration::from_secs(1));

            loop {
                let _timestamp = timer.tick().await;

                let seeds = session::settings(&seeds_store)
                    .await
                    .expect("Failed to read session store")
                    .coco
                    .seeds;
                let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
                    log::error!("Error parsing seed list {:?}: {}", seeds, err);
                    vec![]
                });

                if seeds == last_seeds {
                    continue;
                }

                if seeds_sender.broadcast(seeds.clone()).is_err() {
                    break;
                }

                last_seeds = seeds;
            }
        });
    }

    let peer_subscriptions = subscriptions.clone();
    let peer_event_broadcast = {
        let mut peer_events = peer.subscribe();

        async move {
            loop {
                if let Some(notification) = notification::Notification::maybe_from(
                    peer_events
                        .recv()
                        .await
                        .expect("Failed to receive peer event"),
                ) {
                    peer_subscriptions.broadcast(notification).await
                }
            }
        }
    };

    let server = async move {
        log::info!("... API");
        let api = http::api(
            ctx,
            subscriptions.clone(),
            killswitch,
        );
        let (_, server) = warp::serve(api).try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], 8080),
            async move {
                poisonpill.recv().await;
                subscriptions.clear().await;
            },
        )?;

        server.await;
        Ok(())
    };
    let peer = async move {
        log::info!("... peer");
        peer.into_running().await
    };

    log::info!("Starting...");
    tokio::select! {
        server_status = server => server_status,
        peer_status = peer => Ok(peer_status?),
        peer_event_broadcast_status = peer_event_broadcast => peer_event_broadcast_status,
    }
}

/// Create [`Rigging`] to run the peer and API.
async fn rig(args: Args) -> Result<Rigging, Box<dyn std::error::Error>> {
    log::debug!("rigging up");

    let pw = keystore::SecUtf8::from("radicle-upstream");

    let (temp, paths, store, key) = if args.test {
        let temp_dir = tempfile::tempdir()?;
        log::debug!(
            "Temporary path being used for this run is: {:?}",
            temp_dir.path()
        );

        std::env::set_var("RAD_HOME", temp_dir.path());
        let paths =
            coco::Paths::try_from(coco::config::Paths::FromRoot(temp_dir.path().to_path_buf()))?;
        let store = {
            let path = temp_dir.path().join("store");
            kv::Store::new(kv::Config::new(path).flush_every_ms(100))
        }?;
        let key = keystore::Keystorage::memory(pw)?.get();

        Ok::<_, Box<dyn std::error::Error>>((Some(temp_dir), paths, store, key))
    } else {
        let paths = coco::Paths::try_from(coco::config::Paths::default())?;
        let store = {
            let path = config::dirs().data_dir().join("store");
            kv::Store::new(kv::Config::new(path).flush_every_ms(100))
        }?;
        let key = keystore::Keystorage::file(&paths, pw).init()?;

        Ok((None, paths, store, key))
    }?;

    let signer = signer::BoxedSigner::new(signer::SomeSigner { signer: key });

    let (peer, state, seeds_sender) = if args.test {
        let config = coco::config::configure(
            paths,
            key,
            *coco::config::INADDR_ANY,
            coco::config::static_seed_discovery(vec![]),
        );
        let (peer, state) = coco::into_peer_state(
            config,
            signer.clone(),
            store.clone(),
            RunConfig {
                sync: SyncConfig {
                    max_peers: 1,
                    on_startup: true,
                    period: Duration::from_secs(5),
                },
                ..RunConfig::default()
            },
        )
        .await?;

        (peer, state, None)
    } else {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let (seeds_sender, seeds_receiver) = watch::channel(seeds);

        let config = coco::config::configure(
            paths,
            key,
            *coco::config::INADDR_ANY,
            coco::config::StreamDiscovery::new(seeds_receiver),
        );

        let (peer, state) = coco::into_peer_state(
            config,
            signer.clone(),
            store.clone(),
            RunConfig {
                sync: SyncConfig {
                    max_peers: 1,
                    on_startup: true,
                    period: Duration::from_secs(5),
                },
                ..RunConfig::default()
            },
        )
        .await?;

        (peer, state, Some(seeds_sender))
    };

    let peer_control = peer.control();
    let subscriptions = notification::Subscriptions::default();
    let ctx = context::Context {
        peer_control,
        state,
        store,
        test: args.test,
    };

    Ok(Rigging {
        temp,
        ctx,
        peer,
        seeds_sender,
        subscriptions,
    })
}
