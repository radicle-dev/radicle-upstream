//! Endpoints and serialisation for [`identity::Identity`] related types.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::coco;
use crate::http;
use crate::identity;
use crate::keystore;
use crate::registry;

/// Combination of all identity routes.
pub fn filters<R: registry::Client>(
    peer: Arc<Mutex<coco::PeerApi>>,
    keystore: http::Shared<keystore::Keystorage>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_filter(Arc::clone(&peer)).or(create_filter(peer, keystore, registry, store))
}

/// `POST /identities`
fn create_filter<R: registry::Client>(
    peer: Arc<Mutex<coco::PeerApi>>,
    keystore: http::Shared<keystore::Keystorage>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("identities")
        .and(warp::post())
        .and(http::with_peer(peer))
        .and(http::with_shared(keystore))
        .and(http::with_shared(registry))
        .and(http::with_store(store))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Create a new unique Identity",
        )))
        .and(document::document(document::tag("Identity")))
        .and(document::document(
            document::body(CreateInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(identity::Identity::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::create)
}

/// `GET /identities/<id>`
fn get_filter(
    peer: Arc<Mutex<coco::PeerApi>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("identities")
        .and(http::with_peer(peer))
        .and(document::param::<String>("id", "Unique ID of the Identity"))
        .and(warp::get())
        .and(document::document(document::description(
            "Find Identity by ID",
        )))
        .and(document::document(document::tag("Identity")))
        .and(document::document(
            document::response(
                200,
                document::body(identity::Identity::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Identity not found"),
        ))
        .and_then(handler::get)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::keystore;
    use crate::registry;
    use crate::session;

    /// Create a new [`identity::Identity`].
    pub async fn create<R: registry::Client>(
        peer: Arc<Mutex<coco::PeerApi>>,
        keystore: http::Shared<keystore::Keystorage>,
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let store = store.read().await;

        if let Some(identity) = session::current(Arc::clone(&peer), &store, &*reg)
            .await?
            .identity
        {
            return Err(Rejection::from(error::Error::EntityExists(identity.id)));
        }

        let keystore = keystore.read().await;
        let key = keystore.get_librad_key().map_err(error::Error::from)?;
        let id = identity::create(peer, key, input.handle.parse()?).await?;

        session::set_identity(&store, id.clone())?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(peer: Arc<Mutex<coco::PeerApi>>, id: String) -> Result<impl Reply, Rejection> {
        let peer = peer.lock().await;
        let id = identity::get(&peer, &id.parse().expect("could not parse id"))?;
        Ok(reply::json(&id))
    }
}

impl ToDocumentedType for identity::Identity {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(5);
        properties.insert("avatarFallback".into(), avatar::Avatar::document());
        properties.insert(
            "id".into(),
            document::string()
                .description("The id of the Identity")
                .example("123abcd.git"),
        );
        properties.insert("metadata".into(), identity::Metadata::document());
        properties.insert(
            "registered".into(),
            document::string()
                .description("ID of the user on the Registry")
                .example("cloudhead")
                .nullable(true),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("cloudhead@123abcd.git"),
        );

        document::DocumentedType::from(properties).description("Unique identity")
    }
}

impl ToDocumentedType for identity::Metadata {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "handle".into(),
            document::string()
                .description("User chosen nickname")
                .example("cloudhead"),
        );
        document::DocumentedType::from(properties)
            .description("User provided metadata attached to the Identity")
    }
}

#[allow(clippy::non_ascii_literal)]
impl ToDocumentedType for avatar::Avatar {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert("background".into(), avatar::Color::document());
        properties.insert(
            "emoji".into(),
            document::string()
                .description("String containing the actual emoji codepoint to display")
                .example("🐽"),
        );

        document::DocumentedType::from(properties)
            .description("Generated avatar based on unique information")
    }
}

impl ToDocumentedType for avatar::Color {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "r".into(),
            document::string().description("Red value").example(122),
        );
        properties.insert(
            "g".into(),
            document::string().description("Green value").example(112),
        );
        properties.insert(
            "b".into(),
            document::string()
                .description("Blue value".to_string())
                .example(90),
        );

        document::DocumentedType::from(properties).description("RGB color")
    }
}

// TODO(xla): Implement Deserialize on identity::Metadata and drop this type entirely, this will
// help to avoid duplicate efforts for documentation.
/// Bundled input data for identity creation.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Handle the user wants to go by.
    handle: String,
}

impl ToDocumentedType for CreateInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "handle".into(),
            document::string()
                .description("User chosen nickname")
                .example("cloudhead"),
        );
        document::DocumentedType::from(properties)
            .description("User provided metadata attached to the Identity")
    }
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::test::request;

    use librad::paths;

    use crate::avatar;
    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::keystore;
    use crate::registry;
    use crate::session;

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(tmp_dir.path())?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut keystore = keystore::Keystorage::new(&paths, pw);
        let key = keystore.init_librad_key()?;

        let config = coco::config::default(key, tmp_dir.path())?;
        let peer = Arc::new(Mutex::new(coco::create_peer_api(config).await?));
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let api = super::filters(
            Arc::clone(&peer),
            Arc::new(RwLock::new(keystore)),
            Arc::clone(&registry),
            Arc::clone(&store),
        );

        let res = request()
            .method("POST")
            .path("/identities")
            .json(&super::CreateInput {
                handle: "cloudhead".into(),
            })
            .reply(&api)
            .await;

        let store = &*store.read().await;
        let registry = &*registry.read().await;
        let session = session::current(peer, store, registry).await?;
        let urn = session.identity.expect("failed to set identity").id;

        http::test::assert_response(&res, StatusCode::CREATED, |have| {
            let avatar = avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity);
            let shareable_entity_identifier = format!("cloudhead@{}", urn);
            assert_eq!(
                have,
                json!({
                    "avatarFallback": avatar,
                    "id": urn,
                    "metadata": {
                        "handle": "cloudhead",
                    },
                    "registered": Value::Null,
                    "shareableEntityIdentifier": &shareable_entity_identifier
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let paths = paths::Paths::from_root(tmp_dir.path())?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut keystore = keystore::Keystorage::new(&paths, pw);
        let key = keystore.init_librad_key()?;

        let config = coco::config::default(key.clone(), tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

        let user = coco::init_user(&peer, key, "cloudhead")?;
        let urn = user.urn();
        let handle = user.name().to_string();
        let shareable_entity_identifier = user.into();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(keystore)),
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let res = request()
            .method("GET")
            .path(&format!("/identities/{}", urn))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(identity::Identity {
                id: urn.clone(),
                shareable_entity_identifier,
                metadata: identity::Metadata { handle },
                registered: None,
                avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
            })
        );

        Ok(())
    }
}
