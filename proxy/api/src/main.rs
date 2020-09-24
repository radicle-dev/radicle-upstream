#![feature(bool_to_option)]

use std::{convert::TryFrom, time::Duration};

use futures::future;
use tempfile::TempDir;
use tokio::signal::unix::{signal, SignalKind};

use api::{config, context, env, http, notification, session};
use coco::{keystore, seed, signer, Peer};

/// Flags accepted by the proxy binary.
#[derive(Clone, Copy)]
struct Args {
    /// Put proxy in test mode to use certain fixtures to serve.
    test: bool,
}

struct Rigging {
    _tmp: Option<TempDir>,
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
        let (handle, reg) = future::AbortHandle::new_pair();
        let rigging = rig(args).await?;
        let runner = future::Abortable::new(run(rigging, handle.clone(), args.test), reg);

        tokio::select! {
            r = runner => match r {
                Ok(Err(coco::peer::Error::Aborted(_))) | Err(future::Aborted) | Ok(Ok(())) => {
                    unreachable!()
                },

                Ok(Err(e)) => return Err(e.into()),
            },

            Some(()) = sighup.recv() => {
                log::info!("SIGHUP received, reloading...");
                handle.abort();
            }
        }

        // Give sled some time to clean up if we're in persistent mode
        if !args.test {
            tokio::time::delay_for(Duration::from_millis(200)).await
        }
    }
}

async fn run(
    rigging: Rigging,
    selfdestruct: future::AbortHandle,
    enable_fixture_creation: bool,
) -> Result<(), coco::peer::Error> {
    let Rigging {
        _tmp,
        ctx,
        peer,
        subscriptions,
    } = rigging;

    log::info!("Starting...");
    let api = http::api(ctx, subscriptions, selfdestruct, enable_fixture_creation);
    tokio::try_join!(
        async move {
            log::info!("... API");
            warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
            Ok(())
        },
        async move {
            log::info!("... peer");
            peer.into_running().await
        }
    )
    .map(|((), ())| ())
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

    let (peer, state) = {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config = coco::config::configure(paths, key, *coco::config::LOCALHOST_ANY, seeds);

        coco::into_peer_state(config, signer.clone(), store.clone()).await?
    };

    if args.test {
        // TODO(xla): Given that we have proper ownership and user handling in coco, we should
        // evaluate how meaningful these fixtures are.
        let owner = state.init_owner(&signer, "cloudhead").await?;
        coco::control::setup_fixtures(&state, &signer, &owner).await?;
    }

    let subscriptions = notification::Subscriptions::default();
    let ctx = context::Context {
        state,
        signer,
        store,
    };

    Ok(Rigging {
        _tmp: temp,
        ctx,
        peer,
        subscriptions,
    })
}
