use std::{convert::TryFrom, time::Duration};

use tempfile::TempDir;
use thiserror::Error;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::mpsc,
};

use api::{config, context, env, http, notification, session};
use coco::{
    keystore,
    request::waiting_room::{self, WaitingRoom},
    seed,
    shared::Shared,
    signer, Peer, RunConfig, SyncConfig,
};

/// Flags accepted by the proxy binary.
#[derive(Clone, Copy)]
struct Args {
    /// Put proxy in test mode to use certain fixtures.
    test: bool,
}

struct Rigging {
    temp: Option<TempDir>,
    ctx: context::Context,
    peer: Peer,
    subscriptions: notification::Subscriptions,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        test: args.contains("--test"),
    };

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    coco::git_helper::setup(&proxy_path, &bin_dir)?;

    let mut sighup = signal(SignalKind::hangup())?;
    loop {
        let rigging = rig(args).await?;
        let (mut tx, rx) = mpsc::channel(1);
        let runner = run(rigging, (tx.clone(), rx), args.test);

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

#[derive(Debug, Error)]
enum RunError {
    #[error(transparent)]
    Peer(#[from] coco::peer::Error),

    #[error(transparent)]
    Warp(#[from] warp::Error),
}

async fn run(
    rigging: Rigging,
    (killswitch, mut poisonpill): (mpsc::Sender<()>, mpsc::Receiver<()>),
    enable_fixture_creation: bool,
) -> Result<(), RunError> {
    let Rigging {
        temp: _dont_drop_me,
        ctx,
        peer,
        subscriptions,
    } = rigging;

    let server = async move {
        log::info!("... API");
        let api = http::api(ctx, subscriptions, killswitch, enable_fixture_creation);
        let (_, server) = warp::serve(api).try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], 8080),
            async move {
                poisonpill.recv().await;
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
    }
}

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

    // TODO(finto): We should store and load the waiting room
    let waiting_room = {
        let mut config = waiting_room::Config::default();
        config.delta = Duration::from_secs(10);
        Shared::from(WaitingRoom::new(config))
    };

    let (peer, state) = {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config = coco::config::configure(paths, key, *coco::config::INADDR_ANY, seeds);

        coco::into_peer_state(
            config,
            signer.clone(),
            store.clone(),
            waiting_room.clone(),
            RunConfig {
                sync: SyncConfig {
                    max_peers: 1,
                    on_startup: true,
                    period: Duration::from_secs(5),
                },
                ..RunConfig::default()
            },
        )
        .await?
    };

    let subscriptions = notification::Subscriptions::default();
    let ctx = context::Context {
        state,
        signer: Some(signer),
        store,
        waiting_room,
    };

    Ok(Rigging {
        temp,
        ctx,
        peer,
        subscriptions,
    })
}
