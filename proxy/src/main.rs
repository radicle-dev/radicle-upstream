//! Proxy to serve a specialised HTTP to the oscoin MVP.

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use warp::{self, path, Filter};

fn main() {
    pretty_env_logger::init();
    info!("Proxy started");

    let projects = path!("projects").map(|| "not implemented".to_string());

    warp::serve(projects).run(([127, 0, 0, 1], 3030));
}
