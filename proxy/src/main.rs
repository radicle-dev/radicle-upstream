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
        "emulator" => radicle_registry_client::Client::new_emulator(),
        _ => panic!(format!("unknown registry source '{}'", args.registry)),
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
    let librad_paths = if args.test {
        let librad_paths =
            librad::paths::Paths::from_root(temp_dir.path()).expect("librad paths failed");

        coco::setup_fixtures(
            &librad_paths,
            temp_dir.path().to_str().expect("path extraction failed"),
        )
        .expect("fixture creation failed");

        librad_paths
    } else {
        librad::paths::Paths::new()?
    };
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
    let api = http::api(librad_paths, cache, store, args.test);

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
