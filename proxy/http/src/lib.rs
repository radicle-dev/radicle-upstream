//! HTTP API delivering JSON over `RESTish` endpoints.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::{path, reject, reply, Filter, Rejection, Reply};

use librad::paths;

use coco;
use core::keystore;

mod avatar;
mod control;
mod doc;
mod error;
mod identity;
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
pub fn api<R>(
    peer_api: coco::Api,
    keystore: keystore::Keystorage,
    store: kv::Store,
    enable_control: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    let ctx = Context {
        peer_api,
        keystore,
        store,
    };
    let ctx = Arc::new(RwLock::new(ctx));

    let avatar_filter = path("avatars").and(avatar::get_filter());
    let control_filter = path("control")
        .map(move || enable_control)
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
    let project_filter = path("projects").and(project::filters(ctx.clone()));
    let session_filter = path("session").and(session::filters(ctx.clone()));
    let source_filter = path("source").and(source::filters(ctx.clone()));

    let api = path("v1").and(combine!(
        avatar_filter,
        control_filter,
        identity_filter,
        project_filter,
        session_filter,
        source_filter,
    ));

    // let docs = path("docs").and(doc::filters(&api));
    let docs = path("docs").and(doc::filters(&api));
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

    let recovered = combine!(api, docs).recover(error::recover);

    recovered.with(cors).with(log)
}

/// Asserts presence of the owner and reject the request early if missing. Otherwise unpacks and
/// passes down.
#[must_use]
fn with_owner_guard(ctx: Ctx) -> BoxedFilter<(coco::User,)>
{
    warp::any()
        .and(with_context(ctx))
        .and_then(|ctx: Ctx| async move {
            let ctx = ctx.read().await;
            let session = core::session::current(&ctx.peer_api, &ctx.store)
                .await
                .expect("unable to get current sesison");

            if let Some(identity) = session.identity {
                let user = ctx
                    .peer_api
                    .get_user(&identity.urn)
                    .expect("unable to get coco user");
                let user = coco::verify_user(user).expect("unable to verify user");

                Ok(user)
            } else {
                Err(Rejection::from(error::Routing::MissingOwner))
            }
        })
        .boxed()
}

/// Container to pass down dependencies into HTTP filter chains.
pub struct Context {
    /// [`coco::Api`] to operate on the local monorepo.
    peer_api: coco::Api,
    /// Storage to manage keys.
    keystore: keystore::Keystorage,
    /// [`kv::Store`] used for session state and cache.
    store: kv::Store,
}

/// Wrapper around the thread-safe handle on [`Context`].
pub type Ctx = Arc<RwLock<Context>>;

/// Resets the peer and keystore within the `Ctx`.
///
/// # Errors
///
///   * If we could not get the librad path.
///   * If we could not initialise the librad key.
///   * If we could not construct the peer API.
///
/// # Panics
///
///   * If we could not get the temporary directory.
pub async fn reset_ctx_peer(ctx: Ctx) -> Result<(), crate::error::Error>
{
    // TmpDir deletes the temporary directory once it DROPS.
    // This means our new directory goes missing, and future calls will fail.
    // The Peer creates the directory again.
    //
    // N.B. this may gather lot's of tmp files on your system. We're sorry.
    let tmp_path = {
        let temp_dir = tempfile::tempdir().expect("test dir creation failed");
        log::debug!("New temporary path is: {:?}", temp_dir.path());
        std::env::set_var("RAD_HOME", temp_dir.path());
        temp_dir.path().to_path_buf()
    };

    let paths = paths::Paths::from_root(tmp_path)?;

    let pw = keystore::SecUtf8::from("radicle-upstream");
    let mut new_keystore = keystore::Keystorage::new(&paths, pw);
    let key = new_keystore.init_librad_key()?;

    let config = coco::config::configure(paths, key.clone(), *coco::config::LOCALHOST_ANY, vec![]);
    let new_peer_api = coco::Api::new(config).await?;

    let mut ctx = ctx.write().await;
    ctx.peer_api = new_peer_api;
    ctx.keystore = new_keystore;

    Ok(())
}

/// Middleware filter to inject a context into a filter chain to be passed down to a handler.
#[must_use]
fn with_context(ctx: Ctx) -> BoxedFilter<(Ctx,)>
{
    warp::any().map(move || ctx.clone()).boxed()
}

impl Context {
    #[cfg(test)]
    async fn tmp(
        tmp_dir: &tempfile::TempDir,
    ) -> Result<Ctx<registry::Cacher<registry::Registry>>, crate::error::Error> {
        let paths = librad::paths::Paths::from_root(tmp_dir.path())?;

        let pw = keystore::SecUtf8::from("radicle-upstream");
        let mut keystore = keystore::Keystorage::new(&paths, pw);
        let key = keystore.init_librad_key()?;

        let peer_api = {
            let config = coco::config::default(key, tmp_dir.path())?;
            coco::Api::new(config).await?
        };

        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        Ok(Arc::new(RwLock::new(Self {
            keystore,
            peer_api,
            store,
        })))
    }
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
/// [`http::error::Routing::InvalidQuery`].
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
/// Otherwise the filter is rejected with [`http::error::Routing::QueryMissing`].
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
}
