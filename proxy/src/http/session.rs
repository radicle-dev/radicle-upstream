//! Endpoints and serialisation for [`session::Session`] related types.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::identity;
use crate::registry;
use crate::session;

/// Prefixed fitlers.
pub fn routes<R>(
    peer: Arc<Mutex<coco::Peer>>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + registry::Client,
{
    path("session").and(
        clear_cache_filter(Arc::clone(&registry))
            .or(delete_filter(Arc::clone(&store)))
            .or(get_filter(peer, registry, Arc::clone(&store)))
            .or(update_settings_filter(store)),
    )
}

/// Combination of all session filters.
#[cfg(test)]
pub fn filters<R>(
    peer: Arc<Mutex<coco::Peer>>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + registry::Client,
{
    clear_cache_filter(Arc::clone(&registry))
        .or(delete_filter(Arc::clone(&store)))
        .or(get_filter(peer, registry, Arc::clone(&store)))
        .or(update_settings_filter(store))
}

/// `DELETE /cache`
fn clear_cache_filter<R: registry::Cache>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("cache")
        .and(warp::delete())
        .and(path::end())
        .and(http::with_shared(registry))
        .and(document::document(document::description(
            "Clear cached data",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Cache cleared"),
        ))
        .and_then(handler::clear_cache)
}

/// `DELETE /`
fn delete_filter(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path::end())
        .and(http::with_store(store))
        .and(document::document(document::description(
            "Clear current Session",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Current session deleted"),
        ))
        .and_then(handler::delete)
}

/// `GET /`
fn get_filter<R: registry::Client>(
    peer: Arc<Mutex<coco::Peer>>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and(http::with_peer(peer))
        .and(http::with_shared(registry))
        .and(http::with_store(store))
        .and(document::document(document::description(
            "Fetch current Session",
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
fn update_settings_filter(
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
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::http;
    use crate::registry;
    use crate::session;

    /// Clear [`registry::Cache`].
    pub async fn clear_cache<R: registry::Cache>(
        cache: http::Shared<R>,
    ) -> Result<impl Reply, Rejection> {
        let cache = cache.read().await;
        cache.clear()?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Clear the current [`session::Session`].
    pub async fn delete(store: Arc<RwLock<kv::Store>>) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        session::clear_current(&store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get<R: registry::Client>(
        peer: Arc<Mutex<coco::Peer>>,
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
    ) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        let reg = registry.read().await;
        let sess = session::current(peer, &store, (*reg).clone()).await?;

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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::test::request;

    use librad::keys::SecretKey;

    use crate::coco;
    use crate::error;
    use crate::registry;
    use crate::session;

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir().unwrap();
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = Arc::new(Mutex::new(coco::Peer::new(config).await?));
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let cache = Arc::new(RwLock::new(registry::Cacher::new(
            registry,
            &*store.read().await,
        )));
        let api = super::filters(Arc::clone(&peer), Arc::clone(&cache), Arc::clone(&store));

        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;
        session::set_settings(&*store.read().await, settings).unwrap();

        let res = request().method("DELETE").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Test that we reset the session to default.
        let store = store.read().await;
        let have = session::current(peer, &*store, cache.read().await.clone())
            .await
            .unwrap()
            .settings;
        let want = session::settings::Settings::default();

        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir().unwrap();
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = Arc::new(Mutex::new(coco::Peer::new(config).await?));
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let api = super::filters(
            Arc::clone(&peer),
            Arc::clone(&cache),
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
                "transactionDeposits": {
                    "memberRegistration": 10,
                    "orgRegistration": 10,
                    "projectRegistration": 10,
                    "userRegistration": 10,
                },
                "minimumTransactionFee": 1,
                "permissions": {
                    "registerHandle": false,
                    "registerOrg": false,
                    "registerProject": false,
                },
            }),
        );

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir().unwrap();
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = Arc::new(Mutex::new(coco::Peer::new(config).await?));
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let api = super::filters(
            Arc::clone(&peer),
            Arc::clone(&cache),
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
                "transactionDeposits": {
                    "memberRegistration": 10,
                    "orgRegistration": 10,
                    "projectRegistration": 10,
                    "userRegistration": 10,
                },
                "minimumTransactionFee": 1,
                "permissions": {
                    "registerHandle": false,
                    "registerOrg": false,
                    "registerProject": false,
                },
            }),
        );

        Ok(())
    }
}
