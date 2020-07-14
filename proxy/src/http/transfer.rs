//! Transfer funds between orgs and users

// use serde::ser::SerializeStruct as _;
// use serde::{Deserialize, Serialize, Serializer};
use serde::{Deserialize, Serialize};
// use std::sync::Arc;
// use tokio::sync::Mutex;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::notification;
use crate::registry;

/// Combination of all transfer routes.
pub fn filters<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    transfer_filter(registry, subscriptions)
}

/// `POST /transfer`
fn transfer_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    path!("transfer")
        .and(http::with_shared(registry))
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description("Transfer funds")))
        .and(document::document(document::tag("Transfer")))
        .and(document::document(
            document::body(Input::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Transaction::document()).mime("application/json"),
            )
            .description("Transfer succeeded"),
        ))
        .and_then(handler::transfer)
}

/// Transfer handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::convert::TryFrom;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::error::Error;
    use crate::http;
    use crate::notification;
    use crate::registry;

    /// Transfer funds to the given `recipient`.
    pub async fn transfer<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        input: super::Input,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // let recipient_key =
        // radicle_registry_client::ed25519::Public::from_string(input.recipient);

        let reg = registry.write().await;
        let tx = if let Some(org_id) = input.maybe_org {
            let org = registry::Id::try_from(org_id).map_err(Error::from)?;
            reg.transfer_from_org(
                &fake_pair,
                org,
                input.recipient,
                input.value,
                input.transaction_fee,
            )
            .await?
        } else {
            reg.transfer_from_user(
                &fake_pair,
                input.recipient,
                input.value,
                input.transaction_fee,
            )
            .await?
        };

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

/// Bundled input data for transfer.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// Id if sent from an Org.
    maybe_org: Option<String>,
    /// Account id of the recipient.
    recipient: radicle_registry_client::ed25519::Public,
    /// Amount that is transferred.
    value: registry::Balance,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for Input {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "maybe_org".into(),
            document::string()
                .description("ID if sent from an org")
                .example("monadic")
                .nullable(true),
        );
        properties.insert(
            "recipient".into(),
            // TODO(merle): Add correct account id example
            document::string()
                .description("Account id of the recipient")
                .example("123"),
        );
        properties.insert(
            "value".into(),
            document::string()
                .description("Amount that is transferred")
                .example(100),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for org registration")
    }
}

#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    // use serde_json::{json, Value};
    use std::convert::TryFrom;
    use std::sync::Arc;
    // use tokio::sync::{Mutex, RwLock};
    use radicle_registry_client::CryptoPair;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    // use librad::keys::SecretKey;

    use crate::error;
    use crate::notification;
    use crate::registry::{self, Client as _};

    #[tokio::test]
    async fn transfer() -> Result<(), error::Error> {
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(Arc::clone(&registry), subscriptions);
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        // Register the user
        registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let fee: registry::Balance = 100;
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Register a second user
        let author2 = radicle_registry_client::ed25519::Pair::from_legacy_string("//Bob", None);
        let handle2 = registry::Id::try_from("bob")?;
        registry
            .write()
            .await
            .register_user(&author2, handle2.clone(), None, 10)
            .await?;

        // Transfer tokens from alice to bob
        let value: registry::Balance = 10;
        let res = request()
            .method("POST")
            .path("/transfer")
            .json(&super::Input {
                recipient: author2.public(),
                maybe_org: None,
                value,
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        // Transfer tokens from the org to bob
        let value: registry::Balance = 10;
        let res = request()
            .method("POST")
            .path("/transfer")
            .json(&super::Input {
                recipient: author2.public(),
                maybe_org: Some(org_id.to_string()),
                value,
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
