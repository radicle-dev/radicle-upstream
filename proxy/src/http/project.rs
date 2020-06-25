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
use crate::notification;
use crate::project;
use crate::registry;

/// Combination of all routes.
pub fn filters<R>(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    list_filter(Arc::clone(&peer))
        .or(create_filter(Arc::clone(&peer), owner))
        .or(get_filter(peer))
        .or(register_filter(registry, subscriptions))
}

/// `POST /projects`
fn create_filter(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects")
        .and(warp::post())
        .and(http::with_peer(peer))
        .and(http::with_shared(owner))
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

/// `POST /projects/register`
fn register_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("projects" / "register")
        .and(warp::post())
        .and(http::with_shared(registry))
        .and(http::with_subscriptions(subscriptions))
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
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::notification;
    use crate::project;
    use crate::registry;

    /// Create a new [`project::Project`].
    pub async fn create(
        peer: Arc<Mutex<coco::Peer>>,
        owner: http::Shared<coco::User>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let owner = &*owner.read().await;
        let mut peer = peer.lock().await;

        let meta = peer
            .init_project(
                owner,
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
                stats: None,
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
        // TODO(sos): Is there a better way to do this? Slicker use of move? Not locking the peer
        // more than I need to?
        let mut projects = peer
            .lock()
            .await
            .list_projects()?
            .into_iter()
            .map(|meta| project::Project {
                id: meta.urn(),
                shareable_entity_identifier: format!("%{}", meta.urn()),
                metadata: meta.into(),
                registration: None,
                stats: None,
            })
            .collect::<Vec<project::Project>>();

        for mut project in &mut projects {
            let stats = peer
                .lock()
                .await
                .with_browser(&project.id, |browser| Ok(browser.get_stats()?))?;
            project.stats = Some(coco::Stats(stats));
        }

        Ok(reply::json(&projects))
    }

    /// Register a project on the Registry.
    pub async fn register<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let reg = registry.read().await;
        let maybe_coco_id = input
            .maybe_coco_id
            .map(|id| librad::uri::RadUrn::from_str(&id).expect("Project RadUrn"));
        let domain_id = registry::Id::try_from(input.domain_id).map_err(Error::from)?;
        let domain: registry::ProjectDomain = match input.domain_type.clone() {
            registry::DomainType::Org => registry::ProjectDomain::Org(domain_id.clone()),
            registry::DomainType::User => registry::ProjectDomain::User(domain_id.clone()),
        };
        let project_name =
            registry::ProjectName::try_from(input.project_name).map_err(Error::from)?;

        let tx = reg
            .register_project(
                &fake_pair,
                domain,
                project_name,
                maybe_coco_id,
                input.transaction_fee,
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
        state.serialize_field(
            "shareableEntityIdentifier",
            &self.shareable_entity_identifier.to_string(),
        )?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("registration", &self.registration)?;
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
    /// The type of domain the project will be registered under.
    domain_type: registry::DomainType,
    /// Id of the domain the project will be registered under.
    domain_id: String,
    /// Unique name under Org of the project.
    project_name: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
    /// Optionally passed coco id to store for attestion.
    maybe_coco_id: Option<String>,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "domainType".into(),
            document::enum_string(vec!["org".into(), "user".into()])
                .description("The type of domain the project will be registered under")
                .example("org"),
        );
        properties.insert(
            "domainId".into(),
            document::string()
                .description("ID of the domain the project will be registered under")
                .example("monadic"),
        );
        properties.insert(
            "projectName".into(),
            document::string()
                .description("Unique name under the Org of the project")
                .example("upstream"),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
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

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::test::request;

    use librad::keys::SecretKey;

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::notification;
    use crate::project;
    use crate::registry::{self, Cache as _, Client as _};

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = coco::Peer::new(config).await?;
        let owner = Arc::new(RwLock::new(coco::fake_owner(&peer).await));
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let subscriptions = notification::Subscriptions::default();

        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let path = dir.path().to_str().unwrap();

        let peer = Arc::new(Mutex::new(peer));

        let api = super::filters(
            Arc::clone(&peer),
            Arc::clone(&owner),
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let subscriptions = notification::Subscriptions::default();

        let platinum_project = peer
            .replicate_platinum(&owner, "git-platinum", "fixture data", "master")
            .await?;
        let urn = platinum_project.urn();

        let project = project::get(&peer, &urn)?;

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner)),
            Arc::new(RwLock::new(registry)),
            subscriptions,
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
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let subscriptions = notification::Subscriptions::default();

        peer.setup_fixtures(&owner).await?;

        let projects = peer
            .list_projects()?
            .into_iter()
            .map(|meta| project::Project {
                id: meta.urn(),
                shareable_entity_identifier: format!("%{}", meta.urn()),
                metadata: meta.into(),
                registration: None,
                stats: None,
            })
            .collect::<Vec<project::Project>>();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner)),
            Arc::new(RwLock::new(registry)),
            subscriptions,
        );
        let res = request().method("GET").path("/projects").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(projects));
        });

        Ok(())
    }

    #[tokio::test]
    async fn register_under_org() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = coco::Peer::new(config).await?;
        let owner = coco::fake_owner(&peer).await;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner.clone())),
            Arc::clone(&cache),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let urn = librad::uri::RadUrn::new(
            owner.root_hash().clone(),
            librad::uri::Protocol::Git,
            librad::uri::Path::new(),
        );

        // Register user.
        cache
            .read()
            .await
            .register_user(&author, handle, None, 10)
            .await?;

        // Register org.
        cache
            .read()
            .await
            .register_org(&author, org_id.clone(), 10)
            .await?;

        let res = request()
            .method("POST")
            .path("/projects/register")
            .json(&super::RegisterInput {
                domain_type: registry::DomainType::Org,
                domain_id: org_id.to_string(),
                project_name: "upstream".into(),
                maybe_coco_id: Some(urn.to_string()),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let txs = cache.read().await.list_transactions(vec![])?;
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(have, json!(tx));

        let tx_msg = tx.messages.first().unwrap();
        match tx_msg {
            registry::Message::ProjectRegistration {
                project_name,
                domain_type,
                domain_id,
            } => {
                assert_eq!(
                    project_name.clone(),
                    registry::ProjectName::try_from("upstream").unwrap()
                );
                assert_eq!(domain_type.clone(), registry::DomainType::Org);
                assert_eq!(domain_id.clone(), org_id);
            },
            _ => panic!("The tx message is an unexpected variant."),
        }

        Ok(())
    }

    #[tokio::test]
    async fn register_under_user() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = coco::Peer::new(config).await?;
        let owner = coco::fake_owner(&peer).await;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner)),
            Arc::clone(&cache),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let urn = librad::uri::RadUrn::new(
            librad::hash::Hash::hash(b"upstream"),
            librad::uri::Protocol::Git,
            librad::uri::Path::new(),
        );

        // Register user.
        cache
            .read()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        let res = request()
            .method("POST")
            .path("/projects/register")
            .json(&super::RegisterInput {
                domain_type: registry::DomainType::User,
                domain_id: handle.to_string(),
                project_name: "upstream".into(),
                maybe_coco_id: Some(urn.to_string()),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let txs = cache.read().await.list_transactions(vec![])?;
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(have, json!(tx));

        let tx_msg = tx.messages.first().unwrap();
        match tx_msg {
            registry::Message::ProjectRegistration {
                project_name,
                domain_type,
                domain_id,
            } => {
                assert_eq!(
                    project_name.clone(),
                    registry::ProjectName::try_from("upstream").unwrap()
                );
                assert_eq!(domain_type.clone(), registry::DomainType::User);
                assert_eq!(domain_id.clone(), handle);
            },
            _ => panic!("The tx message is an unexpected variant."),
        }

        Ok(())
    }
}
