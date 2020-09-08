use futures::future;
use futures::StreamExt as _;

mod common;
use common::{build_peer, build_peer_with_seeds, init_logging, shia_le_pathbuf, wait_connected};

#[tokio::test]
async fn announces_updates() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_addr = alice_state.lock().await.listen_addr();
    let alice_peer_id = alice_state.lock().await.peer_id();
    let alice_events = alice_peer.subscribe();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![coco::seed::Seed {
            addr: alice_addr,
            peer_id: alice_peer_id,
        }],
    )
    .await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;
    let bob_events = bob_peer.subscribe();
    let moar_events = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.run());
    tokio::task::spawn(bob_peer.run());

    wait_connected(bob_events, &alice_state.lock().await.peer_id()).await?;

    {
        let alice_state = alice_state.lock().await;
        let alice = alice_state.init_owner(&alice_signer, "alice")?;
        alice_state.init_project(&alice_signer, &alice, &shia_le_pathbuf(alice_repo_path))?
    };

    tokio::time::delay_for(std::time::Duration::from_secs(10)).await;

    alice_events
        .into_stream()
        .take(10)
        .for_each(|event| {
            println!("ALICE EVENT {:?}", event.unwrap());
            future::ready(())
        })
        .await;

    moar_events
        .into_stream()
        .take(10)
        .for_each(|event| {
            println!("BOB EVENT {:?}", event.unwrap());
            future::ready(())
        })
        .await;

    Ok(())
}
