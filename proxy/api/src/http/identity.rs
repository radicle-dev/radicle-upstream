//! Manage the state and stateful interactions with the underlying peer API of librad.

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
    path::end()
        .and(warp::post())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `GET /<id>`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<coco::Urn>()
        .and(warp::path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get)
}

/// `GET /`
fn list_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(http::with_context_unsealed(ctx))
        .and(warp::get())
        .and_then(handler::list)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, identity, session};

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Unsealed,
        metadata: identity::Metadata,
    ) -> Result<impl Reply, Rejection> {
        if let Some(session) = session::get_current(&ctx.store)? {
            return Err(Rejection::from(error::Error::SessionInUse(
                session.identity.urn,
            )));
        }

        let id = identity::create(&ctx.peer, metadata).await?;

        session::initialize(&ctx.store, id.clone(), &ctx.default_seeds)?;

        Ok(reply::with_status(reply::json(&id), StatusCode::CREATED))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(id: coco::Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let id = identity::get(&ctx.peer, id.clone()).await?;
        Ok(reply::json(&id))
    }

    /// Retrieve the list of identities known to the session user.
    pub async fn list(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let users = identity::list(&ctx.peer).await?;
        Ok(reply::json(&users))
    }
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::convert::TryInto;
    use warp::{http::StatusCode, test::request};

    use radicle_avatar as avatar;

    use crate::{context, error, http, identity, session};

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let res = request()
            .method("POST")
            .path("/")
            .json(&identity::Metadata {
                handle: "cloudhead".into(),
                ethereum: Some(identity::Ethereum {
                    address: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B"
                        .to_string()
                        .try_into()
                        .expect("Invalid address"),
                    expiration: "2021-03-19T23:15:30.001Z".parse().expect("Invalid date"),
                }),
            })
            .reply(&api)
            .await;

        let urn = {
            let session = session::get_current(&ctx.store)?.expect("no session exists");
            session.identity.urn
        };

        let peer_id = ctx.peer.peer_id();

        // Assert that we set the default owner and it's the same one as the session
        {
            assert_eq!(
                coco::state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                coco::state::get_user(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
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
                        "ethereum": {
                            "address": "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B",
                            "expiration": "2021-03-19T23:15:30.001Z",
                        }
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
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let user = coco::state::init_user(&ctx.peer, "cloudhead".to_string()).await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}", user.urn()))
            .reply(&api)
            .await;

        let handle = user.subject().name.to_string();
        let peer_id = ctx.peer.peer_id();
        let urn = user.urn();
        let shareable_entity_identifier = (peer_id, user.into_inner().into_inner()).into();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(identity::Identity {
                peer_id,
                urn: urn.clone(),
                shareable_entity_identifier,
                metadata: identity::Metadata {
                    handle,
                    ethereum: None
                },
                avatar_fallback: avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity),
            })
        );

        Ok(())
    }

    // TODO(xla): Reintroduce when tracking is properly supported at the level of state
    // manipulation.
    #[ignore]
    #[tokio::test]
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let fintohaps: identity::Identity = {
            let metadata = identity::Metadata {
                handle: "cloudhead".to_string(),
                ethereum: None,
            };
            let id = identity::create(&ctx.peer, metadata).await?;

            let owner = coco::state::get_user(&ctx.peer, id.urn.clone())
                .await?
                .unwrap();

            session::initialize(&ctx.store, id, &ctx.default_seeds)?;

            let platinum_project = coco::control::replicate_platinum(
                &ctx.peer,
                &owner,
                "git-platinum",
                "fixture data",
                coco::control::default_branch(),
            )
            .await?;

            let (peer_id, local_identity) =
                coco::control::track_fake_peer(&ctx.peer, &platinum_project, "fintohaps").await;
            (peer_id, local_identity.into_inner().into_inner()).into()
        };

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!([fintohaps]));
        Ok(())
    }
}
