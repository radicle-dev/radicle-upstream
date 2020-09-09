//! Endpoints for Avatar.

use serde::Deserialize;
use warp::{filters::BoxedFilter, path, Filter, Reply};

/// `GET /<id>?usage=<usage>`
pub fn get_filter() -> BoxedFilter<(impl Reply,)> {
    warp::any()
        .and(warp::get())
        .and(path::param::<String>())
        .and(warp::filters::query::query::<GetAvatarQuery>())
        .and_then(handler::get)
        .boxed()
}

/// Avatar handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{avatar, http::error};

    /// Get the avatar for the given `id`.
    #[allow(clippy::wildcard_enum_match_arm)]
    pub async fn get(
        id: String,
        super::GetAvatarQuery { usage }: super::GetAvatarQuery,
    ) -> Result<impl Reply, Rejection> {
        let avatar = avatar::Avatar::from(
            &id,
            match usage.as_deref() {
                Some("identity") => avatar::Usage::Identity,
                Some("any") | None => avatar::Usage::Any,
                _ => {
                    return Ok(reply::with_status(
                        reply::json(&error::Error {
                            message: "BAD_REQUEST".to_string(),
                            variant: "Invalid query input".to_string(),
                        }),
                        StatusCode::BAD_REQUEST,
                    ))
                },
            },
        );

        Ok(reply::with_status(reply::json(&avatar), StatusCode::OK))
    }
}

/// Bundled query params to pass to the avatar handler.
#[derive(Debug, Deserialize)]
pub struct GetAvatarQuery {
    /// Kind of avatar usage.
    usage: Option<String>,
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::{http::StatusCode, test::request};

    #[tokio::test]
    async fn get() {
        let api = super::get_filter();
        let res = request()
            .method("GET")
            .path(&format!("/{}?usage={}", "monadic", "any"))
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
                "emoji": "🎮",
            })
        );
    }
}
