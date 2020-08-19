use std::convert::TryFrom;

use librad::paths;

use proxy::coco;
use proxy::coco::signer;
use proxy::config;
use proxy::env;
use proxy::http;
use proxy::registry;
use proxy::seed;
use proxy::session;

/// Flags accepted by the proxy binary.
struct Args {
    /// Host name or IP for the registry node to connect to. If the special value "emulator" is
    /// provided the proxy will not connect to a node but emulate the chain in memory.
    registry: String,
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
        registry: args.value_from_str("--registry")?,
        test: args.contains("--test"),
    };

    let registry_client = match args.registry.as_str() {
        "emulator" => {
            let (client, control) = radicle_registry_client::Client::new_emulator();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    control.add_blocks(1);
                }
            });

            client
        }
        host => {
            let host = url17::Host::parse(host)?;
            radicle_registry_client::Client::create_with_executor(host)
                .await
                .expect("unable to construct devnet client")
        }
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

    let paths = paths::Paths::try_from(paths_config)?;

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

    let pw = signer::SecUtf8::from("radicle-upstream");
    let signer = signer::Store::init(&paths, pw)?;

    let coco_api = {
        let seeds = session::settings(&store).await?.coco.seeds;
        let seeds = seed::resolve(&seeds).await.unwrap_or_else(|err| {
            log::error!("Error parsing seed list {:?}: {}", seeds, err);
            vec![]
        });
        let config = coco::config::configure(paths, signer.clone(), seeds);

        coco::Api::new(config).await?
    };

    if args.test {
        // TODO(xla): Given that we have proper ownership and user handling in coco, we should
        // evaluate how meaningful these fixtures are.
        let owner = coco_api.init_owner(&signer, "cloudhead")?;
        coco::control::setup_fixtures(&coco_api, &signer, &owner).expect("fixture creation failed");
    }

    let proxy_path = config::proxy_path()?;
    let bin_dir = config::bin_dir()?;
    coco::git_helper::setup(&proxy_path, &bin_dir).expect("Git remote helper setup failed");

    log::info!("Starting API");

    let cache = registry::Cacher::new(registry::Registry::new(registry_client), &store);
    let api = http::api(coco_api, cache.clone(), signer, store, args.test);

    tokio::spawn(async move {
        cache.run().await.expect("cacher run failed");
    });

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
