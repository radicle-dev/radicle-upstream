use radicle_registry_client::{
    self as registry, ed25519, message, Client, ClientT, CryptoPair, String32, TransactionExtra,
    H256,
};

use crate::schema::error::{Error, ProjectValidation};

/// A container to dissiminate and apply operations on the [`Registry`].
#[derive(GraphQLObject)]
pub struct Transaction {
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

/// Required information to issue a new project registration on the [`Regisry`].
#[derive(juniper::GraphQLObject)]
pub struct ProjectRegistration {
    /// The domain/namespace the project should be registered for.
    pub domain: String,
    /// The name of the project, which MUST be uniqure for the domain.
    pub name: String,
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
    client: Client,
}

/// Registry client wrapper methods
impl Registry {
    /// Wrap a registry client.
    pub const fn new(client: Client) -> Self {
        Self { client }
    }

    /// Register a new project on the chain.
    pub async fn register_project(
        &self,
        author: &ed25519::Pair,
        domain: String,
        name: String,
    ) -> Result<H256, Error> {
        // Verify that inputs are valid.
        let project_name = String32::from_string(name).map_err(ProjectValidation::NameTooLong)?;
        let project_domain =
            String32::from_string(domain).map_err(ProjectValidation::DomainTooLong)?;

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
            id: (project_name, project_domain),
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
        self.client
            .submit_transaction(register_tx)
            .await?
            .await?
            .result?;

        Ok(H256::random())
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
    assert_eq!(result.is_err(), false);
}
