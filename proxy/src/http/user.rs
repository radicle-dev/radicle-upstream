//! Endpoints and serialisaton for [`registry::User`] related types.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::Arc;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::notification;
use crate::registry;

/// Prefixed filter
pub fn routes(
    registry: super::Registry,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("users").and(
        register_filter(Arc::clone(&registry), subscriptions)
            .or(list_orgs_filter(Arc::clone(&registry)))
            .or(get_filter(registry)),
    )
}

/// Combination of all user filters.
#[cfg(test)]
fn filters(
    registry: super::Registry,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_orgs_filter(Arc::clone(&registry))
        .or(get_filter(Arc::clone(&registry)))
        .or(register_filter(registry, subscriptions))
}

/// GET /<handle>
fn get_filter(
    registry: super::Registry,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(super::with_registry(registry))
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
fn register_filter(
    registry: super::Registry,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(super::with_registry(registry))
        .and(super::with_subscriptions(subscriptions))
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
fn list_orgs_filter(
    registry: super::Registry,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(super::with_registry(registry))
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
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::http;
    use crate::notification;

    /// Get the [`registry::User`] for the given `handle`.
    pub async fn get(registry: http::Registry, handle: String) -> Result<impl Reply, Rejection> {
        let user = registry.read().await.get_user(handle).await?;
        Ok(reply::json(&user))
    }

    /// Register a user on the Registry.
    pub async fn register(
        registry: http::Registry,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fake_fee: Balance = 100;

        let mut reg = registry.write().await;
        let tx = reg
            .register_user(&fake_pair, input.handle, input.maybe_entity_id, fake_fee)
            .await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// List the orgs the user is a member of.
    pub async fn list_orgs(
        registry: http::Registry,
        handle: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let orgs = reg.list_orgs(handle).await?;

        Ok(reply::json(&orgs))
    }
}

impl Serialize for registry::User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 2)?;
        state.serialize_field("handle", &self.handle.to_string())?;
        state.serialize_field("maybeEntityId", &self.maybe_entity_id)?;

        state.end()
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
            .description("Input for Uesr registration")
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
    use radicle_registry_client::ed25519;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::http;
    use crate::notification;
    use crate::registry;

    #[tokio::test]
    async fn get() {
        let registry: http::Registry = Arc::new(RwLock::new(Box::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))));
        let subscriptions = notification::Subscriptions::default();
        let author = ed25519::Pair::from_legacy_string("//Alice", None);

        let handle = "cloudhead";

        let _tx = registry
            .write()
            .await
            .register_user(&author, handle.to_string(), None, 100)
            .await
            .unwrap();

        let api = super::filters(registry, subscriptions);
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
    async fn list_orgs() {
        let registry: http::Registry = Arc::new(RwLock::new(Box::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))));
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(Arc::clone(&registry), subscriptions);

        // Register the user
        let alice = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        registry
            .write()
            .await
            .register_user(&alice, "alice".into(), Some("123abcd.git".into()), 100)
            .await
            .unwrap();

        // Register the org
        let fee: radicle_registry_client::Balance = 100;
        registry
            .write()
            .await
            .register_org(&alice, "monadic".to_string(), fee)
            .await
            .unwrap();

        let res = request()
            .method("GET")
            .path("/alice/orgs")
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!([registry::Org {
                id: "monadic".to_string(),
                avatar_fallback: avatar::Avatar::from("monadic", avatar::Usage::Org),
            }])
        );
    }

    #[tokio::test]
    async fn register() {
        let registry: http::Registry = Arc::new(RwLock::new(Box::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        ))));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(Arc::clone(&registry), subscriptions);
        let res = request()
            .method("POST")
            .path("/")
            .json(&super::RegisterInput {
                handle: "cloudhead".into(),
                maybe_entity_id: Some("cloudhead@123abcd.git".into()),
            })
            .reply(&api)
            .await;

        let txs = registry
            .read()
            .await
            .list_transactions(vec![])
            .await
            .unwrap();
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));
    }
}
