use futures::compat::Future01CompatExt;
use std::convert::TryFrom;

use radicle_registry_client::{
    ed25519::Pair, Client, ClientT, CreateCheckpointParams, RegisterProjectParams, String32, H256,
};

use crate::schema::error::{Error, ProjectValidation};

/// TODO
#[derive(Clone)]
pub struct Registry<R>
where
    R: ClientT,
{
    /// TODO
    client: R,
}

#[derive(GraphQLObject)]
pub struct Metadata {
    // TODO
}

#[derive(GraphQLObject)]
pub struct RegistrationResult {
    pub id: juniper::ID,
    pub metadata: Metadata,
}

/// TODO
impl<R> Registry<R>
where
    R: ClientT,
{
    /// TODO
    pub fn new(client: R) -> Self {
        Self { client }
    }

    /// TODO
    pub async fn register_project(
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
            .compat()
            .await?
            .compat()
            .await?
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
            .compat()
            .await?
            .compat()
            .await?
            .result
            .map_err(|error| error.into())?;
        // TODO return project
    }
}

impl<R> TryFrom<&str> for Registry<R>
where
    R: ClientT,
{
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let node_name = url::Host::parse(s)?;
        let client = Client::create_with_executor(node_name);
        Ok(Self { client })
    }
}

#[test]
fn test_register_project() {
    // Test that project registration submits valid transactions and they succeed.
    use futures::executor::block_on;
    use radicle_registry_client::Client;
    let client = Client::new_emulator();
    let registry = Registry::new(client);
    let alice = Pair::from_legacy_string("//Alice", None);
    let pending_result = registry.register_project(
        &alice,
        "hello".into(),
        "world".into(),
        "a jolly old time".into(),
        "example.com/icon".into(),
    );
    let result = block_on(pending_result);
    assert_eq!(result.is_err(), false);
}
