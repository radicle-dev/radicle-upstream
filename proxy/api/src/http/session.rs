//! Endpoints and serialisation for [`crate::session::Session`] related types.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all session filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    delete_filter(ctx.clone())
        .or(get_filter(ctx.clone()))
        .or(update_settings_filter(ctx))
        .boxed()
}

/// `DELETE /`
fn delete_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path::end())
        .and(http::with_context(ctx))
        .and_then(handler::delete)
}

/// `GET /`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path::end())
        .and(http::with_context(ctx))
        .and_then(handler::get)
}

/// `Post /settings`
fn update_settings_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("settings")
        .and(warp::post())
        .and(path::end())
        .and(http::with_session_context(ctx))
        .and(warp::body::json())
        .and_then(handler::update_settings)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, session};

    /// Clear the current [`session::Session`].
    pub async fn delete(ctx: context::Context) -> Result<impl Reply, Rejection> {
        session::clear_current(&ctx.store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        if let Some(session) = session::get_current(&ctx.store)? {
            Ok(reply::json(&session).into_response())
        } else {
            Ok(warp::reply::with_status(
                reply::json(&serde_json::Value::Null),
                StatusCode::NOT_FOUND,
            )
            .into_response())
        }
    }

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings(
        ctx: session::Context,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection> {
        ctx.set_settings(settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::{http::StatusCode, test::request};

    use crate::{context, error, session};

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let session_ctx = session::initialize_test(&ctx, "cloudhead").await;
        let api = super::filters(ctx.clone());

        let mut settings = session::settings::Settings::default();
        settings.appearance.theme = session::settings::Theme::Dark;
        session_ctx.set_settings(settings)?;

        let res = request().method("DELETE").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        assert_eq!(session::get_current(&ctx.store)?.is_none(), true);
        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let session_context = session::initialize_test(&ctx, "xla").await;
        let api = super::filters(ctx.clone());

        let res = request().method("GET").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::OK);

        let session = serde_json::from_slice::<session::Session>(res.body())?;

        assert_eq!(session, session_context.session);

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        session::initialize_test(&ctx, "xla").await;
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
        assert_eq!(res.status(), StatusCode::OK);

        let session_res = serde_json::from_slice::<session::Session>(res.body())?;
        assert_eq!(session_res.settings, settings);
        Ok(())
    }
}
