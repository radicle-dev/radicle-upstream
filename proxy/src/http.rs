#![allow(warnings, missing_docs)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
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

impl reject::Reject for error::Error {}

impl From<error::Error> for Rejection {
    fn from(err: error::Error) -> Rejection {
        reject::custom(err)
    }
}

#[derive(serde_derive::Serialize)]
struct Error {
    message: &'static str,
    variant: &'static str,
}

async fn rejection_handle(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, variant, message) = (
        StatusCode::NOT_IMPLEMENTED,
        "INTERNAL_ERROR",
        "Something went wrong",
    );
    let res = reply::json(&Error { message, variant });

    Ok(reply::with_status(res, code))
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter().or(get_filter())
}

fn get_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / String).and(get()).and_then(get_project)
}

fn list_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects").and(get()).and_then(projects)
}

async fn get_project(id: String) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&foo(id)?))
}

fn foo(id: String) -> Result<project::Project, error::Error> {
    Ok(project::Project {
        id: librad::project::ProjectId::from_str(&id)?,
        metadata: project::Metadata {
            name: "Upstream".into(),
            description: "Desktop client for radicle.".into(),
            default_branch: "master".into(),
            img_url: "https://avatars0.githubusercontent.com/u/48290027".into(),
        },
    })
}

async fn projects() -> Result<impl Reply, Infallible> {
    let content: Vec<String> = vec![];
    Ok(reply::json(&content))
}

impl Serialize for project::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 2)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("metadata", &self.metadata)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn get_project() {
        let api = super::routes();
        let res = request()
            .method("GET")
            .path("/projects/123.git")
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": "1230000000000000000000000000000000000000.git",
            "metadata": {
                "default_branch": "master",
                "description": "Desktop client for radicle.",
                "img_url": "https://avatars0.githubusercontent.com/u/48290027",
                "name": "Upstream",
            },
        });

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, want);
    }

    #[tokio::test]
    async fn projects() {
        let api = super::routes();
        let res = request().method("GET").path("/projects").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!([]);

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, want);
    }
}
