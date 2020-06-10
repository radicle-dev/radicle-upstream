//! Endpoints and serialisaton for [`registry::User`] related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::notification;
use crate::registry;

/// Prefixed filter
pub fn routes<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("users").and(
        register_filter(Arc::clone(&registry), store, subscriptions)
            .or(list_orgs_filter(Arc::clone(&registry)))
            .or(get_filter(registry)),
    )
}

/// Combination of all user filters.
#[cfg(test)]
fn filters<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_orgs_filter(Arc::clone(&registry))
        .or(get_filter(Arc::clone(&registry)))
        .or(register_filter(registry, store, subscriptions))
}

/// GET /<handle>
fn get_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_shared(registry))
        .and(document::param::<String>(
            "handle",
            "ID of the user to query for",
        ))
        .and(document::document(document::description("Fetch a User")))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::User::document()).mime("application/json"),
            )
            .description("User with the given id"),
        ))
        .and_then(handler::get)
}

/// POST /
fn register_filter<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(http::with_shared(registry))
        .and(http::with_store(store))
        .and(http::with_subscriptions(subscriptions))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a handle on the Registry",
        )))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(document::response(
            201,
            document::body(registry::Transaction::document()).mime("application/json"),
        )))
        .and_then(handler::register)
}

/// `GET /<handle>/orgs`
fn list_orgs_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_shared(registry))
        .and(document::param::<String>(
            "handle",
            "ID of the user to query for",
        ))
        .and(path("orgs"))
        .and(document::document(document::description(
            "List all orgs the user is a member of",
        )))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(registry::Org::document())).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::list_orgs)
}

/// User handlers for conversion between core domain and http request fullfilment.
mod handler {
    use radicle_registry_client::Balance;
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::http;
    use crate::notification;
    use crate::registry;
    use crate::session;

    /// Get the [`registry::User`] for the given `handle`.
    pub async fn get<R: registry::Client>(
        registry: http::Shared<R>,
        handle: String,
    ) -> Result<impl Reply, Rejection> {
        let handle = registry::Id::try_from(handle).map_err(crate::error::Error::from)?;
        let user = registry.read().await.get_user(handle).await?;
        Ok(reply::json(&user))
    }

    /// List the orgs the user is a member of.
    pub async fn list_orgs<R: registry::Client>(
        registry: http::Shared<R>,
        handle: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let handle = registry::Id::try_from(handle).map_err(crate::error::Error::from)?;
        let orgs = reg.list_orgs(handle).await?;

        Ok(reply::json(&orgs))
    }

    /// Register a user on the Registry.
    pub async fn register<R: registry::Client>(
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fake_fee: Balance = 100;

        let handle = registry::Id::try_from(input.handle).map_err(crate::error::Error::from)?;
        let reg = registry.write().await;
        let tx = reg
            .register_user(&fake_pair, handle.clone(), input.maybe_entity_id, fake_fee)
            .await?;

        // TODO(xla): This should only happen once the corresponding tx is confirmed.
        // Store registered user in session.
        let store = store.read().await;
        session::set_handle(&store, handle)?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

impl ToDocumentedType for registry::User {
    fn document() -> document::DocumentedType {
        let mut props = HashMap::with_capacity(2);
        props.insert(
            "handle".into(),
            document::string()
                .description("Handle/ID of the User to be registered under")
                .example("cloudhead"),
        );
        props.insert(
            "maybeEntityId".into(),
            document::string()
                .description("Exisiting entity id for attestion")
                .example("cloudhead@123abcd.git")
                .nullable(true),
        );

        document::DocumentedType::from(props)
            .description("Input for User registration")
            .nullable(true)
    }
}

/// Bundled input data for user registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Handle the User registered under.
    handle: String,
    /// Optionally passed entity id to store for attestion.
    maybe_entity_id: Option<String>,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut props = HashMap::with_capacity(2);
        props.insert(
            "handle".into(),
            document::string()
                .description("Handle/ID of the User to be registered under")
                .example("cloudhead"),
        );
        props.insert(
            "maybeEntityId".into(),
            document::string()
                .description("Exisiting project id for attestion")
                .example("cloudhead@123abcd.git")
                .nullable(true),
        );

        document::DocumentedType::from(props).description("Input for Uesr registration")
    }
}

#[allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use radicle_registry_client as protocol;

    use crate::avatar;
    use crate::error;
    use crate::notification;
    use crate::registry::{self, Cache as _, Client as _};

    #[tokio::test]
    async fn get() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let subscriptions = notification::Subscriptions::default();

        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead").unwrap();

        let _tx = registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 100)
            .await
            .unwrap();

        let api = super::filters(registry, store, subscriptions);
        let res = request()
            .method("GET")
            .path(&format!("/{}", handle))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "handle": "cloudhead",
                "maybeEntityId": Value::Null,
            })
        );
    }

    #[tokio::test]
    async fn list_orgs() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(Arc::clone(&registry), store, subscriptions);

        // Register the user
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead")?;
        let org_id = registry::Id::try_from("radicle")?;

        registry
            .write()
            .await
            .register_user(&author, handle.clone(), Some("123abcd.git".into()), 100)
            .await?;

        let user = registry
            .read()
            .await
            .get_user(handle.clone())
            .await?
            .unwrap();

        // Register the org
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), 100)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/orgs", handle))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!([registry::Org {
                id: org_id.clone(),
                shareable_entity_identifier: format!("%{}", org_id.to_string()),
                avatar_fallback: avatar::Avatar::from(&org_id.to_string(), avatar::Usage::Org),
                members: vec![user]
            }])
        );

        Ok(())
    }

    #[tokio::test]
    async fn register() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::clone(&cache),
            Arc::new(RwLock::new(store)),
            subscriptions,
        );

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::RegisterInput {
                handle: "cloudhead".into(),
                maybe_entity_id: Some("cloudhead@123abcd.git".into()),
            })
            .reply(&api)
            .await;

        let txs = cache.read().await.list_transactions(vec![]).unwrap();
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));
    }
}
