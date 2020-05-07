//! Endpoints and serialisation for [`project::Project`] related types.

use librad::paths::Paths;
use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::notification;
use crate::project;
use crate::registry;

/// Combination of all routes.
pub fn filters(
    paths: Arc<RwLock<Paths>>,
    registry: Arc<RwLock<registry::Registry>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter(Arc::<RwLock<Paths>>::clone(&paths))
        .or(create_filter(Arc::<RwLock<Paths>>::clone(&paths)))
        .or(get_filter(paths))
        .or(register_filter(registry, subscriptions))
}

/// `POST /projects`
fn create_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(warp::post())
        .and(super::with_paths(paths))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Create a new project",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::body(CreateInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(project::Project::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::create)
}

/// `GET /projects/<id>`
fn get_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("projects")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::param::<String>("id", "Project id"))
        .and(document::document(document::description(
            "Find Project by ID",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::response(
                200,
                document::body(project::Project::document()).mime("application/json"),
            )
            .description("Project found"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Project not found"),
        ))
        .and_then(handler::get)
}

/// `GET /projects`
fn list_filter(
    paths: Arc<RwLock<Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(warp::get())
        .and(super::with_paths(paths))
        .and(document::document(document::description("List projects")))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(project::Project::document()).description("List of projects"),
                )
                .mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::list)
}

/// `POST /projects/register`
fn register_filter(
    registry: Arc<RwLock<registry::Registry>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / "register")
        .and(warp::post())
        .and(super::with_registry(registry))
        .and(super::with_subscriptions(subscriptions))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a Project on the Registry",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(
                    document::array(registry::Transaction::document())
                        .description("RegisterProject transaction"),
                )
                .mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register)
}

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use librad::paths::Paths;
    use librad::surf;
    use radicle_registry_client::Balance;
    use std::str::FromStr;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::notification;
    use crate::project;
    use crate::registry;

    /// Create a new [`project::Project`].
    pub async fn create(
        librad_paths: Arc<RwLock<Paths>>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        if surf::git::git2::Repository::open(input.path.clone()).is_err() {
            coco::init_repo(input.path.clone())?;
        };

        let paths = librad_paths.read().await;
        let (id, meta) = coco::init_project(
            &paths,
            &input.path,
            &input.metadata.name,
            &input.metadata.description,
            &input.metadata.default_branch,
        )?;

        Ok(reply::with_status(
            reply::json(&project::Project {
                id: librad::project::ProjectId::from(id),
                metadata: meta.into(),
                registration: None,
                stats: project::Stats {
                    branches: 11,
                    commits: 267,
                    contributors: 8,
                },
            }),
            StatusCode::CREATED,
        ))
    }

    /// Get the [`project::Project`] for the given `id`.
    pub async fn get(
        librad_paths: Arc<RwLock<Paths>>,
        id: String,
    ) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        Ok(reply::json(&project::get(&paths, id.as_ref()).await?))
    }

    /// List all known projects.
    pub async fn list(librad_paths: Arc<RwLock<Paths>>) -> Result<impl Reply, Rejection> {
        let paths = librad_paths.read().await;
        let projects = coco::list_projects(&paths)
            .into_iter()
            .map(|(id, meta)| project::Project {
                id,
                metadata: meta.into(),
                registration: None,
                stats: project::Stats {
                    branches: 11,
                    commits: 267,
                    contributors: 8,
                },
            })
            .collect::<Vec<project::Project>>();

        Ok(reply::json(&projects))
    }

    /// Register a project on the Registry.
    pub async fn register(
        registry: Arc<RwLock<registry::Registry>>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fake_fee: Balance = 100;

        let mut reg = registry.write().await;
        let maybe_coco_id = input
            .maybe_coco_id
            .map(|id| librad::project::ProjectId::from_str(&id).expect("Project id"));
        let tx = reg
            .register_project(
                &fake_pair,
                input.org_id,
                input.project_name,
                maybe_coco_id,
                fake_fee,
            )
            .await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

impl Serialize for project::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("registration", &self.registration)?;
        state.serialize_field("stats", &self.stats)?;
        state.end()
    }
}

impl ToDocumentedType for project::Project {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(4);
        properties.insert(
            "id".into(),
            document::string()
                .description("ID of the project")
                .example("ac1cac587b49612fbac39775a07fb05c6e5de08d.git"),
        );
        properties.insert("metadata".into(), project::Metadata::document());
        properties.insert("registration".into(), project::Registration::document());
        properties.insert("stats".into(), project::Stats::document());

        document::DocumentedType::from(properties)
            .description("Radicle project for sharing and collaborating")
    }
}

impl ToDocumentedType for project::Metadata {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Project name")
                .example("upstream"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("High-level description of the Project")
                .example("Desktop client for radicle"),
        );
        properties.insert(
            "defaultBranch".into(),
            document::string()
                .description("Default branch for checkouts, often used as mainline as well")
                .example("master"),
        );

        document::DocumentedType::from(properties).description("Project metadata")
    }
}

impl Serialize for project::Registration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Org(org_id) => {
                serializer.serialize_newtype_variant("Registration", 0, "Org", &org_id.to_string())
            },
            Self::User(user_id) => serializer.serialize_newtype_variant(
                "Registration",
                1,
                "User",
                &user_id.to_string(),
            ),
        }
    }
}

impl ToDocumentedType for project::Registration {
    fn document() -> document::DocumentedType {
        let org = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert(
                "org".into(),
                document::string().description("Org id").example("monadic"),
            );
            document::DocumentedType::from(fields).description("Registered under an Org")
        };
        let user = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert(
                "user".into(),
                document::string().description("User id").example("monadic"),
            );
            document::DocumentedType::from(fields).description("Registered under a User")
        };

        document::one_of(vec![org, user])
            .description("Variants for possible registration states of a Project on the Registry")
            .example(Self::Org(
                registry::Id::try_from("monadic").expect("unable to parse org id"),
            ))
    }
}

impl Serialize for project::Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stats", 3)?;
        state.serialize_field("branches", &self.branches)?;
        state.serialize_field("commits", &self.commits)?;
        state.serialize_field("contributors", &self.contributors)?;
        state.end()
    }
}

impl ToDocumentedType for project::Stats {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "branches".into(),
            document::string()
                .description("Amount of known branches")
                .example(11),
        );
        properties.insert(
            "commits".into(),
            document::string()
                .description("Numbner of commits in the default branch")
                .example(267),
        );
        properties.insert(
            "contributors".into(),
            document::string()
                .description("Amount of unique commiters on the default branch")
                .example(8),
        );

        document::DocumentedType::from(properties)
            .description("Coarse statistics for the Project source code")
    }
}

/// Bundled input data for project creation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Location on the filesystem of the project, an empty directory means we set up a fresh git
    /// repo at the path before initialising the project.
    path: String,
    /// User provided metadata for the project.
    metadata: MetadataInput,
}

impl ToDocumentedType for CreateInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "path".into(),
            document::string()
                .description("Filesystem location of the git repository")
                .example("/home/xla/dev/src/github.com/radicle-dev/radicle-upstream"),
        );
        properties.insert("metadata".into(), MetadataInput::document());

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

/// User provided metadata for project manipulation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInput {
    /// Name of the proejct.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
}

impl ToDocumentedType for MetadataInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the project")
                .example("upstream"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("Long-form text describing the project")
                .example("Desktop client for radicle"),
        );
        properties.insert(
            "defaultBranch".into(),
            document::string()
                .description("Projects mainline branch")
                .example("stable"),
        );

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

/// Bundled input data for project registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Id of the Org the project will be registered under.
    org_id: String,
    /// Unique name under Org of the project.
    project_name: String,
    /// Optionally passed coco id to store for attestion.
    maybe_coco_id: Option<String>,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "orgId".into(),
            document::string()
                .description("ID of the Org the project will be registered under")
                .example("monadic"),
        );
        properties.insert(
            "projectName".into(),
            document::string()
                .description("Unique name under the Org of the project")
                .example("upstream"),
        );
        properties.insert(
            "maybeCocoId".into(),
            document::string()
                .description("Optionally passed coco id to store for attestion")
                .example("ac1cac587b49612fbac39775a07fb05c6e5de08d.git"),
        );

        document::DocumentedType::from(properties).description("Input for Project registration")
    }
}

#[allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use librad::paths::Paths;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;
    use crate::notification;
    use crate::project;
    use crate::registry;

    #[tokio::test]
    async fn create() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let subscriptions = notification::Subscriptions::default();

        let repos_dir = tempfile::tempdir_in(tmp_dir.path()).unwrap();
        let dir = tempfile::tempdir_in(repos_dir.path()).unwrap();
        let path = dir.path().to_str().unwrap();

        let api = super::filters(
            Arc::new(RwLock::new(librad_paths.clone())),
            Arc::new(RwLock::new(registry)),
            subscriptions,
        );
        let res = request()
            .method("POST")
            .path("/projects")
            .json(&super::CreateInput {
                path: path.into(),
                metadata: super::MetadataInput {
                    name: "Upstream".into(),
                    description: "Desktop client for radicle.".into(),
                    default_branch: "master".into(),
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
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "name": "Upstream",
            },
            "registration": Value::Null,
            "stats": {
                "branches": 11,
                "commits": 267,
                "contributors": 8,
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
        let subscriptions = notification::Subscriptions::default();

        let repo_dir = tempfile::tempdir_in(tmp_dir.path()).unwrap();
        let path = repo_dir.path().to_str().unwrap().to_string();
        coco::init_repo(path.clone()).unwrap();

        let (id, _meta) = coco::init_project(
            &librad_paths,
            &path,
            "Upstream",
            "Desktop client for radicle.",
            "master",
        )
        .unwrap();
        let project = project::get(&librad_paths, &id.to_string()).await.unwrap();

        let api = super::filters(
            Arc::new(RwLock::new(librad_paths)),
            Arc::new(RwLock::new(registry)),
            subscriptions,
        );
        let res = request()
            .method("GET")
            .path(&format!("/projects/{}", id.to_string()))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(project));
    }

    #[tokio::test]
    async fn list() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Paths::from_root(tmp_dir.path()).unwrap();
        let registry = registry::Registry::new(radicle_registry_client::Client::new_emulator());
        let subscriptions = notification::Subscriptions::default();

        coco::setup_fixtures(&librad_paths, tmp_dir.path().as_os_str().to_str().unwrap()).unwrap();

        let projects = coco::list_projects(&librad_paths)
            .into_iter()
            .map(|(id, meta)| project::Project {
                id,
                metadata: meta.into(),
                registration: None,
                stats: project::Stats {
                    branches: 11,
                    commits: 267,
                    contributors: 8,
                },
            })
            .collect::<Vec<project::Project>>();

        let api = super::filters(
            Arc::new(RwLock::new(librad_paths)),
            Arc::new(RwLock::new(registry)),
            subscriptions,
        );
        let res = request().method("GET").path("/projects").reply(&api).await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(have, json!(projects));
    }

    #[tokio::test]
    async fn register() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let librad_paths = Arc::new(RwLock::new(Paths::from_root(tmp_dir.path()).unwrap()));
        let registry = Arc::new(RwLock::new(registry::Registry::new(
            radicle_registry_client::Client::new_emulator(),
        )));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::<RwLock<Paths>>::clone(&librad_paths),
            Arc::<RwLock<registry::Registry>>::clone(&registry),
            subscriptions,
        );
        let res = request()
            .method("POST")
            .path("/projects/register")
            .json(&super::RegisterInput {
                project_name: "upstream".into(),
                org_id: "radicle".into(),
                maybe_coco_id: Some("1234.git".to_string()),
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

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));
    }
}
