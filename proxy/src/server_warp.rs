use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::Filter;

use crate::schema::{Context, Schema};

pub fn run(schema: Schema, context: Context) {
    let index = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(juniper::graphiql::graphiql_source("/graphql"))
    });

    let routes = index
        .or(make_graphql_filter("graphql", schema, context))
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
        .with(warp::log("app-proxy"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080))
}

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

fn build_response(response: Result<Vec<u8>, serde_json::Error>) -> warp::http::Response<Vec<u8>> {
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
