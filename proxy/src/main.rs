//! Proxy to serve a specialised HTTP to the oscoin MVP.

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
extern crate pretty_env_logger;

#[macro_use]
extern crate juniper;

/// Defines the schema served to the application via `GraphQL`.
mod schema;
/// Server infrastructure used to power the API.
mod server_warp;
/// Origin of data required like the on-chain Registry.
mod source;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let source_type = std::env::args().nth(1).expect("no source was given");

    let context = if let "memory" = source_type.as_ref() {
        let client = radicle_registry_client::MemoryClient::new();
        let src = source::Ledger::new(client);
        schema::Context::new(src)
    } else {
        let client = radicle_registry_client::ClientWithExecutor::create()
            .expect("creating registry client failed");
        let src = source::Ledger::new(client);
        schema::Context::new(src)
    };

    info!("Creating GraphQL schema and context");
    let schema = schema::create();

    info!("Starting HTTP server");
    server_warp::run(schema, context);
}
