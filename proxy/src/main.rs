//! Proxy to serve a specialised `GraphQL` API to radicle-upstream.

#![deny(missing_docs, unused_import_braces, unused_qualifications, warnings)]
#![deny(
    clippy::all,
    // clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
// TODO(xla): Handle all Results properly and never panic outside of main.
// TODO(xla): Remove exception for or_fun_call lint.
// TODO(xla): Remove let_underscore_must_use once the issue is resolved: https://github.com/rust-lang/rust-clippy/issues/4980
#![allow(
    clippy::implicit_return,
    // clippy::let_underscore_must_use,
    clippy::option_expect_used,
    clippy::or_fun_call,
    clippy::result_expect_used,
    clippy::unseparated_literal_suffix
)]

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

/// Flags accepted by the proxy binary.
struct Args {
    /// Signaling which backend type to use.
    _source_type: String,
    /// Put proxy in test mode to use certain fixtures to serve.
    test: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        _source_type: args.value_from_str("--source")?,
        test: args.contains("--test"),
    };

    let temp_dir = tempfile::tempdir().expect("test dir creation failed");
    let (dummy_repo, librad_paths) = if args.test {
        let librad_paths =
            librad::paths::Paths::from_root(temp_dir.path()).expect("librad paths failed");

        crate::schema::git::setup_fixtures(
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

    let context = schema::Context::new(
        dummy_repo.into(),
        librad_paths,
        radicle_registry_client::Client::new_emulator(),
    );

    info!("Creating GraphQL schema and context");
    let schema = schema::create();

    info!("Starting HTTP server");
    server_warp::run(schema, context);

    Ok(())
}
