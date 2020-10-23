//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use coco::git;

use crate::context;

/// Combination of all control filters.
pub fn filters(
    ctx: context::Context,
    selfdestruct: mpsc::Sender<()>,
) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx)
        .or(reset_filter(selfdestruct))
        .boxed()
}

/// POST /create-project
fn create_project_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .and(super::with_context(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /reset
fn reset_filter(
    selfdestruct: mpsc::Sender<()>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("reset")
        .map(move || selfdestruct.clone())
        .and_then(handler::reset)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use tokio::sync::mpsc;
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use coco::user;

    use crate::{context, error, project};

    /// Create a project from the fixture repo.
    #[allow(clippy::let_underscore_must_use)]
    pub async fn create_project(
        ctx: context::Context,
        owner: user::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let meta = coco::control::replicate_platinum(
            &ctx.state,
            &owner,
            &input.name,
            &input.description,
            input.default_branch,
        )
        .await
        .map_err(error::Error::from)?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ = coco::control::track_fake_peer(&ctx.state, &meta, &user_handle);
            }
        }
        let branch = ctx
            .state
            .get_branch(meta.urn(), None, None)
            .await
            .map_err(error::Error::from)?;
        let stats = ctx
            .state
            .with_browser(branch, |browser| Ok(browser.get_stats()?))
            .await
            .map_err(error::Error::from)?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Abort the server task, which causes `main` to restart it.
    pub async fn reset(mut notify: mpsc::Sender<()>) -> Result<impl Reply, Rejection> {
        log::warn!("reload requested");
        Ok(reply::json(&notify.send(()).await.is_ok()))
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
    default_branch: git::ext::OneLevel,
    /// Create and track fake peers
    fake_peers: Option<Vec<String>>,
}
