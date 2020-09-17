use std::time::Duration;

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::net::protocol::ProtocolEvent;

use coco::{seed::Seed, AnnounceEvent, Hash, Urn};

mod common;
use common::{
    build_peer, build_peer_with_seeds, init_logging, radicle_project, shia_le_pathbuf,
    wait_connected,
};

#[tokio::test]
async fn can_announce_new_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_events = alice_peer.subscribe();

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store));

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;

    {
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(&alice_signer, &alice, &shia_le_pathbuf(alice_repo_path))
                .expect("unable to init project")
        })
        .await?;
    }

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

#[tokio::test(core_threads = 4)]
async fn can_observe_announcement_from_connected_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_addr = alice_state.lock().await.listen_addr();
    let alice_peer_id = alice_state.lock().await.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
    )
    .await?;
    let bob_connected = bob_peer.subscribe();
    let bob_events = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store));
    wait_connected(bob_connected, &alice_peer_id).await?;

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;
    let project = {
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(&alice_signer, &alice, &shia_le_pathbuf(alice_repo_path))
                .expect("unable to init project")
        })
        .await?
    };

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
#[tokio::test]
async fn providers_is_none() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let tmp_dir = tempfile::tempdir()?;
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
    let (peer, state, _signer) = build_peer(&tmp_dir).await?;

    tokio::task::spawn(peer.run(state.clone(), store));

    let unkown_urn = Urn {
        id: Hash::hash(b"project0"),
        proto: librad::uri::Protocol::Git,
        path: "user/imperative-language".parse::<librad::uri::Path>()?,
    };

    let res = state
        .lock()
        .await
        .providers(unkown_urn, Duration::from_secs(5))
        .await
        .next()
        .await;

    assert!(res.is_none(), "didn't expected to obtain any providers");

    Ok(())
}

/// Verify that asking the network for a URN owned by a seed peer returns said peer.
#[tokio::test(core_threads = 4)]
async fn providers() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_addr = alice_state.lock().await.listen_addr();
    let alice_peer_id = alice_state.lock().await.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, _bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
    )
    .await?;
    let bob_events = bob_peer.subscribe();

    tokio::spawn(alice_peer.run(alice_state.clone(), alice_store));
    tokio::spawn(bob_peer.run(bob_state.clone(), bob_store));
    wait_connected(bob_events, &alice_peer_id).await?;

    let ally = alice_state.lock_owned().await;
    let target_urn = tokio::task::spawn_blocking(move || {
        let project = radicle_project(alice_repo_path.clone());
        let user = ally.init_owner(&alice_signer, "cloudhead").unwrap();
        let created_project = ally.init_project(&alice_signer, &user, &project).unwrap();
        created_project.urn()
    })
    .await?;

    let res = bob_state
        .lock()
        .await
        .providers(target_urn, Duration::from_secs(5))
        .await
        .next()
        .await;

    assert_eq!(res.map(|info| info.peer_id), Some(alice_peer_id));

    Ok(())
}
