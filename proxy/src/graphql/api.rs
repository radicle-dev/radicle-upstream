use std::sync::Arc;
use warp::filters;
use warp::http;
use warp::Filter;

use super::schema;

/// Runs the warp server with the given schema and context.
pub async fn run(
    dummy_repo_path: String,
    librad_paths: librad::paths::Paths,
    registry_client: radicle_registry_client::Client,
    enable_control: bool,
) {
    let context = schema::Context::new(dummy_repo_path, librad_paths, registry_client);
    let state = warp::any().map(move || context.clone());
    let graphql_filter = make_graphql_filter(schema::create(), state.clone().boxed());
    let control_filter = make_graphql_filter(schema::create_control(), state.boxed());
    let routes = warp::path("control")
        .map(move || enable_control)
        .and_then(|enable_control| async move {
            if enable_control {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one()
        .and(control_filter)
        .or(warp::path("graphql").and(graphql_filter))
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
        .with(warp::log("proxy::api"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}

/// Filter for the graphql endpoint.
fn make_graphql_filter<Context, Mutation, Query>(
    schema: juniper::RootNode<'static, Query, Mutation>,
    context_extractor: filters::BoxedFilter<(Context,)>,
) -> impl Filter<Extract = (http::Response<Vec<u8>>,), Error = warp::Rejection> + Clone
where
    Context: Clone + Send + Sync + 'static,
    Mutation: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
    Query: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
{
    let schema = Arc::new(schema);

    warp::post()
        .map(move || schema.clone())
        .and(context_extractor)
        .and(warp::body::json())
        .and_then(handle_request)
}

async fn handle_request<Context, Mutation, Query>(
    schema: Arc<juniper::RootNode<'static, Query, Mutation>>,
    context: Context,
    request: juniper::http::GraphQLRequest,
) -> Result<http::Response<Vec<u8>>, std::convert::Infallible>
where
    Context: Clone + Send + Sync + 'static,
    Mutation: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
    Query: juniper::GraphQLType<Context = Context, TypeInfo = ()> + Send + Sync + 'static,
{
    match serde_json::to_vec(&request.execute(&schema, &context)) {
        Ok(body) => Ok(http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(body)
            .unwrap()),
        Err(_) => Ok(http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .unwrap()),
    }
}
