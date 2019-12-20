//! Proxy to serve a specialised `GraphQL` API to radicle-upstream.

#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(
    clippy::all,
    // clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::result_unwrap_used
)]
#![allow(clippy::unseparated_literal_suffix, clippy::implicit_return)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate juniper;

/// Utilities to manipulate the process environment.
mod env;
/// Defines the schema served to the application via `GraphQL`.
mod schema;
/// Server infrastructure used to power the API.
mod server_warp;
/// Origin of data required like the on-chain Registry.
mod source;

/// Flags accepted by the proxy binary.
struct Args {
    /// Signaling which backend type to use.
    source_type: String,
    /// Put proxy in test mode to use certain fixtures to serve.
    test: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        source_type: args.value_from_str("--source")?,
        test: args.contains("--test"),
    };

    let (dummy_repo, librad_paths) = if args.test {
        let temp_dir = tempfile::tempdir().expect("test dir creation failed");
        let librad_paths =
            librad::paths::Paths::from_root(temp_dir.path()).expect("librad paths failed");

        crate::schema::git::setup_fixtures(
            &librad_paths,
            temp_dir
                .path()
                .to_str()
                .expect("path extraction failed")
                .to_string(),
        )
        .expect("fixture setup failed");

        ("../fixtures/git-platinum", librad_paths)
    } else {
        (
            "..",
            librad::paths::Paths::new().expect("librad paths failed"),
        )
    };

    let context = if "memory" == args.source_type {
        schema::Context::new(dummy_repo.into(), librad_paths)
    } else {
        schema::Context::new(dummy_repo.into(), librad_paths)
    };

    info!("Creating GraphQL schema and context");
    let schema = schema::create();

    info!("Starting HTTP server");
    server_warp::run(schema, context);

    Ok(())
}
