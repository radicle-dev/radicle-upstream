//! Recovery and conversion of [`error::Error`] to proper JSON responses, which expose variants
//! for API consumers to act on.

use librad::surf;
use serde::Serialize;
use std::convert::Infallible;
use warp::document::{self, ToDocumentedType};
use warp::http::StatusCode;
use warp::{reject, reply, Rejection, Reply};

use crate::error;

impl reject::Reject for error::Error {}

impl From<error::Error> for Rejection {
    fn from(err: error::Error) -> Self {
        reject::custom(err)
    }
}

/// Error type to carry context for failed requests.
#[derive(Serialize)]
pub struct Error {
    /// Human readable message to convery error case.
    pub message: String,
    /// The triggered error variant.
    pub variant: String,
}

impl ToDocumentedType for Error {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert(
            "message".into(),
            document::string()
                .description("Human readable description of the error case")
                .example("Malformed ID, supported charactes: [a-zA-Z0-9]"),
        );
        properties.insert(
            "variant".into(),
            document::string()
                .description("Enum of the error for the client to vary on")
                .example("INVALID_ID"),
        );
        properties.into()
    }
}

/// Handler to convert [`error::Error`] to [`Error`] response.
pub async fn recover(err: Rejection) -> Result<impl Reply, Infallible> {
    log::error!("{:?}", err);

    let (code, variant, message) = {
        if err.is_not_found() {
            (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            )
        } else if let Some(err) = err.find::<error::Error>() {
            match err {
                error::Error::Git(git_error) => match git_error {
                    surf::git::error::Error::Git(error) => (
                        StatusCode::BAD_REQUEST,
                        "GIT_ERROR",
                        format!("Internal Git error: {:?}", error),
                    ),
                    _ => (
                        StatusCode::BAD_REQUEST,
                        "BAD_REQUEST",
                        "Incorrect input".to_string(),
                    ),
                },
                _ => {
                    // TODO(xla): Match all variants and properly transform similar to
                    // gaphql::error.
                    (
                        StatusCode::BAD_REQUEST,
                        "BAD_REQUEST",
                        "Incorrect input".to_string(),
                    )
                },
            }
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Something went wrong".to_string(),
            )
        }
    };
    let res = reply::json(&Error {
        message,
        variant: variant.to_string(),
    });

    Ok(reply::with_header(
        reply::with_status(res, code),
        "content-type",
        "application/json",
    ))
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod tests {
    use futures::stream::TryStreamExt;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::reply::Reply as _;
    use warp::Rejection;

    #[tokio::test]
    async fn recover_custom() {
        let have: Value = response(warp::reject::custom(
            crate::error::Error::InordinateString32(),
        ))
        .await;
        let want = json!({
            "message": "Incorrect input",
            "variant": "BAD_REQUEST",
        });

        assert_eq!(have, want);
    }

    #[tokio::test]
    async fn recover_not_found() {
        let have: Value = response(warp::reject::not_found()).await;
        let want = json!({
            "message": "Resource not found",
            "variant": "NOT_FOUND",
        });

        assert_eq!(have, want);
    }

    async fn response(err: Rejection) -> Value {
        let res = super::recover(err).await.unwrap();

        let body = res
            .into_response()
            .body_mut()
            .try_fold(Vec::new(), |mut data, chunk| async move {
                data.extend_from_slice(&chunk);
                Ok(data)
            })
            .await
            .unwrap();

        serde_json::from_slice(&body).unwrap()
    }
}
