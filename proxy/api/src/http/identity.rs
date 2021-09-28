// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Manage the state and stateful interactions with the underlying peer API of librad.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use link_identities::git::Urn;

use crate::{context, http};

/// Combination of all identity routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_remote_filter(ctx.clone())
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

    use link_identities::git::Urn;

    use crate::{context, http, identity, session};

    /// Create a new [`identity::Identity`].
    pub async fn create(
        ctx: context::Unsealed,
        metadata: identity::Metadata,
    ) -> Result<impl Reply, Rejection> {
        if session::get_current(&ctx.store)?.is_some() {
            return Err(http::error::Response {
                status_code: StatusCode::BAD_REQUEST,
                variant: "SESSION_IN_USE",
                message: "A session already exists".to_string(),
            }
            .into());
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

    /// Get the [`identity::Person`] for the given `id`.
    pub async fn get_remote(id: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        match identity::get(&ctx.peer, id.clone()).await? {
            Some(id) => Ok(reply::json(&id)),
            None => Err(reject::not_found()),
        }
    }
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryInto;
    use warp::{http::StatusCode, test::request};

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
                radicle_daemon::state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                radicle_daemon::state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::CREATED, |have| {
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
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
                radicle_daemon::state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                radicle_daemon::state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
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
                radicle_daemon::state::default_owner(&ctx.peer)
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner(),
                radicle_daemon::state::get_local(&ctx.peer, urn.clone())
                    .await?
                    .unwrap()
                    .into_inner()
                    .into_inner()
            );
        }

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!({
                    "peerId": peer_id,
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
}
