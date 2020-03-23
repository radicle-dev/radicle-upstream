use serde::ser::SerializeStruct as _;
use std::convert::Infallible;
use warp::{get, path, reply, Filter, Rejection, Reply};

use crate::project;

/// Combination of all routes.
pub fn filters(
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

impl serde::Serialize for project::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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

        let api = super::filters(librad_paths);
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

        let api = super::filters(librad_paths);
        let res = request().method("GET").path("/projects").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!([]);

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, want);
    }
}
