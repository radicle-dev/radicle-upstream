use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::io;
use std::sync::Arc;

use crate::schema::{Context, Schema};

pub fn run(schema: Arc<Schema>, source: Arc<Context>) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(source.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::method(http::Method::OPTIONS).to(options))
                    .route(web::post().to_async(graphql)),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .set_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(user))
    })
}

fn options() -> HttpResponse {
    HttpResponse::NoContent()
        .set_header(
            http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            "http://localhost:8000",
        )
        .set_header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET,HEAD,POST")
        .set_header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "content-type")
        .finish()
}
