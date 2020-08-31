use std::convert::TryFrom;

use librad::paths;

use coco::announce;
use coco::seed;

use api::config;
use api::context;
use api::env;
use api::http;
use api::keystore;
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

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
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
    let pw = keystore::SecUtf8::from("radicle-upstream");

    let paths = paths::Paths::try_from(paths_config)?;

    let mut keystore = keystore::Keystorage::new(&paths, pw);
    let key = keystore.init_librad_key()?;

    let store = {
        let store_path = if args.test {
            temp_dir.path().join("store")
        } else {
            let dirs = config::dirs();
            dirs.data_dir().join("store")
        };
        let config = kv::Config::new(store_path).flush_every_ms(100);

        kv::Store::new(config)?
    };

    let peer_api = {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config =
            coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, seeds);

        coco::Api::new(config).await?
    };

    if args.test {
        // TODO(xla): Given that we have proper ownership and user handling in coco, we should
        // evaluate how meaningful these fixtures are.
        let owner = peer_api.init_owner(&key, "cloudhead")?;
        coco::control::setup_fixtures(&peer_api, &key, &owner).expect("fixture creation failed");
    }

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    coco::git_helper::setup(&proxy_path, &bin_dir).expect("Git remote helper setup failed");

    let watcher_ctx = ctx.clone();
    tokio::task::spawn_local(async move { announcement_watcher(watcher_ctx).await });

    log::info!("Starting API");
    let ctx = context::Ctx::from(context::Context {
        peer_api,
        keystore,
        store,
    });
    let api = http::api(ctx, args.test);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

async fn announcement_watcher(ctx: context::Ctx) {
    // TODO(xla): Take interval timings from config/settings.
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        timer.tick().await;

        let ctx = ctx.read().await;

        // TODO(xla): get/load old state
        let old: Vec<announce::Announcement> = vec![];
        let new = announce::build(&ctx.peer_api).expect("unable to build state");
        let updates = announce::diff(&old, &new);
        ctx.peer_api.with_api(|api| {
            announce::announce(api, updates)
                .await
                .expect("announce failed");
        })?
        // TODO(xla): save new state
    }
}
