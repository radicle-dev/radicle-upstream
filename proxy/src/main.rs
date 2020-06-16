use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use librad::keys;
use librad::net::{self, discovery};
use librad::paths;
use librad::peer;

use proxy::coco;
use proxy::env;
use proxy::http;
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
    let tmp_path = temp_dir.path().to_str().expect("path extraction failed");

    let mut peer = if args.test {
        let key = keys::SecretKey::new();
        let config = coco::default_config(key, tmp_path).expect("failed to get config");
        coco::Peer::new(config)
            .await
            .expect("failed to create /tmp user peer")
    } else {
        // TODO(finto): There should be a coco::config module that knows how to parse the
        // configs/parameters to give us back a `PeerConfig`

        // TODO(finto): Should be fetched from key storage
        let key = keys::SecretKey::new();
        // TODO(finto): Should be read from config file
        let gossip_params = Default::default();
        // TODO(finto): Read from config or passed as param
        let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
        // TODO(finto): could we initialise with known seeds from a cache?
        let seeds: Vec<(peer::PeerId, SocketAddr)> = vec![];
        let disco = discovery::Static::new(seeds);
        // TODO(finto): read in from config or passed as param
        let paths = paths::Paths::new()?;
        let config = net::peer::PeerConfig {
            key,
            paths,
            listen_addr,
            gossip_params,
            disco,
        };

        coco::Peer::new(config)
            .await
            .expect("failed to create /tmp user peer")
    };

    let owner = coco::fake_owner(&peer).await;

    if args.test {
        peer.setup_fixtures(&owner)
            .await
            .expect("fixture creation failed");
    }

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
    let api = http::api(peer, owner, cache.clone(), store, args.test);

    tokio::spawn(async move {
        cache.run().await.expect("cacher run failed");
    });

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
