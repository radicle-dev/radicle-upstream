use std::convert::TryFrom;

use librad::paths;
use radicle_keystore::pinentry::SecUtf8;

use proxy::coco;
use proxy::config;
use proxy::env;
use proxy::http;
use proxy::keystore;
use proxy::registry;

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
        },
        host => {
            let host = url17::Host::parse(host)?;
            radicle_registry_client::Client::create_with_executor(host)
                .await
                .expect("unable to construct devnet client")
        },
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");

    let paths_config = if args.test {
        coco::config::Paths::FromRoot(temp_dir.path().to_path_buf())
    } else {
        coco::config::Paths::default()
    };
    let pw = SecUtf8::from("radicle-upstream");

    let paths = paths::Paths::try_from(paths_config)?;

    let mut keystore = keystore::Keystorage::new(&paths, pw);
    let key = keystore.init_librad_key()?;

    let (peer_api, owner) = {
        let config = coco::config::configure(paths, key.clone());
        let peer_api = coco::create_peer_api(config).await?;
        let owner = coco::init_user(&peer_api, key.clone(), "cloudhead")?;
        let owner = coco::verify_user(owner).await?;

        (peer_api, owner)
    };

    if args.test {
        coco::control::setup_fixtures(&peer_api, key, &owner).expect("fixture creation failed");
    }

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

    log::info!("Starting API");

    let cache = registry::Cacher::new(registry::Registry::new(registry_client), &store);
    let api = http::api(peer_api, owner, keystore, cache.clone(), store, args.test);

    tokio::spawn(async move {
        cache.run().await.expect("cacher run failed");
    });

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
