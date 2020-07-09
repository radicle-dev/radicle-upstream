//! Endpoints for Org.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::coco;
use crate::http;
use crate::notification;
use crate::project;
use crate::registry;

/// Prefixed filters.
pub fn routes<R>(
    peer: Arc<Mutex<coco::PeerApi>>,
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    path("orgs").and(
        get_filter(Arc::clone(&registry))
            .or(register_project_filter(
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(get_project_filter(Arc::clone(&registry)))
            .or(get_projects_filter(peer, Arc::clone(&registry)))
            .or(register_filter(
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(register_member_filter(registry, subscriptions)),
    )
}

/// Combination of all org routes.
#[cfg(test)]
fn filters<R>(
    peer: Arc<Mutex<coco::PeerApi>>,
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    get_filter(Arc::clone(&registry))
        .or(register_project_filter(
            Arc::clone(&registry),
            subscriptions.clone(),
        ))
        .or(get_project_filter(Arc::clone(&registry)))
        .or(get_projects_filter(peer, Arc::clone(&registry)))
        .or(register_filter(
            Arc::clone(&registry),
            subscriptions.clone(),
        ))
        .or(register_member_filter(registry, subscriptions))
}

/// `GET /<id>`
fn get_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    http::with_shared(registry)
        .and(warp::get())
        .and(document::param::<String>("id", "Unique ID of the Org"))
        .and(path::end())
        .and(document::document(document::description("Find Org by ID")))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("Org not found"),
        ))
        .and_then(handler::get)
}

/// `POST /<id>/projects/<name>`
fn register_project_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(document::param::<String>("org_id", "Unique ID of the Org"))
        .and(path("projects"))
        .and(document::param::<String>(
            "project_name",
            "Name of the project",
        ))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new project under the org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(http::RegisterProjectInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Registration succeeded"),
        ))
        .and_then(handler::register_project)
}

/// `GET /<id>/projects/<project_name>`
fn get_project_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(warp::get())
        .and(document::param::<String>("org_id", "Unique ID of the Org"))
        .and(path("projects"))
        .and(document::param::<String>(
            "project_name",
            "Name of the project",
        ))
        .and(path::end())
        .and(document::document(document::description(
            "Find Project for Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Project::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("Project not found"),
        ))
        .and_then(handler::get_project)
}

/// `GET /<id>/projects`
fn get_projects_filter<R>(
    peer: Arc<Mutex<coco::PeerApi>>,
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    http::with_shared(registry)
        .and(http::with_peer(peer))
        .and(warp::get())
        .and(document::param::<String>("org_id", "Unique ID of the Org"))
        .and(path("projects"))
        .and(path::end())
        .and(document::document(document::description(
            "Lists all Projects of the Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Project::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::get_projects)
}

/// `POST /`
fn register_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new unique Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register)
}

/// `POST /<id>/members`
fn register_member_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(document::param::<String>("id", "Unique ID of the Org"))
        .and(path("members"))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a member",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(RegisterMemberInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register_member)
}

/// Org handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::convert::TryFrom;
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

    /// Get the Org for the given `id`.
    pub async fn get<R: registry::Client>(
        registry: http::Shared<R>,
        org_id: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(Error::from)?;
        let org = reg.get_org(org_id).await?;

        Ok(reply::json(&org))
    }

    /// Register a project in the Registry.
    pub async fn register_project<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        org_id: String,
        project_name: String,
        input: http::RegisterProjectInput,
    ) -> Result<impl Reply, Rejection> {
        http::register_project(
            registry,
            subscriptions,
            registry::DomainType::Org,
            org_id,
            project_name,
            input,
        )
        .await
    }

    /// Get the [`registry::Project`] under the given org id.
    pub async fn get_project<R: registry::Client>(
        registry: http::Shared<R>,
        org_id: String,
        project_name: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(Error::from)?;
        let project_domain = registry::ProjectDomain::Org(org_id);
        let project_name = registry::ProjectName::try_from(project_name).map_err(Error::from)?;
        let project = reg.get_project(project_domain, project_name).await?;

        Ok(reply::json(&project))
    }

    /// Get all projects under the given org id.
    pub async fn get_projects<R>(
        registry: http::Shared<R>,
        peer: Arc<Mutex<coco::PeerApi>>,
        org_id: String,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
    {
        let reg = registry.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(Error::from)?;
        let projects = reg.list_org_projects(org_id).await?;
        let peer = peer.lock().await;
        let mut mapped_projects = Vec::new();
        for p in &projects {
            let maybe_project = if let Some(urn) = &p.maybe_project_id {
                Some(project::get(&peer, urn).expect("Project not found"))
            } else {
                None
            };

            let org_project = super::Project {
                name: p.name.to_string(),
                org_id: p.domain.id().to_string(),
                shareable_entity_identifier: format!(
                    "%{}/{}",
                    p.domain.id().to_string(),
                    p.name.to_string()
                ),
                maybe_project,
            };
            mapped_projects.push(org_project);
        }

        Ok(reply::json(&mapped_projects))
    }

    /// Register an org on the Registry.
    pub async fn register<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let reg = registry.read().await;
        let org_id = registry::Id::try_from(input.id).map_err(Error::from)?;
        let tx = reg
            .register_org(&fake_pair, org_id, input.transaction_fee)
            .await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Register a member under an org on the Registry.
    pub async fn register_member<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        id: String,
        input: super::RegisterMemberInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let reg = registry.read().await;
        let org_id = registry::Id::try_from(id).map_err(Error::from)?;
        let handle = registry::Id::try_from(input.handle).map_err(Error::from)?;
        let tx = reg
            .register_member(&fake_pair, org_id, handle, input.transaction_fee)
            .await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

impl ToDocumentedType for registry::Org {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert("avatarFallback".into(), avatar::Avatar::document());
        properties.insert(
            "id".into(),
            document::string()
                .description("The id of the org")
                .example("monadic"),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%monadic"),
        );
        properties.insert(
            "members".into(),
            document::array(registry::User::document()),
        );

        document::DocumentedType::from(properties).description("Org")
    }
}

impl Serialize for registry::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 3)?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("orgId", &self.domain.id().to_string())?;
        state.serialize_field("maybeProjectId", &self.maybe_project_id)?;

        state.end()
    }
}

impl ToDocumentedType for registry::Project {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the project")
                .example("upstream"),
        );
        properties.insert(
            "orgId".into(),
            document::string()
                .description("The id of the org")
                .example("radicle"),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%monadic/radicle-link"),
        );
        properties.insert(
            "maybeProjectId".into(),
            document::string()
                .description("The id project attested in coco")
                .example("123abdcd.git")
                .nullable(true),
        );

        document::DocumentedType::from(properties).description("Project")
    }
}

/// Object the API returns for a project that is registered under an org.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Id of the Org.
    org_id: String,
    /// Unambiguous identifier pointing at this identity.
    shareable_entity_identifier: String,
    /// Name of the project.
    name: String,
    /// Associated CoCo project.
    maybe_project: Option<project::Project>,
}

/// Bundled input data for org registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Id of the Org.
    id: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "id".into(),
            document::string()
                .description("ID of the org")
                .example("monadic"),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for org registration")
    }
}

/// Bundled input data for member registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMemberInput {
    /// Id of the User.
    handle: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for RegisterMemberInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "handle".into(),
            document::string()
                .description("Handle of the user")
                .example("cloudhead"),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for member registration")
    }
}

#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]
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
    use radicle_registry_client as protocol;

    use crate::avatar;
    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::notification;
    use crate::registry::{self, Cache as _, Client as _};

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key, tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&registry),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        // Register the user
        registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        let user = registry.read().await.get_user(handle).await?.unwrap();

        // Register the org
        let fee: registry::Balance = 100;
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), fee)
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}", org_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!(registry::Org {
                    id: org_id.clone(),
                    shareable_entity_identifier: format!("%{}", org_id.to_string()),
                    avatar_fallback: avatar::Avatar::from(&org_id.to_string(), avatar::Usage::Org),
                    members: vec![user]
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn register_project() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key.clone(), tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let owner = coco::init_user(&peer, key, "cloudhead")?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&cache),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let urn = coco::Urn::new(
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

        // Register project
        let project_name = "upstream";

        let res = request()
            .method("POST")
            .path(&format!("/{}/projects/{}", org_id, project_name))
            .json(&http::RegisterProjectInput {
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
    async fn get_project() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key, tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&registry),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let project_name = registry::ProjectName::try_from("upstream")?;
        let project_domain = registry::ProjectDomain::Org(org_id.clone());

        // Register the user
        registry
            .write()
            .await
            .register_user(&author, handle, None, 10)
            .await?;

        // Register the org.
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), 10)
            .await?;

        // Register the project.
        registry
            .write()
            .await
            .register_project(
                &author,
                project_domain.clone(),
                project_name.clone(),
                None,
                10,
            )
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/projects/{}", org_id, project_name))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!(registry::Project {
                    name: project_name,
                    domain: project_domain,
                    maybe_project_id: None,
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_projects() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key.clone(), tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let owner = coco::init_user(&peer, key.clone(), "cloudhead")?;
        let owner = coco::verify_user(owner)?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let subscriptions = notification::Subscriptions::default();

        let project_name = "upstream";
        let project_description = "desktop client for radicle";
        let default_branch = "master";

        let platinum_project = coco::control::replicate_platinum(
            &peer,
            key,
            &owner,
            project_name,
            project_description,
            default_branch,
        )?;
        let urn = platinum_project.urn();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&registry),
            subscriptions,
        );

        // Register the user
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let project_name = registry::ProjectName::try_from(project_name)?;
        let project_domain = registry::ProjectDomain::Org(org_id.clone());

        registry
            .write()
            .await
            .register_user(&author, handle, None, 10)
            .await?;

        // Register the org.
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), 10)
            .await?;

        // Register the project.
        registry
            .write()
            .await
            .register_project(
                &author,
                project_domain,
                project_name.clone(),
                Some(urn.clone()),
                10,
            )
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/projects", org_id.to_string()))
            .reply(&api)
            .await;

        let want = json!([{
            "name": project_name.to_string(),
            "orgId": org_id.to_string(),
            "shareableEntityIdentifier": format!("%{}/{}", org_id.to_string(), project_name.to_string()),
            "maybeProject": {
                "id": urn.to_string(),
                "metadata": {
                    "defaultBranch": default_branch.to_string(),
                    "description": project_description.to_string(),
                    "name": project_name.to_string(),
                },
                "registration": Value::Null,
                "shareableEntityIdentifier": format!("%{}", urn),
                "stats": {
                  "branches": 2,
                  "commits": 14,
                  "contributors": 4
                }
            },
        }]);

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, want);
        });

        Ok(())
    }

    #[tokio::test]
    async fn register() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key, tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&cache),
            subscriptions,
        );
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        // Register the user
        cache
            .write()
            .await
            .register_user(&author, handle, None, 10)
            .await?;

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::RegisterInput {
                id: org_id.to_string(),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        let txs = cache.write().await.list_transactions(vec![])?;

        // Get the registered org
        let org = cache.read().await.get_org(org_id.clone()).await?.unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(txs.len(), 2);
        assert_eq!(org.id, org_id);

        Ok(())
    }

    #[tokio::test]
    async fn register_member() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key, tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::clone(&cache),
            subscriptions,
        );
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        // Register the user
        cache
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        cache
            .write()
            .await
            .register_org(&author, org_id.clone(), 10)
            .await?;

        // Register a second user
        let author2 = protocol::ed25519::Pair::from_legacy_string("//Bob", None);
        let handle2 = registry::Id::try_from("bob")?;
        cache
            .write()
            .await
            .register_user(&author2, handle2.clone(), None, 10)
            .await?;

        // Register the second user as a member of the org
        let res = request()
            .method("POST")
            .path(&format!("/{}/members", org_id.clone()))
            .json(&super::RegisterMemberInput {
                handle: handle2.clone().to_string(),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        let txs = cache.write().await.list_transactions(vec![])?;

        // Get the org and its members
        let org = cache.read().await.get_org(org_id).await?.unwrap();
        let member_handles: Vec<registry::Id> =
            org.members.iter().map(|user| user.handle.clone()).collect();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(txs.len(), 4);
        assert_eq!(org.members.len(), 2);
        assert!(member_handles.contains(&handle));
        assert!(member_handles.contains(&handle2));

        Ok(())
    }
}
