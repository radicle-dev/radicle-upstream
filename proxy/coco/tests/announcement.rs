use librad::net::protocol::ProtocolEvent;

use coco::PeerEvent;

mod common;
use common::build_peer;

#[tokio::test]
async fn announces_updates() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;

    tokio::task::spawn(alice_peer.run());

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;

    let mut bob_events = bob_peer.subscribe();

    tokio::task::spawn(bob_peer.run());

    let event = bob_events.recv().await?;
    if !expect_event(event, PeerEvent::Protocol(ProtocolEvent::Listening())) {
        panic!("wrong event");
    }
    println!("EVENT {:?}", event);

    Ok(())
}

fn expect_event(got: PeerEvent, expect: PeerEvent) -> bool {
    match got {
        expect => true,
        _ => false,
    }
}
