// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Manage the state and stateful interactions with the underlying peer API of librad.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use radicle_daemon::Urn;

use crate::{context, http};

/// Combination of all identity routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_filter(ctx.clone())
        .or(get_remote_filter(ctx.clone()))
        .or(create_filter(ctx.clone()))
        .or(update_filter(ctx))
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

/// `PUT /`
fn update_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::update)
}

/// `GET /<id>`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(warp::path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get)
}

/// `GET /remote/<id>`
fn get_remote_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("remote")
        .and(path::param::<Urn>())
        .and(warp::path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get_remote)
}

/// Identity handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{http::StatusCode, reject, reply, Rejection, Reply};

    use radicle_daemon::Urn;

    use crate::{context, error, http, identity, session};

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

    /// Update the [`identity::Identity`] metadata.
    pub async fn update(
        ctx: context::Unsealed,
        metadata: identity::Metadata,
    ) -> Result<impl Reply, Rejection> {
        session::get_current(&ctx.store)?.ok_or(http::error::Routing::NoSession)?;
        let id = identity::update(&ctx.peer, metadata).await?;
        session::update_identity(&ctx.store, id.clone())?;

        Ok(reply::with_status(reply::json(&id), StatusCode::OK))
    }

    /// Get the [`identity::Identity`] for the given `id`.
    pub async fn get(id: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let id = identity::get(&ctx.peer, id.clone()).await?;
        Ok(reply::json(&id))
    }

    /// Get the [`identity::Person`] for the given `id`.
    pub async fn get_remote(id: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        match identity::get_remote(&ctx.peer, id.clone()).await? {
            Some(id) => Ok(reply::json(&id)),
            None => Err(reject::not_found()),
        }
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
    use radicle_daemon::state;

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
                state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::CREATED, |have| {
            let avatar = avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity);
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
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let res = request()
            .method("POST")
            .path("/")
            .json(&identity::Metadata {
                handle: "cloudhead".into(),
                ethereum: None,
            })
            .reply(&api)
            .await;
        http::test::assert_response(&res, StatusCode::CREATED, |_| ());

        let res = request()
            .method("PUT")
            .path("/")
            .json(&identity::Metadata {
                handle: "cloudhead_next".into(),
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
                state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::OK, |have| {
            let avatar = avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity);
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
                    "avatarFallback": avatar,
                    "urn": urn,
                    "metadata": {
                        "handle": "cloudhead_next",
                        "ethereum": {
                            "address": "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B",
                            "expiration": "2021-03-19T23:15:30.001Z",
                        }
                    },
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn update_remove_ethereum_claim() -> Result<(), error::Error> {
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
        http::test::assert_response(&res, StatusCode::CREATED, |_| ());

        let res = request()
            .method("PUT")
            .path("/")
            .json(&identity::Metadata {
                handle: "cloudhead".into(),
                ethereum: None,
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
                state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::OK, |have| {
            let avatar = avatar::Avatar::from(&urn.to_string(), avatar::Usage::Identity);
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
                    "avatarFallback": avatar,
                    "urn": urn,
                    "metadata": {
                        "handle": "cloudhead",
                        "ethereum": null
                    },
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

        let user = state::init_user(&ctx.peer, "cloudhead".to_string()).await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}", user.urn()))
            .reply(&api)
            .await;

        let handle = user.subject().name.to_string();
        let peer_id = ctx.peer.peer_id();
        let urn = user.urn();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!(identity::Identity {
                peer_id,
                urn: urn.clone(),
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

            let owner = state::get_local(&ctx.peer, id.urn.clone()).await?.unwrap();

            session::initialize(&ctx.store, id, &ctx.default_seeds)?;

            let platinum_project = crate::control::replicate_platinum(
                &ctx.peer,
                &owner,
                "git-platinum",
                "fixture data",
                crate::control::default_branch(),
            )
            .await?;

            let (peer_id, local_identity) =
                crate::control::track_fake_peer(&ctx.peer, &platinum_project, "fintohaps").await;
            (peer_id, local_identity.into_inner().into_inner()).into()
        };

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!([fintohaps]));
        Ok(())
    }
}
