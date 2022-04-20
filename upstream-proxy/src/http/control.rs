// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::context;

/// Combination of all control filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx.clone())
        .or(seal_filter(ctx.clone()))
        .or(reset_filter(ctx))
        .boxed()
}

/// POST /create-project
fn create_project_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .and(warp::post())
        .and(super::with_context_unsealed(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /reset
fn reset_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("reset")
        .and(warp::get())
        .and(super::with_context(ctx))
        .and_then(handler::reset)
}

/// GET /seal
fn seal_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("seal")
        .and(warp::get())
        .and(super::with_context(ctx))
        .and_then(handler::seal)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, project};

    /// Create a project from the fixture repo.
    pub async fn create_project(
        ctx: context::Unsealed,
        owner: crate::daemon::LocalIdentity,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let meta = crate::control::replicate_platinum(
            &ctx.peer,
            &owner,
            &input.name,
            &input.description,
            input.default_branch,
        )
        .await
        .map_err(error::Error::from)?;

        let seed = ctx.git_fetch.get_seed(meta.urn().id);
        let project = project::get(&ctx.peer, meta.urn(), seed)
            .await
            .map_err(error::Error::from)?;

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Abort the server task, which causes `main` to restart it.
    pub async fn reset(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        tracing::info!("reload requested");
        ctx.service_handle().reset();
        Ok(reply::json(&()))
    }

    /// Seals the keystore.
    pub async fn seal(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        tracing::info!("keystore seal requested");
        ctx.service_handle().seal();
        Ok(reply::with_status("keystore sealed", StatusCode::OK))
    }
}

/// Inputs for project creation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Name of the project.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: radicle_git_ext::OneLevel,
}
