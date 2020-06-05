//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, reject, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::registry;

/// Prefixed control filters.
pub fn routes<R>(
    enable: bool,
    peer: Arc<coco::Peer>,
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
    peer: Arc<coco::Peer>,
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
    peer: Arc<coco::Peer>,
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
    peer: Arc<coco::Peer>,
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
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::http;
    use crate::project;
    use crate::registry;
    use crate::session;

    /// Create a project from the fixture repo.
    pub async fn create_project(
        mut peer: Arc<coco::Peer>,
        owner: http::Shared<coco::User>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let owner = &*owner.read().await;
        let peer = Arc::make_mut(&mut peer);

        let meta = peer
            .replicate_platinum(
                &owner,
                &input.name,
                &input.description,
                &input.default_branch,
            )
            .await?;

        Ok(reply::with_status(
            reply::json(&project::Project {
                id: meta.urn().clone(),
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
    pub async fn nuke_coco(_peer: Arc<coco::Peer>) -> Result<impl Reply, Rejection> {
        // let tmp = coco::Coco::tmp()?;
        // let mut coco: coco::Coco = &mut *coco.write().await;
        // *coco = tmp;

        // Ok(reply::json(&true))

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
