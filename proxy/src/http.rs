#![allow(warnings, missing_docs)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{get, path, reject, reply, Filter, Rejection, Reply};

mod error;
mod project;

/// Main entry point for HTTP API.
pub async fn run(librad_paths: &librad::paths::Paths) {
    let api = path("v1").and(project::filters(librad_paths.clone()).recover(error::recover));

    warp::serve(api).run(([127, 0, 0, 1], 8090)).await;
}

/// State filter to expose the [`librad::paths::Paths`] to handlers.
pub fn with_paths(
    paths: librad::paths::Paths,
) -> impl Filter<Extract = (librad::paths::Paths,), Error = Infallible> + Clone {
    warp::any().map(move || paths.clone())
}
