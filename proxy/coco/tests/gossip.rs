use std::time::Duration;

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::net::protocol::ProtocolEvent;

use coco::{seed::Seed, AnnounceConfig, AnnounceEvent, Hash, RunConfig, Urn};

mod common;
use common::{
    build_peer, build_peer_with_seeds, connected, init_logging, radicle_project, shia_le_pathbuf,
};

#[tokio::test(core_threads = 2)]
async fn can_announce_new_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: AnnounceConfig {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_events = alice_peer.subscribe();

    let _alice_runs = alice_peer.into_running();

    let alice = alice_state.init_owner(&alice_signer, "alice").await?;

    {
        alice_state
            .init_project(&alice_signer, &alice, shia_le_pathbuf(alice_repo_path))
            .await
            .expect("unable to init project");
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

#[tokio::test(core_threads = 2)]
async fn can_observe_announcement_from_connected_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(
        &alice_tmp_dir,
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
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
        RunConfig::default(),
    )
    .await?;
    let _bob = bob_state.init_owner(&bob_signer, "bob").await?;
    let bob_connected = bob_peer.subscribe();
    let bob_events = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

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
    let (peer, state, _signer) = build_peer(&tmp_dir, RunConfig::default()).await?;

    tokio::task::spawn(peer.into_running());

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
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, _bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
        RunConfig::default(),
    )
    .await?;
    let bob_events = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

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
