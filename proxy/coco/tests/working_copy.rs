use coco::{
    identities::payload::Person,
    project::checkout,
    state::{self, init_owner},
    RunConfig,
};

use assert_matches::assert_matches;
use pretty_assertions::assert_eq;

#[macro_use]
mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn upstream_for_default() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;

    let alice_peer = {
        let peer = alice_peer.peer.clone();
        tokio::task::spawn(alice_peer.run());
        peer
    };

    let create = shia_le_pathbuf(alice_tmp_dir.path().to_path_buf());
    let working_copy_path = create.repo.full_path();
    let _ = state::init_project(&alice_peer, &alice, create).await?;

    let repo = git2::Repository::open(working_copy_path)
        .map_err(radicle_surf::vcs::git::error::Error::from)?;
    let remote = repo.branch_upstream_remote("refs/heads/it")?;

    assert_eq!(remote.as_str().unwrap(), "rad");

    let branch = repo.find_branch("rad/it", git2::BranchType::Remote);
    assert!(branch.is_ok(), "could not find `rad/it`");

    Ok(())
}

#[tokio::test]
async fn checkout_twice_fails() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;

    let alice_peer = {
        let peer = alice_peer.peer.clone();
        tokio::task::spawn(alice_peer.run());
        peer
    };

    let project = state::init_project(
        &alice_peer,
        &alice,
        shia_le_pathbuf(alice_tmp_dir.path().to_path_buf()),
    )
    .await?;

    let _ = state::checkout(
        &alice_peer,
        project.urn(),
        None,
        alice_tmp_dir.path().join("checkout"),
    )
    .await?;

    assert_matches!(
        state::checkout(
            &alice_peer,
            project.urn(),
            None,
            alice_tmp_dir.path().join("checkout"),
        )
        .await
        .err(),
        Some(state::Error::Checkout(checkout::Error::AlreadExists(_)))
    );

    Ok(())
}
