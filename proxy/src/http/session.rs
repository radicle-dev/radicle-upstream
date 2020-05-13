//! Endpoints and serialisation for [`session::Session`] related types.

use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::identity;
use crate::registry;
use crate::session;

/// `GET /`
pub fn get_filter<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("session")
        .and(warp::get())
        .and(http::with_shared(registry))
        .and(http::with_store(store))
        .and(document::document(document::description(
            "Fetch active Session",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(
                200,
                document::body(session::Session::document()).mime("application/json"),
            )
            .description("Currently active Session"),
        ))
        .and_then(handler::get)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::{reply, Rejection, Reply};

    use crate::http;
    use crate::registry;
    use crate::session;

    /// Fetch the [`session::Session`].
    pub async fn get<R: registry::Client>(
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
    ) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        let reg = registry.read().await;
        let sess = session::current(&store, (*reg).clone()).await?;

        Ok(reply::json(&sess))
    }
}

impl ToDocumentedType for session::Session {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "identity".into(),
            identity::Identity::document().nullable(true),
        );
        properties.insert("orgs".into(), document::array(registry::Org::document()));

        document::DocumentedType::from(properties).description("Session")
    }
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::registry;

    #[tokio::test]
    async fn get() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let api = super::get_filter(
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let res = request().method("GET").path("/session").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "orgs": [],
            }),
        );
    }
}
