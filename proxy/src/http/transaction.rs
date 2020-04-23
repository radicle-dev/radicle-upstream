//! Endpoints and serialisation for [`registry::Transaction`] related types.

use hex::ToHex;
use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::registry;

/// Combination of all transaction routes.
pub fn filters(
    registry: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter(registry)
}

/// POST /transactions
fn list_filter(
    registry: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("transactions")
        .and(warp::post())
        .and(super::with_registry(registry))
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
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::{reply, Rejection, Reply};

    use crate::registry;

    /// List all transactions.
    pub async fn list(
        reg: Arc<RwLock<registry::Registry>>,
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
        let txs = reg.read().await.list_transactions(tx_ids).await?;

        Ok(reply::json(&txs))
    }
}

impl Serialize for registry::Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Transaction", 4)?;
        state.serialize_field("id", &self.id.encode_hex::<String>())?;
        state.serialize_field("messages", &self.messages)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.end()
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
        properties.insert("state".into(), registry::TransactionState::document());
        properties.insert("timestamp".into(), timestamp);

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

impl Serialize for registry::Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OrgRegistration(org_id) => {
                let mut state = serializer.serialize_struct("OrgRegistration", 2)?;
                state.serialize_field("type", "ORG_REGISTRATION")?;
                state.serialize_field("orgId", &org_id.to_string())?;

                state.end()
            },
            Self::OrgUnregistration(org_id) => {
                let mut state = serializer.serialize_struct("OrgUnegistration", 2)?;
                state.serialize_field("type", "ORG_UNREGISTRATION")?;
                state.serialize_field("orgId", &org_id.to_string())?;

                state.end()
            },
            Self::ProjectRegistration {
                org_id,
                project_name,
            } => {
                let mut state = serializer.serialize_struct("ProjectRegistration", 3)?;
                state.serialize_field("type", "PROJECT_REGISTRATION")?;
                state.serialize_field("orgId", &org_id.to_string())?;
                state.serialize_field("projectName", &project_name.to_string())?;
                state.end()
            },
            Self::UserRegistration { handle, id } => {
                let mut state = serializer.serialize_struct("UserRegistration", 3)?;
                state.serialize_field("type", "USER_REGISTRATION")?;
                state.serialize_field("handle", &handle.to_string())?;
                state.serialize_field("id", &id.to_string())?;
                state.end()
            },
        }
    }
}

impl ToDocumentedType for registry::Message {
    fn document() -> document::DocumentedType {
        let properties = std::collections::HashMap::with_capacity(2);

        document::DocumentedType::from(properties).description("Transaction payload")
    }
}

impl Serialize for registry::TransactionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Applied(block_hash) => {
                let mut state = serializer.serialize_struct("TransactionApplied", 2)?;
                state.serialize_field("type", "APPLIED")?;
                state.serialize_field("blockHash", &block_hash.to_string())?;

                state.end()
            },
        }
    }
}

impl ToDocumentedType for registry::TransactionState {
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
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time;

    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::registry;

    #[tokio::test]
    async fn list() {
        let mut registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());

        let tx = registry::Transaction {
            id: radicle_registry_client::TxHash::random(),
            messages: vec![registry::Message::ProjectRegistration {
                project_name: radicle_registry_client::ProjectName::from_str("upstream").unwrap(),
                org_id: radicle_registry_client::OrgId::from_str("radicle").unwrap(),
            }],
            state: registry::TransactionState::Applied(radicle_registry_client::Hash::random()),
            timestamp: time::SystemTime::now(),
        };

        registry.cache_transaction(tx.clone()).await;

        let transactions = registry.list_transactions(vec![]).await.unwrap();

        let api = super::filters(Arc::new(RwLock::new(registry)));
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
