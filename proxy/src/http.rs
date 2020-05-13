//! HTTP API delivering JSON over `RESTish` endpoints.

use librad::paths;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, Filter, Reply};

use crate::registry;

mod avatar;
mod control;
mod doc;
mod error;
mod identity;
mod notification;
mod org;
mod project;
mod session;
mod source;
mod transaction;
mod user;

/// Main entry point for HTTP API.
pub fn routes<R>(
    librad_paths: Arc<RwLock<paths::Paths>>,
    registry: Container<R>,
    store: Arc<RwLock<kv::Store>>,
    enable_control: bool,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone
where
    R: registry::Cache + registry::Client,
{
    let subscriptions = crate::notification::Subscriptions::default();

    let api = path("v1").and(
        avatar::get_filter()
            .or(control::routes(
                enable_control,
                Arc::clone(&librad_paths),
                Arc::clone(&registry),
                Arc::clone(&store),
            ))
            .or(identity::filters(Arc::clone(&registry), Arc::clone(&store)))
            .or(notification::filters(subscriptions.clone()))
            .or(org::routes(
                Arc::clone(&librad_paths),
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(project::filters(
                Arc::clone(&librad_paths),
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(session::get_filter(Arc::clone(&registry), store))
            .or(source::routes(librad_paths))
            .or(transaction::filters(Arc::clone(&registry)))
            .or(user::routes(registry, subscriptions)),
    );
    // let docs = path("docs").and(doc::filters(&api));
    let docs = path("docs").and(doc::index_filter().or(doc::describe_filter(&api)));

    api.or(docs)
        .with(warp::log("proxy::http"))
        .recover(error::recover)
}

/// State filter to expose the [`librad::paths::Paths`] to handlers.
#[must_use]
pub fn with_paths(
    paths: Arc<RwLock<paths::Paths>>,
) -> impl Filter<Extract = (Arc<RwLock<paths::Paths>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&paths))
}

/// Thread-safe container for threadsafe pass-through to filters and handlers.
pub type Container<T> = Arc<RwLock<T>>;

/// State filter to expose a [`Container`] and its content.
#[must_use]
pub fn with_container<T>(
    container: Container<T>,
) -> impl Filter<Extract = (Container<T>,), Error = Infallible> + Clone
where
    T: Send + Sync,
{
    warp::any().map(move || Arc::clone(&container))
}

/// State filter to expose [`kv::Store`] to handlers.
#[must_use]
pub fn with_store(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = (Arc<RwLock<kv::Store>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&store))
}

/// State filter to expose [`notification::Subscriptions`] to handlers.
#[must_use]
pub fn with_subscriptions(
    subscriptions: crate::notification::Subscriptions,
) -> impl Filter<Extract = (crate::notification::Subscriptions,), Error = Infallible> + Clone {
    warp::any().map(move || crate::notification::Subscriptions::clone(&subscriptions))
}
