#[macro_use]
extern crate log;

use proxy::coco;
use proxy::env;
use proxy::graphql;

/// Flags accepted by the proxy binary.
struct Args {
    /// Signaling which backend type to use.
    _source_type: String,
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
        _source_type: args.value_from_str("--source")?,
        test: args.contains("--test"),
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
    let (registry_client, dummy_repo, librad_paths) = if args.test {
        let librad_paths = librad::paths::Paths::from_root(temp_dir.path())?;

        coco::setup_fixtures(
            &librad_paths,
            temp_dir.path().to_str().expect("path extraction failed"),
        )
        .expect("fixture creation failed");

        (
            radicle_registry_client::Client::new_emulator(),
            "../fixtures/git-platinum",
            librad_paths,
        )
    } else {
        let node_host = url17::Host::parse("35.241.138.91")?;
        (
            radicle_registry_client::Client::create_with_executor(node_host)
                .await
                .expect("registry client creation failed"),
            "..",
            librad::paths::Paths::new()?,
        )
    };

    info!("Starting GraphQL HTTP API");
    graphql::api::run(dummy_repo.into(), librad_paths, registry_client, args.test).await;

    Ok(())
}
