use radicle_registry_client::{
    self as registry, ed25519, message, Client, ClientT, CryptoPair, String32, TransactionExtra,
    H256,
};
use serde_cbor::to_vec;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::schema::error::{Error, ProjectValidation};

/// A container to dissiminate and apply operations on the [`Registry`].
#[derive(GraphQLObject)]
pub struct Transaction {
    // TODO(xla): Use actual transaction hash type.
    /// Unique identifier, in actuality the Hash of the transaction. This handle should be used by
    /// the API consumer to query state changes of a transaction.
    pub id: juniper::ID,
    /// List of operations to be applied to the Registry. Currently limited to exactly one. We use
    /// a Vec here to accommodate future extensibility.
    pub messages: Vec<Message>,
    /// Current state of the transaction.
    pub state: TransactionState,
    /// Creation time.
    pub timestamp: String,
}

/// Required information to issue a new project registration on the [`Registry`].
#[derive(GraphQLObject)]
pub struct ProjectRegistration {
    // TODO(xla): Use String32 type.
    /// The domain the project should be registered for.
    pub domain: String,
    // TODO(xla): Use String32 type.
    /// The name of the project, which MUST be unique for the domain.
    pub name: String,
}

/// `ProjectID` wrapper for serde de/serialization
#[derive(Serialize, Deserialize)]
pub struct ProjectCbor {
    /// TODO
    pub id: String,
    /// TODO
    pub version: u8,
}

/// Possible messages a [`Transaction`] can carry.
pub enum Message {
    /// Issue a new project registration. See [`ProjectRegistration`] for the shape of the data.
    ProjectRegistration(ProjectRegistration),
}

juniper::graphql_union!(Message: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &ProjectRegistration => match *self { Message::ProjectRegistration(ref p) => Some(p) },
    }
});

/// Possible states a [`Transaction`] can have. Useful to reason about the lifecycle and
/// whereabouts of a given [`Transaction`].
pub enum TransactionState {
    /// [`Transaction`] has been applied to a block. See [`Applied`] for the shape of the data.
    Applied(Applied),
}

juniper::graphql_union!(TransactionState: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &Applied => match *self { TransactionState::Applied(ref a) => Some(a) },
    }
});

/// Signals that [`Transaction`] has been successfully applied by a Node to a block. Carries the
/// hash of the Block for further inspection and/or tracking of propagation.
#[derive(GraphQLObject)]
pub struct Applied {
    /// Block hash the [`Transaction`] has been applied to.
    pub block: juniper::ID,
}

/// Registry client wrapper.
#[derive(Clone)]
pub struct Registry {
    /// Registry client, whether an emulator or otherwise.
    pub client: Client,
}

/// Registry client wrapper methods
impl Registry {
    /// Wrap a registry client.
    pub const fn new(client: Client) -> Self {
        Self { client }
    }

    /// List projects of the Registry.
    pub async fn list_projects(&self) -> Result<Vec<registry::ProjectId>, Error> {
        self.client.list_projects().await.map_err(|e| e.into())
    }

    /// Register a new project on the chain.
    pub async fn register_project(
        &self,
        author: &ed25519::Pair,
        domain: String,
        name: String,
        maybe_pid: Option<String>,
    ) -> Result<Transaction, Error> {
        // Verify that inputs are valid.
        let project_name =
            String32::from_string(name.clone()).map_err(ProjectValidation::NameTooLong)?;
        let project_domain =
            String32::from_string(domain.clone()).map_err(ProjectValidation::DomainTooLong)?;

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

        let register_metadata_vec = if let Some(pid_string) = maybe_pid {
            let pid_cbor = ProjectCbor {
                id: pid_string,
                version: 1,
            };
            to_vec(&pid_cbor).expect("unable to serialize project metadata")
        } else {
            vec![]
        };

        // TODO: remove .expect() call, see: https://github.com/radicle-dev/radicle-registry/issues/185
        let register_metadata =
            registry::Bytes128::from_vec(register_metadata_vec).expect("unable construct metadata");

        // Prepare and submit project registration transaction.
        let register_message = message::RegisterProject {
            id: (project_name, project_domain),
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
            id: juniper::ID::new(register_applied.tx_hash.to_string()),
            messages: vec![Message::ProjectRegistration(ProjectRegistration {
                domain,
                name,
            })],
            state: TransactionState::Applied(Applied {
                block: juniper::ID::new(register_applied.block.to_string()),
            }),
            timestamp: radicle_surf::git::git2::Time::new(
                i64::try_from(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs(),
                )?,
                0,
            )
            .seconds()
            .to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::registry::{ProjectCbor, Registry};
    use radicle_registry_client::ed25519;
    use radicle_registry_client::{Client, ClientT, String32};
    use serde_cbor::from_reader;

    #[test]
    fn test_register_project() {
        // Test that project registration submits valid transactions and they succeed.
        let client = Client::new_emulator();
        let registry = Registry::new(client);
        let alice = ed25519::Pair::from_legacy_string("//Alice", None);
        let result = futures::executor::block_on(registry.register_project(
            &alice,
            "hello".into(),
            "world".into(),
            Some(String::from("radicle")),
        ));
        assert!(result.is_ok());
        let project_domain = String32::from_string("hello".into()).unwrap();
        let project_name = String32::from_string("world".into()).unwrap();
        let pid = (project_name, project_domain);
        let future_project = registry.client.get_project(pid);
        let maybe_project = futures::executor::block_on(future_project).unwrap();
        assert_eq!(maybe_project.is_some(), true);
        let project = maybe_project.unwrap();
        let metadata_vec: Vec<u8> = project.metadata.into();
        let metadata: ProjectCbor = from_reader(&metadata_vec[..]).unwrap();
        assert_eq!(metadata.version, 1);
    }
}
