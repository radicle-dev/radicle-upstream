//! Endpoints and serialisations for [`project::Project`] related types.

use librad::paths::Paths;
use serde::ser::{SerializeStruct as _, SerializeStructVariant as _};
use serde::{Deserialize, Serialize, Serializer};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, reply, Filter, Rejection, Reply};

use crate::project;
use crate::registry;

/// Combination of all routes.
pub fn filters(
    paths: Paths,
    registry: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter(paths.clone())
        .or(create_filter(paths.clone()))
        .or(get_filter(paths))
        .or(register_filter(registry))
}

/// POST /projects
fn create_filter(paths: Paths) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(path::end())
        .and(warp::post())
        .and(super::with_paths(paths))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// GET /projects/<id>
fn get_filter(paths: Paths) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / String)
        .and(path::end())
        .and(warp::get())
        .and(super::with_paths(paths))
        .and_then(handler::get)
}

/// GET /projects
fn list_filter(paths: Paths) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(path::end())
        .and(warp::get())
        .and(super::with_paths(paths))
        .and_then(handler::list)
}

/// POST /projects/register
fn register_filter(
    registry: Arc<RwLock<registry::Registry>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / "register")
        .and(path::end())
        .and(warp::post())
        .and(super::with_registry(registry))
        .and(warp::body::json())
        .and_then(handler::register)
}

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use librad::paths;
    use librad::surf;
    use radicle_registry_client::{ed25519, Balance};
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::project;
    use crate::registry;

    /// Create a new [`project::Project`].
    pub async fn create(
        paths: paths::Paths,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        if surf::git::git2::Repository::open(input.path.clone()).is_err() {
            coco::init_repo(input.path.clone())?;
        };

        let (id, meta) = coco::init_project(
            &paths,
            &input.path,
            &input.metadata.name,
            &input.metadata.description,
            &input.metadata.default_branch,
            &input.metadata.img_url,
        )?;

        Ok(reply::with_status(
            reply::json(&project::Project {
                id: librad::project::ProjectId::from(id),
                metadata: meta.into(),
            }),
            StatusCode::CREATED,
        ))
    }

    /// Get the [`project::Project`] for the given `id`.
    pub async fn get(id: String, paths: paths::Paths) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&project::get(&paths, id.as_ref()).await?))
    }

    /// List all known projects.
    pub async fn list(paths: paths::Paths) -> Result<impl Reply, Infallible> {
        let projects = coco::list_projects(&paths)
            .into_iter()
            .map(|(id, meta)| project::Project {
                id,
                metadata: meta.into(),
            })
            .collect::<Vec<project::Project>>();

        Ok(reply::json(&projects))
    }

    /// Register a project on the Registry.
    pub async fn register(
        registry: Arc<RwLock<registry::Registry>>,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fake_fee: Balance = 100;

        let mut reg = registry.write().await;
        let tx = reg
            .register_project(&fake_pair, input.project_name, input.org_id, None, fake_fee)
            .await?;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
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

impl Serialize for registry::Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Transaction", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("messages", &self.messages)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.end()
    }
}

impl Serialize for registry::Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OrgRegistration(org_id) => serializer.serialize_newtype_variant(
                "Message",
                0,
                "OrgRegistration",
                &org_id.to_string(),
            ),
            Self::OrgUnregistration(org_id) => serializer.serialize_newtype_variant(
                "Message",
                1,
                "OrgUnregistration",
                &org_id.to_string(),
            ),
            Self::ProjectRegistration {
                org_id,
                project_name,
            } => {
                let mut sv =
                    serializer.serialize_struct_variant("Message", 2, "ProjectRegistration", 2)?;
                sv.serialize_field("org_id", &org_id.to_string())?;
                sv.serialize_field("project_name", &project_name.to_string())?;
                sv.end()
            },
            Self::UserRegistration { handle, id } => {
                let mut sv =
                    serializer.serialize_struct_variant("Message", 3, "UserRegistration", 2)?;
                sv.serialize_field("handle", &handle.to_string())?;
                sv.serialize_field("id", &id.to_string())?;
                sv.end()
            },
        }
    }
}

impl Serialize for registry::TransactionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Applied(block_hash) => {
                let mut state = serializer.serialize_struct("TransactionApplied", 2)?;
                state.serialize_field("type", "TransactionApplied")?;
                state.serialize_field("block_hash", &block_hash.to_string())?;

                state.end()
            },
        }
    }
}

/// Bundled input data for project creation.
#[derive(Deserialize, Serialize)]
pub struct CreateInput {
    /// Location on the filesystem of the project, an empty directory means we set up a fresh git
    /// repo at the path before initialising the project.
    path: String,
    /// User provided metadata for the project.
    metadata: MetadataInput,
}

/// User provided metadata for project manipulation.
#[derive(Deserialize, Serialize)]
pub struct MetadataInput {
    /// Name of the proejct.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
    /// Display image of the project.
    img_url: String,
}

/// Bundled input data for project registration.
#[derive(Deserialize, Serialize)]
pub struct RegisterInput {
    /// Id of the Org the project will be registered under.
    org_id: String,
    /// Unique name under Org of the project.
    project_name: String,
    /// Optionally passed coco id to store for attestion.
    maybe_coco_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use librad::paths::Paths;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tempfile::tempdir_in as _;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;
    use crate::error;
    use crate::project;
    use crate::registry;

    #[tokio::test]
    async fn create() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());

        let repos_dir = tempfile::tempdir_in(tmp_dir.path()).unwrap();
        let dir = tempfile::tempdir_in(repos_dir.path()).unwrap();
        let path = dir.path().to_str().unwrap();

        let api = super::filters(librad_paths.clone(), Arc::new(RwLock::new(registry)));
        let res = request()
            .method("POST")
            .path("/projects")
            .json(&super::CreateInput {
                path: path.into(),
                metadata: super::MetadataInput {
                    name: "Upstream".into(),
                    description: "Desktop client for radicle.".into(),
                    default_branch: "master".into(),
                    img_url: "https://avatars0.githubusercontent.com/u/48290027".into(),
                },
            })
            .reply(&api)
            .await;

        let projects = coco::list_projects(&librad_paths);
        let (id, _) = projects.first().unwrap();

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

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);
    }

    #[tokio::test]
    async fn get() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());

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

        let api = super::filters(librad_paths, Arc::new(RwLock::new(registry)));
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
    async fn list() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());

        coco::setup_fixtures(&librad_paths, tmp_dir.path().as_os_str().to_str().unwrap());

        let projects = coco::list_projects(&librad_paths)
            .into_iter()
            .map(|(id, meta)| project::Project {
                id,
                metadata: meta.into(),
            })
            .collect::<Vec<project::Project>>();

        let api = super::filters(librad_paths, Arc::new(RwLock::new(registry)));
        let res = request().method("GET").path("/projects").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!(projects);

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, want);
    }

    #[tokio::test]
    async fn register() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        )));

        let api = super::filters(librad_paths, registry.clone());
        let res = request()
            .method("POST")
            .path("/projects/register")
            .json(&super::RegisterInput {
                project_name: "upstream".into(),
                org_id: "radicle".into(),
                maybe_coco_id: None,
            })
            .reply(&api)
            .await;

        let txs = registry
            .write()
            .await
            .list_transactions(vec![])
            .await
            .unwrap();
        let tx = txs.first().unwrap();

        let block_hash = if let registry::TransactionState::Applied(block_hash) = tx.state {
            block_hash
        } else {
            radicle_registry_client::Hash::random()
        };

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": tx.id.to_string(),
            "messages": [
                {
                    "ProjectRegistration": {
                        "org_id": "radicle",
                        "project_name": "upstream",
                    }
                },
            ],
            "state": {
                "block_hash": block_hash.to_string(),
                "type": "TransactionApplied",
            },
            "timestamp": {
                "nanos_since_epoch": tx.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos(),
                "secs_since_epoch": tx.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u64,
            },
        });

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);
    }
}
