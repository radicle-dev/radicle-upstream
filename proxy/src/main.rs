use proxy::coco;
use proxy::env;
use proxy::http;
use proxy::registry;

/// Flags accepted by the proxy binary.
struct Args {
    /// Signaling which backend type to use.
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

    let devnet_host = url17::Host::parse("35.241.138.91")?;
    let registry_client = match args.registry.as_str() {
        "devnet" => radicle_registry_client::Client::create_with_executor(devnet_host)
            .await
            .expect("unable to construct devnet client"),
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
        _ => panic!(format!("unknown registry source '{}'", args.registry)),
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
    let tmp_path = temp_dir.path().to_str().expect("path extraction failed");

    let mut user_peer = if args.test {
        coco::Peer::tmp(tmp_path)
            .await
            .expect("failed to create /tmp user peer")
    } else {
        todo!()
    };

    let owner = coco::fake_owner(user_peer.api.key().clone()).await;
    user_peer
        .setup_fixtures(&owner, tmp_path)
        .await
        .expect("fixture creation failed");

    let store = {
        let store_path = if args.test {
            temp_dir.path().join("store")
        } else {
            let dir = directories::ProjectDirs::from("xyz", "radicle", "upstream").unwrap();
            dir.data_dir().join("store")
        };
        let config = kv::Config::new(store_path).flush_every_ms(100);

        kv::Store::new(config)?
    };

    log::info!("Starting API");

    let cache = registry::Cacher::new(registry::Registry::new(registry_client), &store);
    let api = http::api(user_peer, owner, cache.clone(), store, args.test);

    tokio::spawn(async move {
        cache.run().await.expect("cacher run failed");
    });

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
