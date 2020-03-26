//! HTTP API delivering JSON over `RESTish` endpoints.

use librad::paths;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, Filter};

use crate::registry;

mod error;
mod project;

/// Main entry point for HTTP API.
pub async fn run(librad_paths: paths::Paths, reg: registry::Registry) {
    let api = path("v1").and(
        project::filters(librad_paths.clone(), Arc::new(RwLock::new(reg))).recover(error::recover),
    );

    // TODO(xla): Pass down as configuration with sane defaults.
    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}

/// State filter to expose the [`librad::paths::Paths`] to handlers.
#[must_use]
pub fn with_paths(
    paths: paths::Paths,
) -> impl Filter<Extract = (paths::Paths,), Error = Infallible> + Clone {
    warp::any().map(move || paths.clone())
}

/// State filter to expose the [`registry::Registry`] to handlers.
#[must_use]
pub fn with_registry(
    reg: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = (Arc<RwLock<registry::Registry>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::<RwLock<registry::Registry>>::clone(&reg))
}
