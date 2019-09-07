use juniper::{EmptyMutation, FieldResult, RootNode};

use crate::source::{Address, Local, Project, Source};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}

pub struct Context {
    source: Local,
}

impl Context {
    pub fn new(source: Local) -> Self {
        Self { source }
    }
}

impl juniper::Context for Context {}

pub struct Query;
graphql_object!(Query: Context | &self | {
    field all_projects(&executor) -> FieldResult<Vec<&Project>> {
        Ok(executor.context().source.get_all_projects())
    },
    field get_project(&executor, addr: Address) -> FieldResult<Option<&Project>> {
        let source = &executor.context().source;

        Ok(source.get_project(addr))
    }
});
