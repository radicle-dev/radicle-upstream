use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::Filter;

use crate::schema::{Context, Schema};

/// Runs the warp server with the given schema and context.
pub fn run(schema: Schema, context: Context) {
    let routes = make_graphql_filter("graphql", schema, context)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(&[warp::http::header::CONTENT_TYPE])
                .allow_methods(&[
                    warp::http::Method::GET,
                    warp::http::Method::POST,
                    warp::http::Method::OPTIONS,
                ]),
        )
        .with(warp::log("proxy"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080))
}

/// Filter for the graphql endpoint.
fn make_graphql_filter<Query, Mutation, Context>(
    path: &'static str,
    schema: juniper::RootNode<'static, Query, Mutation>,
    ctx: Context,
) -> BoxedFilter<(impl warp::Reply,)>
where
    Context: juniper::Context + Clone + Send + Sync + 'static,
    Query: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
{
    let schema = Arc::new(schema);
    let context_extractor = warp::any().map(move || -> Context { ctx.clone() });

    let handle_request = move |context: Context,
                               request: juniper::http::GraphQLRequest|
          -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&request.execute(&schema, &context))
    };

    warp::post2()
        .and(warp::path(path))
        .and(context_extractor)
        .and(warp::body::json())
        .map(handle_request)
        .map(build_response)
        .boxed()
}

/// Helper for standard response shape.
fn build_response(response: Result<Vec<u8>, serde_json::Error>) -> Response<Vec<u8>> {
    match response {
        Ok(body) => warp::http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(body)
            .expect("response is valid"),
        Err(_) => warp::http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .expect("status code is valid"),
    }
}
