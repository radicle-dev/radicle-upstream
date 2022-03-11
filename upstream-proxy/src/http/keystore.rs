// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints for handling the keystore.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http, keystore};

/// Combination of all keystore filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    unseal_filter(ctx.clone()).or(create_filter(ctx)).boxed()
}

/// `POST /unseal`
fn unseal_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("unseal")
        .and(warp::post())
        .and(path::end())
        .and(http::with_context(ctx))
        .and(warp::body::json())
        .and_then(handler::unseal)
}

/// `POST /`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path::end())
        .and(http::with_context(ctx))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// Keystore handlers for conversion between core domain and HTTP request fulfilment.
mod handler {
    use warp::{http::StatusCode, Rejection, Reply};

    use crate::context;

    /// Unseal the keystore.
    pub async fn unseal(
        mut ctx: context::Context,
        input: super::UnsealInput,
    ) -> Result<impl Reply, Rejection> {
        ctx.unseal_keystore(input.passphrase).await?;
        Ok(warp::reply::with_status(
            "Keystore unsealed",
            StatusCode::OK,
        ))
    }

    /// Initialize the keystore with a new key.
    pub async fn create(
        mut ctx: context::Context,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        ctx.create_key(input.passphrase).await?;
        Ok(warp::reply::with_status("New key created", StatusCode::OK))
    }
}

/// Bundled input data for unseal request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsealInput {
    /// Passphrase to unlock the keystore.
    passphrase: keystore::SecUtf8,
}

/// Bundled input data for `create` request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Passphrase to encrypt the keystore with.
    passphrase: keystore::SecUtf8,
}
