use std::time::{Duration, SystemTime};

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::net::{
    gossip::{Has, Info},
    peer::Gossip,
    protocol::ProtocolEvent,
};

use coco::{peer::run_config, seed::Seed, RunConfig};

#[macro_use]
mod common;
use common::{
    assert_cloned, build_peer, build_peer_with_seeds, connected, init_logging, radicle_project,
    requested, shia_le_pathbuf,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn can_announce_new_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: run_config::Announce {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let mut alice_events = alice_peer.subscribe();

    tokio::spawn(alice_peer.into_running());

    let alice = alice_state.init_owner("alice".to_string()).await?;
    alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await
        .expect("unable to init project");

    let announced = async_stream::stream! { loop { yield alice_events.recv().await } }
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Announced(updates) if updates.len() == 1 => future::ready(Some(())),
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn can_observe_announcement_from_connected_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: run_config::Announce {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_addrs = alice_state.listen_addrs().collect();
    let alice_peer_id = alice_state.peer_id();
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    let _bob = bob_state.init_owner("bob".to_string()).await?;
    let bob_connected = bob_peer.subscribe();
    let mut bob_events = bob_peer.subscribe();

    tokio::spawn(alice_peer.into_running());
    tokio::spawn(bob_peer.into_running());

    connected(bob_connected, &alice_peer_id).await?;

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let announced = async_stream::stream! { loop { yield bob_events.recv().await } }
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Protocol(ProtocolEvent::Gossip(info)) => match info {
                Info::Has(Has {
                    provider,
                    val: Gossip { urn, .. },
                }) if provider.peer_id == alice_peer_id && urn.id == project.urn().id => {
                    future::ready(Some(()))
                },
                _ => future::ready(None),
            },
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn can_ask_and_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice_addrs = alice_state.listen_addrs().collect();
    let alice_peer_id = alice_state.peer_id();
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    bob_state.init_owner("bob".to_string()).await?;
    let bob_events = bob_peer.subscribe();
    let mut bob_control = bob_peer.control();
    let clone_listener = bob_peer.subscribe();
    let query_listener = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    connected(bob_events, &alice_peer_id).await?;

    let urn = {
        let project = radicle_project(alice_repo_path.clone());
        alice_state.init_project(&alice, project).await?.urn()
    };

    bob_control.request_project(&urn, SystemTime::now()).await;

    requested(query_listener, &urn).await?;
    assert_cloned(clone_listener, &urn.clone(), alice_peer_id).await?;
    // TODO(finto): List projects
    let project = bob_state.get_project(urn).await;
    assert!(project.is_ok());

    Ok(())
}
