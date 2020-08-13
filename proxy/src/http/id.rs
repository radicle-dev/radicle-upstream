//! Endpoints for Id.

use serde::{Deserialize, Serialize};
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Reply};

use crate::coco;
use crate::http;
use crate::registry;

/// `GET /<id>/status`
pub fn get_status_filter<R, S>(ctx: http::Ctx<R, S>) -> BoxedFilter<(impl Reply,)>
where
    R: registry::Client + 'static,
    S: coco::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(document::param::<String>(
            "id",
            "The id whose status will be obtained",
        ))
        .and(path("status"))
        .and(path::end())
        .and(document::document(document::tag("Id")))
        .and(document::document(document::description(
            "Fetch the availability status of the given id",
        )))
        .and(document::document(
            document::response(
                200,
                document::body(Status::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::get_status)
        .boxed()
}

/// The status of an org or user id in the Registry.
/// Only used for implementing `ToDocumentedType`.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Status;

impl ToDocumentedType for Status {
    fn document() -> document::DocumentedType {
        document::enum_string(vec!["available".into(), "taken".into(), "retired".into()])
            .description("Variants for possible id statuses.")
            .example("available")
    }
}

/// Org handlers for conversion between core domain and http request fullfilment.
#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic, warnings)]
mod handler {
    use std::convert::TryFrom;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::registry;

    /// Get the status for the given `id`.
    pub async fn get_status<R, S>(
        ctx: http::Ctx<R, S>,
        input: String,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: coco::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;
        let id = registry::Id::try_from(input).map_err(Error::from)?;
        let id_status = ctx.registry.get_id_status(&id).await?;

        Ok(reply::json(&id_status))
    }
}

#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic, warnings)]
#[cfg(test)]
mod test {
    use crate::registry::Client;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::error;
    use crate::http;
    use crate::registry;

    #[tokio::test]
    async fn get_status_available() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::get_status_filter(ctx.clone());

        let id = registry::Id::try_from("monadic")?;
        let res = request()
            .method("GET")
            .path(&format!("/{}/status", id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(registry::IdStatus::Available));
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_status_taken_by_user() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::get_status_filter(ctx.clone());

        let ctx = ctx.read().await;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;

        // Register the user
        ctx.registry
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/status", handle.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(registry::IdStatus::Taken));
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_status_taken_by_org() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::get_status_filter(ctx.clone());

        let ctx = ctx.read().await;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user so that it can register orgs
        ctx.registry
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let org_id = registry::Id::try_from("monadic")?;
        ctx.registry
            .register_org(&author, org_id.clone(), 10)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/status", org_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(registry::IdStatus::Taken));
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_status_retired_by_user() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::get_status_filter(ctx.clone());

        let ctx = ctx.read().await;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user
        ctx.registry
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Unregister the user
        ctx.registry
            .unregister_user(&author, handle.clone(), 10)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/status", handle.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(registry::IdStatus::Retired));
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_status_retired_by_org() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::get_status_filter(ctx.clone());

        let ctx = ctx.read().await;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user so that it can register orgs
        ctx.registry
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let org_id = registry::Id::try_from("monadic")?;
        ctx.registry
            .register_org(&author, org_id.clone(), 10)
            .await?;

        // Unregister the org
        ctx.registry
            .unregister_org(&author, org_id.clone(), 10)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/status", org_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(registry::IdStatus::Retired));
        });

        Ok(())
    }
}
