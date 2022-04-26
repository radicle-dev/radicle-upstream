// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;

/// Provides the following endpoint:
/// * `GET /session` Returns information about the current identity if initialized.
pub fn router() -> axum::Router {
    axum::Router::new().route("/session", axum::routing::get(get))
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Session {
    pub identity: crate::identity::Identity,
}

async fn get(
    ctx: axum::Extension<crate::context::Context>,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    match ctx.0 {
        crate::context::Context::Unsealed(ctx) => {
            let person = ctx
                .peer
                .librad_peer()
                .using_storage(lnk_identities::local::default)
                .await
                .context("failed to get storage")?
                .context("failed to get local identity")?;

            Ok(axum::response::Json(Session {
                identity: (
                    ctx.peer.librad_peer().peer_id(),
                    person.into_inner().into_inner(),
                )
                    .into(),
            }))
        },
        crate::context::Context::Sealed(ctx) => {
            if ctx.keystore.has_key() {
                Err(super::Error::Custom {
                    status_code: http::StatusCode::FORBIDDEN,
                    variant: "FORBIDDEN",
                    message: "keystore is sealed".to_string(),
                    details: None,
                })
            } else {
                Err(super::Error::Custom {
                    status_code: http::StatusCode::NOT_FOUND,
                    variant: "NO_FOUND",
                    message: "no locale identity found".to_string(),
                    details: None,
                })
            }
        },
    }
}
