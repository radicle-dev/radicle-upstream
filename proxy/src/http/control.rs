//! Endpoints to manipulate app state in test mode.

use librad::paths;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{path, reject, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Prefixed control filters.
pub fn routes<R: registry::Client>(
    enable: bool,
    librad_paths: Arc<RwLock<paths::Paths>>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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
            create_project_filter(Arc::clone(&librad_paths))
                .or(nuke_coco_filter(librad_paths))
                .or(nuke_registry_filter(registry))
                .or(nuke_session_filter(store)),
        )
}

/// Combination of all control filters.
#[allow(dead_code)]
fn filters<R: registry::Client>(
    librad_paths: Arc<RwLock<paths::Paths>>,
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    create_project_filter(Arc::clone(&librad_paths))
        .or(nuke_coco_filter(librad_paths))
        .or(nuke_registry_filter(registry))
        .or(nuke_session_filter(store))
}

/// POST /nuke/create-project
fn create_project_filter(
    librad_paths: Arc<RwLock<paths::Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("create-project")
        .and(super::with_paths(librad_paths))
        .and(warp::body::json())
        .and_then(handler::create_project)
}

/// GET /nuke/coco
fn nuke_coco_filter(
    librad_paths: Arc<RwLock<paths::Paths>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("nuke" / "coco")
        .and(super::with_paths(librad_paths))
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
    use librad::paths::Paths;
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
        librad_paths: Arc<RwLock<Paths>>,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection> {
        let dir = tempfile::tempdir().expect("tmp dir creation failed");
        let paths = librad_paths.read().await;
        let (id, meta) = coco::replicate_platinum(
            &dir,
            &paths,
            &input.name,
            &input.description,
            &input.default_branch,
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

    /// Reset the coco state by creating a new temporary directory for the librad paths.
    pub async fn nuke_coco(librad_paths: Arc<RwLock<Paths>>) -> Result<impl Reply, Rejection> {
        let dir = tempfile::tempdir().expect("tmp dir creation failed");
        let new = Paths::from_root(dir.path()).expect("unable to get paths");

        let mut paths = librad_paths.write().await;

        *paths = new;

        Ok(reply::json(&true))
    }

    /// Reset the Registry state by replacing the emulator in place.
    pub async fn nuke_registry<R: registry::Client>(
        registry: http::Shared<R>,
    ) -> Result<impl Reply, Rejection> {
        registry
            .write()
            .await
            .reset(radicle_registry_client::Client::new_emulator());

        Ok(reply::json(&true))
    }

    /// Reset the session state by clearing all buckets of the underlying store.
    pub async fn nuke_session(store: Arc<RwLock<Store>>) -> Result<impl Reply, Rejection> {
        let store = store.read().await;
        session::clear(&store)?;

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
