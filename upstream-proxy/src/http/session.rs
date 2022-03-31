// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints and serialisation for [`crate::session::Session`] related types.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all session filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_filter(ctx.clone()).or(get_filter(ctx)).boxed()
}

/// `GET /`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::get())
        .and(http::with_context(ctx))
        .and_then(handler::get)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, http};

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Session {
        pub identity: crate::identity::Identity,
    }

    #[allow(clippy::unused_async)]
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        match ctx {
            context::Context::Unsealed(ctx) => {
                let person = ctx
                    .peer
                    .librad_peer()
                    .using_storage(lnk_identities::local::default)
                    .await
                    .expect("failed to get storage")
                    .expect("failed to get local identity");

                Ok(reply::json(&Session {
                    identity: (
                        ctx.peer.librad_peer().peer_id(),
                        person.into_inner().into_inner(),
                    )
                        .into(),
                }))
            },
            context::Context::Sealed(ctx) => {
                if ctx.keystore.has_key() {
                    Err(Rejection::from(error::Error::KeystoreSealed))
                } else {
                    Err(Rejection::from(http::error::Response {
                        status_code: StatusCode::NOT_FOUND,
                        variant: "NO_FOUND",
                        message: "no locale identity found".to_string(),
                    }))
                }
            },
        }
    }
}
