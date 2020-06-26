//! Endpoints and serialisation for [`project::Project`] related types.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::project;
use crate::registry;

/// Combination of all routes.
pub fn filters(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<Option<coco::User>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_filter(Arc::clone(&peer))
        .or(create_filter(Arc::clone(&peer), owner))
        .or(get_filter(peer))
}

/// `POST /projects`
fn create_filter(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<Option<coco::User>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(warp::post())
        .and(http::with_peer(peer))
        .and(http::with_owner_guard(owner))
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
    peer: Arc<Mutex<coco::Peer>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("projects")
        .and(warp::get())
        .and(http::with_peer(peer))
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
    peer: Arc<Mutex<coco::Peer>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(warp::get())
        .and(http::with_peer(peer))
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

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::error::Error;
    use crate::project;

    /// Create a new [`project::Project`].
    pub async fn create(
        peer: Arc<Mutex<coco::Peer>>,
        owner: coco::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let mut peer = peer.lock().await;

        let meta = peer
            .init_project(
                &owner,
                &input.path,
                &input.metadata.name,
                &input.metadata.description,
                &input.metadata.default_branch,
            )
            .await?;
        let urn = meta.urn();

        let shareable_entity_identifier = format!("%{}", urn);
        Ok(reply::with_status(
            reply::json(&project::Project {
                id: urn,
                shareable_entity_identifier,
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
    pub async fn get(peer: Arc<Mutex<coco::Peer>>, urn: String) -> Result<impl Reply, Rejection> {
        let urn = urn.parse().map_err(Error::from)?;
        let peer = peer.lock().await;

        Ok(reply::json(&project::get(&peer, &urn)?))
    }

    /// List all known projects.
    pub async fn list(peer: Arc<Mutex<coco::Peer>>) -> Result<impl Reply, Rejection> {
        let projects = peer
            .lock()
            .await
            .list_projects()?
            .into_iter()
            .map(|meta| project::Project {
                id: meta.urn(),
                shareable_entity_identifier: format!("%{}", meta.urn()),
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
}

impl Serialize for project::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field(
            "shareableEntityIdentifier",
            &self.shareable_entity_identifier.to_string(),
        )?;
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
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%123abcd.git"),
        );
        properties.insert("metadata".into(), project::Metadata::document());
        properties.insert("registration".into(), project::Registration::document());
        properties.insert("stats".into(), project::Stats::document());

        document::DocumentedType::from(properties)
            .description("Radicle project for sharing and collaborating")
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

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::test::request;

    use librad::keys::SecretKey;

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::project;

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = coco::Peer::new(config).await?;
        let owner = Arc::new(RwLock::new(Some(coco::fake_owner(&peer).await)));
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let path = dir.path().to_str().unwrap();

        let peer = Arc::new(Mutex::new(peer));
        let api = super::filters(Arc::clone(&peer), Arc::clone(&owner));
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

        let projects = peer.lock().await.list_projects()?;
        let meta = projects.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": meta.urn().to_string(),
            "metadata": {
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "name": "Upstream",
            },
            "registration": Value::Null,
            "shareableEntityIdentifier": format!("%{}", meta.urn().to_string()),
            "stats": {
                "branches": 11,
                "commits": 267,
                "contributors": 8,
            },
        });

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let mut peer = coco::Peer::new(config).await?;
        let owner = coco::fake_owner(&peer).await;
        let platinum_project = peer
            .replicate_platinum(&owner, "git-platinum", "fixture data", "master")
            .await?;
        let urn = platinum_project.urn();

        let project = project::get(&peer, &urn)?;

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(Some(owner))),
        );

        let res = request()
            .method("GET")
            .path(&format!("/projects/{}", urn))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(project));
        });

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let mut peer = coco::Peer::new(config).await?;
        let owner = coco::fake_owner(&peer).await;

        peer.setup_fixtures(&owner).await?;

        let projects = peer
            .list_projects()?
            .into_iter()
            .map(|meta| project::Project {
                id: meta.urn(),
                shareable_entity_identifier: format!("%{}", meta.urn()),
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
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(Some(owner))),
        );
        let res = request().method("GET").path("/projects").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(projects));
        });

        Ok(())
    }
}
