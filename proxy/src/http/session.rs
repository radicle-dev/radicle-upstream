//! Endpoints and serialisation for [`session::Session`] related types.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::identity;
use crate::registry;
use crate::session;

/// Prefixed fitlers.
pub fn routes<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("session").and(get_filter(registry, Arc::clone(&store)).or(update_settings_filter(store)))
}

/// Combination of all session filters.
#[cfg(test)]
pub fn filters<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_filter(registry, Arc::clone(&store)).or(update_settings_filter(store))
}

/// `GET /`
pub fn get_filter<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
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

/// `Post /settings`
pub fn update_settings_filter(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("settings")
        .and(warp::post())
        .and(path::end())
        .and(http::with_store(store))
        .and(warp::body::json())
        .and(document::document(document::description("Update settings")))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Settings successfully updated"),
        ))
        .and_then(handler::update_settings)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
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

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings(
        store: Arc<RwLock<kv::Store>>,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        session::set_settings(&store, settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

impl ToDocumentedType for session::Session {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert(
            "identity".into(),
            identity::Identity::document().nullable(true),
        );
        properties.insert("orgs".into(), document::array(registry::Org::document()));
        properties.insert("settings".into(), session::settings::Settings::document());

        document::DocumentedType::from(properties).description("Session")
    }
}

impl ToDocumentedType for session::settings::Settings {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "appearance".into(),
            session::settings::Appearance::document(),
        );
        properties.insert("registry".into(), session::settings::Registry::document());

        document::DocumentedType::from(properties).description("Settings")
    }
}

impl ToDocumentedType for session::settings::Appearance {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert("theme".into(), session::settings::Theme::document());

        document::DocumentedType::from(properties).description("Appearance")
    }
}

impl ToDocumentedType for session::settings::Theme {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["dark".into(), "light".into()])
            .description("Variants for possible color schemes.")
            .example("dark")
    }
}

impl ToDocumentedType for session::settings::Registry {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert("network".into(), session::settings::Network::document());

        document::DocumentedType::from(properties).description("Registry")
    }
}

impl ToDocumentedType for session::settings::Network {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["emulator".into(), "ffnet".into(), "testnet".into()])
            .description("Variants for possible networks of the Registry to connect to.")
            .example("testnet")
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
    use crate::session;

    #[tokio::test]
    async fn get() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let api = super::filters(
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "orgs": [],
                "settings": {
                    "appearance": {
                        "theme": "light",
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
            }),
        );
    }

    #[tokio::test]
    async fn udpate_settings() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let api = super::filters(
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;

        let res = request()
            .method("POST")
            .path("/settings")
            .json(&settings)
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        let res = request().method("GET").path("/").reply(&api).await;
        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "orgs": [],
                "settings": {
                    "appearance": {
                        "theme": "dark",
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
            }),
        );
    }
}
