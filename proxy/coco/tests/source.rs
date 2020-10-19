use nonempty::NonEmpty;
use pretty_assertions::assert_eq;

use coco::RunConfig;

mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_browse_peers_branch() -> Result<(), Box<dyn std::error::Error + 'static>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let bob = bob_state.init_owner("bob").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let urn = {
        let alice_peer_id = alice_state.peer_id();
        let alice_addr = alice_state.listen_addr();
        bob_state
            .clone_project(
                project.urn().into_rad_url(alice_peer_id),
                vec![alice_addr].into_iter(),
            )
            .await?
    };

    let peers = bob_state.tracked(urn.clone()).await?;

    let bob = bob.to_data().build()?;

    let branch = bob_state.find_default_branch(urn).await?;
    let revisions = bob_state
        .with_browser(branch, |browser| {
            coco::source::revisions(browser, bob_state.peer_id(), bob, peers)
        })
        .await?;

    let expected = coco::source::Revisions {
        peer_id: alice_state.peer_id(),
        user: alice.to_data().build()?,
        branches: vec![coco::source::Branch::from("it".to_string())],
        tags: vec![],
    };
    assert_eq!(
        revisions,
        NonEmpty {
            head: expected,
            tail: vec![]
        }
    );

    Ok(())
}
