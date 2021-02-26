use std::time::{Duration, SystemTime};

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use coco::{peer::run_config, seed::Seed, state, RunConfig};

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
    let alice_peer = build_peer(
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

    let alice_peer = {
        let peer = alice_peer.peer.clone();
        tokio::task::spawn(alice_peer.into_running());
        peer
    };

    let alice = state::init_owner(&alice_peer, "alice".to_string()).await?;
    state::init_project(&alice_peer, &alice, shia_le_pathbuf(alice_repo_path))
        .await
        .expect("unable to init project");

    let announced = async_stream::stream! { loop { yield alice_events.recv().await } }
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Announced(updates) if updates.len() == 1 => future::ready(Some(())),
            res => {
                println!("What? {:?}", res);
                future::ready(None)
            },
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
    let alice_peer = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: run_config::Announce {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_peer_id = alice_peer.peer.peer_id();
    let alice = state::init_owner(&alice_peer.peer, "alice".to_string()).await?;

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let listen_addrs = alice_peer.listen_addrs.clone();
        tokio::task::spawn(alice_peer.into_running());
        (peer, listen_addrs)
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_peer = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    let bob_connected = bob_peer.subscribe();
    let mut bob_events = bob_peer.subscribe();

    let bob_peer = {
        let peer = bob_peer.peer.clone();
        tokio::task::spawn(bob_peer.into_running());
        peer
    };
    let _bob = state::init_owner(&bob_peer, "bob".to_string()).await?;
    connected(bob_connected, 1).await?;

    let project =
        state::init_project(&alice_peer, &alice, shia_le_pathbuf(alice_repo_path)).await?;

    let announced = async_stream::stream! { loop { yield bob_events.recv().await } }
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::GossipFetched {
                gossip, provider, ..
            } if provider.peer_id == alice_peer_id && gossip.urn.id == project.urn().id => {
                future::ready(Some(()))
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
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice_peer_id = alice_peer.peer.peer_id();
    let alice = state::init_owner(&alice_peer.peer, "alice".to_string()).await?;

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let listen_addrs = alice_peer.listen_addrs.clone();
        tokio::task::spawn(alice_peer.into_running());
        (peer, listen_addrs)
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_peer = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    state::init_owner(&bob_peer.peer, "bob".to_string()).await?;
    let bob_events = bob_peer.subscribe();
    let mut bob_control = bob_peer.control();
    let clone_listener = bob_peer.subscribe();
    let query_listener = bob_peer.subscribe();

    let bob_peer = {
        let peer = bob_peer.peer.clone();
        tokio::task::spawn(bob_peer.into_running());
        peer
    };

    connected(bob_events, 1).await?;

    let urn = {
        let project = radicle_project(alice_repo_path.clone());
        state::init_project(&alice_peer, &alice, project)
            .await?
            .urn()
    };

    bob_control.request_project(&urn, SystemTime::now()).await;

    requested(query_listener, &urn).await?;
    assert_cloned(clone_listener, &urn.clone(), alice_peer_id).await?;
    // TODO(finto): List projects
    let project = state::get_project(&bob_peer, urn).await;
    assert!(project.is_ok());

    Ok(())
}
