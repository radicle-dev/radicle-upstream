use juniper::{FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
use radicle_registry_client::{Error as RegistryError};
use std::sync::Arc;

use crate::source::{AccountId, Project, ProjectId, Source};

/// Glue to bundle our read and write APIs together.
pub type Schema = RootNode<'static, Query, Mutation>;

/// Returns a `Schema` with the default parameterised `Query` and `Mutation`.
pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

/// Container to pass the `Source` around for data access.
#[derive(Clone)]
pub struct Context {
    /// Origin of data needed to server APIs.
    source: Arc<dyn Source + Send + Sync>,
}

impl Context {
    /// Returns a new `Context`.
    pub fn new<S: Source + Send + Sync + 'static>(source: S) -> Self {
        Self {
            source: Arc::new(source),
        }
    }
}

impl juniper::Context for Context {}

#[derive(GraphQLInputObject)]
struct IdInput {
    name: String,
    domain: String,
}

impl Into<ProjectId> for IdInput {
    fn into(self) -> ProjectId {
        ProjectId {
            name: self.name,
            domain: self.domain,
        }
    }
}

enum ProxyError {
    RegistryError,
}

impl juniper::IntoFieldError for ProxyError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            RegistryError => juniper::FieldError::new(
                "Error interacting with the Radicle Registry.",
                graphql_value!({ "type": "RegistryError" }),
            ),
        }
    }
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn projects(ctx: &Context) -> Result<Vec<Project>, ProxyError> {
        ctx.source.get_all_projects()
    }

    fn project(ctx: &Context, id: IdInput) -> Result<Option<Project>, ProxyError> {
        ctx.source.get_project(id.into())
    }
}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn register_project(
        ctx: &Context,
        name: String,
        description: String,
        img_url: String,
    ) -> FieldResult<Project> {
        Ok(ctx.source.register_project(name, description, img_url)?)
    }
}

#[test]
fn test_schema_projects() {
    use juniper::Variables;

    use crate::source::Ledger;

    let registry_client = radicle_registry_client::MemoryClient::new();
    let mut source = Ledger::new(registry_client);

    crate::source::setup_fixtures(&mut source);

    let ctx = Context::new(source);

    let (res, _errors) = juniper::execute(
        "query { projects { name } }",
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    )
    .expect("juniper execute failed for projects");

    assert_eq!(
        res,
        graphql_value!({
            "projects": [
                {"name": "monokel"},
                {"name": "Monadic"},
                {"name": "open source coin"},
                {"name": "radicle"},
            ]
        })
    );
}

juniper::graphql_scalar!(AccountId where Scalar = <S> {
    description: "AccountId"

    resolve(&self) -> Value {
        Value::scalar(hex::encode(self.0.as_ref() as &[u8]))
    }

    from_input_value(v: &InputValue) -> Option<AccountId> {
        let mut bytes = [0_u8; 32];

        v.as_scalar_value::<String>()
            .map(|s| hex::decode_to_slice(s, &mut bytes as &mut [u8]));

        Some(AccountId(radicle_registry_client::AccountId::from_raw(bytes)))
    }

    // Define how to parse a string value.
    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});
