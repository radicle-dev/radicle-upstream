#[macro_use]
extern crate log;

use proxy::coco;
use proxy::env;
use proxy::graphql;

/// Flags accepted by the proxy binary.
struct Args {
    /// Signaling which backend type to use.
    registry: String,
    /// Put proxy in test mode to use certain fixtures to serve.
    test: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        registry: args.value_from_str("--source")?,
        test: args.contains("--test"),
    };

    let registry_client = match args.registry.as_str() {
        "devnet" => radicle_registry_client::Client::new_emulator(),
        "emulator" => futures::executor::block_on(construct_registry_client()),
        _ => panic!(format!("unknown registry source '{}'", args.registry)),
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
    let (dummy_repo, librad_paths) = if args.test {
        let librad_paths =
            librad::paths::Paths::from_root(temp_dir.path()).expect("librad paths failed");

        coco::setup_fixtures(
            &librad_paths,
            temp_dir.path().to_str().expect("path extraction failed"),
        )
        .expect("fixture setup failed");

        ("../fixtures/git-platinum", librad_paths)
    } else {
        (
            "..",
            librad::paths::Paths::new().expect("librad paths failed"),
        )
    };

    info!("Starting GraphQL HTTP API");
    graphql::api::run(dummy_repo.into(), librad_paths, registry_client);

    Ok(())
}

/// Helper to set up the Registry client against devnet.
async fn construct_registry_client() -> radicle_registry_client::Client {
    let node_host = url17::Host::parse("35.241.138.91").expect("unable to parse URL");
    radicle_registry_client::Client::create_with_executor(node_host)
        .await
        .expect("unable to construct registry client")
}
