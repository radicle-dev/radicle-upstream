//! Endpoints for registry accounts.

use std::sync::Arc;
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Prefixed filters.
pub fn filters<R>(registry: &http::Shared<R>) -> BoxedFilter<(impl Reply,)>
where
    R: registry::Client + 'static,
{
    exists_filter(Arc::clone(registry))
        .or(get_balance_filter(Arc::clone(registry)))
        .boxed()
}

/// `GET /<id>/exists`
fn exists_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    http::with_shared(registry)
        .and(warp::get())
        .and(document::param::<String>("id", "Account id in SS58 format"))
        .and(path("exists"))
        .and(path::end())
        .and(document::document(document::tag("Account")))
        .and(document::document(document::description(
            "Check whether a given account exists on chain",
        )))
        .and(document::document(
            document::response(
                400,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("A bad account id was provided"),
        ))
        .and_then(handler::exists)
}

/// `GET /<id>/balance`
fn get_balance_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(warp::get())
        .and(document::param::<String>("id", "Account id in SS58 format"))
        .and(path("balance"))
        .and(path::end())
        .and(document::document(document::tag("Account")))
        .and(document::document(document::description(
            "Fetch the balance of the account from the Registry",
        )))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Balance::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                400,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("A bad account id was provided"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("Account not found"),
        ))
        .and_then(handler::get_balance)
}

/// Account handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{http::StatusCode, reply, Rejection, Reply};

    use crate::error;
    use crate::http;
    use crate::registry;

    /// Check whether the given account exists on chain
    pub async fn exists<R: registry::Client>(
        registry: http::Shared<R>,
        account_id_string: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let account_id: registry::AccountId = match registry::parse_ss58_address(&account_id_string)
        {
            Ok(x) => x,
            Err(_) => return Ok(bad_account_id_reply()),
        };

        let exists = reg.account_exists(&account_id).await?;
        Ok(warp::reply::with_status(
            reply::json(&exists),
            StatusCode::OK,
        ))
    }

    /// Get the [`registry::Balance`] of a given account.
    pub async fn get_balance<R: registry::Client>(
        registry: http::Shared<R>,
        account_id_string: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let account_id: registry::AccountId = match registry::parse_ss58_address(&account_id_string)
        {
            Ok(x) => x,
            Err(_) => return Ok(bad_account_id_reply()),
        };
        match reg.free_balance(&account_id).await {
            Ok(balance) => Ok(warp::reply::with_status(
                reply::json(&balance),
                StatusCode::OK,
            )),
            Err(error::Error::AccountNotFound(_)) => Err(warp::reject::not_found()),
            Err(other_error) => Err(Rejection::from(other_error)),
        }
    }

    fn bad_account_id_reply() -> warp::reply::WithStatus<reply::Json> {
        warp::reply::with_status(
            reply::json(&"A bad account id was provided. It needs to be in the SS58 format."),
            StatusCode::BAD_REQUEST,
        )
    }
}

#[allow(clippy::unwrap_used, clippy::all, clippy::panic)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use radicle_registry_client::CryptoPair;

    use crate::error;
    use crate::http;
    use crate::registry::{self, Client as _};

    #[tokio::test]
    async fn account_exists() -> Result<(), error::Error> {
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
        let user = registry.read().await.get_user(handle).await?.unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/{}/exists", user.account_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(true), "Account was expected to exist on chain");
        });

        Ok(())
    }

    #[tokio::test]
    async fn account_does_not_exists() -> Result<(), error::Error> {
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));
        let author =
            radicle_registry_client::ed25519::Pair::from_legacy_string("//Cloudhead", None);
        let res = request()
            .method("GET")
            .path(&format!("/{}/exists", author.public()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!(false),
                "Account was not expected to exist on chain"
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn account_exists_bad_request() -> Result<(), error::Error> {
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));
        let non_ss58_address = "abc";

        let res = request()
            .method("GET")
            .path(&format!("/{}/exists", non_ss58_address))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tokio::test]
    async fn existing_account_balance() -> Result<(), error::Error> {
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
        let user = registry.read().await.get_user(handle).await?.unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/{}/balance", user.account_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have.to_string(),
                "1152921504606846965",
                "Account doesn't have the expected amount"
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn unexisting_account_balance() -> Result<(), error::Error> {
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));
        let unkown_account =
            radicle_registry_client::ed25519::Pair::from_legacy_string("//Cloudhead", None)
                .public();

        let res = request()
            .method("GET")
            .path(&format!("/{}/balance", unkown_account.to_string()))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn account_balance_bad_request() -> Result<(), error::Error> {
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let api = super::filters(&Arc::clone(&registry));
        let non_ss58_address = "abc";

        let res = request()
            .method("GET")
            .path(&format!("/{}/balance", non_ss58_address))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
