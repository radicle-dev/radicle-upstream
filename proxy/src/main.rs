//! Proxy to serve a specialised HTTP to the oscoin MVP.

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    info!("Proxy started");
}
