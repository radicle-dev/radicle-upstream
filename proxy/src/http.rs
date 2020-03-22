#![allow(warnings, missing_docs)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::convert::Infallible;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::{get, path, reject, reply, Filter, Rejection, Reply};

use crate::error;
use crate::project;

/// Main entry point for HTTP API.
pub async fn run() -> Result<(), error::Error> {
    let librad_paths = librad::paths::Paths::new()?;

    let api = path("api").and(path("v1"));
    let routes = routes(librad_paths.clone()).recover(rejection_handle);

    Ok(warp::serve(routes).run(([127, 0, 0, 1], 8090)).await)
}

impl reject::Reject for error::Error {}

impl From<error::Error> for Rejection {
    fn from(err: error::Error) -> Self {
        reject::custom(err)
    }
}

/// Error type to carry context for failed requests.
#[derive(serde_derive::Serialize)]
struct Error {
    message: &'static str,
    variant: &'static str,
}

/// Handler to convert [`error::Error`] to [`Error`] response.
async fn rejection_handle(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, variant, message) = (
        StatusCode::NOT_IMPLEMENTED,
        "INTERNAL_ERROR",
        "Something went wrong",
    );
    let res = reply::json(&Error { message, variant });

    Ok(reply::with_status(res, code))
}

/// Combination of all routes.
pub fn routes(
    paths: librad::paths::Paths,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter().or(get_filter(paths))
}

fn get_filter(
    paths: librad::paths::Paths,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / String)
        .and(path::end())
        .and(get())
        .and(with_paths(paths))
        .and_then(get_project)
}

fn list_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(path::end())
        .and(get())
        .and_then(projects)
}

fn with_paths(
    paths: librad::paths::Paths,
) -> impl Filter<Extract = (librad::paths::Paths,), Error = Infallible> + Clone {
    warp::any().map(move || paths.clone())
}

async fn get_project(id: String, paths: librad::paths::Paths) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&project::get(&paths, id.as_ref()).await?))
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
    use librad::paths::Paths;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use tempfile::tempdir_in as _;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;
    use crate::error;

    #[tokio::test]
    async fn get_project() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();

        let repo_dir = tempfile::tempdir_in(tmp_dir.path()).unwrap();
        let path = repo_dir.path().to_str().unwrap().to_string();
        coco::init_repo(path.clone()).unwrap();

        let (id, meta) = coco::init_project(
            &librad_paths,
            &path,
            "Upstream",
            "Desktop client for radicle.",
            "master",
            "https://avatars0.githubusercontent.com/u/48290027",
        )
        .unwrap();

        let api = super::routes(librad_paths);
        let res = request()
            .method("GET")
            .path(&format!("/projects/{}", id.to_string()))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": id.to_string(),
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
        let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
        let librad_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");

        let api = super::routes(librad_paths);
        let res = request().method("GET").path("/projects").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!([]);

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, want);
    }
}
