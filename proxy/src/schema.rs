use juniper::{EmptyMutation, FieldResult, RootNode};
use std::sync::Arc;

use crate::source::{Address, Local, Project, Source};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
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

    fn all_projects(ctx: &Context) -> FieldResult<Vec<&Project>> {
        Ok(ctx.source.get_all_projects())
    }

    fn get_project(ctx: &Context, address: Address) -> FieldResult<Option<&Project>> {
        Ok(ctx.source.get_project(address))
    }
}
