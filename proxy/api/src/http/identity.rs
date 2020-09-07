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
        .and_then(handler::get)
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
    use crate::session;

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Ctx,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        if let Some(identity) = session::current(ctx.state.clone(), &ctx.store)
            .await?
            .identity
        {
            return Err(Rejection::from(error::Error::from(
                coco::Error::EntityExists(identity.urn),
            )));
        }

        let id = identity::create(&(*ctx.state.lock().await), &ctx.signer, &input.handle)?;

        session::set_identity(&ctx.store, id.clone())?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(ctx: context::Ctx, id: coco::Urn) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        let state = ctx.state.lock().await;
        let id = identity::get(&state, &id)?;
        Ok(reply::json(&id))
    }

    /// Retrieve the list of identities known to the session user.
    pub async fn list(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        let state = ctx.state.lock().await;
        let users = identity::list(&state)?;
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

        let urn = {
            let session = session::current(ctx.state.clone(), &ctx.store).await?;
            session.identity.expect("failed to set identity").urn
        };

        let state = ctx.state.lock().await;
        let peer_id = state.peer_id();

        // Assert that we set the default owner and it's the same one as the session
        {
            assert_eq!(state.default_owner(), Some(state.get_user(&urn)?));
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

        let user = {
            let state = ctx.state.lock().await;
            state.init_user(&ctx.signer, "cloudhead")?
        };

        let res = request()
            .method("GET")
            .path(&format!("/{}", user.urn()))
            .reply(&api)
            .await;

        let state = ctx.state.lock().await;
        let handle = user.name().to_string();
        let peer_id = state.peer_id();
        let urn = user.urn();
        let shareable_entity_identifier = (peer_id.clone(), user).into();

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
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;

        let fintohaps: identity::Identity = {
            let state = ctx.state.lock().await;
            let id = identity::create(&state, &ctx.signer, "cloudhead")?;

            let owner = state.get_user(&id.urn)?;
            let owner = coco::user::verify(owner)?;

            session::set_identity(&ctx.store, id)?;

            let platinum_project = coco::control::replicate_platinum(
                &state,
                &ctx.signer,
                &owner,
                "git-platinum",
                "fixture data",
                "master",
            )?;

            coco::control::track_fake_peer(&state, &ctx.signer, &platinum_project, "fintohaps")
                .into()
        };

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!([fintohaps]));
        Ok(())
    }
}
