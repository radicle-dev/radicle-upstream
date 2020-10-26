//! Endpoints for project search requests.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    create_filter(ctx.clone()).or(list_filter(ctx)).boxed()
}

/// `PUT /<urn>`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::put())
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and_then(handler::create)
}

/// `GET /`
fn list_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::end())
        .and_then(handler::list)
}

/// Request handlers for initiating searches for projects on the network.
mod handler {
    use std::time::Instant;

    use warp::{reply, Rejection, Reply};

    use crate::context;

    /// Kick off a network request for the [`project::Project`] of the given `id`.
    pub async fn create(
        mut ctx: context::Context,
        urn: coco::Urn,
    ) -> Result<impl Reply, Rejection> {
        let request = ctx.peer_control.request_urn(&urn, Instant::now()).await;

        Ok(reply::json(&request))
    }

    /// List all project requests the current user has issued.
    pub async fn list(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        let requests = ctx.peer_control.get_project_requests().await;

        Ok(reply::json(&requests))
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use pretty_assertions::assert_eq;
    use serde_json::json;
    use warp::{http::StatusCode, test::request};

    use crate::{context, http};

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let mut ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let urn = coco::Urn::new(
            coco::Hash::hash(b"kisses-of-the-sun"),
            coco::uri::Protocol::Git,
            coco::uri::Path::empty(),
        );

        let res = request()
            .method("PUT")
            .path(&format!("/{}", urn))
            .reply(&api)
            .await;
        let want = ctx.peer_control.get_project_request(&urn).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let mut ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let urn = coco::Urn::new(
            coco::Hash::hash(b"kisses-of-the-sun"),
            coco::uri::Protocol::Git,
            coco::uri::Path::empty(),
        );

        let want = ctx.peer_control.request_urn(&urn, Instant::now()).await;
        let res = request().method("GET").path("/").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!([want]));
        });

        Ok(())
    }
}
