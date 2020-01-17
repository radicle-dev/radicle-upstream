use futures::future::Future;

use radicle_registry_client::{
    ed25519::Pair, ClientT, CreateCheckpointParams, RegisterProjectParams, String32, H256,
};

use crate::schema::error::{Error, ProjectValidation};

/// Registry client wrapper.
#[derive(Clone)]
pub struct Registry<R>
where
    R: ClientT,
{
    /// Registry client, whether an emulator or otherwise.
    client: R,
}

/// Registry client wrapper methods
impl<R> Registry<R>
where
    R: ClientT,
{
    // TODO(xla): Remvoe once integrated in the schema.
    #[allow(dead_code)]
    /// Wrap a registry client.
    pub fn new(client: R) -> Self {
        Self { client }
    }

    // TODO(xla): Remvoe once integrated in the schema.
    #[allow(dead_code)]
    /// Register a new project on the chain.
    pub fn register_project(
        &self,
        author: &Pair,
        name: String,
        domain: String,
    ) -> Result<(), Error> {
        let project_name = String32::from_string(name).map_err(ProjectValidation::NameTooLong)?;
        let project_domain =
            String32::from_string(domain).map_err(ProjectValidation::DomainTooLong)?;
        let project_hash = H256::random();
        let checkpoint_id = self
            .client
            .sign_and_submit_call(
                author,
                CreateCheckpointParams {
                    project_hash,
                    previous_checkpoint_id: None,
                },
            )
            // TODO(garbados): futurize
            .wait()?
            .wait()?
            .result?;
        self.client
            .sign_and_submit_call(
                author,
                RegisterProjectParams {
                    id: (project_name, project_domain),
                    checkpoint_id,
                },
            )
            // TODO(garbados): futurize
            .wait()?
            .wait()?
            .result
            .map_err(|error| error.into())
    }
}

#[test]
fn test_register_project() {
    // Test that project registration submits valid transactions and they succeed.
    use radicle_registry_client::Client;
    let client = Client::new_emulator();
    let registry = Registry::new(client);
    let alice = Pair::from_legacy_string("//Alice", None);
    let result = registry.register_project(&alice, "hello".into(), "world".into());
    assert_eq!(result.is_err(), false);
}
