#![allow(warnings, missing_docs)]

use serde_derive::Serialize;
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{get, path, reject, reply, Filter, Rejection, Reply};

use crate::error;
use crate::project;

pub async fn run() {
    let routes = routes().recover(rejection_handle);

    warp::serve(routes).run(([127, 0, 0, 1], 8090)).await
}

#[derive(Serialize)]
struct Error {
    message: &'static str,
    variant: &'static str,
}

async fn rejection_handle(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, variant, message) = (
        StatusCode::INTERNAL_SERVER_ERROR,
        "INTERNAL_ERROR",
        "Something went wrong",
    );
    let res = reply::json(&Error { message, variant });

    Ok(reply::with_status(res, code))
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter().or(get_filter())
}

fn get_filter() -> impl Filter<Extract = impl Reply, Error = error::Error> + Clone {
    path!("projects" / String).and(get()).and_then(get_project)
}

fn list_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects").and(get()).and_then(projects)
}

async fn get_project(id: String) -> Result<impl Reply, error::Error> {
    Ok(reply::json(&project::Project {
        id: librad::project::ProjectId::from_str(&id)?,
        metadata: project::Metadata {
            name: "Upstream".into(),
            description: "Desktop client for radicle.".into(),
            default_branch: "master".into(),
            img_url: "https://avatars0.githubusercontent.com/u/48290027".into(),
        },
    }))
}

async fn projects() -> Result<impl Reply, Infallible> {
    let content: Vec<String> = vec![];
    Ok(reply::json(&content))
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn get_project() {
        let api = super::routes();
        let res = request()
            .method("GET")
            .path("/projects/123")
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn projects() {
        let api = super::routes();
        let res = request().method("GET").path("/projects").reply(&api).await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.body(), "[]");
    }
}
