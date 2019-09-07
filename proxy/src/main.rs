//! Proxy to serve a specialised HTTP to the oscoin MVP.

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate juniper;

use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::Filter;

mod schema;
// mod server_actix;
mod source;

use crate::schema::Context;
use crate::source::Local;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    info!("Setting up source");
    let source = Local::new();

    info!("Creating Juniper schema and context");
    let schema = schema::create();
    let context = Context::new(source);

    info!("Starting HTTP server");
    let index = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(juniper::graphiql::graphiql_source("/graphql"))
    });

    let _cors = warp::any().map(warp::reply).with(
        warp::cors()
            .allow_origin("http://localhost:8000")
            .allow_methods(&[
                warp::http::Method::GET,
                warp::http::Method::POST,
                warp::http::Method::OPTIONS,
            ]),
    );

    let routes = index
        .or(make_graphql_filter("graphql", schema, context))
        .with(
            warp::cors()
                .allow_origin("http://localhost:8000")
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
