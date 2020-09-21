//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::context;

/// Combination of all control filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx)
}

/// POST /create-project
fn create_project_filter(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    path!("create-project")
        .and(super::with_context(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
        .boxed()
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
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
            &ctx.signer,
            &owner,
            &input.name,
            &input.description,
            &input.default_branch,
        )
        .await
        .map_err(error::Error::from)?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ =
                    coco::control::track_fake_peer(&ctx.state, &ctx.signer, &meta, &user_handle);
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::{http::StatusCode, test::request};

    use crate::{context, error, http};

    // TODO(xla): This can't hold true anymore, given that we nuke the owner. Which is required in
    // order to register a project. Should we rework the test? How do we make sure an owner is
    // present?
    #[ignore]
    #[tokio::test]
    async fn create_project_after_nuke() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx);

        // Create project before nuke.
        let res = request()
            .method("POST")
            .path("/create-project")
            .json(&super::CreateInput {
                name: "Monadic".into(),
                description: "blabla".into(),
                default_branch: "master".into(),
                fake_peers: None,
            })
            .reply(&api)
            .await;
        http::test::assert_response(&res, StatusCode::CREATED, |_have| {});

        // Reset state.
        let res = request().method("GET").path("/nuke/coco").reply(&api).await;
        assert_eq!(res.status(), StatusCode::OK);

        let res = request()
            .method("POST")
            .path("/create-project")
            .json(&super::CreateInput {
                name: "Monadic".into(),
                description: "blabla".into(),
                default_branch: "master".into(),
                fake_peers: None,
            })
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::CREATED, |_have| {});

        Ok(())
    }
}
