use std::time::Duration;

use futures::future;
use futures::StreamExt as _;
use tokio::time::timeout;

use librad::net::protocol::ProtocolEvent;

use coco::seed::Seed;

mod common;
use common::{build_peer, build_peer_with_seeds, init_logging, shia_le_pathbuf, wait_connected};

#[tokio::test]
async fn announce_solo() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_events = alice_peer.subscribe();

    tokio::task::spawn(alice_peer.run());

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
            coco::PeerEvent::Announced(updates) if updates.len() == 1 => future::ready(Some(())),
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test]
async fn announce_connected() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_addr = alice_state.lock().await.listen_addr();
    let alice_peer_id = alice_state.lock().await.peer_id();
    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
    )
    .await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;
    let bob_connected = bob_peer.subscribe();
    let bob_events = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.run());
    tokio::task::spawn(bob_peer.run());

    wait_connected(bob_connected, &alice_peer_id).await?;

    let project = alice_state.lock().await.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path),
    )?;

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
