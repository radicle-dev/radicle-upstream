//! Proxy to serve a specialised HTTP to the oscoin MVP.

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate juniper;

use std::io;

mod schema;
mod server;
mod source;

use crate::schema::Context;
use crate::source::Local;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    info!("Creating Juniper schema");
    let schema = std::sync::Arc::new(schema::create());

    info!("Setting up source");
    let context = std::sync::Arc::new(Context::new(Local::new()));

    info!("Starting HTTP server...");
    server::run(schema, context)
}
