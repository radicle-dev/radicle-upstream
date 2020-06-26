//! Endpoints to manipulate app state in test mode.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
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
                .or(nuke_registry_filter(Arc::clone(&registry)))
                .or(register_user_filter(registry)),
        )
}

/// Combination of all control filters.
#[allow(dead_code)]
fn filters<R>(
    peer: Arc<Mutex<coco::Peer>>,
    owner: http::Shared<coco::User>,
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    create_project_filter(Arc::clone(&peer), owner)
        .or(nuke_coco_filter(peer))
        .or(nuke_registry_filter(Arc::clone(&registry)))
        .or(register_user_filter(registry))
}

/// POST /create-project
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

/// POST /register-user
fn register_user_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("register-user")
        .and(http::with_shared(registry))
        .and(warp::body::json())
        .and_then(handler::register_user)
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

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use librad::keys::SecretKey;

    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::project;
    use crate::registry;

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
        let stats = peer.with_api(|api| {
            coco::Peer::with_browser(api, &meta.urn(), |browser| Ok(browser.get_stats()?))
        })?;

        Ok(reply::with_status(
            reply::json(&project::Project::from_project_stats(meta, stats)),
            StatusCode::CREATED,
        ))
    }

    /// Register a user with another key
    pub async fn register_user<R: registry::Client>(
        registry: http::Shared<R>,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        let fake_pair =
            radicle_registry_client::ed25519::Pair::from_legacy_string(&input.handle, None);

        let handle = registry::Id::try_from(input.handle).map_err(Error::from)?;
        let reg = registry.write().await;
        reg.register_user(&fake_pair, handle.clone(), None, input.transaction_fee)
            .await
            .expect("unable to register user");

        Ok(reply::json(&true))
    }

    /// Reset the coco state by creating a new temporary directory for the librad paths.
    pub async fn nuke_coco(peer: Arc<Mutex<coco::Peer>>) -> Result<impl Reply, Rejection> {
        // TmpDir deletes the temporary directory once it DROPS.
        // This means our new directory goes missing, and future calls will fail.
        // The Peer creates the directory again.
        //
        // N.B. this may gather lot's of tmp files on your system. We're sorry.
        let tmp_path = {
            let temp_dir = tempfile::tempdir().expect("test dir creation failed");
            temp_dir.path().to_path_buf()
        };

        let config = coco::default_config(
            SecretKey::new(),
            tmp_path.to_str().expect("path extraction failed"),
        )?;
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

    #[allow(clippy::unwrap_used, clippy::panic)]
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

            let (old_paths, old_key, old_peer_id) = {
                let p = peer.lock().await;
                p.with_api(|api| Ok((api.paths().clone(), api.public_key(), api.peer_id())))?
            };

            super::nuke_coco(Arc::clone(&peer)).await.unwrap();

            let (new_paths, new_key, new_peer_id) = {
                let p = peer.lock().await;
                p.with_api(|api| Ok((api.paths().clone(), api.public_key(), api.peer_id())))?
            };

            assert_ne!(old_paths.all_dirs(), new_paths.all_dirs());
            assert_ne!(old_key, new_key);
            assert_ne!(old_peer_id, new_peer_id);

            let can_open = {
                let p = peer.lock().await;
                p.with_api(|api| {
                    let _ = api.storage().reopen().expect("failed to reopen Storage");
                    Ok(true)
                })?
            };
            assert!(can_open);

            Ok(())
        }
    }
}

/// Inputs for project creation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Name of the project.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
}
/// Input for user registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Handle of the user.
    handle: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
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

        let api = super::filters(
            Arc::new(Mutex::new(peer)),
            Arc::new(RwLock::new(owner)),
            Arc::new(RwLock::new(registry)),
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
