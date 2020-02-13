use serde_cbor::to_vec;
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

use radicle_registry_client::{
    self as registry, ed25519, message, Client, ClientT, CryptoPair, Hash, String32,
    TransactionExtra, H256,
};

use crate::error;

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

/// `ProjectID` wrapper for serde de/serialization
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    /// Librad project ID.
    pub id: String,
    /// Metadata version.
    pub version: u8,
}

/// Possible messages a [`Transaction`] can carry.
pub enum Message {
    /// Issue a new project registration with (domain, name).
    ProjectRegistration {
        /// Actual project name, unique for domain.
        name: String32,
        /// Domain part of the project id.
        org_id: String32,
    },
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
        name: String,
        org_id: String,
        maybe_project_id: Option<librad::project::ProjectId>,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let project_name =
            String32::from_string(name.clone()).map_err(error::ProjectValidation::NameTooLong)?;
        let org_id =
            String32::from_string(org_id.clone()).map_err(error::ProjectValidation::OrgTooLong)?;

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

        let register_metadata_vec = if let Some(pid_string) = maybe_project_id {
            let pid_cbor = Metadata {
                id: pid_string.to_string(),
                version: 1,
            };
            // TODO(garbados): unpanic
            to_vec(&pid_cbor).expect("unable to serialize project metadata")
        } else {
            vec![]
        };

        // TODO: remove .expect() call, see: https://github.com/radicle-dev/radicle-registry/issues/185
        let register_metadata =
            registry::Bytes128::from_vec(register_metadata_vec).expect("unable construct metadata");

        // Prepare and submit project registration transaction.
        let register_message = message::RegisterProject {
            id: (org_id.clone(), project_name.clone()),
            checkpoint_id,
            metadata: register_metadata,
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
                name: project_name,
                org_id: org_id,
            }],
            state: TransactionState::Applied(register_applied.block),
            timestamp: SystemTime::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::{Metadata, Registry};
    use radicle_registry_client::ed25519;
    use radicle_registry_client::{Client, ClientT, String32};
    use serde_cbor::from_reader;

    #[test]
    fn test_register_project() {
        // Test that project registration submits valid transactions and they succeed.
        let client = Client::new_emulator();
        let registry = Registry::new(client.clone());
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);
        let result = futures::executor::block_on(registry.register_project(
            &alice,
            "radicle".into(),
            "monadic".into(),
            Some(librad::git::ProjectId::new(librad::surf::git::git2::Oid::zero()).into()),
        ));
        assert!(result.is_ok());
        let org_id = String32::from_string("monadic".into()).unwrap();
        let project_name = String32::from_string("radicle".into()).unwrap();
        let pid = (org_id, project_name);
        let future_project = client.get_project(pid);
        let maybe_project = futures::executor::block_on(future_project).unwrap();
        assert!(maybe_project.is_some());
        let project = maybe_project.unwrap();
        let metadata_vec: Vec<u8> = project.metadata.into();
        let metadata: Metadata = from_reader(&metadata_vec[..]).unwrap();
        assert_eq!(metadata.version, 1);
    }
}
