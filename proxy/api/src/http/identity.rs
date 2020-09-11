//! Endpoints and serialisation for [`identity::Identity`] related types.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::context;
use crate::http;

/// Combination of all identity routes.
pub fn filters(ctx: context::Ctx) -> BoxedFilter<(impl Reply,)> {
    get_filter(Arc::clone(&ctx))
        .or(create_filter(Arc::clone(&ctx)))
        .or(projects_filter(Arc::clone(&ctx)))
        .or(list_filter(ctx))
        .boxed()
}

/// `POST /`
fn create_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `GET /<id>`
fn get_filter(ctx: context::Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and_then(handler::get)
}

/// `GET /<id>/projects`
fn projects_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::param::<coco::Urn>())
        .and(path("projects"))
        .and(path::end())
        .and_then(handler::get_projects)
}

/// `GET /`
fn list_filter(ctx: context::Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::end())
        .and_then(handler::list)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::context;
    use crate::error;
    use crate::identity;
    use crate::project;
    use crate::session;

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Ctx,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        if let Some(identity) = session::current(&ctx.peer_api, &ctx.store).await?.identity {
            return Err(Rejection::from(error::Error::from(
                coco::Error::EntityExists(identity.urn),
            )));
        }

        let id = identity::create(&ctx.peer_api, &ctx.signer, &input.handle)?;

        session::set_identity(&ctx.store, id.clone())?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(ctx: context::Ctx, id: coco::Urn) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        let id = identity::get(&ctx.peer_api, &id)?;
        Ok(reply::json(&id))
    }

    /// Get all projects tracked by the identity of the given `peer_urn`.
    pub async fn get_projects(
        ctx: context::Ctx,
        peer_urn: coco::Urn,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        Ok(reply::json(&project::list_projects_for_user(
            &ctx.peer_api,
            &peer_urn,
        )?))
    }

    /// Retrieve the list of identities known to the session user.
    pub async fn list(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        let users = identity::list(&ctx.peer_api)?;
        Ok(reply::json(&users))
    }
}

// TODO(xla): Implement Deserialize on identity::Metadata and drop this type entirely, this will
// help to avoid duplicate efforts for documentation.
/// Bundled input data for identity creation.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Handle the user wants to go by.
    handle: String,
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::avatar;
    use crate::context;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::project;
    use crate::session;

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::CreateInput {
                handle: "cloudhead".into(),
            })
            .reply(&api)
            .await;

        let ctx = ctx.read().await;
        let peer_id = ctx.peer_api.peer_id();
        let session = session::current(&ctx.peer_api, &ctx.store).await?;
        let urn = session.identity.expect("failed to set identity").urn;

        // Assert that we set the default owner and it's the same one as the session
        {
            assert_eq!(
                ctx.peer_api.default_owner(),
                Some(ctx.peer_api.get_user(&urn)?)
            );
        }

        http::test::assert_response(&res, StatusCode::CREATED, |have| {
            let avatar = avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity);
            let shareable_entity_identifier = format!("cloudhead@{}", peer_id);
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
                    "avatarFallback": avatar,
                    "urn": urn,
                    "metadata": {
                        "handle": "cloudhead",
                    },
                    "shareableEntityIdentifier": &shareable_entity_identifier,
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let user = ctx.peer_api.init_user(&ctx.signer, "cloudhead")?;
        let urn = user.urn();
        let handle = user.name().to_string();
        let peer_id = ctx.peer_api.peer_id();
        let shareable_entity_identifier = (peer_id.clone(), user).into();

        let res = request()
            .method("GET")
            .path(&format!("/{}", urn))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(identity::Identity {
                peer_id,
                urn: urn.clone(),
                shareable_entity_identifier,
                metadata: identity::Metadata { handle },
                avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
            })
        );

        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::indexing_slicing)]
    async fn projects() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, &ctx.signer, &owner)?;

        let projects = project::Projects::list(&ctx.peer_api)?;
        let project = projects.into_iter().next().unwrap();
        let coco_project = ctx.peer_api.get_project(&project.id, None)?;

        let peer: identity::Identity =
            coco::control::track_fake_peer(&ctx.peer_api, &ctx.signer, &coco_project, "rafalca")
                .into();

        let res = request()
            .method("GET")
            .path(&format!("/{}/projects", peer.urn))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(have, json!(vec![project]));

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let id = identity::create(&ctx.peer_api, &ctx.signer, "cloudhead")?;

        let owner = ctx.peer_api.get_user(&id.clone().urn)?;
        let owner = coco::verify_user(owner)?;

        session::set_identity(&ctx.store, id)?;

        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;

        let fintohaps: identity::Identity = coco::control::track_fake_peer(
            &ctx.peer_api,
            &ctx.signer,
            &platinum_project,
            "fintohaps",
        )
        .into();

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!([fintohaps]));
        Ok(())
    }
}
