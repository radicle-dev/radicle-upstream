//! Endpoints and serialisation for [`registry::Transaction`] related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Combination of all transaction routes.
pub fn filters<C: registry::Cache>(
    cache: http::Shared<C>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter(cache)
}

/// `POST /transactions`
fn list_filter<C: registry::Cache>(
    cache: http::Shared<C>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("transactions")
        .and(warp::post())
        .and(http::with_container(cache))
        .and(warp::body::json())
        .and(document::document(document::description(
            "List transactions",
        )))
        .and(document::document(document::tag("Transaction")))
        .and(document::document(
            document::body(ListInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(registry::Transaction::document())
                        .description("List of transactions"),
                )
                .mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::list)
}

/// Transaction handlers to implement conversion and translation between core domain and http
/// request fullfilment.
mod handler {
    use std::str::FromStr;
    use warp::{reply, Rejection, Reply};

    use crate::http;
    use crate::registry;

    /// List all transactions.
    pub async fn list<C: registry::Cache>(
        cache: http::Shared<C>,
        input: super::ListInput,
    ) -> Result<impl Reply, Rejection> {
        let tx_ids = input
            .ids
            .iter()
            .map(|id| {
                radicle_registry_client::TxHash::from_str(id)
                    .expect("unable to get hash from string")
            })
            .collect();
        let txs = cache.read().await.list_transactions(tx_ids).await?;

        Ok(reply::json(&txs))
    }
}

impl ToDocumentedType for registry::Transaction {
    fn document() -> document::DocumentedType {
        let timestamp = {
            let mut fields = HashMap::with_capacity(2);
            fields.insert(
                "nanos_since_epoch".into(),
                document::integer()
                    .description("Nanosecond part of timestamp")
                    .example(561_320_872),
            );
            fields.insert(
                "secs_since_epoch".into(),
                document::integer()
                    .description("Seconds since epoch")
                    .example(1_586_852_801),
            );
            document::DocumentedType::from(fields).description(
                "Time since epoch, broken apart in seconds since and the leftover nanos",
            )
        };

        let mut properties = std::collections::HashMap::with_capacity(4);
        properties.insert(
            "id".into(),
            document::string()
                .description("Unique identifier")
                .example("0x7079...f93b"),
        );
        properties.insert(
            "messages".into(),
            document::array(registry::Message::document()),
        );
        properties.insert("state".into(), registry::State::document());
        properties.insert("timestamp".into(), timestamp);

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

impl ToDocumentedType for registry::Message {
    fn document() -> document::DocumentedType {
        let properties = std::collections::HashMap::with_capacity(2);

        document::DocumentedType::from(properties).description("Transaction payload")
    }
}

impl ToDocumentedType for registry::State {
    fn document() -> document::DocumentedType {
        let properties = std::collections::HashMap::with_capacity(2);

        document::DocumentedType::from(properties).description("Transaction lifecycle state")
    }
}

/// Bundled input data for a transaction listing.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInput {
    /// The transaction ids that the list should be filtered by.
    ids: Vec<String>,
}

impl ToDocumentedType for ListInput {
    fn document() -> document::DocumentedType {
        let id = document::string()
            .description("Transaction ID")
            .example("0x7079...f93b");

        let mut properties = HashMap::with_capacity(1);
        properties.insert(
            "ids".into(),
            document::array(id).description("List of transaction IDs"),
        );

        document::DocumentedType::from(properties)
            .description("Input data for a transaction list request")
    }
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use std::sync::Arc;
    use std::time;

    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::registry::{self, Cache as _};

    #[tokio::test]
    async fn list() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let mut cache = registry::Cacher::new(registry, &store);

        let tx = registry::Transaction {
            id: registry::Hash(radicle_registry_client::TxHash::random()),
            messages: vec![registry::Message::ProjectRegistration {
                project_name: registry::ProjectName::try_from("upstream").unwrap(),
                org_id: registry::Id::try_from("radicle").unwrap(),
            }],
            state: registry::State::Applied {
                block: registry::Hash(radicle_registry_client::Hash::random()),
            },
            timestamp: time::SystemTime::now(),
        };

        cache.cache_transaction(tx.clone()).await.unwrap();

        let transactions = cache.list_transactions(vec![]).await.unwrap();

        let api = super::filters(Arc::new(RwLock::new(cache)));
        let res = request()
            .method("POST")
            .path("/transactions")
            .json(&super::ListInput { ids: vec![] })
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(transactions));
    }
}
