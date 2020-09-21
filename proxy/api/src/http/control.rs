//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::context;

/// Combination of all control filters.
pub fn filters(ctx: context::Ctx) -> BoxedFilter<(impl Reply,)> {
    create_project_filter(ctx.clone())
        .or(reset_coco_filter(ctx))
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

/// GET /reset/coco
fn reset_coco_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("reset" / "coco")
        .and(super::with_context(ctx))
        .and_then(handler::reset_coco)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use coco::{keystore, signer, user};

    use crate::{context, error, project};

    /// Create a project from the fixture repo.
    #[allow(clippy::let_underscore_must_use)]
    pub async fn create_project(
        ctx: context::Ctx,
        owner: user::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let meta = coco::control::replicate_platinum(
            &(*ctx.state.lock().await),
            &ctx.signer,
            &owner,
            &input.name,
            &input.description,
            &input.default_branch,
        )
        .map_err(error::Error::from)?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ = coco::control::track_fake_peer(
                    &(*ctx.state.lock().await),
                    &ctx.signer,
                    &meta,
                    &user_handle,
                );
            }
        }
        let stats = ctx
            .state
            .lock()
            .await
            .with_browser(&meta.urn(), |browser| Ok(browser.get_stats()?))
            .map_err(error::Error::from)?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Reset the local `CoCo` state.
    pub async fn reset_coco(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
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

        let paths = coco::Paths::from_root(tmp_path.clone()).map_err(error::Error::from)?;

        let store =
            kv::Store::new(kv::Config::new(tmp_path.join("store"))).map_err(error::Error::from)?;
        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut new_keystore = keystore::Keystorage::new(&paths, pw);
        let key = new_keystore.init().map_err(error::Error::from)?;
        let signer = signer::BoxedSigner::from(key.clone());

        let (_new_peer, new_state) = {
            let config =
                coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, vec![]);
            coco::into_peer_state(config, signer.clone(), store.clone())
                .await
                .map_err(error::Error::from)?
        };

        let mut ctx = ctx.write().await;
        ctx.state = new_state;
        ctx.signer = signer;
        ctx.store = store;

        Ok(reply::json(&true))
    }

    #[allow(clippy::unwrap_used, clippy::panic)]
    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_ne;

        use crate::{context, error};

        #[tokio::test]
        async fn reset_coco() -> Result<(), error::Error> {
            let tmp_dir = tempfile::tempdir()?;
            let ctx = context::Context::tmp(&tmp_dir).await?;

            let (old_paths, old_peer_id) = {
                let ctx = ctx.read().await;
                let state = ctx.state.lock().await;
                (state.paths(), state.peer_id())
            };

            super::reset_coco(ctx.clone()).await.unwrap();

            let (new_paths, new_peer_id) = {
                let ctx = ctx.read().await;
                let state = ctx.state.lock().await;
                (state.paths(), state.peer_id())
            };

            assert_ne!(old_paths.all_dirs(), new_paths.all_dirs());
            assert_ne!(old_peer_id, new_peer_id);

            let can_open = {
                let ctx = ctx.read().await;
                ctx.state.lock().await.reopen()?;
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
