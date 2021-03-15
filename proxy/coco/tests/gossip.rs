use std::time::{Duration, SystemTime};

use assert_matches::assert_matches;
use futures::{future, StreamExt as _};
use tokio::time::timeout;

use coco::{
    identities::payload::Person,
    peer::run_config,
    seed::Seed,
    state::{self, init_owner},
    RunConfig,
};

#[macro_use]
mod common;
use common::{
    assert_cloned, build_peer, build_peer_with_seeds, connected, init_logging, radicle_project,
    requested, shia_le_pathbuf, started,
};

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
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
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
        let events = bob_peer.subscribe();
        tokio::task::spawn(bob_peer.run());
        started(events).await?;

        peer
    };
    let _bob = init_owner(&bob_peer, Person { name: "bob".into() }).await?;
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
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;
    let mut alice_events = alice_peer.subscribe();

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
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
    let bob_peer_id = bob_peer.peer.peer_id();

    init_owner(&bob_peer.peer, Person { name: "bob".into() }).await?;
    let bob_events = bob_peer.subscribe();
    let mut bob_control = bob_peer.control();
    let clone_listener = bob_peer.subscribe();
    let query_listener = bob_peer.subscribe();

    let bob_peer = {
        let peer = bob_peer.peer.clone();
        let events = bob_peer.subscribe();
        tokio::task::spawn(bob_peer.run());
        started(events).await?;

        peer
    };

    connected(bob_events, 1).await?;

    let urn = {
        let project = radicle_project(alice_repo_path.clone());
        let urn = state::init_project(&alice_peer, &alice, project)
            .await?
            .urn();

        urn
    };

    // Alice will track Bob in anticipation of upcoming contributions.
    state::track(&alice_peer, urn.clone(), bob_peer_id).await?;

    // Make sure Bob is NotReplicated.
    assert_eq!(
        state::tracked(&alice_peer, urn.clone()).await?,
        vec![coco::project::peer::Peer::Remote {
            peer_id: bob_peer_id,
            status: coco::project::peer::Status::NotReplicated,
        }]
    );

    bob_control.request_project(&urn, SystemTime::now()).await;

    requested(query_listener, &urn).await?;
    assert_cloned(clone_listener, &urn.clone(), alice_peer_id).await?;
    let project = state::get_project(&bob_peer, urn.clone()).await;
    assert!(project.is_ok());

    let announced = async_stream::stream! { loop { yield alice_events.recv().await } }
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::GossipFetched {
                gossip, provider, ..
            } if provider.peer_id == bob_peer_id && gossip.urn.id == urn.id => {
                future::ready(Some(()))
            },
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    let projects = state::list_projects(&bob_peer).await?;
    assert_eq!(projects.len(), 1);

    let alice_tracked = state::tracked(&alice_peer, urn.clone()).await?;

    assert_matches!(
        alice_tracked.first().unwrap(),
        coco::project::peer::Peer::Remote {
            peer_id,
            status: coco::project::peer::Status::Replicated(coco::project::peer::Replicated { role, .. }),
        } => {
            assert_eq!(*peer_id, bob_peer_id);
            assert_eq!(*role, coco::project::peer::Role::Tracker);
        }
    );

    Ok(())
}
