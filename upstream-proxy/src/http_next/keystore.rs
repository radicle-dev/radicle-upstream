// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use axum::response::IntoResponse as _;

/// Provides the following endpoints
/// * `POST /keystore/unseal
/// * `POST /keystore
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/keystore", axum::routing::post(create))
        .route("/keystore/unseal", axum::routing::post(unseal))
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct UnsealBody {
    /// Passphrase to unlock the keystore.
    passphrase: crate::keystore::SecUtf8,
}

async fn unseal(
    mut ctx: axum::extract::Extension<crate::context::Context>,
    body: axum::extract::Json<UnsealBody>,
) -> Result<axum::response::Response, super::Error> {
    match ctx.0.unseal_keystore(body.0.passphrase).await {
        Ok(()) => Ok(http::StatusCode::NO_CONTENT.into_response()),
        Err(err) => {
            if err.is_invalid_passphrase() {
                Ok(crate::http::error::Response {
                    status_code: http::StatusCode::FORBIDDEN,
                    variant: "INCORRECT_PASSPHRASE",
                    message: "That\u{2019}s the wrong passphrase.".to_string(),
                }
                .into_response())
            } else {
                Err(err.into())
            }
        },
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CreateBody {
    /// Passphrase to encrypt the keystore with.
    passphrase: crate::keystore::SecUtf8,
}

async fn create(
    mut ctx: axum::extract::Extension<crate::context::Context>,
    body: axum::extract::Json<CreateBody>,
) -> Result<axum::response::Response, super::Error> {
    match ctx.0.create_key(body.0.passphrase).await {
        Ok(()) => Ok(http::StatusCode::CREATED.into_response()),
        Err(err) => {
            if err.is_key_exists() {
                Ok(crate::http::error::Response {
                    status_code: http::StatusCode::CONFLICT,
                    variant: "KEY_EXISTS",
                    message: "A key already exists".to_string(),
                }
                .into_response())
            } else {
                Err(err.into())
            }
        },
    }
}
