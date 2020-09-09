use std::time::Duration;

use futures::future;
use futures::StreamExt as _;
use tokio::time::timeout;

mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn solo() -> Result<(), Box<dyn std::error::Error>> {
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
    alice_state.lock().await.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path),
    )?;

    let announced = alice_events
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Announced(updates) if updates == 1 => future::ready(Some(())),
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}
