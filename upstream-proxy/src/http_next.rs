// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use futures::prelude::*;

mod diagnostics;
mod keystore;

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
        .layer(axum::Extension(ctx));

    axum::Router::new()
        .nest("/v1", handlers)
        .fallback(warp_service)
        .layer(trace_layer)
        .layer(cors)
}

/// Wrapper for [`anyhow::Error`] that implements [`axum::response::IntoResponse`]. The HTTP
/// response has status code 500 and the following body.
///
/// ```json
/// {
///   "variant": "INTERNAL_SERVER_ERROR",
///   "message": "<error message>",
///   "details": "<error message and list of causes>",
/// }
/// ```
struct Error {
    inner: anyhow::Error,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(err = ?self.inner, "internal server error");
        axum::response::IntoResponse::into_response((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::response::Json(serde_json::json!({
                "variant": "INTERNAL_SERVER_ERROR",
                "message": self.inner.to_string(),
                "details": format!("{:?}", self.inner)
            })),
        ))
    }
}
impl<T: std::error::Error + Send + Sync + 'static> From<T> for Error {
    fn from(err: T) -> Self {
        Error {
            inner: anyhow::Error::from(err),
        }
    }
}

/// Extract an unsealed context. If the context is sealed the request responds with an apropriate
/// error.
///
/// Panics if there is no extension for [`crate::context::Context`]
struct ExtractUnsealedContext(crate::context::Unsealed);

#[axum::async_trait]
impl axum::extract::FromRequest<axum::body::Body> for ExtractUnsealedContext {
    type Rejection = Error;

    async fn from_request(
        req: &mut axum::extract::RequestParts<axum::body::Body>,
    ) -> Result<Self, Self::Rejection> {
        let ctx = req
            .extensions()
            .get::<crate::context::Context>()
            .expect("context request extension not set");

        match ctx {
            crate::context::Context::Sealed(_) => Err(crate::error::Error::KeystoreSealed.into()),
            crate::context::Context::Unsealed(unsealed) => {
                Ok(ExtractUnsealedContext(unsealed.clone()))
            },
        }
    }
}
