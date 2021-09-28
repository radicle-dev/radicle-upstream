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

    /// Fetch the [`session::Session`].
    #[allow(clippy::unused_async)]
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        let sess =
            crate::session::get_current(ctx.store())?.ok_or(http::error::Routing::NoSession)?;
        match ctx {
            context::Context::Unsealed(_) => Ok(reply::json(&sess)),
            context::Context::Sealed(_) => Err(Rejection::from(error::Error::KeystoreSealed)),
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn get_seeds(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let seeds = match session::get_current(&ctx.store)? {
            None => vec![],
            Some(session) => session.settings.coco.seeds,
        };

        Ok(reply::json(&seeds))
    }

    #[allow(clippy::unused_async)]
    pub async fn update_seeds(
        ctx: context::Unsealed,
        seeds: Vec<String>,
    ) -> Result<impl Reply, Rejection> {
        session::update_seeds(&ctx.store, seeds)?;

        Ok(warp::reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::{http::StatusCode, test::request};

    use crate::{context, session};

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        let session = session::initialize_test(&ctx, "xla").await;

        let res = request().method("GET").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::OK);

        let session_response = serde_json::from_slice::<session::Session>(res.body())?;

        assert_eq!(session_response, session);

        Ok(())
    }
}
