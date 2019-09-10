use juniper::{FieldResult, RootNode};
use std::sync::Arc;

use crate::source::{Address, Local, Project, Source};

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

#[derive(Clone)]
pub struct Context {
    source: Arc<Local>,
}

impl Context {
    pub fn new(source: Local) -> Self {
        Self {
            source: Arc::new(source),
        }
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn projects(ctx: &Context) -> FieldResult<Vec<Project>> {
        Ok(ctx.source.get_all_projects())
    }

    fn get_project(ctx: &Context, address: Address) -> FieldResult<Option<Project>> {
        Ok(ctx.source.get_project(address))
    }
}

#[test]
fn test_schema_all_projects() {
    use juniper::Variables;

    let ctx = Context::new(Local::new());
    let (res, _errors) = juniper::execute(
        "query { projects { name } }",
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    assert_eq!(
        res,
        graphql_value!({
            "projects": [
                {"name": "Monadic"},
                {"name": "monokel"},
                {"name": "open source coin"},
                {"name": "radicle"},
            ]
        })
    );
}

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
