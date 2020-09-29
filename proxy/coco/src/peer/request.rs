//! Perform request commands to query and clone identities.

use std::time::{Duration, Instant};

use librad::{
    net::peer::Gossip,
    uri::{RadUrl, RadUrn},
};

use crate::{
    error,
    request::waiting_room::{self, WaitingRoom},
    shared::Shared,
    state::Lock,
};

/// An error that can occur when attempting to work with the waiting room or cloning a project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred interacting with the waiting room.
    #[error(transparent)]
    WaitingRoom(#[from] waiting_room::Error),
    /// An error occurred when attempting to clone a project.
    #[error(transparent)]
    Crate(#[from] error::Error),
}

/// Emit a [`Gossip`] request for the given `urn` and mark the `urn` as [`WaitingRoom`::queried`].
///
/// # Errors
///
///   * Failed to mark the `urn` as queried in the `WaitingRoom`.
pub async fn query(
    urn: RadUrn,
    state: Lock,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
) -> Result<(), waiting_room::Error> {
    let protocol = state.lock().await.api.protocol().clone();

    protocol
        .query(Gossip {
            urn: urn.clone(),
            rev: None,
            origin: None,
        })
        .await;

    Ok(waiting_room.write().await.queried(&urn, Instant::now())?)
}

/// Mark the `url` as [`WaitingRoom`::found`].
///
/// # Errors
///
///   * Failed to mark the `url` as found in the `WaitingRoom`.
pub async fn found(
    url: RadUrl,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
) -> Result<(), waiting_room::Error> {
    Ok(waiting_room.write().await.found(url, Instant::now())?)
}

/// Attempt to clone the given `url`.
///
/// If the clone was successful then we mark the `url` as [`WaitingRoom::cloned`], otherwise we
/// mark it as [`WaitingRoom::cloning_failed`].
///
/// # Errors
///
///   * Failed to mark the `url` as cloned/cloning failed in the `WaitingRoom`.
///   * Failed to clone the project from the `url`.
pub async fn clone(
    url: RadUrl,
    state: Lock,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
) -> Result<(), Error> {
    waiting_room
        .write()
        .await
        .cloning(url.clone(), Instant::now())?;
    {
        let state = state.clone();
        let state = state.lock_owned().await;

        let res: Result<RadUrn, _> = {
            let url = url.clone();
            tokio::task::spawn_blocking(move || state.clone_project(url.clone(), None))
                .await
                .expect("failed to join thread")
        };

        let mut waiting_room = waiting_room.write().await;
        match res {
            Ok(_) => Ok(waiting_room.cloned(&url.urn.clone(), url, Instant::now())?),
            Err(err) => {
                waiting_room.cloning_failed(url.clone(), Instant::now())?;
                Err(Error::from(err))
            },
        }
    }
}
