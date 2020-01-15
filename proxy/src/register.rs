use futures::compat::Future01CompatExt;

use radicle_registry_client::{
    ed25519::Pair, ClientT, CreateCheckpointParams, DispatchError, Error as ProtocolError,
    RegisterProjectParams, String32, H256,
};

/// TODO
pub struct ProjectParams {
    /// TODO
    name: String32,
    /// TODO
    domain: String32,
    /// TODO
    description: String,
    /// TODO
    img_url: String,
}

/// TODO
pub enum ClientError {
    /// TODO
    Protocol(ProtocolError),
    /// TODO
    Runtime(DispatchError),
}

impl From<ProtocolError> for ClientError {
    fn from(error: ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

impl From<DispatchError> for ClientError {
    fn from(error: DispatchError) -> Self {
        Self::Runtime(error)
    }
}

/// TODO
pub struct Registry<R>
where
    R: ClientT,
{
    /// TODO
    client: R,
}

/// TODO
impl<R> Registry<R>
where
    R: ClientT,
{
    /// TODO
    fn new(client: R) -> Self {
        Self { client }
    }

    /// TODO
    async fn register_project(
        &self,
        author: &Pair,
        project: ProjectParams,
    ) -> Result<(), ClientError> {
        let project_id = (project.name, project.domain);
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
                    description: project.description,
                    img_url: project.img_url,
                    checkpoint_id,
                },
            )
            .compat()
            .await?
            .compat()
            .await?
            .result
            .map_err(|error| error.into())
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
    let params = ProjectParams {
        name: String32::from_string(String::from("hello")).expect("String too long."),
        domain: String32::from_string(String::from("world")).expect("String too long"),
        description: String::from("a jolly old time"),
        img_url: String::from("example.com/icon"),
    };
    let pending_result = registry.register_project(&alice, params);
    let result = block_on(pending_result);
    assert_eq!(result.is_err(), false);
}
