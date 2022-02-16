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
    pub fn new(
        status_code: StatusCode,
        variant: &'static str,
        err: &impl std::error::Error,
    ) -> Self {
        Self {
            status_code,
            variant,
            message: err.to_string(),
        }
    }

    pub fn internal_server_error(err: &impl std::error::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            variant: "INTERNAL_SERVER_ERROR",
            message: err.to_string(),
        }
    }
}

impl warp::reject::Reject for Response {}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::json!({
            "message": self.message,
            "variant": self.variant,
        });

        (self.status_code, axum::response::Json(body)).into_response()
    }
}

pub async fn recover(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let error_response = if err.is_not_found() {
        Response {
            status_code: StatusCode::NOT_FOUND,
            variant: "NOT_FOUND",
            message: "Resource not found".to_string(),
        }
    } else if let Some(err) = err.find::<error::Error>() {
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

    if error_response.status_code == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(message = %error_response.message, variant = %error_response.variant, ?err, "internal server error");
    }

    let body = serde_json::json!({
        "message": error_response.message,
        "variant": error_response.variant,
    });

    Ok(warp::reply::with_status(
        warp::reply::json(&body),
        error_response.status_code,
    ))
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

impl From<&crate::daemon::state::Error> for Response {
    fn from(err: &crate::daemon::state::Error) -> Self {
        let (status_code, variant, message) = match err {
            crate::daemon::state::Error::Checkout(checkout_error) => match checkout_error {
                crate::daemon::project::checkout::Error::AlreadExists(_) => (
                    StatusCode::CONFLICT,
                    "PATH_EXISTS",
                    checkout_error.to_string(),
                ),
                crate::daemon::project::checkout::Error::Git(git_error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "GIT_ERROR",
                    git_error.message().to_string(),
                ),
                crate::daemon::project::checkout::Error::Include(include_error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    include_error.to_string(),
                ),
                crate::daemon::project::checkout::Error::Io(io) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    io.to_string(),
                ),
                crate::daemon::project::checkout::Error::Transport(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "TRANSPORT_ERROR",
                    err.to_string(),
                ),
                crate::daemon::project::checkout::Error::Prefix(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PREFIX_ERROR",
                    err.to_string(),
                ),
            },
            crate::daemon::state::Error::Create(
                crate::daemon::project::create::Error::Validation(err),
            ) => match err {
                crate::daemon::project::create::validation::Error::AlreadExists(_) => {
                    (StatusCode::CONFLICT, "PATH_EXISTS", err.to_string())
                },
                crate::daemon::project::create::validation::Error::EmptyExistingPath(_) => {
                    (StatusCode::BAD_REQUEST, "EMPTY_PATH", err.to_string())
                },
                crate::daemon::project::create::validation::Error::Git(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "GIT_ERROR",
                    err.to_string(),
                ),
                crate::daemon::project::create::validation::Error::MissingDefaultBranch {
                    ..
                } => (
                    StatusCode::BAD_REQUEST,
                    "MISSING_DEFAULT_BRANCH",
                    err.to_string(),
                ),
                crate::daemon::project::create::validation::Error::PathDoesNotExist(_) => (
                    StatusCode::NOT_FOUND,
                    "PATH_DOES_NOT_EXIST",
                    err.to_string(),
                ),
                crate::daemon::project::create::validation::Error::NotARepo(_) => {
                    (StatusCode::BAD_REQUEST, "NOT_A_REPO", err.to_string())
                },
                crate::daemon::project::create::validation::Error::Io(err) => {
                    (StatusCode::BAD_REQUEST, "IO_ERROR", err.to_string())
                },
                crate::daemon::project::create::validation::Error::UrlMismatch { .. } => {
                    (StatusCode::BAD_REQUEST, "URL_MISMATCH", err.to_string())
                },

                crate::daemon::project::create::validation::Error::Transport(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "TRANSPORT_ERROR",
                    err.to_string(),
                ),
                crate::daemon::project::create::validation::Error::Remote(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MISSING_REMOTE",
                    err.to_string(),
                ),
            },
            crate::daemon::state::Error::Git(git_error) => (
                StatusCode::BAD_REQUEST,
                "GIT_ERROR",
                format!("Internal Git error: {:?}", git_error),
            ),
            crate::daemon::state::Error::MissingOwner => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", err.to_string())
            },
            crate::daemon::state::Error::Storage(
                crate::daemon::state::error::storage::Error::Blob(
                    crate::daemon::state::error::blob::Error::NotFound(_),
                ),
            ) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "entity not found".to_string(),
            ),
            crate::daemon::state::Error::IdentityExists(_) => {
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
            error::Error::KeystoreSealed => Self {
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
            error::Error::OpenReadOnlyGitStorage(_)
            | error::Error::Peer(_)
            | error::Error::Io(_)
            | error::Error::Store(_)
            | error::Error::WaitingRoom(_) => Self::internal_server_error(err),
            error::Error::Other(err) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                variant: "INTERNAL_SERVER_ERROR",
                message: format!("{:?}", err),
            },
        }
    }
}

impl From<lnk_identities::person::Error> for Response {
    fn from(err: lnk_identities::person::Error) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "IDENTITIES_PERSON_ERROR",
            &err,
        )
    }
}
