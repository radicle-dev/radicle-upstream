//! Proxy to serve a specialised HTTP to the oscoin MVP.

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate juniper;

mod schema;
// mod server_actix;
mod server_warp;
mod source;

use crate::schema::Context;
use crate::source::Local;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    info!("Setting up source");
    let source = Local::new();

    info!("Creating GraphQL schema and context");
    let schema = schema::create();
    let context = Context::new(source);

    info!("Starting HTTP server");
    server_warp::run(schema, context);
}
