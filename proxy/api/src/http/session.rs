//! Endpoints and serialisation for [`crate::session::Session`] related types.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all session filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    get_filter(ctx.clone())
        .or(update_settings_filter(ctx))
        .boxed()
}

/// `GET /`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::get())
        .and(http::with_context(ctx))
        .and_then(handler::get)
}

/// `Post /settings`
fn update_settings_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("settings")
        .and(path::end())
        .and(warp::post())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::update_settings)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, http, session};

    /// Fetch the [`session::Session`].
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        let sess =
            crate::session::get_current(ctx.store())?.ok_or(http::error::Routing::NoSession)?;
        match ctx {
            context::Context::Unsealed(_) => Ok(reply::json(&sess)),
            context::Context::Sealed(_) => Err(Rejection::from(error::Error::KeystoreSealed)),
        }
    }

    /// Set the [`session::settings::Settings`] to the passed value.
    pub async fn update_settings(
        ctx: context::Unsealed,
        settings: session::settings::Settings,
    ) -> Result<impl Reply, Rejection> {
        session::set_settings(&ctx.store, settings)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::{http::StatusCode, test::request};

    use crate::{context, session};

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        let session = session::initialize_test(&ctx, "xla").await;

        let res = request().method("GET").path("/").reply(&api).await;
        assert_eq!(res.status(), StatusCode::OK);

        let session_response = serde_json::from_slice::<session::Session>(res.body())?;

        assert_eq!(session_response, session);

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        session::initialize_test(&ctx, "cloudhead").await;

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
        let session_res = serde_json::from_slice::<session::Session>(res.body())?;
        assert_eq!(session_res.settings, settings);
        Ok(())
    }
}
