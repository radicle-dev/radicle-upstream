//! Endpoints for Id.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Prefixed filters.
pub fn routes<R>(
    registry: &http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    get_status_filter(Arc::clone(registry))
}

/// Combination of all ids routes.
#[cfg(test)]
fn filters<R>(
    registry: &http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    get_status_filter(Arc::clone(registry))
}

/// `GET /<id>/status`
fn get_status_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    http::with_shared(registry)
        .and(warp::get())
        .and(document::param::<String>(
            "id",
            "The id whose status will be obtained",
        ))
        .and(path("status"))
        .and(path::end())
        .and(document::document(document::description(
            "Get the status for the given id",
        )))
        .and(document::document(
            document::response(
                200,
                document::body(Status::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::get_status)
}

/// The type of domain under which a project is registered.
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
mod handler {
    use std::convert::TryFrom;
    use warp::{reply, Rejection, Reply};

    use crate::error::Error;
    use crate::http;
    use crate::registry;

    /// Get the status for the given `id`.
    pub async fn get_status<R: registry::Client>(
        registry: http::Shared<R>,
        id_string: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let id = registry::Id::try_from(id_string).map_err(Error::from)?;
        let id_status = reg.get_id_status(&id).await?;

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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));

        let id = registry::Id::try_from("monadic")?;
        let reg = registry.read().await;
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;

        // Register the user
        registry
            .write()
            .await
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user to that it can register orgs
        registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let org_id = registry::Id::try_from("monadic")?;
        registry
            .write()
            .await
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user
        registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Unregister the user
        registry
            .write()
            .await
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        // Register the user to that it can register orgs
        registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let org_id = registry::Id::try_from("monadic")?;
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), 10)
            .await?;

        // Unregister the org
        registry
            .write()
            .await
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
