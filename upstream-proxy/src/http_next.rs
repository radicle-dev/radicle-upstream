// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use futures::prelude::*;

mod diagnostics;
mod identity;
mod keystore;
mod session;

pub fn serve(
    ctx: crate::context::Context,
    listen_addr: std::net::SocketAddr,
    restart_signal: impl Future<Output = ()> + Send + 'static,
) -> impl Future<Output = anyhow::Result<()>> {
    let ctx_shutdown = match &ctx {
        crate::context::Context::Sealed(sealed) => sealed.shutdown.clone(),
        crate::context::Context::Unsealed(unsealed) => unsealed.rest.shutdown.clone(),
    };

    async move {
        let router = make_router(ctx);
        let server = hyper::Server::bind(&listen_addr)
            .serve(router.into_make_service_with_connect_info::<std::net::SocketAddr>());

        server
            .with_graceful_shutdown({
                async move {
                    restart_signal.await;
                    ctx_shutdown.notify_waiters()
                }
            })
            .await?;
        Ok(())
    }
}

fn make_router(ctx: crate::context::Context) -> axum::Router {
    let legacy_api = crate::http::api(ctx.clone());
    let warp_service = tower::util::MapResponse::new(
        warp::service(legacy_api),
        |response: hyper::Response<hyper::Body>| {
            response.map(|body| {
                use warp::hyper::body::HttpBody;
                HttpBody::map_err(body, axum::Error::new).boxed_unsync()
            })
        },
    );

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Origin::predicate(|_, _| true))
        .allow_credentials(true)
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([
            http::Method::DELETE,
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::OPTIONS,
        ]);

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let handlers = axum::Router::new()
        .merge(keystore::router())
        .merge(diagnostics::router())
        .merge(identity::router())
        .merge(session::router())
        .layer(axum::Extension(ctx));

    axum::Router::new()
        .nest("/v1", handlers)
        .fallback(warp_service)
        .layer(trace_layer)
        .layer(cors)
}

/// Error type for request handlers that return [`Result`].
///
/// Error has two variants: [`Error::Internal`] is intended for errors that the client cannot
/// recover from (like failed file system access). [`Error::Custom`] is intended for errors that
/// convey some information so the client can take action (like failed authentication).
///
/// ## HTTP Response
///
/// The response body generated from `Error` is a JSON object with the following schema:
///
/// ```json
/// {
///   "variant": string,
///   "message": string,
///   "details": string | null,
/// }
/// ```
///
/// [`Error::Custom`] allows you to control the response status code and the individual fields of
/// the response object. [`Error::Internal`] wraps [`anyhow::Error`] and results in a response with
/// a 500 status code, "INTERNAL_SERVER_ERROR" in the `variant` field, the error message in the
/// `message` field and the chain of causes and the backtrace in the `details` field.
pub enum Error {
    Internal(anyhow::Error),
    Custom {
        status_code: http::StatusCode,
        /// The triggered error variant.
        variant: &'static str,
        /// Human readable message to convery error case.
        message: String,
        details: Option<String>,
    },
}

impl Error {
    pub fn internal(err: impl std::error::Error + Sync + Send + 'static) -> Self {
        Self::Internal(anyhow::Error::new(err))
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Internal(err) => {
                tracing::error!(?err, "internal server error");
                axum::response::IntoResponse::into_response((
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::response::Json(serde_json::json!({
                        "variant": "INTERNAL_SERVER_ERROR",
                        "message":err.to_string(),
                        "details": format!("{:?}",err)
                    })),
                ))
            },
            Error::Custom {
                status_code,
                variant,
                message,
                details,
            } => axum::response::IntoResponse::into_response((
                status_code,
                axum::response::Json(serde_json::json!({
                    "variant": variant,
                    "message": message,
                    "details": details,
                })),
            )),
        }
    }
}
impl From<anyhow::Error> for Error {
    fn from(inner: anyhow::Error) -> Self {
        Error::Internal(inner)
    }
}

mod extract {
    /// Extract an unsealed context. If the context is sealed the request responds with an
    /// apropriate error.
    ///
    /// Panics if there is no extension for [`crate::context::Context`]
    pub struct UnsealedContext(pub crate::context::Unsealed);

    #[axum::async_trait]
    impl axum::extract::FromRequest<axum::body::Body> for UnsealedContext {
        type Rejection = super::Error;

        async fn from_request(
            req: &mut axum::extract::RequestParts<axum::body::Body>,
        ) -> Result<Self, Self::Rejection> {
            let ctx = req
                .extensions()
                .get::<crate::context::Context>()
                .expect("context request extension not set");

            match ctx {
                crate::context::Context::Sealed(_) => Err(super::Error::Custom {
                    status_code: http::StatusCode::FORBIDDEN,
                    variant: "FORBIDDEN",
                    message: "keystore is sealed".to_string(),
                    details: None,
                }),
                crate::context::Context::Unsealed(unsealed) => {
                    Ok(UnsealedContext(unsealed.clone()))
                },
            }
        }
    }
    /// Wrapper around [`librad::git::Urn`] that can be used in a [`axum::extract::Path`] extractor.
    ///
    /// This is necessary until <https://github.com/tokio-rs/axum/pull/990> is released.
    pub struct Urn(pub librad::git::Urn);

    impl<'de> serde::Deserialize<'de> for Urn {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            {
                let s = <String as serde::Deserialize>::deserialize(deserializer)?;
                match s.parse::<librad::git::Urn>() {
                    Ok(urn) => Ok(Urn(urn)),
                    Err(err) => Err(serde::de::Error::custom(err)),
                }
            }
        }
    }

    /// Extractor that replaces [`axum::extract::Path`] and responds with [`super::Error`] if
    /// extraction fails.
    pub struct Path<T>(pub T);

    #[axum::async_trait]
    impl<T> axum::extract::FromRequest<axum::body::Body> for Path<T>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        type Rejection = super::Error;

        async fn from_request(
            req: &mut axum::extract::RequestParts<axum::body::Body>,
        ) -> Result<Self, Self::Rejection> {
            let result = axum::extract::Path::<T>::from_request(req).await;
            match result {
                Ok(path_wrapper) => Ok(Path(path_wrapper.0)),
                Err(err) => {
                    dbg!(&err);
                    Err(super::Error::Custom {
                        status_code: http::StatusCode::BAD_REQUEST,
                        variant: "INVALID_PATH_PARAMETER",
                        message: err.to_string(),
                        details: None,
                    })
                },
            }
        }
    }
}
