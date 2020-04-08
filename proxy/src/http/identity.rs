use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::identity;

/// Combination of identity all routes.
pub fn filters() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_filter().or(create_filter())
}

/// POST /identities
fn create_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("identities")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::create)
}

/// GET /identities/<id>
fn get_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("identities" / String)
        .and(warp::get())
        .and_then(handler::get)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::avatar;
    use crate::identity;

    /// Create a new [`identity::Identity`].
    pub async fn create(input: super::CreateInput) -> Result<impl Reply, Rejection> {
        let id = "123abcd.git";
        let res = reply::json(&identity::Identity {
            id: id.into(),
            shareable_entity_identifier: format!("{}@123abcd.git", input.handle),
            metadata: identity::Metadata {
                handle: input.handle,
                display_name: input.display_name,
                avatar_url: input.avatar_url,
            },
            registered: None,
            avatar_fallback: avatar::Avatar::from(id, avatar::Usage::Identity),
        });

        Ok(reply::with_status(res, StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(id: String) -> Result<impl Reply, Rejection> {
        let id = identity::Identity {
            id: id.to_string(),
            shareable_entity_identifier: format!("cloudhead@{}", id.to_string()),
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

#[allow(clippy::non_ascii_literal, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::identity;

    #[tokio::test]
    async fn create() {
        let api = super::filters();
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
                "emoji": "💡",
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
        let id = "123abcd.git";

        let api = super::filters();
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
