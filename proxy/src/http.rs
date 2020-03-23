#![allow(warnings, missing_docs)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{get, path, reject, reply, Filter, Rejection, Reply};

use crate::error;

mod project;

/// Main entry point for HTTP API.
pub async fn run(librad_paths: &librad::paths::Paths) {
    let api = path("v1");
    let routes = api.and(project::filters(librad_paths.clone()).recover(rejection_handle));

    warp::serve(routes).run(([127, 0, 0, 1], 8090)).await;
}

impl reject::Reject for error::Error {}

impl From<error::Error> for Rejection {
    fn from(err: error::Error) -> Self {
        reject::custom(err)
    }
}

/// Error type to carry context for failed requests.
#[derive(serde_derive::Serialize)]
struct Error {
    message: &'static str,
    variant: &'static str,
}

/// Handler to convert [`error::Error`] to [`Error`] response.
async fn rejection_handle(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, variant, message) = (
        StatusCode::NOT_IMPLEMENTED,
        "INTERNAL_ERROR",
        "Something went wrong",
    );
    let res = reply::json(&Error { message, variant });

    Ok(reply::with_status(res, code))
}

