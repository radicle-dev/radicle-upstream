//! Endpoints for Org.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::notification;
use crate::registry;

/// Combination of all org routes.
pub fn filters(
    registry: Arc<RwLock<registry::Registry>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_filter(Arc::clone(&registry)).or(register_filter(registry, subscriptions))
}

/// `POST /orgs`
fn register_filter(
    registry: Arc<RwLock<registry::Registry>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("orgs")
        .and(warp::post())
        .and(super::with_registry(registry))
        .and(super::with_subscriptions(subscriptions))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new unique Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register)
}

/// `GET /orgs/<id>`
fn get_filter(
    registry: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("orgs")
        .and(document::param::<String>("id", "Unique ID of the Org"))
        .and(warp::get())
        .and(super::with_registry(registry))
        .and(document::document(document::description("Find Org by ID")))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Org not found"),
        ))
        .and_then(handler::get)
}

/// Org handlers for conversion between core domain and http request fullfilment.
mod handler {
    use radicle_registry_client::Balance;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::notification;
    use crate::registry;

    /// Register an org on the Registry.
    pub async fn register(
        registry: Arc<RwLock<registry::Registry>>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fake_fee: Balance = 100;

        let mut reg = registry.write().await;
        let tx = reg.register_org(&fake_pair, input.id, fake_fee).await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Get the Org for the given `id`.
    pub async fn get(
        id: String,
        registry: Arc<RwLock<registry::Registry>>,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let org = reg.get_org(id).await?;

        Ok(reply::json(&org))
    }
}

impl Serialize for registry::Org {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Org", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("avatarFallback", &self.avatar_fallback)?;

        state.end()
    }
}

impl ToDocumentedType for registry::Org {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert("avatarFallback".into(), avatar::Avatar::document());
        properties.insert(
            "id".into(),
            document::string()
                .description("The id of the org")
                .example("monadic"),
        );

        document::DocumentedType::from(properties).description("Org")
    }
}

/// Bundled input data for org registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Id of the Org.
    id: String,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "id".into(),
            document::string()
                .description("ID of the org")
                .example("monadic"),
        );

        document::DocumentedType::from(properties).description("Input for org registration")
    }
}

#[allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::notification;
    use crate::registry;

    #[tokio::test]
    async fn register() {
        let registry = Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        )));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(Arc::clone(&registry), subscriptions);

        let res = request()
            .method("POST")
            .path("/orgs")
            .json(&super::RegisterInput {
                id: "monadic".into(),
            })
            .reply(&api)
            .await;

        let txs = registry
            .write()
            .await
            .list_transactions(vec![])
            .await
            .unwrap();
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));
    }

    #[tokio::test]
    async fn get() {
        let registry = Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        )));
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(Arc::clone(&registry), subscriptions);

        // Register the org
        let alice = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let fee: radicle_registry_client::Balance = 100;
        registry
            .write()
            .await
            .register_org(&alice, "monadic".to_string(), fee)
            .await
            .unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/orgs/{}", "monadic"))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(registry::Org {
                id: "monadic".to_string(),
                avatar_fallback: avatar::Avatar::from("monadic", avatar::Usage::Org),
            })
        );
    }
}
