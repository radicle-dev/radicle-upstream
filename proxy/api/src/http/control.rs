//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::context;

/// Combination of all control filters.
pub fn filters(ctx: context::Ctx) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx.clone())
        .or(nuke_coco_filter(ctx))
        .boxed()
}

/// POST /create-project
fn create_project_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .and(super::with_context(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /nuke/coco
fn nuke_coco_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("nuke" / "coco")
        .and(super::with_context(ctx))
        .and_then(handler::nuke_coco)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use coco::keystore;
    use coco::signer;

    use crate::context;
    use crate::error;
    use crate::project;

    /// Create a project from the fixture repo.
    #[allow(clippy::let_underscore_must_use)]
    pub async fn create_project(
        ctx: context::Ctx,
        owner: coco::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let meta = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
            &owner,
            &input.name,
            &input.description,
            &input.default_branch,
        )
        .map_err(error::Error::from)?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ =
                    coco::control::track_fake_peer(&ctx.peer_api, &ctx.signer, &meta, &user_handle);
            }
        }
        let stats = ctx
            .peer_api
            .with_browser(&meta.urn(), |browser| Ok(browser.get_stats()?))
            .map_err(error::Error::from)?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Reset the coco state.
    pub async fn nuke_coco(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        // TmpDir deletes the temporary directory once it DROPS.
        // This means our new directory goes missing, and future calls will fail.
        // The Peer creates the directory again.
        //
        // N.B. this may gather lot's of tmp files on your system. We're sorry.
        let tmp_path = {
            let temp_dir = tempfile::tempdir().expect("test dir creation failed");
            log::debug!("New temporary path is: {:?}", temp_dir.path());
            std::env::set_var("RAD_HOME", temp_dir.path());
            temp_dir.path().to_path_buf()
        };

        let paths = coco::Paths::from_root(tmp_path).map_err(error::Error::from)?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut new_keystore = keystore::Keystorage::new(&paths, pw);
        let key = new_keystore.init().map_err(error::Error::from)?;
        let signer = signer::BoxedSigner::new(signer::SomeSigner {
            signer: key.clone(),
        });

        let config = coco::config::configure(paths, key, *coco::config::LOCALHOST_ANY, vec![]);
        let new_peer_api = coco::Api::new(config).await.map_err(error::Error::from)?;

        let mut ctx = ctx.write().await;
        ctx.peer_api = new_peer_api;
        ctx.signer = signer;

        Ok(reply::json(&true))
    }

    #[allow(clippy::unwrap_used, clippy::panic)]
    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_ne;

        use crate::context;
        use crate::error;

        #[tokio::test]
        async fn nuke_coco() -> Result<(), error::Error> {
            let tmp_dir = tempfile::tempdir()?;
            let ctx = context::Context::tmp(&tmp_dir).await?;

            let (old_paths, old_peer_id) = {
                let ctx = ctx.read().await;
                (ctx.peer_api.paths(), ctx.peer_api.peer_id())
            };

            super::nuke_coco(ctx.clone()).await.unwrap();

            let (new_paths, new_peer_id) = {
                let ctx = ctx.read().await;
                (ctx.peer_api.paths(), ctx.peer_api.peer_id())
            };

            assert_ne!(old_paths.all_dirs(), new_paths.all_dirs());
            assert_ne!(old_peer_id, new_peer_id);

            let can_open = {
                let ctx = ctx.read().await;
                ctx.peer_api.reopen()?;
                true
            };
            assert!(can_open);

            Ok(())
        }
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
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::context;
    use crate::error;
    use crate::http;

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
