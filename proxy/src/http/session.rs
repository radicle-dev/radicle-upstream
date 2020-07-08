//! Endpoints and serialisation for [`session::Session`] related types.

use std::collections::HashMap;
use std::sync::Arc;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::identity;
use crate::registry;
use crate::session;

/// Prefixed fitlers.
pub fn routes<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    path("session").and(
        clear_cache_filter(Arc::clone(&ctx))
            .or(delete_filter(Arc::clone(&ctx)))
            .or(get_filter(Arc::clone(&ctx)))
            .or(update_settings_filter(ctx)),
    )
}

/// Combination of all session filters.
#[cfg(test)]
pub fn filters<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    clear_cache_filter(Arc::clone(&ctx))
        .or(delete_filter(Arc::clone(&ctx)))
        .or(get_filter(Arc::clone(&ctx)))
        .or(update_settings_filter(ctx))
}

/// `DELETE /cache`
fn clear_cache_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    path("cache")
        .and(warp::delete())
        .and(path::end())
        .and(http::with_context(ctx))
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
fn delete_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    warp::delete()
        .and(path::end())
        .and(http::with_context(ctx))
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
fn get_filter<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    warp::get()
        .and(path::end())
        .and(http::with_context(ctx))
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
fn update_settings_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: http::Registry,
{
    path("settings")
        .and(warp::post())
        .and(path::end())
        .and(http::with_context(ctx))
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

    use crate::http;
    use crate::session;

    /// Clear [`registry::Cache`].
    pub async fn clear_cache<R>(ctx: http::Ctx<R>) -> Result<impl Reply, Rejection>
    where
        R: http::Registry,
    {
        let ctx = ctx.lock().await;
        ctx.registry.clear()?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Clear the current [`session::Session`].
    pub async fn delete<R>(ctx: http::Ctx<R>) -> Result<impl Reply, Rejection>
    where
        R: http::Registry,
    {
        let ctx = ctx.lock().await;
        session::clear_current(&ctx.store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get<R>(ctx: http::Ctx<R>) -> Result<impl Reply, Rejection>
    where
        R: http::Registry,
    {
        let ctx = ctx.lock().await;

        let sess = session::current(&ctx.peer_api, &ctx.registry, &ctx.store).await?;

        Ok(reply::json(&sess))
    }

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings<R>(
        ctx: http::Ctx<R>,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection>
    where
        R: http::Registry,
    {
        let ctx = ctx.lock().await;
        session::set_settings(&ctx.store, settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

impl ToDocumentedType for session::Session {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert(
            "identity".into(),
            identity::Identity::document().nullable(true),
        );
        properties.insert("orgs".into(), document::array(registry::Org::document()));
        properties.insert("settings".into(), session::settings::Settings::document());

        document::DocumentedType::from(properties).description("Session")
    }
}

impl ToDocumentedType for session::settings::Settings {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "appearance".into(),
            session::settings::Appearance::document(),
        );
        properties.insert("registry".into(), session::settings::Registry::document());

        document::DocumentedType::from(properties).description("Settings")
    }
}

impl ToDocumentedType for session::settings::Appearance {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert("theme".into(), session::settings::Theme::document());

        document::DocumentedType::from(properties).description("Appearance")
    }
}

impl ToDocumentedType for session::settings::Theme {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["dark".into(), "light".into()])
            .description("Variants for possible color schemes.")
            .example("dark")
    }
}

impl ToDocumentedType for session::settings::Registry {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(1);
        properties.insert("network".into(), session::settings::Network::document());

        document::DocumentedType::from(properties).description("Registry")
    }
}

impl ToDocumentedType for session::settings::Network {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["emulator".into(), "ffnet".into(), "testnet".into()])
            .description("Variants for possible networks of the Registry to connect to.")
            .example("testnet")
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::error;
    use crate::http;
    use crate::session;

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

        let ctx = ctx.lock().await;
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
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

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
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
                "transactionDeposits": {
                    "memberRegistration": 10,
                    "orgRegistration": 10,
                    "projectRegistration": 10,
                    "userRegistration": 10,
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
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

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
                    },
                    "registry": {
                        "network": "emulator",
                    },
                },
                "transactionDeposits": {
                    "memberRegistration": 10,
                    "orgRegistration": 10,
                    "projectRegistration": 10,
                    "userRegistration": 10,
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
