use juniper::{FieldResult, ParseScalarResult, ParseScalarValue, RootNode, Value};
use std::sync::Arc;

use crate::source::{Project, ProjectId, Source};

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

#[derive(Clone)]
pub struct Context {
    source: Arc<dyn Source + Send + Sync>,
}

impl Context {
    pub fn new<S: Source + Send + Sync + 'static>(source: S) -> Self {
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

    fn project(ctx: &Context, id: ProjectId) -> FieldResult<Option<Project>> {
        Ok(ctx.source.get_project(id))
    }
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

#[test]
fn test_schema_projects() {
    use juniper::Variables;

    use crate::source::Local;

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

juniper::graphql_scalar!(ProjectId where Scalar = <S> {
    description: "ProjectId"

    // Define how to convert your custom scalar into a primitive type.
    resolve(&self) -> Value {
        Value::scalar(hex::encode(self.0))
    }

    // Define how to parse a primitive type into your custom scalar.
    from_input_value(v: &InputValue) -> Option<ProjectId> {
        let mut bytes = [0_u8; 20];

        v.as_scalar_value::<String>()
            .map(|s| hex::decode_to_slice(s, &mut bytes as &mut [u8]).unwrap());

        Some(ProjectId(bytes))
    }

    // Define how to parse a string value.
    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});
