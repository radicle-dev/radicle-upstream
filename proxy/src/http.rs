#![allow(warnings, missing_docs)]

use librad::paths;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{get, path, reject, reply, Filter, Rejection, Reply};

use crate::registry;

mod error;
mod project;

/// Main entry point for HTTP API.
pub async fn run(librad_paths: paths::Paths, registry: registry::Registry) {
    let api = path("v1")
        .and(project::filters(librad_paths.clone(), registry.clone()).recover(error::recover));

    // TODO(xla): Pass down as configuration with sane defaults.
    warp::serve(api).run(([127, 0, 0, 1], 8090)).await;
}

/// State filter to expose the [`librad::paths::Paths`] to handlers.
pub fn with_paths(
    paths: paths::Paths,
) -> impl Filter<Extract = (paths::Paths,), Error = Infallible> + Clone {
    warp::any().map(move || paths.clone())
}

/// State filter to expose the [`registry::Registry`] to handlers.
pub fn with_registry(
    registry: registry::Registry,
) -> impl Filter<Extract = (registry::Registry,), Error = Infallible> + Clone {
    warp::any().map(move || registry.clone())
}
