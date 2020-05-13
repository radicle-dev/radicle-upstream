//! Endpoints and serialisation for [`identity::Identity`] related types.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::http;
use crate::identity;
use crate::registry;

/// Combination of all identity routes.
pub fn filters<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_filter().or(create_filter(registry, store))
}

/// `POST /identities`
fn create_filter<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("identities")
        .and(warp::post())
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
fn get_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("identities")
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
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::avatar;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::registry;
    use crate::session;

    /// Create a new [`identity::Identity`].
    pub async fn create<R: registry::Client>(
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let store = store.read().await;

        if let Some(identity) = session::get((*reg).clone(), &store).await?.identity {
            return Err(Rejection::from(error::Error::IdentityExists(identity.id)));
        }

        let id = identity::create(input.handle, input.display_name, input.avatar_url)?;

        session::set(
            &store,
            session::Session {
                identity: Some(id.clone()),
                orgs: vec![],
            },
        )?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(id: String) -> Result<impl Reply, Rejection> {
        let id = identity::Identity {
            id: id.to_string(),
            shareable_entity_identifier: format!("cloudhead@{}", id),
            metadata: identity::Metadata {
                handle: "cloudhead".into(),
                display_name: Some("Alexis Sellier".into()),
                avatar_url: Some("https://avatars1.githubusercontent.com/u/40774".into()),
            },
            registered: None,
            avatar_fallback: avatar::Avatar::from(&id, avatar::Usage::Identity),
        };

        Ok(reply::json(&id))
    }
}

impl Serialize for identity::Identity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Identity", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field(
            "shareableEntityIdentifier",
            &self.shareable_entity_identifier,
        )?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field(
            "registered",
            &self.registered.as_ref().map(ToString::to_string),
        )?;
        state.serialize_field("avatarFallback", &self.avatar_fallback)?;

        state.end()
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
                .example("upstream://coco/identity/cloudhead@123abcd.git"),
        );

        document::DocumentedType::from(properties).description("Unique identity")
    }
}

impl Serialize for identity::Metadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Metadata", 3)?;
        state.serialize_field("handle", &self.handle)?;
        state.serialize_field("displayName", &self.display_name)?;
        state.serialize_field("avatarUrl", &self.avatar_url)?;

        state.end()
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
        properties.insert(
            "displayName".into(),
            document::string()
                .description("Long-form name to be displayed next to the Identity")
                .example("Alexis Sellier")
                .nullable(true),
        );
        properties.insert(
            "avatarUrl".into(),
            document::string()
                .description("Location of the image to shown as the avatar of the Idenityt")
                .example("https://avatars1.githubusercontent.com/u/40774")
                .nullable(true),
        );

        document::DocumentedType::from(properties)
            .description("User provided metadata attached to the Identity")
    }
}

impl Serialize for avatar::Avatar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Avatar", 2)?;
        state.serialize_field("background", &self.background)?;
        state.serialize_field("emoji", &self.emoji.to_string())?;

        state.end()
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

impl Serialize for avatar::Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;

        state.end()
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
    /// Name to be displayed for this identity.
    display_name: Option<String>,
    /// A url to an image that can be displayed for the identity.
    avatar_url: Option<String>,
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
        properties.insert(
            "displayName".into(),
            document::string()
                .description("Long-form name to be displayed next to the Identity")
                .example("Alexis Sellier")
                .nullable(true),
        );
        properties.insert(
            "avatarUrl".into(),
            document::string()
                .description("Location of the image to shown as the avatar of the Idenityt")
                .example("https://avatars1.githubusercontent.com/u/40774")
                .nullable(true),
        );

        document::DocumentedType::from(properties)
            .description("User provided metadata attached to the Identity")
    }
}

#[allow(clippy::non_ascii_literal, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::identity;
    use crate::registry;

    #[tokio::test]
    async fn create() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let api = super::filters(
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let res = request()
            .method("POST")
            .path("/identities")
            .json(&super::CreateInput {
                handle: "cloudhead".into(),
                display_name: Some("Alexis Sellier".into()),
                avatar_url: Some("https://avatars1.githubusercontent.com/u/40774".into()),
            })
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "avatarFallback": {
                "background": {
                    "r": 122,
                    "g": 112,
                    "b": 90,
                },
                "emoji": "🐽",
            },
            "id": "123abcd.git",
            "metadata": {
                "avatarUrl": "https://avatars1.githubusercontent.com/u/40774",
                "displayName": "Alexis Sellier",
                "handle": "cloudhead",
            },
            "registered": Value::Null,
            "shareableEntityIdentifier": "cloudhead@123abcd.git",
        });

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);
    }

    #[tokio::test]
    async fn get() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let api = super::filters(
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        let id = "123abcd.git";

        let res = request()
            .method("GET")
            .path(&format!("/identities/{}", id))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(identity::Identity {
                id: id.to_string(),
                shareable_entity_identifier: format!("cloudhead@{}", id.to_string()),
                metadata: identity::Metadata {
                    handle: "cloudhead".into(),
                    display_name: Some("Alexis Sellier".into()),
                    avatar_url: Some("https://avatars1.githubusercontent.com/u/40774".into()),
                },
                registered: None,
                avatar_fallback: avatar::Avatar::from(id, avatar::Usage::Identity),
            })
        );
    }
}
