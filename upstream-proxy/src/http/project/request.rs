// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints for project search requests.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use link_identities::git::Urn;

use crate::{context, http};

/// Combination of all routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    cancel_filter(ctx.clone())
        .or(create_filter(ctx.clone()))
        .or(list_filter(ctx))
        .boxed()
}

/// `DELETE /<urn>`
fn cancel_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path::end())
        .and(http::with_context_unsealed(ctx))
        .and(warp::delete())
        .and_then(handler::cancel)
}

/// `PUT /<urn>`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path::end())
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::create)
}

/// `GET /`
fn list_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::list)
}

/// Request handlers for initiating searches for projects on the network.
mod handler {
    use std::time::SystemTime;

    use warp::{http::StatusCode, reply, Rejection, Reply};

    use link_identities::git::Urn;

    use crate::{context, error};

    /// Abort search for an ongoing request.
    pub async fn cancel(urn: Urn, mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        ctx.peer
            .daemon_control()
            .cancel_project_request(&urn, SystemTime::now())
            .await
            .map_err(error::Error::from)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Kick off a network request for the [`crate::project::Project`] of the given `id`.
    ///
    /// FIXME(xla): Endpoint ought to return `201` if the request was newly created, otherwise
    /// `200` if there was a request present for the urn.
    pub async fn create(urn: Urn, mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let request = ctx
            .peer
            .daemon_control()
            .request_project(&urn, SystemTime::now())
            .await;

        ctx.git_fetch.add(urn.id).await;

        Ok(reply::json(&request))
    }

    /// List all project requests the current user has issued.
    pub async fn list(mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let requests = ctx.peer.daemon_control().get_project_requests().await;

        Ok(reply::json(&requests))
    }
}
