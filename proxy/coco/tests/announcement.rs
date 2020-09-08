mod common;
use common::{build_peer, build_peer_with_seeds, init_logging, wait_connected};

#[tokio::test]
async fn announces_updates() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;

    tokio::task::spawn(alice_peer.run());

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![
            coco::seed::Seed::from_str(&alice_state.lock().await.listen_addr().to_string()).await?,
        ],
    )
    .await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;
    let bob_events = bob_peer.subscribe();

    tokio::task::spawn(bob_peer.run());

    wait_connected(bob_events, &alice_state.lock().await.peer_id()).await?;

    Ok(())
}
