use serde_cbor::to_vec;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::SystemTime;

use radicle_registry_client::{
    self as registry, ed25519, message, Balance, Client, ClientT, CryptoPair, Hash, OrgId,
    ProjectName, TransactionExtra, UserId, H256,
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
    /// Issue a new project registration with a given name under a given org.
    ProjectRegistration {
        /// Actual project name, unique for org.
        project_name: ProjectName,
        /// The Org in which to register the project.
        org_id: OrgId,
    },

    /// Issue a new org registration with a given id.
    #[allow(dead_code)]
    OrgRegistration(OrgId),

    /// Issue an org unregistration with a given id.
    OrgUnregistration(OrgId),

    /// Issue a user registration for a given handle storing the corresponding identity id.
    UserRegistration {
        /// Globally unique user handle.
        handle: UserId,
        /// Identity id originated from librad.
        id: registry::String32,
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
    #[must_use]
    pub const fn new(client: Client) -> Self {
        Self { client }
    }

    /// Replaces the underlying client. Useful to reset the state of an emulator client, or connect
    /// to a different nework.
    pub fn reset(&mut self, client: Client) {
        self.client = client;
    }

    /// List projects of the Registry.
    pub async fn list_projects(&self) -> Result<Vec<registry::ProjectId>, error::Error> {
        self.client.list_projects().await.map_err(|e| e.into())
    }

    /// Create a new unique Org on the Registry.
    #[allow(dead_code)]
    pub async fn register_org(
        &self,
        author: &ed25519::Pair,
        org_id: String,
        fee: Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let org_id = OrgId::try_from(org_id.clone())?;

        // Prepare and submit org registration transaction.
        let register_message = message::RegisterOrg {
            org_id: org_id.clone(),
        };
        let register_tx = registry::Transaction::new_signed(
            author,
            register_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: register_applied.tx_hash,
            messages: vec![Message::OrgRegistration(org_id)],
            state: TransactionState::Applied(register_applied.block),
            timestamp: SystemTime::now(),
        })
    }

    /// Remove a registered Org from the Registry.
    #[allow(dead_code)]
    pub async fn unregister_org(
        &self,
        author: &ed25519::Pair,
        org_id: String,
        fee: Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let org_id = OrgId::try_from(org_id.clone())?;

        // Prepare and submit org unregistration transaction.
        let unregister_message = message::UnregisterOrg {
            org_id: org_id.clone(),
        };
        let register_tx = registry::Transaction::new_signed(
            author,
            unregister_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let unregister_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: unregister_applied.tx_hash,
            messages: vec![Message::OrgUnregistration(org_id)],
            state: TransactionState::Applied(unregister_applied.block),
            timestamp: SystemTime::now(),
        })
    }

    /// Register a new project on the chain.
    pub async fn register_project(
        &self,
        author: &ed25519::Pair,
        name: String,
        org_id: String,
        maybe_project_id: Option<librad::project::ProjectId>,
        fee: Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let project_name = ProjectName::try_from(name.clone())?;
        let org_id = OrgId::try_from(org_id.clone())?;

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
                fee,
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
            project_name: project_name.clone(),
            org_id: org_id.clone(),
            checkpoint_id,
            metadata: register_metadata,
        };
        let register_tx = registry::Transaction::new_signed(
            author,
            register_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: register_applied.tx_hash,
            messages: vec![Message::ProjectRegistration {
                project_name: project_name,
                org_id: org_id,
            }],
            state: TransactionState::Applied(register_applied.block),
            timestamp: SystemTime::now(),
        })
    }

    /// Try to retrieve user from the Registry by handle.
    pub async fn get_user(&self, handle: String) -> Result<Option<String>, error::Error> {
        let user_id = UserId::try_from(handle.clone())?;
        Ok(self.client.get_user(user_id).await?.map(|_user| handle))
    }

    /// Graciously pay some tokens to the recipient out of Alices pocket.
    pub async fn prepay_account(
        &self,
        recipient: registry::AccountId,
        balance: Balance,
    ) -> Result<(), error::Error> {
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);

        let _tx_applied = self
            .client
            .sign_and_submit_message(&alice, message::Transfer { recipient, balance }, 1)
            .await?;

        Ok(())
    }

    /// Create a new unique user on the Registry.
    pub async fn register_user(
        &self,
        author: &ed25519::Pair,
        handle: String,
        id: String,
        fee: Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let user_id = UserId::try_from(handle.clone())?;
        let id = registry::String32::from_string(id)?;

        // Prepare and submit user registration transaction.
        let register_message = message::RegisterUser {
            user_id: user_id.clone(),
        };
        let register_tx = registry::Transaction::new_signed(
            author,
            register_message,
            TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: register_applied.tx_hash,
            messages: vec![Message::UserRegistration {
                handle: user_id,
                id,
            }],
            state: TransactionState::Applied(register_applied.block),
            timestamp: SystemTime::now(),
        })
    }
}

#[allow(clippy::panic, clippy::option_unwrap_used, clippy::result_unwrap_used)]
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::registry::{Metadata, Registry};
    use radicle_registry_client::{ed25519, Client, ClientT, OrgId, ProjectName};

    use serde_cbor::from_reader;

    #[test]
    fn test_register_org() {
        // Test that org registration submits valid transactions and they succeed.
        let client = Client::new_emulator();
        let registry = Registry::new(client.clone());
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);

        let result =
            futures::executor::block_on(registry.register_org(&alice, "monadic".into(), 10));
        assert!(result.is_ok());

        let org_id = OrgId::try_from("monadic").unwrap();
        let maybe_org = futures::executor::block_on(client.get_org(org_id.clone())).unwrap();
        assert!(maybe_org.is_some());
        let org = maybe_org.unwrap();
        assert_eq!(org.id, org_id);
        assert_eq!(org.members.len(), 1);
    }

    #[test]
    fn test_unregister_org() {
        // Test that org unregistration submits valid transactions and they succeed.
        let client = Client::new_emulator();
        let registry = Registry::new(client.clone());
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);

        // Register the org
        let org_id = OrgId::try_from("monadic").unwrap();
        let registration =
            futures::executor::block_on(registry.register_org(&alice, org_id.clone().into(), 10));
        assert!(registration.is_ok());

        // The org needs funds to submit transactions.
        let org = futures::executor::block_on(client.get_org(org_id))
            .unwrap()
            .unwrap();
        futures::executor::block_on(registry.prepay_account(org.account_id.clone(), 1000)).unwrap();

        // Unregister the org
        let unregistration =
            futures::executor::block_on(registry.unregister_org(&alice, "monadic".into(), 10));
        assert!(unregistration.is_ok());
    }

    #[test]
    fn test_register_project() {
        // Test that project registration submits valid transactions and they succeed.
        let client = Client::new_emulator();
        let registry = Registry::new(client.clone());
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);

        // Register the org
        let org_id = OrgId::try_from("monadic").unwrap();
        let org_result =
            futures::executor::block_on(registry.register_org(&alice, org_id.clone().into(), 10));
        assert!(org_result.is_ok());

        // The org needs funds to submit transactions.
        let org = futures::executor::block_on(client.get_org(org_id.clone()))
            .unwrap()
            .unwrap();
        futures::executor::block_on(registry.prepay_account(org.account_id.clone(), 1000)).unwrap();

        // Register the project
        let result = futures::executor::block_on(registry.register_project(
            &alice,
            "radicle".into(),
            org_id.into(),
            Some(librad::git::ProjectId::new(librad::surf::git::git2::Oid::zero()).into()),
            10,
        ));
        assert!(result.is_ok());
        let org_id = OrgId::try_from("monadic").unwrap();
        let project_name = ProjectName::try_from("radicle").unwrap();
        let future_project = client.get_project(project_name.clone(), org_id.clone());
        let maybe_project = futures::executor::block_on(future_project).unwrap();
        assert!(maybe_project.is_some());
        let project = maybe_project.unwrap();
        assert_eq!(project.name, project_name);
        assert_eq!(project.org_id, org_id);
        let metadata_vec: Vec<u8> = project.metadata.into();
        let metadata: Metadata = from_reader(&metadata_vec[..]).unwrap();
        assert_eq!(metadata.version, 1);
    }

    #[test]
    fn register_user() {
        let client = Client::new_emulator();
        let registry = Registry::new(client);
        let robo = ed25519::Pair::from_legacy_string("//Alice", None);

        let res = futures::executor::block_on(registry.register_user(
            &robo,
            "cloudhead".into(),
            "123abcd.git".into(),
            100,
        ));
        assert!(res.is_ok());
    }
}
