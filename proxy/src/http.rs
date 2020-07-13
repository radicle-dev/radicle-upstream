//! HTTP API delivering JSON over `RESTish` endpoints.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::{
    document::{self, ToDocumentedType},
    path, reply, Filter, Rejection, Reply,
};

use crate::coco;
use crate::keystore;
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

/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process.
///
/// https://github.com/seanmonstar/warp/issues/507#issuecomment-615974062
/// https://github.com/rs-ipfs/rust-ipfs/commit/ae3306686209afa5911b1ad02170c1ac3bacda7c
macro_rules! combine {
    ($x:expr, $($y:expr),+) => {
        {
            let filter = $x.boxed();
            $(
                let filter = filter.or($y).boxed();
            )+
            filter
        }
    }
}

/// Main entry point for HTTP API.
pub fn api<R>(
    peer: coco::PeerApi,
    keystore: keystore::Keystorage,
    registry: R,
    store: kv::Store,
    enable_control: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + registry::Client + 'static,
{
    let peer = Arc::new(Mutex::new(peer));
    let keystore = Arc::new(RwLock::new(keystore));
    let registry = Arc::new(RwLock::new(registry));
    let store = Arc::new(RwLock::new(store));
    let subscriptions = crate::notification::Subscriptions::default();

    let avatar_filter = avatar::get_filter();
    let control_filter = control::routes(
        enable_control,
        Arc::clone(&peer),
        Arc::clone(&keystore),
        Arc::clone(&registry),
        Arc::clone(&store),
    );
    let identity_filter = identity::filters(
        Arc::clone(&peer),
        Arc::clone(&keystore),
        Arc::clone(&registry),
        Arc::clone(&store),
    );
    let notification_filter = notification::filters(subscriptions.clone());
    let org_filter = org::routes(
        Arc::clone(&peer),
        Arc::clone(&registry),
        subscriptions.clone(),
    );
    let project_filter = project::filters(
        Arc::clone(&peer),
        keystore,
        Arc::clone(&registry),
        Arc::clone(&store),
    );
    let session_filter =
        session::routes(Arc::clone(&peer), Arc::clone(&registry), Arc::clone(&store));
    let source_filter = source::routes(peer, Arc::clone(&registry), Arc::clone(&store));
    let transaction_filter = transaction::filters(Arc::clone(&registry));
    let user_filter = user::routes(registry, store, subscriptions);

    let api = path("v1").and(combine!(
        avatar_filter,
        control_filter,
        identity_filter,
        notification_filter,
        org_filter,
        project_filter,
        session_filter,
        source_filter,
        transaction_filter,
        user_filter
    ));

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

    let recovered = combine!(api, docs).recover(error::recover);

    recovered.with(cors).with(log)
}

/// Asserts presence of the owner and reject the request early if missing. Otherwise unpacks and
/// passes down.
#[must_use]
pub fn with_owner_guard<R>(
    api: Arc<Mutex<coco::PeerApi>>,
    registry: Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> BoxedFilter<(coco::User,)>
where
    R: registry::Client + 'static,
{
    warp::any()
        .and(with_peer(api))
        .and(with_shared(registry))
        .and(with_shared(store))
        .and_then(
            |api: Arc<Mutex<coco::PeerApi>>, registry: Shared<R>, store: Arc<RwLock<kv::Store>>| async move {
                let session =
                    crate::session::current(Arc::clone(&api), &*registry.read().await, &*store.read().await)
                        .await
                        .expect("unable to get current sesison");

                if let Some(identity) = session.identity {
                    let api = api.lock().await;
                    let user = coco::get_user(&*api, &identity.id).expect("unable to get coco user");
                    let user = coco::verify_user(user).expect("unable to verify user");

                    Ok(user)
                } else {
                    Err(Rejection::from(error::Routing::MissingOwner))
                }
            },
        )
        .boxed()
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

/// State filter to expose a [`coco::PeerApi`].
#[must_use]
pub fn with_peer(
    peer: Arc<Mutex<coco::PeerApi>>,
) -> impl Filter<Extract = (Arc<Mutex<coco::PeerApi>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&peer))
}

/// State filter to expose [`kv::Store`] to handlers.
#[must_use]
pub fn with_store(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = (Arc<RwLock<kv::Store>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&store))
}

/// Deserialise a query string using [`serde_qs`]. This is useful for more complicated query
/// structures that involve nesting, enums, etc.
#[must_use]
pub fn with_qs<T>() -> BoxedFilter<(T,)>
where
    for<'de> T: Deserialize<'de> + Send + Sync,
{
    warp::filters::query::raw()
        .map(|raw: String| {
            log::debug!("attempting to decode query string '{}'", raw);
            let utf8 = percent_encoding::percent_decode_str(&raw).decode_utf8_lossy();
            log::debug!("attempting to deserialize query string '{}'", utf8);
            match serde_qs::from_str(utf8.as_ref()) {
                Ok(result) => result,
                Err(err) => {
                    log::error!("failed to deserialize query string '{}': {}", raw, err);
                    panic!("{}", err)
                },
            }
        })
        .boxed()
}

/// State filter to expose [`notification::Subscriptions`] to handlers.
#[must_use]
pub fn with_subscriptions(
    subscriptions: crate::notification::Subscriptions,
) -> impl Filter<Extract = (crate::notification::Subscriptions,), Error = Infallible> + Clone {
    warp::any().map(move || crate::notification::Subscriptions::clone(&subscriptions))
}

/// Bundled input data for project registration, shared
/// between users and orgs.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterProjectInput {
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
    /// Optionally passed coco id to store for attestion.
    maybe_coco_id: Option<String>,
}

impl ToDocumentedType for RegisterProjectInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );
        properties.insert(
            "maybeCocoId".into(),
            document::string()
                .description("Optionally passed coco id to store for attestion")
                .example("ac1cac587b49612fbac39775a07fb05c6e5de08d.git"),
        );

        document::DocumentedType::from(properties).description("Input for Project registration")
    }
}

/// Register a project in the registry under the given domain.
///
/// # Errors
///
/// Might return an http error
pub async fn register_project<R: registry::Client>(
    registry: Shared<R>,
    subscriptions: crate::notification::Subscriptions,
    domain_type: registry::DomainType,
    domain_id_str: String,
    project_name: String,
    input: RegisterProjectInput,
) -> Result<impl Reply, Rejection> {
    // TODO(xla): Get keypair from persistent storage.
    let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

    let reg = registry.read().await;
    let maybe_coco_id = input
        .maybe_coco_id
        .map(|id| coco::Urn::from_str(&id).expect("Project RadUrn"));
    let domain_id = registry::Id::try_from(domain_id_str).map_err(crate::error::Error::from)?;
    let domain = match domain_type {
        registry::DomainType::Org => registry::ProjectDomain::Org(domain_id),
        registry::DomainType::User => registry::ProjectDomain::User(domain_id),
    };
    let project_name =
        registry::ProjectName::try_from(project_name).map_err(crate::error::Error::from)?;

    let tx = reg
        .register_project(
            &fake_pair,
            domain,
            project_name,
            maybe_coco_id,
            input.transaction_fee,
        )
        .await?;

    subscriptions
        .broadcast(crate::notification::Notification::Transaction(tx.clone()))
        .await;

    Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
}

#[cfg(test)]
mod test {
    use bytes::Bytes;
    use http::response::Response;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use warp::http::StatusCode;

    pub fn assert_response<F>(res: &Response<Bytes>, code: StatusCode, checks: F)
    where
        F: FnOnce(Value),
    {
        assert_eq!(
            res.status(),
            code,
            "response status was not {}, the body is:\n{:#?}",
            code,
            res.body()
        );

        let have: Value = serde_json::from_slice(res.body()).expect("failed to deserialise body");
        checks(have);
    }
}
