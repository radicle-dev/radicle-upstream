use std::convert::TryFrom;

use coco::keystore;
use coco::seed;
use coco::signer;

use api::config;
use api::context;
use api::env;
use api::http;
use api::notification;
use api::session;

/// Flags accepted by the proxy binary.
struct Args {
    /// Put proxy in test mode to use certain fixtures to serve.
    test: bool,
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

    let store_config = {
        let store_path = if args.test {
            temp_dir.path().join("store")
        } else {
            let dirs = config::dirs();
            dirs.data_dir().join("store")
        };
        kv::Config::new(store_path).flush_every_ms(100)
    };

    let pw = keystore::SecUtf8::from("radicle-upstream");
    let mut keystore = keystore::Keystorage::new(&paths, pw);
    let key = keystore.init()?;
    let signer = signer::BoxedSigner::new(signer::SomeSigner {
        signer: key.clone(),
    });

    let (peer, state) = {
        let store = kv::Store::new(store_config.clone())?;
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config =
            coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, seeds);

        coco::into_peer_state(config, signer.clone(), store).await?
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
        store: kv::Store::new(store_config)?,
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
