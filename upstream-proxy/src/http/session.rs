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
    get_filter(ctx.clone())
        .or(get_seeds_filter(ctx.clone()))
        .or(put_seeds_filter(ctx))
        .boxed()
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

/// `GET /seeds`
fn get_seeds_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("seeds")
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get_seeds)
}

/// `PUT /seeds`
fn put_seeds_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("seeds")
        .and(path::end())
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::update_seeds)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, http, session};

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Session {
        pub identity: crate::identity::Identity,
    }

    /// Fetch the [`session::Session`].
    #[allow(clippy::unused_async)]
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        match ctx {
            context::Context::Unsealed(ctx) => {
                let person = ctx
                    .peer
                    .librad_peer()
                    .using_storage(rad_identities::local::default)
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

    #[allow(clippy::unused_async)]
    pub async fn get_seeds(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let seeds = if let Some(seeds) = ctx.rest.seeds {
            seeds
        } else {
            match session::seeds(&ctx.rest.store)? {
                None => vec![],
                Some(seeds) => seeds,
            }
        };

        Ok(reply::json(&seeds))
    }

    #[allow(clippy::unused_async)]
    pub async fn update_seeds(
        ctx: context::Unsealed,
        seeds: Vec<String>,
    ) -> Result<impl Reply, Rejection> {
        if ctx.rest.seeds.is_none() {
            session::update_seeds(&ctx.rest.store, seeds)?;
            Ok(warp::reply::with_status(reply(), StatusCode::NO_CONTENT))
        } else {
            Err(http::error::Response {
                status_code: StatusCode::BAD_REQUEST,
                variant: "STATIC_SEEDS",
                message: "Seeds are statically configured and cannot be changed".to_string(),
            }
            .into())
        }
    }
}
