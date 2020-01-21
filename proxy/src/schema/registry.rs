use radicle_registry_client::{ed25519, message, Client, ClientT, String32, H256};

use crate::schema::error::{Error, ProjectValidation};

#[derive(GraphQLObject)]
pub struct Transaction {
    pub id: juniper::ID,
    pub messages: Vec<Message>,
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

/// Registry client wrapper.
#[derive(Clone)]
pub struct Registry {
    /// Registry client, whether an emulator or otherwise.
    client: Client,
}

/// Registry client wrapper methods
impl Registry {
    // TODO(xla): Remvoe once integrated in the schema.
    #[allow(dead_code)]
    /// Wrap a registry client.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    // TODO(xla): Remvoe once integrated in the schema.
    #[allow(dead_code)]
    /// Register a new project on the chain.
    pub async fn register_project(
        &self,
        author: &ed25519::Pair,
        domain: String,
        name: String,
    ) -> Result<(), Error> {
        let project_name = String32::from_string(name).map_err(ProjectValidation::NameTooLong)?;
        let project_domain =
            String32::from_string(domain).map_err(ProjectValidation::DomainTooLong)?;
        let project_hash = H256::random();
        let checkpoint_id = self
            .client
            .sign_and_submit_message(
                author,
                message::CreateCheckpoint {
                    project_hash,
                    previous_checkpoint_id: None,
                },
            )
            // TODO(garbados): futurize
            .await?
            .await?
            .result?;
        self.client
            .sign_and_submit_message(
                author,
                message::RegisterProject {
                    id: (project_name, project_domain),
                    checkpoint_id,
                },
            )
            // TODO(garbados): futurize
            .await?
            .await?
            .result
            .map_err(|error| error.into())
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
