use juniper::{FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
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
    domain: String,
    name: String,
}

impl Into<ProjectId> for IdInput {
    fn into(self) -> ProjectId {
        ProjectId {
            name: self.name,
            domain: self.domain,
        }
    }
}

#[derive(GraphQLObject)]
struct Branch {
    name: String,
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn branches(ctx: &Context, id: IdInput) -> FieldResult<Vec<Branch>> {
        use radicle_surf::git::{GitBrowser, GitRepository};

        let repo =
            GitRepository::new("../data/git-golden").expect("Pointing to golden repo failed");
        let browser = GitBrowser::new(&repo).expect("setting up browser for repo failed");
        let branche_names = browser.list_branches().expect("Getting branches failed");
        let branches = branche_names
            .into_iter()
            .map(|branch_name| Branch {
                name: branch_name.name(),
            })
            .collect();

        Ok(branches)
    }

    fn projects(ctx: &Context) -> FieldResult<Vec<Project>> {
        Ok(ctx.source.get_all_projects())
    }

    fn project(ctx: &Context, id: IdInput) -> FieldResult<Option<Project>> {
        Ok(ctx.source.get_project(id.into()))
    }
}

#[test]
fn test_schema_branches() {
    use indexmap::IndexMap;
    use juniper::{InputValue, Variables};
    use radicle_registry_client::MemoryClient;

    use crate::source::{setup_fixtures, Ledger};

    let registry_client = MemoryClient::new();
    let mut source = Ledger::new(registry_client);

    setup_fixtures(&mut source);

    let ctx = Context::new(source);

    let mut vars = Variables::new();
    let mut id_map: IndexMap<String, InputValue> = IndexMap::new();

    id_map.insert("domain".into(), InputValue::scalar("rad"));
    id_map.insert("name".into(), InputValue::scalar("upstream"));

    vars.insert("id".into(), InputValue::object(id_map));

    let (res, _errors) = juniper::execute(
        "query($id: IdInput!) { branches(id: $id) { name } }",
        None,
        &Schema::new(Query, Mutation),
        &vars,
        &ctx,
    )
    .expect("juniper execute failed");

    assert_eq!(
        res,
        graphql_value!({
            "branches": [
                { "name": "master" },
                { "name": "origin/HEAD" },
                { "name": "origin/add-tests" },
                { "name": "origin/master" },
            ]
        }),
    );
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
        Ok(ctx.source.register_project(name, description, img_url))
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
