//! Proxy to serve a specialised GraphQL API to radicle-upstream.

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

struct Args {
    source_type: String,
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

    let dummy_repo = if args.test {
        "../fixtures/git-platinum"
    } else {
        ".."
    };
    let context = if "memory" == args.source_type {
        let client = radicle_registry_client::MemoryClient::new();
        let mut src = source::Ledger::new(client);

        source::setup_fixtures(&mut src);

        schema::Context::new(dummy_repo.into(), src)
    } else {
        let client = radicle_registry_client::ClientWithExecutor::create()
            .expect("creating registry client failed");
        let src = source::Ledger::new(client);
        schema::Context::new(dummy_repo.into(), src)
    };

    info!("Creating GraphQL schema and context");
    let schema = schema::create();

    info!("Starting HTTP server");
    server_warp::run(schema, context);

    Ok(())
}
