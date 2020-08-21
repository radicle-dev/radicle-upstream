//! Endpoints and serialisation for [`session::Session`] related types.

use std::collections::HashMap;
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use core::identity;
use core::session;

use crate::{Ctx, with_context};

/// Combination of all session filters.
pub fn filters(ctx: Ctx) -> BoxedFilter<(impl Reply,)>
{
    clear_cache_filter(ctx.clone())
        .or(delete_filter(ctx.clone()))
        .or(get_filter(ctx.clone()))
        .or(update_settings_filter(ctx))
        .boxed()
}

/// `DELETE /cache`
fn clear_cache_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("cache")
        .and(warp::delete())
        .and(path::end())
        .and(with_context(ctx))
        .and(document::document(document::description(
            "Clear cached data",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Cache cleared"),
        ))
        .and_then(handler::clear_cache)
}

/// `DELETE /`
fn delete_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::delete()
        .and(path::end())
        .and(with_context(ctx))
        .and(document::document(document::description(
            "Clear current Session",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Current session deleted"),
        ))
        .and_then(handler::delete)
}

/// `GET /`
fn get_filter(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::get()
        .and(path::end())
        .and(with_context(ctx))
        .and(document::document(document::description(
            "Fetch current Session",
        )))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(
                200,
                document::body(session::Session::document()).mime("application/json"),
            )
            .description("Currently active Session"),
        ))
        .and_then(handler::get)
}

/// `Post /settings`
fn update_settings_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("settings")
        .and(warp::post())
        .and(path::end())
        .and(with_context(ctx))
        .and(warp::body::json())
        .and(document::document(document::description("Update settings")))
        .and(document::document(document::tag("Session")))
        .and(document::document(
            document::response(204, None).description("Settings successfully updated"),
        ))
        .and_then(handler::update_settings)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use core::session;

    use crate::Ctx;

    /// Clear [`registry::Cache`].
    pub async fn clear_cache(ctx: Ctx) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        ctx.registry.clear()?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Clear the current [`session::Session`].
    pub async fn delete(ctx: Ctx) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        session::clear_current(&ctx.store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get(ctx: Ctx) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;

        let sess = session::current(&ctx.peer_api, &ctx.registry, &ctx.store).await?;

        Ok(reply::json(&sess))
    }

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings(
        ctx: Ctx,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        session::set_settings(&ctx.store, settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use core::error;
    use core::session;

    use crate::Context;

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;
        session::set_settings(&ctx.store, settings).unwrap();

        let res = request().method("DELETE").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Test that we reset the session to default.
        let have = session::current(&ctx.peer_api, &ctx.registry, &ctx.store)
            .await
            .unwrap()
            .settings;
        let want = session::settings::Settings::default();

        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "orgs": [],
                "settings": {
                    "appearance": {
                        "theme": "light",
                        "hints": {
                            "showRemoteHelper": true,
                        }
                    },
                    "coco": {
                        "seeds": [
                            "seed.radicle.xyz",
                        ]
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
                "registrationFee": {
                    "user": 10,
                    "org": 10,
                    "member": Value::Null,
                    "project": Value::Null,
                },
                "minimumTransactionFee": 1,
                "permissions": {
                    "registerHandle": false,
                    "registerOrg": false,
                    "registerProject": false,
                },
            }),
        );

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;

        let res = request()
            .method("POST")
            .path("/settings")
            .json(&settings)
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        let res = request().method("GET").path("/").reply(&api).await;
        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "orgs": [],
                "settings": {
                    "appearance": {
                        "theme": "dark",
                        "hints": {
                            "showRemoteHelper": true,
                        }
                    },
                    "coco": {
                        "seeds": [
                            "seed.radicle.xyz",
                        ],
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
                "registrationFee": {
                    "user": 10,
                    "org": 10,
                    "member": Value::Null,
                    "project": Value::Null,
                },
                "minimumTransactionFee": 1,
                "permissions": {
                    "registerHandle": false,
                    "registerOrg": false,
                    "registerProject": false,
                },
            }),
        );

        Ok(())
    }
}
