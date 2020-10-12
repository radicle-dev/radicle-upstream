use std::time::{Duration, Instant};

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::net::protocol::ProtocolEvent;

use coco::{
    request::waiting_room::{self, WaitingRoom},
    seed::Seed,
    shared::Shared,
    AnnounceConfig, AnnounceEvent, Hash, RunConfig, Urn,
};

#[macro_use]
mod common;
use common::{
    assert_cloned, build_peer, build_peer_with_seeds, connected, init_logging, radicle_project,
    requested, shia_le_pathbuf,
};

#[tokio::test(core_threads = 2)]
async fn can_announce_new_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) = build_peer(
        &alice_tmp_dir,
        waiting_room,
        RunConfig {
            announce: AnnounceConfig {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_events = alice_peer.subscribe();

    tokio::spawn(alice_peer.into_running());

    let alice = alice_state.init_owner(&alice_signer, "alice").await?;
    alice_state
        .init_project(&alice_signer, &alice, shia_le_pathbuf(alice_repo_path))
        .await
        .expect("unable to init project");

    let announced = alice_events
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Announce(AnnounceEvent::Succeeded(updates)) if updates.len() == 1 => {
                future::ready(Some(()))
            },
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test(core_threads = 2)]
async fn can_observe_announcement_from_connected_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) = build_peer(
        &alice_tmp_dir,
        waiting_room,
        RunConfig {
            announce: AnnounceConfig {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();
    let alice = alice_state.init_owner(&alice_signer, "alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id,
        }],
        waiting_room,
        RunConfig::default(),
    )
    .await?;
    let _bob = bob_state.init_owner(&bob_signer, "bob").await?;
    let bob_connected = bob_peer.subscribe();
    let bob_events = bob_peer.subscribe();

    tokio::spawn(alice_peer.into_running());
    tokio::spawn(bob_peer.into_running());

    connected(bob_connected, &alice_peer_id).await?;

    let project = alice_state
        .init_project(&alice_signer, &alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let announced = bob_events
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Protocol(ProtocolEvent::Gossip(info)) => match info {
                librad::net::gossip::Info::Has(librad::net::gossip::Has {
                    provider,
                    val: librad::net::peer::Gossip { urn, .. },
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

/// Verify that asking the network for an unkown urn returns no providers.
#[tokio::test(core_threads = 2)]
async fn providers_is_none() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (peer, state, _signer) = build_peer(&tmp_dir, waiting_room, RunConfig::default()).await?;

    tokio::spawn(peer.into_running());

    let unkown_urn = Urn {
        id: Hash::hash(b"project0"),
        proto: librad::uri::Protocol::Git,
        path: "user/imperative-language".parse::<librad::uri::Path>()?,
    };

    let res = state
        .providers(unkown_urn, Duration::from_secs(5))
        .await
        .next()
        .await;

    assert!(res.is_none(), "didn't expected to obtain any providers");

    Ok(())
}

/// Verify that asking the network for a URN owned by a seed peer returns said peer.
#[tokio::test(core_threads = 2)]
async fn providers() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, _bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id,
        }],
        waiting_room,
        RunConfig::default(),
    )
    .await?;
    let bob_events = bob_peer.subscribe();

    tokio::spawn(alice_peer.into_running());
    tokio::spawn(bob_peer.into_running());

    connected(bob_events, &alice_peer_id).await?;

    let target_urn = {
        let project = radicle_project(alice_repo_path.clone());
        let user = alice_state
            .init_owner(&alice_signer, "cloudhead")
            .await
            .unwrap();
        let created_project = alice_state
            .init_project(&alice_signer, &user, project)
            .await
            .unwrap();
        created_project.urn()
    };

    let res = bob_state
        .providers(target_urn, Duration::from_secs(1))
        .await
        .next()
        .await;

    assert_eq!(res.map(|info| info.peer_id), Some(alice_peer_id));

    Ok(())
}

#[tokio::test(core_threads = 2)]
async fn can_ask_and_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id,
        }],
        bob_waiting_room.clone(),
        RunConfig::default(),
    )
    .await?;
    let bob_events = bob_peer.subscribe();
    let clone_listener = bob_peer.subscribe();
    let query_listener = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    connected(bob_events, &alice_peer_id).await?;

    bob_state.init_owner(&bob_signer, "bob").await?;

    let urn = {
        let alice = alice_state.init_owner(&alice_signer, "alice").await?;
        let project = radicle_project(alice_repo_path.clone());
        alice_state
            .init_project(&alice_signer, &alice, project)
            .await?
            .urn()
    };

    {
        let mut bob_waiting_room = bob_waiting_room.write().await;
        let _ = bob_waiting_room.request(urn.clone(), Instant::now());
    }

    requested(query_listener, &urn).await?;
    assert_cloned(clone_listener, &urn.clone().into_rad_url(alice_peer_id)).await?;

    // TODO(finto): List projects
    let project = bob_state.get_project(urn, None).await;
    assert!(project.is_ok());

    Ok(())
}
