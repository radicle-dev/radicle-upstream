use radicle_registry_client::{
    self as registry, ed25519, message, Client, ClientT, CryptoPair, Hash, String32,
    TransactionExtra, H256,
};
use std::time::SystemTime;

use super::error;

/// A container to dissiminate and apply operations on the [`Registry`].
pub struct Transaction {
    /// Unique identifier, in actuality the Hash of the transaction. This handle should be used by
    /// the API consumer to query state changes of a transaction.
    pub id: registry::TxHash,
    /// List of operations to be applied to the Registry. Currently limited to exactly one. We use
    /// a Vec here to accommodate future extensibility.
    pub messages: Vec<Message>,
    /// Current state of the transaction.
    pub state: TransactionState,
    /// Creation time.
    pub timestamp: SystemTime,
}

/// Possible messages a [`Transaction`] can carry.
pub enum Message {
    /// Issue a new project registration with (domain, name).
    ProjectRegistration { domain: String32, name: String32 },
}

/// Possible states a [`Transaction`] can have. Useful to reason about the lifecycle and
/// whereabouts of a given [`Transaction`].
pub enum TransactionState {
    /// [`Transaction`] has been applied to a block, carries the hash of the block.
    Applied(Hash),
}

/// Registry client wrapper.
#[derive(Clone)]
pub struct Registry {
    /// Registry client, whether an emulator or otherwise.
    client: Client,
}

/// Registry client wrapper methods
impl Registry {
    /// Wrap a registry client.
    pub const fn new(client: Client) -> Self {
        Self { client }
    }

    /// List projects of the Registry.
    pub async fn list_projects(&self) -> Result<Vec<registry::ProjectId>, error::Error> {
        self.client.list_projects().await.map_err(|e| e.into())
    }

    /// Register a new project on the chain.
    pub async fn register_project(
        &self,
        author: &ed25519::Pair,
        domain: String,
        name: String,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let project_name =
            String32::from_string(name.clone()).map_err(error::ProjectValidation::NameTooLong)?;
        let project_domain = String32::from_string(domain.clone())
            .map_err(error::ProjectValidation::DomainTooLong)?;

        // Prepare and submit checkpoint transaction.
        let checkpoint_message = message::CreateCheckpoint {
            project_hash: H256::random(),
            previous_checkpoint_id: None,
        };
        let checkpoint_tx = registry::Transaction::new_signed(
            author,
            checkpoint_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
            },
        );
        let checkpoint_id = self
            .client
            .submit_transaction(checkpoint_tx)
            .await?
            .await?
            .result?;

        // Prepare and submit project registration transaction.
        let register_message = message::RegisterProject {
            id: (project_name.clone(), project_domain.clone()),
            checkpoint_id,
            metadata: registry::Bytes128::from_vec(vec![]).expect("unable construct metadata"),
        };
        let register_tx = registry::Transaction::new_signed(
            author,
            register_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: register_applied.tx_hash,
            messages: vec![Message::ProjectRegistration {
                domain: project_domain,
                name: project_name,
            }],
            state: TransactionState::Applied(register_applied.block),
            timestamp: SystemTime::now(),
        })
    }
}

#[test]
fn test_register_project() {
    use radicle_registry_client::Client;

    // Test that project registration submits valid transactions and they succeed.
    let client = Client::new_emulator();
    let registry = Registry::new(client);
    let alice = ed25519::Pair::from_legacy_string("//Alice", None);
    let result = futures::executor::block_on(registry.register_project(
        &alice,
        "hello".into(),
        "world".into(),
    ));
    assert!(result.is_ok());
}
