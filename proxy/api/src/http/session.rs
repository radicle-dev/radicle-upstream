//! Endpoints and serialisation for [`crate::session::Session`] related types.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all session filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    delete_filter(ctx.clone())
        .or(get_filter(ctx.clone()))
        .or(unseal_filter(ctx.clone()))
        .or(update_settings_filter(ctx))
        .boxed()
}

/// `DELETE /`
fn delete_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(path::end())
        .and(http::with_context_unsealed(ctx))
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
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::update_settings)
}

/// `POST /unseal`
fn unseal_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("unseal")
        .and(warp::post())
        .and(path::end())
        .and(http::with_context(ctx))
        .and_then(handler::unseal)
}

/// Session handlers for conversion between core domain and HTTP request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::{context, error, session};

    pub async fn unseal(ctx: context::Context) -> Result<impl Reply, Rejection> {
        match ctx {
            context::Context::Unsealed(unsealed) => {},
            context::Context::Sealed(mut sealed) => {
                // let key = coco::keystore::Keystorage::file(
                //     &sealed.paths,
                //     coco::keystore::SecUtf8::from("my-secret-key"),
                // )
                // .init()
                // .map_err(crate::error::Error::from)?;
                let key = coco::keys::SecretKey::new();
                sealed.service_handle.set_secret_key(key);
            },
        }
        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Clear the current [`session::Session`].
    pub async fn delete(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        session::clear_current(&ctx.store)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Fetch the [`session::Session`].
    pub async fn get(ctx: context::Context) -> Result<impl Reply, Rejection> {
        match ctx {
            context::Context::Unsealed(unsealed) => {
                let sess = session::current(unsealed.state.clone(), &unsealed.store).await?;

                Ok(reply::json(&sess))
            },
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
    use serde_json::{json, Value};
    use warp::{http::StatusCode, test::request};

    use crate::{context, error, session};

    #[tokio::test]
    async fn delete() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Unsealed::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone().into());

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
        let ctx = context::Unsealed::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone().into());

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
                        "seeds": ["hybh5cb7spafgs7skjg6qkssts3uxht31zskpgs4ypdzrnaq7ye83k@seedling.radicle.xyz:12345"],
                    },
                },
            }),
        );

        Ok(())
    }

    #[tokio::test]
    async fn update_settings() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Unsealed::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone().into());

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
                        "seeds": ["hybh5cb7spafgs7skjg6qkssts3uxht31zskpgs4ypdzrnaq7ye83k@seedling.radicle.xyz:12345"],
                    },
                },
            }),
        );

        Ok(())
    }
}
