//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use warp::{path, reject, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::registry;

/// Prefixed control filters.
pub fn routes<R>(
    enable: bool,
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    path("control")
        .map(move || enable)
        .and_then(|enable| async move {
            if enable {
                Ok(())
            } else {
                Err(reject::not_found())
            }
        })
        .untuple_one()
        .and(
            create_project_filter(Arc::clone(&peer), owner)
                .or(nuke_coco_filter(peer))
                .or(nuke_registry_filter(registry))
                .or(nuke_session_filter(store)),
        )
}

/// Combination of all control filters.
#[allow(dead_code)]
fn filters<R>(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    create_project_filter(Arc::clone(&peer), owner)
        .or(nuke_coco_filter(peer))
        .or(nuke_registry_filter(registry))
        .or(nuke_session_filter(store))
}

/// POST /nuke/create-project
fn create_project_filter(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .and(super::with_peer(peer))
        .and(super::with_shared(owner))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /nuke/coco
fn nuke_coco_filter(
    peer: Arc<Mutex<coco::Peer>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("nuke" / "coco")
        .and(super::with_peer(peer))
        .and_then(handler::nuke_coco)
}

/// GET /nuke/registry
fn nuke_registry_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("nuke" / "registry")
        .and(http::with_shared(registry))
        .and_then(handler::nuke_registry)
}

/// GET /nuke/session
fn nuke_session_filter(
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("nuke" / "session")
        .and(super::with_store(store))
        .and_then(handler::nuke_session)
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use kv::Store;
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use librad::keys::SecretKey;

    use crate::coco;
    use crate::http;
    use crate::project;
    use crate::registry;
    use crate::session;

    /// Create a project from the fixture repo.
    pub async fn create_project(
        peer: Arc<Mutex<coco::Peer>>,
        owner: http::Shared<coco::User>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let owner = &*owner.read().await;
        let mut peer = peer.lock().await;

        let meta = peer
            .replicate_platinum(
                owner,
                &input.name,
                &input.description,
                &input.default_branch,
            )
            .await?;

        Ok(reply::with_status(
            reply::json(&project::Project {
                id: meta.urn(),
                shareable_entity_identifier: format!("%{}", meta.urn()),
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

    /// Reset the coco state by creating a new temporary directory for the librad paths.
    pub async fn nuke_coco(peer: Arc<Mutex<coco::Peer>>) -> Result<impl Reply, Rejection> {
        let temp_dir = tempfile::tempdir().expect("test dir creation failed");
        let tmp_path = temp_dir.path().to_str().expect("path extraction failed");
        let config = coco::default_config(SecretKey::new(), tmp_path)?;
        let new_peer = coco::Peer::new(config).await?;

        let mut peer = peer.lock().await;
        *peer = new_peer;

        Ok(reply::json(&true))
    }

    /// Reset the Registry state by replacing the emulator in place.
    pub async fn nuke_registry<R: registry::Client>(
        registry: http::Shared<R>,
    ) -> Result<impl Reply, Rejection> {
        let (client, _) = radicle_registry_client::Client::new_emulator();
        registry.write().await.reset(client);

        Ok(reply::json(&true))
    }

    /// Reset the session state by clearing all buckets of the underlying store.
    pub async fn nuke_session(store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        session::clear_current(&store)?;

        Ok(reply::json(&true))
    }

    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_ne;
        use std::sync::Arc;
        use tokio::sync::Mutex;

        use crate::coco;
        use crate::error;

        #[tokio::test]
        async fn nuke_coco() -> Result<(), error::Error> {
            let tmp_dir = tempfile::tempdir()?;
            let key = librad::keys::SecretKey::new();
            let config = coco::default_config(key, tmp_dir)?;
            let peer = Arc::new(Mutex::new(coco::Peer::new(config).await?));

            let old_paths = {
                let p = peer.lock().await;
                p.with_api(|api| api.paths().clone())?
            };

            super::nuke_coco(Arc::clone(&peer)).await.unwrap();

            let new_paths = {
                let p = peer.lock().await;
                p.with_api(|api| api.paths().clone())?
            };

            assert_ne!(old_paths.all_dirs(), new_paths.all_dirs());

            Ok(())
        }
    }
}

/// Inputs for project creation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Name of the proejct.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::registry;

    #[tokio::test]
    async fn create_project_after_nuke() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = librad::keys::SecretKey::new();
        let config = coco::default_config(key, tmp_dir.path())?;
        let peer = coco::Peer::new(config).await?;
        let owner = coco::fake_owner(&peer).await;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner)),
            Arc::new(RwLock::new(registry)),
            Arc::new(RwLock::new(store)),
        );

        // Create project before nuke.
        let res = request()
            .method("POST")
            .path("/create-project")
            .json(&super::CreateInput {
                name: "Monadic".into(),
                description: "blabla".into(),
                default_branch: "master".into(),
            })
            .reply(&api)
            .await;
        http::test::assert_response(&res, StatusCode::CREATED, |_have| {});

        // Reset state.
        let res = request().method("GET").path("/nuke/coco").reply(&api).await;
        assert_eq!(res.status(), StatusCode::OK);

        let res = request()
            .method("POST")
            .path("/create-project")
            .json(&super::CreateInput {
                name: "Monadic".into(),
                description: "blabla".into(),
                default_branch: "master".into(),
            })
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::CREATED, |_have| {});

        Ok(())
    }
}
