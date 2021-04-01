//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use coco::git_ext;

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
    use std::convert::TryFrom;

    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, project};

    /// Create a project from the fixture repo.
    #[allow(clippy::let_underscore_must_use)]
    pub async fn create_project(
        ctx: context::Unsealed,
        owner: coco::LocalIdentity,
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

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ = crate::control::track_fake_peer(&ctx.peer, &meta, &user_handle);
            }
        }
        let branch = coco::state::get_branch(&ctx.peer, meta.urn(), None, None)
            .await
            .map_err(error::Error::from)?;
        let stats =
            coco::state::with_browser(&ctx.peer, branch, |browser| Ok(browser.get_stats()?))
                .await
                .map_err(error::Error::from)?;
        let project = project::Full::try_from((meta, stats))?;

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Abort the server task, which causes `main` to restart it.
    pub async fn reset(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        log::warn!("reload requested");
        ctx.service_handle().reset();
        Ok(reply::json(&()))
    }

    /// Seals the keystore.
    pub async fn seal(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        log::warn!("keystore seal requested");
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
    default_branch: git_ext::OneLevel,
    /// Create and track fake peers
    fake_peers: Option<Vec<String>>,
}
