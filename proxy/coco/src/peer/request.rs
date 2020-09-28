use std::time::{Instant, Duration};

use librad::{uri::RadUrn, net::peer::Gossip};

use crate::{shared::Shared, request::waiting_room::WaitingRoom, peer::run_state::RequestCommand, state::Lock};

/// Execute a [`RequestCommand`] modifying the `waiting_room` as necessary.
pub async fn request(
    request_command: RequestCommand,
    state: Lock,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
) {
    let err_msg = request_command.err_msg();
    match request_command {
        RequestCommand::Clone(url) => {
            waiting_room
                .write()
                .await
                .cloning(url.clone(), Instant::now())
                .unwrap_or_else(|err| log::warn!("{}:\n{}", err_msg, err));
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
                    Ok(_) => waiting_room
                        .cloned(&url.urn, Instant::now())
                        .unwrap_or_else(|err| log::warn!("{}:\n{}", err_msg, err)),
                    Err(err) => {
                        log::warn!("failed cloning from URL '{}':\n{}", url, err);
                        waiting_room
                            .cloning_failed(url, Instant::now())
                            .unwrap_or_else(|err| log::warn!("{}:\n{}", err_msg, err))
                    },
                }
            }
        },
        RequestCommand::Found(url) => waiting_room
            .write()
            .await
            .found(url, Instant::now())
            .unwrap_or_else(|err| log::warn!("{}:\n{}", err_msg, err)),
        RequestCommand::Query(urn) => {
            let protocol = state.lock().await.api.protocol().clone();

            protocol
                .query(Gossip {
                    urn: urn.clone(),
                    rev: None,
                    origin: None,
                })
                .await;

            waiting_room
                .write()
                .await
                .queried(&urn, Instant::now())
                .unwrap_or_else(|err| log::warn!("{}:\n{}", err_msg, err))
        },
    }
}

