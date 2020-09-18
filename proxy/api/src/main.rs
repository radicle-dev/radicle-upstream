use std::{convert::TryFrom, fs::remove_dir_all, io};

use coco::{control, keystore, seed, signer};

use api::{config, context, env, http, notification, session};

/// Flags accepted by the proxy binary.
struct Args {
    /// Wipe all local state, use with caution.
    factory_reset: bool,
    /// Put proxy in test mode to use certain fixtures.
    test: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        factory_reset: args.contains("--factory-reset"),
        test: args.contains("--test"),
    };

    if args.factory_reset {
        return Ok(factory_reset()?);
    }

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    coco::git_helper::setup(&proxy_path, &bin_dir)?;

    let temp_dir = tempfile::tempdir()?;
    log::debug!(
        "Temporary path being used for this run is: {:?}",
        temp_dir.path()
    );

    let paths_config = if args.test {
        std::env::set_var("RAD_HOME", temp_dir.path());
        coco::config::Paths::FromRoot(temp_dir.path().to_path_buf())
    } else {
        coco::config::Paths::default()
    };
    let paths = coco::Paths::try_from(paths_config)?;

    let store = {
        let store_path = if args.test {
            temp_dir.path().join("store")
        } else {
            let dirs = config::dirs();
            dirs.data_dir().join("store")
        };

        kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?
    };

    let pw = keystore::SecUtf8::from("radicle-upstream");
    let mut keystore = keystore::Keystorage::new(&paths, pw);
    let key = keystore.init()?;
    let signer = signer::BoxedSigner::new(signer::SomeSigner {
        signer: key.clone(),
    });

    let (peer, state) = {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config =
            coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, seeds);

        coco::into_peer_state(config, signer.clone(), store.clone()).await?
    };

    if args.test {
        let state = state.lock().await;
        // TODO(xla): Given that we have proper ownership and user handling in coco, we should
        // evaluate how meaningful these fixtures are.
        let owner = state.init_owner(&signer, "cloudhead")?;
        coco::control::setup_fixtures(&state, &signer, &owner)?;
    }

    let subscriptions = notification::Subscriptions::default();
    let ctx = context::Ctx::from(context::Context {
        state,
        signer,
        store,
    });

    log::info!("starting coco peer");
    tokio::spawn(async move {
        peer.run().await.expect("peer run loop crashed");
    });

    log::info!("Starting API");
    let api = http::api(ctx, subscriptions, args.test);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

fn factory_reset() -> Result<(), io::Error> {
    log::info!("Resetting application state...");
    if let Err(err) = remove_dir_all(config::dirs().data_dir()) {
        if err.kind() == io::ErrorKind::NotFound {
            log::info!("already gone");
        } else {
            log::error!("{:?}", err);
            return Err(err);
        }
    };
    log::info!("done");

    log::info!("Resetting CoCo state...");
    if let Err(err) = control::wipe_monorepo() {
        if err.kind() == io::ErrorKind::NotFound {
            log::info!("already gone");
        } else {
            log::error!("{:?}", err);
            return Err(err);
        }
    };
    log::info!("done");

    Ok(())
}
