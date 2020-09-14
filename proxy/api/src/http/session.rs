//! Endpoints and serialisation for [`crate::session::Session`] related types.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all session filters.
pub fn filters(ctx: context::Ctx) -> BoxedFilter<(impl Reply,)> {
    delete_filter(ctx.clone())
        .or(get_filter(ctx.clone()))
        .or(update_settings_filter(ctx))
        .boxed()
}

/// `DELETE /`
fn delete_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path::end())
        .and(http::with_context(ctx))
        .and_then(handler::delete)
}

/// `GET /`
fn get_filter(ctx: context::Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and(http::with_context(ctx))
        .and_then(handler::get)
}

/// `Post /settings`
fn update_settings_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("settings")
        .and(warp::post())
        .and(path::end())
        .and(http::with_context(ctx))
        .and(warp::body::json())
        .and_then(handler::update_settings)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, session};

    /// Clear the current [`session::Session`].
    pub async fn delete(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        session::clear_current(&ctx.store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let sess = session::current(ctx.state.clone(), &ctx.store).await?;

        Ok(reply::json(&sess))
    }

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings(
        ctx: context::Ctx,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        session::set_settings(&ctx.store, settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::{http::StatusCode, test::request};

    use crate::{context, error, session};

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;

        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;
        session::set_settings(&ctx.store, settings)?;

        let res = request().method("DELETE").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Test that we reset the session to default.
        let have = session::current(ctx.state.clone(), &ctx.store)
            .await?
            .settings;
        let want = session::settings::Settings::default();

        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let res = request().method("GET").path("/").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body())?;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "settings": {
                    "appearance": {
                        "theme": "light",
                        "hints": {
                            "showRemoteHelper": true,
                        }
                    },
                    "coco": {
                        "seeds": [],
                    },
                },
            }),
        );

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
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
        let have: Value = serde_json::from_slice(res.body())?;
        assert_eq!(
            have,
            json!({
                "identity": Value::Null,
                "settings": {
                    "appearance": {
                        "theme": "dark",
                        "hints": {
                            "showRemoteHelper": true,
                        }
                    },
                    "coco": {
                        "seeds": [],
                    },
                },
            }),
        );

        Ok(())
    }
}
