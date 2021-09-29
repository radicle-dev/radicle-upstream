// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Recovery and conversion of [`error::Error`] to proper JSON responses, which expose variants
//! for API consumers to act on.

use std::convert::Infallible;

use warp::http::StatusCode;

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

impl warp::reject::Reject for Routing {}

impl warp::reject::Reject for error::Error {}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: StatusCode,
    /// The triggered error variant.
    pub variant: &'static str,
    /// Human readable message to convery error case.
    pub message: String,
}

impl Response {
    pub fn internal_server_error(err: impl std::error::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            variant: "INTERNAL_SERVER_ERROR",
            message: err.to_string(),
        }
    }
}

impl warp::reject::Reject for Response {}

#[allow(clippy::unused_async)]
pub async fn recover(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    tracing::error!(?err, "request error");

    let error_response = if err.is_not_found() {
        Response {
            status_code: StatusCode::NOT_FOUND,
            variant: "NOT_FOUND",
            message: "Resource not found".to_string(),
        }
    } else if let Some(err) = err.find::<error::Error>() {
        Response::from(err)
    } else if let Some(err) = err.find::<Routing>() {
        Response::from(err)
    } else if let Some(err) = err.find::<Response>() {
        err.clone()
    } else {
        Response {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            variant: "INTERNAL_SERVER_ERROR",
            message: "Something went wrong".to_string(),
        }
    };

    let body = serde_json::json!({
        "message": error_response.message,
        "variant": error_response.variant,
    });

    Ok(warp::reply::with_status(
        warp::reply::json(&body),
        error_response.status_code,
    ))
}

impl From<&Routing> for Response {
    fn from(err: &Routing) -> Self {
        let (status_code, variant) = match err {
            Routing::NoSession => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Routing::InvalidQuery { .. } => (StatusCode::BAD_REQUEST, "INVALID_QUERY"),
            Routing::QueryMissing { .. } => (StatusCode::BAD_REQUEST, "QUERY_MISSING"),
        };
        Self {
            status_code,
            variant,
            message: err.to_string(),
        }
    }
}

impl From<&crate::keystore::Error> for Response {
    fn from(err: &crate::keystore::Error) -> Self {
        if err.is_invalid_passphrase() {
            Self {
                status_code: StatusCode::FORBIDDEN,
                variant: "INCORRECT_PASSPHRASE",
                message: "That\u{2019}s the wrong passphrase.".to_string(),
            }
        } else if err.is_key_exists() {
            Self {
                status_code: StatusCode::CONFLICT,
                variant: "KEY_EXISTS",
                message: "A key already exists".to_string(),
            }
        } else {
            Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                variant: "INTERNAL_SERVER_ERROR",
                message: err.to_string(),
            }
        }
    }
}

impl From<&radicle_source::error::Error> for Response {
    fn from(err: &radicle_source::error::Error) -> Self {
        let (status_code, variant) = match err {
            radicle_source::error::Error::Git(_) => (StatusCode::BAD_REQUEST, "GIT_ERROR"),
            radicle_source::error::Error::NoBranches => (StatusCode::BAD_REQUEST, "NO_BRANCHES"),
            radicle_source::error::Error::PathNotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };
        Self {
            status_code,
            variant,
            message: err.to_string(),
        }
    }
}

impl From<&radicle_daemon::state::Error> for Response {
    #[allow(clippy::too_many_lines)]
    fn from(err: &radicle_daemon::state::Error) -> Self {
        let (status_code, variant, message) = match err {
            radicle_daemon::state::Error::Checkout(checkout_error) => match checkout_error {
                radicle_daemon::project::checkout::Error::AlreadExists(_) => (
                    StatusCode::CONFLICT,
                    "PATH_EXISTS",
                    checkout_error.to_string(),
                ),
                radicle_daemon::project::checkout::Error::Git(git_error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "GIT_ERROR",
                    git_error.message().to_string(),
                ),
                radicle_daemon::project::checkout::Error::Include(include_error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    include_error.to_string(),
                ),
                radicle_daemon::project::checkout::Error::Io(io) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    io.to_string(),
                ),
                radicle_daemon::project::checkout::Error::Transport(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "TRANSPORT_ERROR",
                    err.to_string(),
                ),
                radicle_daemon::project::checkout::Error::Prefix(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PREFIX_ERROR",
                    err.to_string(),
                ),
            },
            radicle_daemon::state::Error::Create(
                radicle_daemon::project::create::Error::Validation(err),
            ) => match err {
                radicle_daemon::project::create::validation::Error::AlreadExists(_) => {
                    (StatusCode::CONFLICT, "PATH_EXISTS", err.to_string())
                },
                radicle_daemon::project::create::validation::Error::EmptyExistingPath(_) => {
                    (StatusCode::BAD_REQUEST, "EMPTY_PATH", err.to_string())
                },
                radicle_daemon::project::create::validation::Error::Git(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "GIT_ERROR",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::MissingAuthorEmail => (
                    StatusCode::BAD_REQUEST,
                    "MISSING_AUTHOR_EMAIL",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::MissingGitConfig => (
                    StatusCode::BAD_REQUEST,
                    "MISSING_GIT_CONFIG",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::MissingAuthorName => (
                    StatusCode::BAD_REQUEST,
                    "MISSING_AUTHOR_NAME",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::MissingDefaultBranch {
                    ..
                } => (
                    StatusCode::BAD_REQUEST,
                    "MISSING_DEFAULT_BRANCH",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::MissingUrl => {
                    (StatusCode::BAD_REQUEST, "MISSING_URL", err.to_string())
                },
                radicle_daemon::project::create::validation::Error::PathDoesNotExist(_) => (
                    StatusCode::NOT_FOUND,
                    "PATH_DOES_NOT_EXIST",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::NotARepo(_) => {
                    (StatusCode::BAD_REQUEST, "NOT_A_REPO", err.to_string())
                },
                radicle_daemon::project::create::validation::Error::Io(err) => {
                    (StatusCode::BAD_REQUEST, "IO_ERROR", err.to_string())
                },
                radicle_daemon::project::create::validation::Error::UrlMismatch { .. } => {
                    (StatusCode::BAD_REQUEST, "URL_MISMATCH", err.to_string())
                },

                radicle_daemon::project::create::validation::Error::Transport(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "TRANSPORT_ERROR",
                    err.to_string(),
                ),
                radicle_daemon::project::create::validation::Error::Remote(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MISSING_REMOTE",
                    err.to_string(),
                ),
            },
            radicle_daemon::state::Error::Git(git_error) => (
                StatusCode::BAD_REQUEST,
                "GIT_ERROR",
                format!("Internal Git error: {:?}", git_error),
            ),
            radicle_daemon::state::Error::MissingOwner => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", err.to_string())
            },
            radicle_daemon::state::Error::Storage(
                radicle_daemon::state::error::storage::Error::Blob(
                    radicle_daemon::state::error::blob::Error::NotFound(_),
                ),
            ) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "entity not found".to_string(),
            ),
            radicle_daemon::state::Error::IdentityExists(_) => {
                (StatusCode::CONFLICT, "IDENTITY_EXISTS", err.to_string())
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                err.to_string(),
            ),
        };
        Self {
            status_code,
            variant,
            message,
        }
    }
}

impl From<&error::Error> for Response {
    fn from(err: &error::Error) -> Self {
        match err {
            error::Error::State(err) => Self::from(err),
            error::Error::Source(err) => Self::from(err),
            error::Error::Keystore(keystore_err) => Self::from(keystore_err),
            error::Error::KeystoreSealed | error::Error::InvalidAuthCookie => Self {
                status_code: StatusCode::FORBIDDEN,
                variant: "FORBIDDEN",
                message: err.to_string(),
            },
            error::Error::ProjectNotFound => Self {
                status_code: StatusCode::NOT_FOUND,
                variant: "PROJECT_NOT_FOUND",
                message: "Project not found".to_string(),
            },
            error::Error::MissingDefaultBranch => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                variant: "MISSING_DEFAULT_BRANCH",
                message: "Default branch for project is missing".to_string(),
            },
            error::Error::Peer(_)
            | error::Error::Io(_)
            | error::Error::Store(_)
            | error::Error::WaitingRoom(_) => Self::internal_server_error(err),
        }
    }
}
