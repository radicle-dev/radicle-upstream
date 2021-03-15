use nonempty::NonEmpty;
use pretty_assertions::assert_eq;

use coco::{
    identities::payload::Person,
    state::{self, init_owner},
    RunConfig,
};

mod common;
use common::{build_peer, init_logging, shia_le_pathbuf, started};

#[allow(clippy::needless_collect)]
#[tokio::test]
async fn can_browse_peers_branch() -> Result<(), Box<dyn std::error::Error + 'static>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_peer = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = init_owner(&bob_peer.peer, Person { name: "bob".into() }).await?;

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
        (peer, listen_addrs)
    };

    let bob_peer = {
        let peer = bob_peer.peer.clone();
        let events = bob_peer.subscribe();
        tokio::task::spawn(bob_peer.run());
        started(events).await?;

        peer
    };

    let project =
        state::init_project(&alice_peer, &alice, shia_le_pathbuf(alice_repo_path)).await?;

    {
        let alice_peer_id = alice_peer.peer_id();
        state::clone_project(
            &bob_peer,
            project.urn(),
            alice_peer_id,
            alice_addrs.into_iter(),
            None,
        )
        .await?
    };

    let peers = state::list_project_peers(&bob_peer, project.urn()).await?;

    let branch = state::find_default_branch(&bob_peer, project.urn()).await?;
    let revisions = state::with_browser(&bob_peer, branch, |browser| {
        peers
            .into_iter()
            .filter_map(coco::project::Peer::replicated)
            .filter_map(|peer| coco::source::revisions(browser, peer).transpose())
            .collect::<Result<Vec<_>, _>>()
    })
    .await?;

    let expected = coco::source::Revisions {
        peer_id: alice_peer.peer_id(),
        user: alice.into_inner().into_inner(),
        branches: NonEmpty::new(coco::source::Branch::from("it".to_string())),
        tags: vec![],
    };
    assert_eq!(revisions, vec![expected],);

    Ok(())
}
