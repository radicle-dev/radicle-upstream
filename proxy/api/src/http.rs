//! HTTP API delivering JSON over `RESTish` endpoints.

use serde::Deserialize;
use tokio::sync::mpsc;
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, notification::Subscriptions};

mod avatar;
mod control;
mod error;
mod identity;
mod notification;
mod project;
mod session;
mod source;

/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process.
///
/// https://github.com/seanmonstar/warp/issues/507#issuecomment-615974062
/// https://github.com/rs-ipfs/rust-ipfs/commit/ae3306686209afa5911b1ad02170c1ac3bacda7c
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
    subscriptions: Subscriptions,
    selfdestruct: mpsc::Sender<()>,
    enable_fixture_creation: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let avatar_filter = path("avatars").and(avatar::get_filter());
    let control_filter =
        path("control")
            .and(with_unsealed_guard(ctx.clone()))
            .and(control::filters(
                ctx.clone(),
                selfdestruct,
                enable_fixture_creation,
            ));
    let identity_filter = path("identities").and(identity::filters(ctx.clone()));
    let notification_filter = path("notifications")
        .and(with_unsealed_guard(ctx.clone()))
        .and(notification::filters(subscriptions));
    let project_filter = path("projects")
        .and(with_unsealed_guard(ctx.clone()))
        .and(project::filters(ctx.clone()));
    let session_filter = path("session").and(session::filters(ctx.clone()));
    let source_filter = path("source")
        .and(with_unsealed_guard(ctx.clone()))
        .and(source::filters(ctx));

    let api = path("v1").and(combine!(
        avatar_filter,
        control_filter,
        identity_filter,
        notification_filter,
        project_filter,
        session_filter,
        source_filter
    ));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(&[warp::http::header::CONTENT_TYPE])
        .allow_methods(&[
            warp::http::Method::DELETE,
            warp::http::Method::GET,
            warp::http::Method::POST,
            warp::http::Method::OPTIONS,
        ]);
    let log = warp::log::custom(|info| {
        log::info!(
            target: "proxy::http",
            "\"{} {} {:?}\" {} {:?}",
            info.method(),
            info.path(),
            info.version(),
            info.status().as_u16(),
            info.elapsed(),
        );
    });

    let recovered = api.recover(error::recover);

    recovered.with(cors).with(log)
}

/// Asserts presence of the owner and rejects the request early if missing. Otherwise unpacks and
/// passes down.
#[must_use]
fn with_owner_guard(ctx: context::Context) -> BoxedFilter<(coco::user::User,)> {
    warp::any()
        .and(with_context(ctx))
        .and_then(|ctx: context::Context| async move {
            let session = crate::session::current(ctx.state.clone(), &ctx.store)
                .await
                .expect("unable to get current sesison");

            if let Some(identity) = session.identity {
                let user = ctx
                    .state
                    .get_user(identity.urn)
                    .await
                    .expect("unable to get coco user");
                let user = coco::user::verify(user).expect("unable to verify user");

                Ok(user)
            } else {
                Err(Rejection::from(error::Routing::MissingOwner))
            }
        })
        .boxed()
}

/// Asserts presence of the signer and rejects the request early if missing.
#[must_use]
fn with_unsealed_guard(ctx: context::Context) -> BoxedFilter<()> {
    warp::any()
        .and(with_context(ctx))
        .and_then(|ctx: context::Context| async move {
            if ctx.signer.is_some() {
                Ok(())
            } else {
                Err(Rejection::from(error::Routing::SealedKeystore))
            }
        })
        .untuple_one()
        .boxed()
}

/// Middleware filter to inject a context into a filter chain to be passed down to a handler.
#[must_use]
fn with_context(ctx: context::Context) -> BoxedFilter<(context::Context,)> {
    warp::any().map(move || ctx.clone()).boxed()
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
/// If the query string cannot be parsed into `T` the filter rejects with
/// [`error::Routing::InvalidQuery`].
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
            if let Some(raw) = raw {
                let query = percent_encoding::percent_decode_str(&raw).decode_utf8_lossy();
                match serde_qs::from_str(&query) {
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(warp::reject::Rejection::from(
                        error::Routing::InvalidQuery {
                            query: query.into_owned(),
                            error: error.to_string(),
                        },
                    )),
                }
            } else {
                Ok(None)
            }
        })
        .boxed()
}

/// Parses the query string with [`serde_qs`] and returns the result.
///
/// Similar to [`with_qs_opt`] but requires a query string to be present.
/// Otherwise the filter is rejected with [`error::Routing::QueryMissing`].
#[must_use]
pub fn with_qs<T>() -> BoxedFilter<(T,)>
where
    for<'de> T: Deserialize<'de> + Send + Sync + 'static,
{
    with_qs_opt()
        .and_then(|opt_query: Option<T>| async move {
            opt_query.ok_or(warp::reject::Rejection::from(error::Routing::QueryMissing))
        })
        .boxed()
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
                    "message": "Invalid query string \"value=not_a_number\": failed with reason: invalid digit found in string",
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
                    "message": "Required query string is missing",
                    "variant": "QUERY_MISSING"
                })
            );
        });
    }

    #[tokio::test]
    async fn with_unsealed_guard() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let mut ctx = context::Context::tmp(&tmp_dir).await?;
        ctx.signer = None;
        let api = with_qs_opt::<Query>()
            .map(|opt_query| warp::reply::json(&opt_query))
            .and(super::with_unsealed_guard(ctx))
            .recover(super::error::recover)
            .boxed();

        let res = warp::test::request()
            .method("GET")
            .path("/?value=72")
            .reply(&api)
            .await;

        assert_response(&res, StatusCode::FORBIDDEN, |have| {
            assert_eq!(
                have,
                serde_json::json!({
                    "message": "Keystore is sealed",
                    "variant": "FORBIDDEN"
                })
            );
        });

        Ok(())
    }
}
