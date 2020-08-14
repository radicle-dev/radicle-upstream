//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::registry;

/// Combination of all control filters.
pub fn filters<R, S>(ctx: http::Ctx<R, S>) -> BoxedFilter<(impl Reply,)>
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    create_project_filter(ctx.clone())
        .or(nuke_registry_filter(ctx.clone()))
        .or(register_user_filter(ctx.clone()))
        .or(reset_filter(ctx))
        .boxed()
}

/// POST /create-project
fn create_project_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    path!("create-project")
        .and(super::with_context(ctx.clone()))
        .and(super::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// POST /register-user
fn register_user_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    path!("register-user")
        .and(http::with_context(ctx))
        .and(warp::body::json())
        .and_then(handler::register_user)
}

/// GET /reset
fn reset_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    path!("reset")
        .and(super::with_context(ctx))
        .and_then(handler::reset)
}

/// GET /nuke/registry
fn nuke_registry_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    path!("nuke" / "registry")
        .and(http::with_context(ctx))
        .and_then(handler::nuke_registry)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use std::convert::TryFrom;

    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use radicle_registry_client::CryptoPair;

    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::project;
    use crate::registry;

    /// Create a project from the fixture repo.
    pub async fn create_project<R, S>(
        ctx: http::Ctx<R, S>,
        owner: coco::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
        S: coco::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;

        let meta = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
            &owner,
            &input.name,
            &input.description,
            &input.default_branch,
        )?;

        if let Some(user_handle_list) = input.fake_peers {
            for user_handle in user_handle_list {
                let _ =
                    coco::control::track_fake_peer(&ctx.peer_api, &ctx.signer, &meta, &user_handle);
            }
        }
        let stats = ctx
            .peer_api
            .with_browser(&meta.urn(), |browser| Ok(browser.get_stats()?))?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Register a user with another key
    pub async fn register_user<R, S>(
        ctx: http::Ctx<R, S>,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: coco::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;

        let fake_pair =
            radicle_registry_client::ed25519::Pair::from_legacy_string(&input.handle, None);

        log::info!(
            "Registering user handle {} with public key {}",
            &input.handle,
            &fake_pair.public()
        );

        let handle = registry::Id::try_from(input.handle).map_err(Error::from)?;
        ctx.registry
            .register_user(&fake_pair, handle.clone(), None, input.transaction_fee)
            .await
            .expect("unable to register user");

        Ok(reply::json(&true))
    }

    /// Reset the known state by replacing the [`http::Context`].
    pub async fn reset<R, S>(_ctx: http::Ctx<R, S>) -> Result<impl Reply, Rejection>
    where
        R: registry::Client + 'static,
        S: coco::Signer,
        S::Error: coco::SignError,
    {
        // TmpDir deletes the temporary directory once it DROPS.
        // This means our new directory goes missing, and future calls will fail.
        // The Peer creates the directory again.
        //
        // N.B. this may gather lot's of tmp files on your system. We're sorry.
        // let tmp_dir = tempfile::tempdir().expect("test dir creation failed");
        // log::debug!("New temporary path is: {:?}", tmp_dir.path());
        // std::env::set_var("RAD_HOME", tmp_dir.path());

        // let mut ctx = ctx.write().await;
        // let new_ctx = http::Context::tmp(&tmp_dir).await?;
        // *ctx = new_ctx;

        Ok(reply::json(&true))
    }

    /// Reset the Registry state by replacing the emulator in place.
    pub async fn nuke_registry<R, S>(ctx: http::Ctx<R, S>) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: coco::Signer,
        S::Error: coco::SignError,
    {
        let (client, _) = radicle_registry_client::Client::new_emulator();
        let mut ctx = ctx.write().await;
        ctx.registry.reset(client);

        Ok(reply::json(&true))
    }

    #[allow(clippy::unwrap_used, clippy::panic)]
    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_ne;

        use crate::error;
        use crate::http;

        #[tokio::test]
        async fn reset() -> Result<(), error::Error> {
            let tmp_dir = tempfile::tempdir()?;
            let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);

            let (old_paths, old_peer_id) = {
                let ctx = ctx.read().await;
                (ctx.peer_api.paths(), ctx.peer_api.peer_id())
            };

            super::reset(ctx.clone()).await.unwrap();

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
/// Input for user registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Handle of the user.
    handle: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::sync::Arc;

    use pretty_assertions::assert_eq;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::error;
    use crate::http;

    // TODO(xla): This can't hold true anymore, given that we nuke the owner. Which is required in
    // order to register a project. Should we rework the test? How do we make sure an owner is
    // present?
    #[ignore]
    #[tokio::test]
    async fn create_project_after_nuke() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = Arc::new(RwLock::new(http::Context::tmp(&tmp_dir).await?));
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
        let res = request().method("GET").path("/reset").reply(&api).await;
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
