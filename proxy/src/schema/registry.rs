use radicle_registry_client::{
    self as registry, ed25519, message, Client, ClientT, CryptoPair, String32, TransactionExtra,
    H256,
};

use crate::schema::error::{Error, ProjectValidation};

#[derive(GraphQLObject)]
pub struct Transaction {
    pub id: juniper::ID,
    pub messages: Vec<Message>,
    pub state: TransactionState,
    pub timestamp: String,
}

#[derive(juniper::GraphQLObject)]
pub struct ProjectRegistration {
    pub domain: String,
    pub name: String,
}

pub enum Message {
    ProjectRegistration(ProjectRegistration),
}

juniper::graphql_union!(Message: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &ProjectRegistration => match *self { Message::ProjectRegistration(ref p) => Some(p) },
    }
});

pub enum TransactionState {
    Applied(Applied),
}

juniper::graphql_union!(TransactionState: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &Applied => match *self { TransactionState::Applied(ref a) => Some(a) },
    }
});

#[derive(GraphQLObject)]
pub struct Applied {
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
    pub fn new(client: Client) -> Self {
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
