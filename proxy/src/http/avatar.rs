//! Endpoints for Avatar.

use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;

/// `GET /avatars/<usage>/<id>`
pub fn get_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("avatars")
        .and(document::param::<String>(
            "usage",
            "Usage of the Avatar: org, identity, any",
        ))
        .and(document::param::<String>(
            "id",
            "ID for the Avatar creation",
        ))
        .and(warp::get())
        .and(document::document(document::description(
            "Return the avatar for the ID",
        )))
        .and(document::document(document::tag("Avatar")))
        .and(document::document(
            document::response(
                200,
                document::body(avatar::Avatar::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Avatar not found"),
        ))
        .and_then(handler::get)
}

/// Avatar handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{reject, reply, Rejection, Reply};

    use crate::avatar;

    /// Get the avatar for the given `id`.
    pub async fn get(usage: String, id: String) -> Result<impl Reply, Rejection> {
        let avatar = avatar::Avatar::from(
            &id,
            match usage.as_str() {
                "identity" => avatar::Usage::Identity,
                "org" => avatar::Usage::Org,
                "any" => avatar::Usage::Any,
                _ => return Err(reject::not_found()),
            },
        );

        Ok(reply::json(&avatar))
    }
}

#[allow(
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::non_ascii_literal
)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn get() {
        let api = super::get_filter();
        let res = request()
            .method("GET")
            .path(&format!("/avatars/{}/{}", "org", "monadic"))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "background": {
                    "r": 148,
                    "g": 187,
                    "b": 61,
                },
                "emoji": "☔️",
            })
        );
    }
}
