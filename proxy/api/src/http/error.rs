//! Recovery and conversion of [`error::Error`] to proper JSON responses, which expose variants
//! for API consumers to act on.

use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

use coco::{project::create, state};

use crate::error;

/// HTTP layer specific rejections.
#[derive(Debug, thiserror::Error)]
pub enum Routing {
    /// The keystore is sealed, context does not have a signer.
    #[error("no session has been created yet")]
    NoSession,
    /// Query part of the URL cannot be deserialized.
    ///
    /// Used by [`crate::http::with_qs`] and [`crate::http::with_qs_opt`].
    #[error("invalid query string \"{query}\": {error}")]
    InvalidQuery {
        /// The original query string
        query: String,
        /// Error message describing the deserialization error.
        // We can’t use `serde_qs::Error` here because it is not `Sync` which is
        // required to implement `reject::Reject`. Instead we
        error: String,
    },
    /// A query string is required but missing
    ///
    /// Used by [`crate::http::with_qs`].
    #[error("required query string is missing")]
    QueryMissing,
}

impl reject::Reject for Routing {}

impl reject::Reject for error::Error {}

/// Error type to carry context for failed requests.
#[derive(Serialize)]
pub struct Error {
    /// Human readable message to convery error case.
    pub message: String,
    /// The triggered error variant.
    pub variant: String,
}

/// Handler to convert [`error::Error`] to [`Error`] response.
#[allow(clippy::too_many_lines)]
pub async fn recover(err: Rejection) -> Result<impl Reply, Infallible> {
    log::error!("{:?}", err);

    let (code, variant, message) = {
        if err.is_not_found() {
            (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            )
        } else if let Some(err) = err.find::<Routing>() {
            match err {
                Routing::NoSession => (StatusCode::NOT_FOUND, "NOT_FOUND", err.to_string()),
                Routing::InvalidQuery { .. } => {
                    (StatusCode::BAD_REQUEST, "INVALID_QUERY", err.to_string())
                },
                Routing::QueryMissing { .. } => {
                    (StatusCode::BAD_REQUEST, "QUERY_MISSING", err.to_string())
                },
            }
        } else if let Some(err) = err.find::<error::Error>() {
            match err {
                error::Error::State(err) => match err {
                    coco::state::Error::Checkout(checkout_error) => match checkout_error {
                        coco::project::checkout::Error::AlreadExists(_) => (
                            StatusCode::CONFLICT,
                            "PATH_EXISTS",
                            checkout_error.to_string(),
                        ),
                        coco::project::checkout::Error::Git(git_error) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "GIT_ERROR",
                            git_error.message().to_string(),
                        ),
                        coco::project::checkout::Error::Include(include_error) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "INTERNAL_ERROR",
                            include_error.to_string(),
                        ),
                        coco::project::checkout::Error::Io(io) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "INTERNAL_ERROR",
                            io.to_string(),
                        ),
                        coco::project::checkout::Error::Transport(err) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "TRANSPORT_ERROR",
                            err.to_string(),
                        ),
                        coco::project::checkout::Error::Prefix(err) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "PREFIX_ERROR",
                            err.to_string(),
                        ),
                    },
                    coco::state::Error::Create(create::Error::Validation(err)) => match err {
                        create::validation::Error::AlreadExists(_) => {
                            (StatusCode::CONFLICT, "PATH_EXISTS", err.to_string())
                        },
                        create::validation::Error::EmptyExistingPath(_) => {
                            (StatusCode::BAD_REQUEST, "EMPTY_PATH", err.to_string())
                        },
                        create::validation::Error::Git(_) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "GIT_ERROR",
                            err.to_string(),
                        ),
                        create::validation::Error::MissingAuthorEmail => (
                            StatusCode::BAD_REQUEST,
                            "MISSING_AUTHOR_EMAIL",
                            err.to_string(),
                        ),
                        create::validation::Error::MissingGitConfig => (
                            StatusCode::BAD_REQUEST,
                            "MISSING_GIT_CONFIG",
                            err.to_string(),
                        ),
                        create::validation::Error::MissingAuthorName => (
                            StatusCode::BAD_REQUEST,
                            "MISSING_AUTHOR_NAME",
                            err.to_string(),
                        ),
                        create::validation::Error::MissingDefaultBranch { .. } => (
                            StatusCode::BAD_REQUEST,
                            "MISSING_DEFAULT_BRANCH",
                            err.to_string(),
                        ),
                        create::validation::Error::MissingUrl => {
                            (StatusCode::BAD_REQUEST, "MISSING_URL", err.to_string())
                        },
                        create::validation::Error::PathDoesNotExist(_) => (
                            StatusCode::NOT_FOUND,
                            "PATH_DOES_NOT_EXIST",
                            err.to_string(),
                        ),
                        create::validation::Error::NotARepo(_) => {
                            (StatusCode::BAD_REQUEST, "NOT_A_REPO", err.to_string())
                        },
                        create::validation::Error::Io(err) => {
                            (StatusCode::BAD_REQUEST, "IO_ERROR", err.to_string())
                        },
                        create::validation::Error::UrlMismatch { .. } => {
                            (StatusCode::BAD_REQUEST, "URL_MISMATCH", err.to_string())
                        },

                        create::validation::Error::Transport(_) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "TRANSPORT_ERROR",
                            err.to_string(),
                        ),
                        create::validation::Error::Remote(_) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "MISSING_REMOTE",
                            err.to_string(),
                        ),
                    },
                    coco::state::Error::Git(git_error) => (
                        StatusCode::BAD_REQUEST,
                        "GIT_ERROR",
                        format!("Internal Git error: {:?}", git_error),
                    ),
                    coco::state::Error::MissingOwner => {
                        (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", err.to_string())
                    },
                    coco::state::Error::Source(coco::source::Error::Git(git_error)) => (
                        StatusCode::BAD_REQUEST,
                        "GIT_ERROR",
                        format!("Internal Git error: {}", git_error),
                    ),
                    coco::state::Error::Source(coco::source::Error::NoBranches) => (
                        StatusCode::BAD_REQUEST,
                        "GIT_ERROR",
                        coco::source::Error::NoBranches.to_string(),
                    ),
                    coco::state::Error::Source(coco::source::Error::PathNotFound(path)) => {
                        (StatusCode::NOT_FOUND, "NOT_FOUND", path.to_string())
                    },
                    coco::state::Error::Storage(state::error::storage::Error::Blob(
                        state::error::blob::Error::NotFound(_),
                    )) => (
                        StatusCode::NOT_FOUND,
                        "NOT_FOUND",
                        "entity not found".to_string(),
                    ),
                    coco::state::Error::IdentityExists(_) => {
                        (StatusCode::CONFLICT, "IDENTITY_EXISTS", err.to_string())
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
                },
                error::Error::Keystore(keystore_err) => {
                    if keystore_err.is_invalid_passphrase() {
                        (
                            StatusCode::FORBIDDEN,
                            "INCORRECT_PASSPHRASE",
                            "incorrect passphrase".to_string(),
                        )
                    } else if keystore_err.is_key_exists() {
                        (
                            StatusCode::CONFLICT,
                            "KEY_EXISTS",
                            "A key already exists".to_string(),
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "INTERNAL_SERVER_ERROR",
                            err.to_string(),
                        )
                    }
                },
                error::Error::KeystoreSealed
                | error::Error::WrongPassphrase
                | error::Error::InvalidAuthCookie => {
                    (StatusCode::FORBIDDEN, "FORBIDDEN", err.to_string())
                },
                error::Error::SessionInUse(_) => {
                    (StatusCode::BAD_REQUEST, "SESSION_IN_USE", err.to_string())
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use futures::stream::TryStreamExt;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::{reply::Reply as _, Rejection};

    #[tokio::test]
    async fn recover_custom() {
        let urn = coco::Urn::new(
            coco::git_ext::Oid::try_from("7ab8629dd6da14dcacde7f65b3d58cd291d7e235")
                .expect("failed to parse Oid"),
        );
        let message = format!("the current session is in use by `{}`", urn);
        let have: Value =
            response(warp::reject::custom(crate::error::Error::SessionInUse(urn))).await;
        let want = json!({
            "message": message,
            "variant": "SESSION_IN_USE"
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
