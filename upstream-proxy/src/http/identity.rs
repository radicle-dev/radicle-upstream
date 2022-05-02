// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Manage the state and stateful interactions with the underlying peer API of librad.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use link_identities::git::Urn;

use crate::{context, http};

/// Combination of all identity routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_remote_filter(ctx.clone())
        .or(create_filter(ctx.clone()))
        .or(update_filter(ctx))
        .boxed()
}

/// `POST /`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::post())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `PUT /`
fn update_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::update)
}

/// `GET /remote/<id>`
fn get_remote_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("remote")
        .and(path::param::<Urn>())
        .and(warp::path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get_remote)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use anyhow::Context;
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use link_identities::git::Urn;

    use crate::{context, http, identity, session};

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Unsealed,
        metadata: identity::Metadata,
    ) -> Result<impl Reply, Rejection> {
        if session::get_current(&ctx.rest.store)?.is_some() {
            return Err(http::error::Response {
                status_code: StatusCode::BAD_REQUEST,
                variant: "SESSION_IN_USE",
                message: "A session already exists".to_string(),
            }
            .into());
        }

        let id = identity::create(ctx.peer.librad_peer(), metadata).await?;

        session::initialize(&ctx.rest.store, &[])?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Update the [`identity::Identity`] metadata.
    pub async fn update(
        ctx: context::Unsealed,
        metadata: identity::Metadata,
    ) -> Result<impl Reply, Rejection> {
        let id = identity::update(ctx.peer.librad_peer(), metadata).await?;

        Ok(reply::with_status(reply::json(&id), StatusCode::OK))
    }

    /// Get the [`identity::Person`] for the given `id`.
    pub async fn get_remote(id: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let user = ctx
            .peer
            .librad_peer()
            .using_storage({
                let id = id.clone();
                move |storage| lnk_identities::person::get(&storage, &id)
            })
            .await
            .context("failed to get librad storage")
            .map_err(crate::error::Error::from)?
            .context(format!("failed to get person {id}"))
            .map_err(crate::error::Error::from)?;

        match user {
            Some(user) => Ok(reply::json(&identity::Person::from(user))),
            None => Err(http::error::Response {
                status_code: StatusCode::NOT_FOUND,
                variant: "NOT_FOUND",
                message: "Person not found".to_string(),
            }
            .into()),
        }
    }
}
