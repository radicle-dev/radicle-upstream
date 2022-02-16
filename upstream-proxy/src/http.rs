// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! HTTP API delivering JSON over `RESTish` endpoints.

use serde::Deserialize;
use warp::{filters::BoxedFilter, path, reject, Filter, Rejection, Reply};

use link_crypto::PeerId;

use crate::context;

mod control;
pub mod error;
mod identity;
mod notification;
mod project;
mod session;
mod source;

/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process.
///
/// <https://github.com/seanmonstar/warp/issues/507#issuecomment-615974062>
/// <https://github.com/rs-ipfs/rust-ipfs/commit/ae3306686209afa5911b1ad02170c1ac3bacda7c>
macro_rules! combine {
    ($x:expr, $($y:expr),+) => {
        {
            let filter = $x.boxed();
            $(
                let filter = filter.or($y).boxed();
            )+
            filter
        }
    }
}

/// Main entry point for HTTP API.
pub fn api(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = std::convert::Infallible> + Clone {
    let test = ctx.test();

    let control_filter = path("control")
        .map(move || test)
        .and_then(|enable| async move {
            if enable {
                Ok(())
            } else {
                Err(reject::not_found())
            }
        })
        .untuple_one()
        .and(control::filters(ctx.clone()));
    let identity_filter = path("identities").and(identity::filters(ctx.clone()));
    let notification_filter = path("notifications").and(notification::filters(ctx.clone()));
    let project_filter = path("projects").and(project::filters(ctx.clone()));
    let session_filter = path("session").and(session::filters(ctx.clone()));
    let source_filter = path("source").and(source::filters(ctx));

    let api = path("v1").and(combine!(
        control_filter,
        identity_filter,
        notification_filter,
        project_filter,
        session_filter,
        source_filter
    ));

    api.recover(error::recover)
}

/// Asserts presence of the owner and rejects the request early if missing. Otherwise unpacks and
/// passes down.
#[must_use]
fn with_owner_guard(ctx: context::Context) -> BoxedFilter<(crate::daemon::LocalIdentity,)> {
    warp::any()
        .and(with_context_unsealed(ctx))
        .and_then(|ctx: context::Unsealed| async move {
            let user = ctx
                .peer
                .librad_peer()
                .using_storage(lnk_identities::local::default)
                .await
                .expect("failed to get storage")
                .expect("failed to get local identity");

            Ok::<_, Rejection>(user)
        })
        .boxed()
}

/// Middleware filter to inject a context into a filter chain to be passed down to a handler.
#[must_use]
fn with_context(ctx: context::Context) -> BoxedFilter<(context::Context,)> {
    warp::any().map(move || ctx.clone()).boxed()
}

/// Assert that the context is unsealed and and passes [`context::Unsealed`] to the handler.
///
/// Otherwise the requests rejects with [`crate::error::Error::KeystoreSealed`].
fn with_context_unsealed(ctx: context::Context) -> BoxedFilter<(context::Unsealed,)> {
    with_context(ctx)
        .and_then(|ctx: context::Context| async move {
            let unsealed_ctx = match ctx {
                context::Context::Sealed(_) => {
                    return Err(Rejection::from(crate::error::Error::KeystoreSealed))
                },
                context::Context::Unsealed(unsealed) => unsealed,
            };

            Ok(unsealed_ctx)
        })
        .boxed()
}

/// Parses an optional query string with [`serde_qs`] and returns the result.
///
/// If no query string is present (i.e. `?` is not included in the path) `None`
/// is returned.
///
/// This filter is different from [`warp::filters::query::query`]. It is able to
/// handle the absence of a query string and can deserialize more complex
/// structures.
///
/// # Errors
///
/// If the query string cannot be parsed into `T` the filter rejects.
#[must_use]
pub fn with_qs_opt<T>() -> BoxedFilter<(Option<T>,)>
where
    for<'de> T: Deserialize<'de> + Send + Sync + 'static,
{
    warp::filters::query::raw()
        .map(Some)
        .or_else(|rejection: Rejection| async {
            if rejection.find::<warp::reject::InvalidQuery>().is_some() {
                Ok((None,))
            } else {
                Err(rejection)
            }
        })
        .and_then(|raw: Option<String>| async move {
            raw.map_or(Ok(None), |raw| {
                let query = percent_encoding::percent_decode_str(&raw).decode_utf8_lossy();
                match serde_qs::from_str(&query) {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(warp::reject::Rejection::from(error::Response::new(
                        warp::http::StatusCode::BAD_REQUEST,
                        "INVALID_QUERY",
                        &error,
                    ))),
                }
            })
        })
        .boxed()
}

/// Parses the query string with [`serde_qs`] and returns the result.
///
/// Similar to [`with_qs_opt`] but requires a query string to be present.
/// Otherwise the filter is rejected and returns a 400 response.
#[must_use]
pub fn with_qs<T>() -> BoxedFilter<(T,)>
where
    for<'de> T: Deserialize<'de> + Send + Sync + 'static,
{
    with_qs_opt()
        .and_then(|opt_query: Option<T>| async move {
            opt_query.ok_or_else(|| {
                warp::reject::Rejection::from(error::Response {
                    status_code: warp::http::StatusCode::BAD_REQUEST,
                    variant: "QUERY_MISSING",
                    message: "required query string is missing".to_string(),
                })
            })
        })
        .boxed()
}

/// Guard against access of wrong paths by the owners Peer ID.
#[must_use]
pub fn guard_self_peer_id(peer: &crate::peer::Peer, peer_id: Option<PeerId>) -> Option<PeerId> {
    match peer_id {
        Some(peer_id) if peer_id == peer.librad_peer().peer_id() => None,
        Some(peer_id) => Some(peer_id),
        None => None,
    }
}

/// Guard against access of the wrong paths by the owners Peer ID when inside a `Revision`.
#[must_use]
pub fn guard_self_revision(
    peer: &crate::peer::Peer,
    revision: Option<radicle_source::Revision<PeerId>>,
) -> Option<radicle_source::Revision<PeerId>> {
    revision.map(|r| {
        if let radicle_source::Revision::Branch { name, peer_id } = r {
            radicle_source::Revision::Branch {
                name,
                peer_id: guard_self_peer_id(peer, peer_id),
            }
        } else {
            r
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use bytes::Bytes;
    use http::response::Response;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use warp::http::StatusCode;

    pub fn assert_response<F>(res: &Response<Bytes>, code: StatusCode, checks: F)
    where
        F: FnOnce(Value),
    {
        assert_eq!(
            res.status(),
            code,
            "response status was not {}, the body is:\n{:#?}",
            code,
            res.body()
        );

        let have: Value = serde_json::from_slice(res.body()).expect("failed to deserialise body");
        checks(have);
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Query {
        value: u32,
    }

    fn with_qs_opt_test_filter() -> BoxedFilter<(impl Reply,)> {
        with_qs_opt::<Query>()
            .map(|opt_query| warp::reply::json(&opt_query))
            .recover(super::error::recover)
            .boxed()
    }

    #[tokio::test]
    async fn with_qs_opt_present() {
        let res = warp::test::request()
            .method("GET")
            .path("/?value=72")
            .reply(&with_qs_opt_test_filter())
            .await;

        assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                serde_json::json!({
                    "value": 72,
                })
            );
        });
    }
    #[tokio::test]
    async fn with_qs_opt_invalid() {
        let res = warp::test::request()
            .method("GET")
            .path("/?value=not_a_number")
            .reply(&with_qs_opt_test_filter())
            .await;

        assert_response(&res, StatusCode::BAD_REQUEST, |have| {
            assert_eq!(
                have,
                serde_json::json!({
                    "message": "failed with reason: invalid digit found in string",
                    "variant": "INVALID_QUERY"
                })
            );
        });
    }
    #[tokio::test]
    async fn with_qs_opt_none() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&with_qs_opt_test_filter())
            .await;

        assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, serde_json::json!(null))
        });
    }

    #[tokio::test]
    async fn with_qs_missing() {
        let api = with_qs::<Query>()
            .map(|opt_query| warp::reply::json(&opt_query))
            .recover(super::error::recover)
            .boxed();

        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&api)
            .await;

        assert_response(&res, StatusCode::BAD_REQUEST, |have| {
            assert_eq!(
                have,
                serde_json::json!({
                    "message": "required query string is missing",
                    "variant": "QUERY_MISSING"
                })
            );
        });
    }
}
