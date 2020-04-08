//! Endpoints and serialisation for [`registry::Transaction`] related types.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
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

#[derive(Deserialize, Serialize)]
pub struct ListInput {
    ids: Vec<String>,
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
