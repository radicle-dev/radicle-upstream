//! HTTP API delivering JSON over `RESTish` endpoints.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::{path, reject, reply, Filter, Rejection, Reply};

use crate::coco;
use crate::registry;

mod account;
mod avatar;
mod control;
mod doc;
mod error;
mod id;
mod identity;
mod notification;
mod org;
mod project;
mod session;
mod source;
mod transaction;
mod user;

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
pub fn api<R, S>(
    peer_api: coco::Api<S>,
    registry: R,
    signer: S,
    store: kv::Store,
    enable_control: bool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + registry::Client + 'static,
    S: coco::ResetSigner,
    S::Error: coco::SignError,
{
    let subscriptions = crate::notification::Subscriptions::default();
    let ctx = Context {
        peer_api,
        registry,
        signer,
        store,
        subscriptions: subscriptions.clone(),
    };
    let ctx = Arc::new(RwLock::new(ctx));

    let account_filter = path("accounts").and(account::filters(ctx.clone()));
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
    let id_filter = path("ids").and(id::get_status_filter(ctx.clone()));
    let identity_filter = path("identities").and(identity::filters(ctx.clone()));
    let notification_filter = path("notifications").and(notification::filters(subscriptions));
    let org_filter = path("orgs").and(org::filters(ctx.clone()));
    let project_filter = path("projects").and(project::filters(ctx.clone()));
    let session_filter = path("session").and(session::filters(ctx.clone()));
    let source_filter = path("source").and(source::filters(ctx.clone()));
    let transaction_filter = path("transactions").and(transaction::filters(ctx.clone()));
    let user_filter = path("users").and(user::filters(ctx));

    let api = path("v1").and(combine!(
        account_filter,
        avatar_filter,
        control_filter,
        id_filter,
        identity_filter,
        notification_filter,
        org_filter,
        project_filter,
        session_filter,
        source_filter,
        transaction_filter,
        user_filter
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
fn with_owner_guard<R, S>(ctx: Ctx<R, S>) -> BoxedFilter<(coco::User,)>
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    warp::any()
        .and(with_context(ctx))
        .and_then(|ctx: Ctx<R, S>| async move {
            let ctx = ctx.read().await;
            let session = crate::session::current(&ctx.peer_api, &ctx.registry, &ctx.store)
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
pub struct Context<R, S>
where
    S: coco::Signer,
    S::Error: coco::SignError,
{
    /// [`coco::Api`] to operate on the local monorepo.
    peer_api: coco::Api<S>,
    /// [`registry::Client`] to perform registry operations.
    registry: R,
    /// [`coco::Signer`] to access methods on the [`coco::Api`].
    signer: S,
    /// [`kv::Store`] used for session state and cache.
    store: kv::Store,
    /// Subscriptions for notification of significant events in the system.
    subscriptions: crate::notification::Subscriptions,
}

/// Wrapper around the thread-safe handle on [`Context`].
pub type Ctx<R, S> = Arc<RwLock<Context<R, S>>>;

/// Middleware filter to inject a context into a filter chain to be passed down to a handler.
#[must_use]
fn with_context<R, S>(ctx: Ctx<R, S>) -> BoxedFilter<(Ctx<R, S>,)>
where
    R: Send + Sync + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    warp::any().map(move || ctx.clone()).boxed()
}

impl<R, S> Context<R, S>
where
    R: registry::Client,
    S: coco::ResetSigner,
    S::Error: coco::SignError,
{
    /// Replaces the fields the [`Context`] holds onto in a new temporary location.
    async fn reset(&mut self) -> Result<(), crate::error::Error> {
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

        let paths =
            librad::paths::Paths::from_root(tmp_path.clone()).map_err(crate::error::Error::from)?;

        // Reset signer.
        let pw = coco::SecUtf8::from("radicle-upstream");
        self.signer
            .reset(&paths, pw)
            .map_err(crate::error::Error::Signer)?;

        // Reset peer api.
        let coco_config = coco::config::configure(paths, self.signer.clone(), vec![]);
        self.peer_api = coco::Api::new(coco_config).await?;

        // Reset store.
        self.store = kv::Store::new(kv::Config::new(tmp_path.join("store")))?;

        // Reset registry.
        let (client, _) = radicle_registry_client::Client::new_emulator();
        self.registry.reset(client);

        Ok(())
    }
}

impl Context<registry::Cacher<registry::Registry>, coco::StoreSigner> {
    #[cfg(test)]
    async fn tmp(tmp_dir: &tempfile::TempDir) -> Result<Self, crate::error::Error> {
        let paths = librad::paths::Paths::from_root(tmp_dir.path())?;

        let pw = coco::SecUtf8::from("radicle-upstream");
        let signer = coco::StoreSigner::init(&paths, pw)?;

        let peer_api = {
            let config = coco::config::default(signer.clone(), tmp_dir.path())?;
            coco::Api::new(config).await?
        };

        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            let reg = registry::Registry::new(client);
            registry::Cacher::new(reg, &store)
        };

        Ok(Self {
            peer_api,
            registry,
            signer,
            store,
            subscriptions: crate::notification::Subscriptions::default(),
        })
    }
}

impl<R, S> From<Context<R, S>> for Ctx<R, S>
where
    S: coco::Signer,
    S::Error: coco::SignError,
{
    fn from(ctx: Context<R, S>) -> Self {
        Arc::new(RwLock::new(ctx))
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

/// State filter to expose [`notification::Subscriptions`] to handlers.
#[must_use]
fn with_subscriptions(
    subscriptions: crate::notification::Subscriptions,
) -> impl Filter<Extract = (crate::notification::Subscriptions,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || crate::notification::Subscriptions::clone(&subscriptions))
}

/// Bundled input data for project registration, shared
/// between users and orgs.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterProjectInput {
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
    /// Optionally passed coco id to store for attestion.
    maybe_coco_id: Option<coco::Urn>,
}

impl ToDocumentedType for RegisterProjectInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );
        properties.insert(
            "maybeCocoId".into(),
            document::string()
                .description("Optionally passed coco id to store for attestion")
                .example("ac1cac587b49612fbac39775a07fb05c6e5de08d.git"),
        );

        document::DocumentedType::from(properties).description("Input for Project registration")
    }
}

/// Register a project in the registry under the given domain.
///
/// # Errors
///
/// Might return an http error
async fn register_project<R, S>(
    ctx: Ctx<R, S>,
    domain_type: registry::DomainType,
    domain_id: registry::Id,
    project_name: registry::ProjectName,
    input: RegisterProjectInput,
) -> Result<impl Reply, Rejection>
where
    R: registry::Client,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    // TODO(xla): Get keypair from persistent storage.
    let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

    let ctx = ctx.read().await;

    let domain = match domain_type {
        registry::DomainType::Org => registry::ProjectDomain::Org(domain_id),
        registry::DomainType::User => registry::ProjectDomain::User(domain_id),
    };

    let tx = ctx
        .registry
        .register_project(
            &fake_pair,
            domain,
            project_name,
            input.maybe_coco_id,
            input.transaction_fee,
        )
        .await?;

    ctx.subscriptions
        .broadcast(crate::notification::Notification::Transaction(tx.clone()))
        .await;

    Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
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
