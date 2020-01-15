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
    /// Wrap a registry client.
    pub fn new(client: R) -> Self {
        Self { client }
    }

    /// Register a new project on the chain.
    pub fn register_project(
        &self,
        author: &Pair,
        name: String,
        domain: String,
        description: String,
        img_url: String,
    ) -> Result<(), Error> {
        let project_name = String32::from_string(name).map_err(ProjectValidation::NameTooLong)?;
        let project_domain =
            String32::from_string(domain).map_err(ProjectValidation::DomainTooLong)?;
        let project_hash = H256::random();
        let project_id = (project_name, project_domain);
        let checkpoint_id = self
            .client
            .sign_and_submit_call(
                author,
                CreateCheckpointParams {
                    project_hash,
                    previous_checkpoint_id: None,
                },
            )
            .wait()?
            .wait()?
            .result?;
        self.client
            .sign_and_submit_call(
                author,
                RegisterProjectParams {
                    id: project_id.clone(),
                    description,
                    img_url,
                    checkpoint_id,
                },
            )
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
    let result = registry.register_project(
        &alice,
        "hello".into(),
        "world".into(),
        "a jolly old time".into(),
        "example.com/icon".into(),
    );
    assert_eq!(result.is_err(), false);
}
