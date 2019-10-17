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

    let osc = oscoin_client::Client::new_from_file().expect("setup of osc client from file failed");
    let source = source::Ledger::new(osc);

    info!("Creating GraphQL schema and context");
    let schema = schema::create();
    let context = schema::Context::new(source);

    info!("Starting HTTP server");
    server_warp::run(schema, context);
}
