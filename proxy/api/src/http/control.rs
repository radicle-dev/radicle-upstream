//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::{filters::BoxedFilter, path, reject, Filter, Rejection, Reply};

use crate::context;

/// Combination of all control filters.
pub fn filters(
    ctx: context::Context,
    selfdestruct: mpsc::Sender<()>,
    enable_fixture_creation: bool,
) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx, enable_fixture_creation)
        .or(reload_filter(selfdestruct))
        .boxed()
}

/// POST /create-project
fn create_project_filter(
    ctx: context::Context,
    enabled: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .map(move || enabled)
        .and_then(|enable| async move {
            if enable {
                Ok(())
            } else {
                Err(reject::not_found())
            }
        })
        .untuple_one()
        .and(super::with_context(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /reload
fn reload_filter(
    selfdestruct: mpsc::Sender<()>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("reload")
        .map(move || selfdestruct.clone())
        .and_then(handler::reload)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use tokio::sync::mpsc;
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use coco::user;

    use crate::{context, error, http, project};

    /// Create a project from the fixture repo.
    #[allow(clippy::let_underscore_must_use)]
    pub async fn create_project(
        ctx: context::Context,
        owner: user::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let signer = ctx
            .signer
            .ok_or_else(|| http::error::Routing::SealedKeystore)?;
        let meta = coco::control::replicate_platinum(
            &ctx.state,
            &signer,
            &owner,
            &input.name,
            &input.description,
            &input.default_branch,
        )
        .await
        .map_err(error::Error::from)?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ = coco::control::track_fake_peer(&ctx.state, &signer, &meta, &user_handle);
            }
        }
        let stats = ctx
            .state
            .with_browser(meta.urn(), |browser| Ok(browser.get_stats()?))
            .await
            .map_err(error::Error::from)?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Abort the server task, which causes `main` to restart it.
    pub async fn reload(mut notify: mpsc::Sender<()>) -> Result<impl Reply, Rejection> {
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
    default_branch: String,
    /// Create and track fake peers
    fake_peers: Option<Vec<String>>,
}
