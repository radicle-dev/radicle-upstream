//! Endpoints for project search requests.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use radicle_daemon::Urn;

use crate::{context, http};

/// Combination of all routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    cancel_filter(ctx.clone())
        .or(create_filter(ctx.clone()))
        .or(list_filter(ctx))
        .boxed()
}

/// `DELETE /<urn>`
fn cancel_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path::end())
        .and(http::with_context_unsealed(ctx))
        .and(warp::delete())
        .and_then(handler::cancel)
}

/// `PUT /<urn>`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path::end())
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::create)
}

/// `GET /`
fn list_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::list)
}

/// Request handlers for initiating searches for projects on the network.
mod handler {
    use std::time::SystemTime;

    use warp::{http::StatusCode, reply, Rejection, Reply};

    use radicle_daemon::Urn;

    use crate::{context, error};

    /// Abort search for an ongoing request.
    pub async fn cancel(urn: Urn, mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        ctx.peer_control
            .cancel_project_request(&urn, SystemTime::now())
            .await
            .map_err(error::Error::from)?;

        Ok(reply::with_status(reply(), StatusCode::NO_CONTENT))
    }

    /// Kick off a network request for the [`crate::project::Project`] of the given `id`.
    ///
    /// FIXME(xla): Endpoint ought to return `201` if the request was newly created, otherwise
    /// `200` if there was a request present for the urn.
    pub async fn create(urn: Urn, mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let request = ctx
            .peer_control
            .request_project(&urn, SystemTime::now())
            .await;

        Ok(reply::json(&request))
    }

    /// List all project requests the current user has issued.
    pub async fn list(mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let requests = ctx.peer_control.get_project_requests().await;

        Ok(reply::json(&requests))
    }
}

#[cfg(test)]
mod test {
    use std::{convert::TryFrom as _, time::SystemTime};

    use pretty_assertions::assert_eq;
    use serde_json::json;
    use warp::{http::StatusCode, test::request};

    use radicle_daemon::{git_ext, Urn};

    use crate::{context, http};

    #[tokio::test]
    async fn cancel() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (mut ctx, run) = context::Unsealed::tmp(&tmp_dir)?;
        let handle = tokio::spawn(run);
        let api = super::filters(ctx.clone().into());

        let urn = Urn::new(git_ext::Oid::try_from(
            "7ab8629dd6da14dcacde7f65b3d58cd291d7e235",
        )?);

        let _request = ctx
            .peer_control
            .request_project(&urn, SystemTime::now())
            .await;
        let res = request()
            .method("DELETE")
            .path(&format!("/{}", urn))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        handle.abort();

        Ok(())
    }

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (mut ctx, run) = context::Unsealed::tmp(&tmp_dir)?;
        let handle = tokio::spawn(run);
        let api = super::filters(ctx.clone().into());

        let urn = Urn::new(git_ext::Oid::try_from(
            "7ab8629dd6da14dcacde7f65b3d58cd291d7e235",
        )?);

        let res = request()
            .method("PUT")
            .path(&format!("/{}", urn))
            .reply(&api)
            .await;
        let want = ctx.peer_control.get_project_request(&urn).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });
        handle.abort();

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (mut ctx, run) = context::Unsealed::tmp(&tmp_dir)?;
        let handle = tokio::spawn(run);
        let api = super::filters(ctx.clone().into());

        let urn = Urn::new(git_ext::Oid::try_from(
            "7ab8629dd6da14dcacde7f65b3d58cd291d7e235",
        )?);

        let want = ctx
            .peer_control
            .request_project(&urn, SystemTime::now())
            .await;
        let res = request().method("GET").path("/").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!([want]));
        });
        handle.abort();

        Ok(())
    }
}
