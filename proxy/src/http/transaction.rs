//! Endpoints and serialisation for [`registry::Transaction`] related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Combination of all transaction routes.
pub fn filters<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + 'static,
{
    list_filter(ctx)
}

/// `POST /transactions`
fn list_filter<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Cache + 'static,
{
    path!("transactions")
        .and(warp::post())
        .and(http::with_context(ctx))
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
    pub async fn list<R>(
        ctx: http::Ctx<R>,
        input: super::ListInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Cache,
    {
        // TODO(xla): Don't panic when trying to convert ids.
        let tx_ids = input
            .ids
            .iter()
            .map(|id| {
                radicle_registry_client::TxHash::from_str(id)
                    .expect("unable to get hash from string")
            })
            .collect();
        let ctx = ctx.read().await;
        let txs = ctx.registry.list_transactions(tx_ids)?;

        Ok(reply::json(&txs))
    }
}

impl ToDocumentedType for registry::Transaction {
    fn document() -> document::DocumentedType {
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
        properties.insert("timestamp".into(), registry::Timestamp::document());

        document::DocumentedType::from(properties).description("A transaction on the Registry")
    }
}

impl ToDocumentedType for registry::Timestamp {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(2);
        properties.insert(
            "nanos".into(),
            document::integer()
                .description("Sub-second nano part")
                .example(561_320_872),
        );
        properties.insert(
            "secs".into(),
            document::integer()
                .description("Seconds since UNIX epoch")
                .example(1_586_852_801),
        );

        document::DocumentedType::from(properties)
            .description("Time since epoch, broken apart in seconds since and the leftover nanos")
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
        let confirmed = {
            let mut fields = HashMap::with_capacity(3);
            fields.insert(
                "block".into(),
                document::integer()
                    .description("The height of the block the transaction has been included in")
                    .example(13),
            );
            fields.insert(
                "confirmations".into(),
                document::integer()
                    .description(
                        "Amount of blocks that have been mined on top of the confirmed block",
                    )
                    .example(4),
            );
            fields.insert("timestamp".into(), registry::Timestamp::document());

            document::DocumentedType::from(fields)
                .description("Transaction has been included and waits for settlement")
        };
        let failed = {
            let mut fields = HashMap::with_capacity(2);
            fields.insert(
                "error".into(),
                document::string()
                    .description("Description of the error")
                    .example("Org exists"),
            );
            fields.insert("timestamp".into(), registry::Timestamp::document());

            document::DocumentedType::from(fields).description("Transaction failed")
        };
        let pending = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert("timestamp".into(), registry::Timestamp::document());

            document::DocumentedType::from(fields)
                .description("Transaction has been sent and waits for confirmation")
        };
        let settled = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert("timestamp".into(), registry::Timestamp::document());

            document::DocumentedType::from(fields).description(
                "Transaction has been settle and is mathematically unlikely to be rejected",
            )
        };

        document::one_of(vec![confirmed, failed, pending, settled])
            .description("Transaction lifecycle state")
            .example(Self::Settled {
                min_confirmations: registry::MIN_CONFIRMATIONS,
                timestamp: registry::Timestamp::now(),
            })
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::error;
    use crate::http;
    use crate::registry::{self, Cache as _};

    #[tokio::test]
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let now = registry::Timestamp::now();
        let fee = registry::MINIMUM_TX_FEE;

        let org_id = registry::Id::try_from("radicle").unwrap();

        let tx = registry::Transaction {
            id: registry::Hash(radicle_registry_client::TxHash::random()),
            messages: vec![registry::Message::ProjectRegistration {
                project_name: registry::ProjectName::try_from("upstream").unwrap(),
                domain_type: registry::DomainType::Org,
                domain_id: org_id,
            }],
            state: registry::State::Confirmed {
                block: 1,
                confirmations: 1,
                min_confirmations: registry::MIN_CONFIRMATIONS,
                timestamp: now,
            },
            timestamp: now,
            fee,
        };

        let ctx = ctx.read().await;
        ctx.registry.cache_transaction(tx.clone()).unwrap();

        let transactions = ctx.registry.list_transactions(vec![]).unwrap();

        let res = request()
            .method("POST")
            .path("/transactions")
            .json(&super::ListInput { ids: vec![] })
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(transactions));

        Ok(())
    }
}
