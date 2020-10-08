//! Manage the state and stateful interactions with the underlying peer API of librad.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all identity routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_filter(ctx.clone())
        .or(create_filter(ctx.clone()))
        .or(list_filter(ctx))
        .boxed()
}

/// `POST /`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `GET /<id>`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::param::<coco::Urn>())
        .and_then(handler::get)
}

/// `GET /`
fn list_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::end())
        .and_then(handler::list)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, http, identity, session};

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Context,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        if let Some(identity) = session::current(ctx.state.clone(), &ctx.store)
            .await?
            .identity
        {
            return Err(Rejection::from(error::Error::from(
                coco::state::Error::already_exists(identity.urn),
            )));
        }

        let signer = ctx
            .signer
            .ok_or_else(|| http::error::Routing::SealedKeystore)?;
        let id = identity::create(&ctx.state, &signer, &input.handle).await?;

        session::set_identity(&ctx.store, id.clone())?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(ctx: context::Context, id: coco::Urn) -> Result<impl Reply, Rejection> {
        let id = identity::get(&ctx.state, id.clone()).await?;
        Ok(reply::json(&id))
    }

    /// Retrieve the list of identities known to the session user.
    pub async fn list(ctx: context::Context) -> Result<impl Reply, Rejection> {
        let users = identity::list(&ctx.state).await?;
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
    use warp::{http::StatusCode, test::request};

    use crate::{avatar, context, error, http, identity, session};

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

        let urn = {
            let session = session::current(ctx.state.clone(), &ctx.store).await?;
            session.identity.expect("failed to set identity").urn
        };

        let peer_id = ctx.state.peer_id();

        // Assert that we set the default owner and it's the same one as the session
        {
            assert_eq!(
                ctx.state.default_owner().await,
                Some(ctx.state.get_user(urn.clone()).await?)
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

        let user = ctx
            .state
            .init_user(&ctx.signer.unwrap(), "cloudhead")
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}", user.urn()))
            .reply(&api)
            .await;

        let handle = user.name().to_string();
        let peer_id = ctx.state.peer_id();
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

        let fintohaps: identity::Identity = {
            let id =
                identity::create(&ctx.state, &ctx.signer.clone().unwrap(), "cloudhead").await?;

            let owner = {
                let user = ctx.state.get_user(id.urn.clone()).await?;
                coco::user::verify(user)?
            };

            session::set_identity(&ctx.store, id)?;

            let platinum_project = coco::control::replicate_platinum(
                &ctx.state,
                &ctx.signer.clone().unwrap(),
                &owner,
                "git-platinum",
                "fixture data",
                "master",
            )
            .await?;

            coco::control::track_fake_peer(
                &ctx.state,
                &ctx.signer.unwrap(),
                &platinum_project,
                "fintohaps",
            )
            .await
            .into()
        };

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!([fintohaps]));
        Ok(())
    }
}
