//! HTTP API delivering JSON over `RESTish` endpoints.

use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
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
pub fn api<R>(
    peer: coco::Peer,
    owner: coco::User,
    registry: R,
    store: kv::Store,
    enable_control: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + registry::Client + 'static,
{
    let peer = Arc::new(peer);
    let owner = Arc::new(RwLock::new(owner));
    let registry = Arc::new(RwLock::new(registry));
    let store = Arc::new(RwLock::new(store));
    let subscriptions = crate::notification::Subscriptions::default();

    let api = path("v1").and(
        avatar::get_filter()
            .or(control::routes(
                enable_control,
                Arc::clone(&peer),
                Arc::clone(&owner),
                Arc::clone(&registry),
                Arc::clone(&store),
            ))
            .or(identity::filters(Arc::clone(&registry), Arc::clone(&store)))
            .or(notification::filters(subscriptions.clone()))
            .or(org::routes(
                Arc::clone(&peer),
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(project::filters(
                Arc::clone(&peer),
                Arc::clone(&owner),
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(session::routes(Arc::clone(&registry), Arc::clone(&store)))
            .or(source::routes(peer))
            .or(transaction::filters(Arc::clone(&registry)))
            .or(user::routes(registry, store, subscriptions)),
    );
    // let docs = path("docs").and(doc::filters(&api));
    let docs = path("docs").and(doc::index_filter().or(doc::describe_filter(&api)));
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(&[warp::http::header::CONTENT_TYPE])
        .allow_methods(&[
            warp::http::Method::DELETE,
            warp::http::Method::GET,
            warp::http::Method::POST,
            warp::http::Method::OPTIONS,
        ]);
    let log = warp::log::custom(|info| {
        log::info!(
            target: "proxy::http",
            "\"{} {} {:?}\" {} {:?}",
            info.method(),
            info.path(),
            info.version(),
            info.status().as_u16(),
            info.elapsed(),
        );
    });

    let recovered = api.or(docs).recover(error::recover);

    recovered.with(cors).with(log)
}

/// Thread-safe container for threadsafe pass-through to filters and handlers.
pub type Shared<T> = Arc<RwLock<T>>;

/// State filter to expose a [`Shared`] and its content.
#[must_use]
pub fn with_shared<T>(
    container: Shared<T>,
) -> impl Filter<Extract = (Shared<T>,), Error = Infallible> + Clone
where
    T: Send + Sync,
{
    warp::any().map(move || Arc::clone(&container))
}

/// State filter to expose a [`Peer`].
#[must_use]
pub fn with_peer(
    peer: Arc<coco::Peer>,
) -> impl Filter<Extract = (Arc<coco::Peer>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&peer))
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

#[cfg(test)]
mod test {
    use bytes::Bytes;
    use http::response::Response;
    use serde_json::Value;
    use warp::http::StatusCode;

    pub fn assert_response<F>(res: &Response<Bytes>, checks: F)
    where
        F: FnOnce(Value) -> (),
    {
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "response status was not 200, the body is:\n{:#?}",
            res.body()
        );

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        checks(have);
    }
}
