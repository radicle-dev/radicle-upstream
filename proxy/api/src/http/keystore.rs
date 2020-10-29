//! Endpoints for handeling the keystore.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

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
        .and_then(handler::create)
}

/// Keystore handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error};

    /// Unseal the keystore.
    pub async fn unseal(
        mut ctx: context::Context,
        input: super::UnsealInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(merle): Replace with correct password check
        if input.passphrase != "radicle-upstream" {
            return Err(Rejection::from(error::Error::WrongPassphrase));
        }
        // TODO Load the real key from disk. The service manager ignores the key for now and uses a
        // hardcoded one.
        let key = coco::keys::SecretKey::new();
        ctx.service_handle().set_secret_key(key);

        let auth_cookie_lock = ctx.auth_cookie();
        let mut cookie = auth_cookie_lock.write().await;
        *cookie = Some("chocolate".into());
        Ok(warp::reply::with_header(
            reply::with_status(reply(), StatusCode::NO_CONTENT),
            "Set-Cookie",
            "auth-cookie=chocolate; Path=/",
        )
        .into_response())
    }

    /// Initialize the key store with a new key
    pub async fn create(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        // TODO Load the real key from disk. The service manager ignores the key for now and uses a
        // hardcoded one.
        let key = coco::keys::SecretKey::new();
        ctx.service_handle().set_secret_key(key);

        let auth_cookie_lock = ctx.auth_cookie();
        let mut cookie = auth_cookie_lock.write().await;
        *cookie = Some("chocolate".into());
        Ok(warp::reply::with_header(
            reply::with_status(reply(), StatusCode::NO_CONTENT),
            "Set-Cookie",
            "auth-cookie=chocolate; Path=/",
        )
        .into_response())
    }
}

/// Bundled input data for unseal request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsealInput {
    /// Passphrase to unlock the keystore.
    passphrase: String,
}
