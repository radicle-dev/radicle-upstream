use coco::RunConfig;

use pretty_assertions::assert_eq;

#[macro_use]
mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn upstream_for_default() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());

    let _ = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path.clone()))
        .await?;

    let repo = git2::Repository::open(alice_repo_path.join("just"))
        .map_err(radicle_surf::vcs::git::error::Error::from)?;
    let remote = repo.branch_upstream_remote("refs/heads/it")?;

    assert_eq!(remote.as_str().unwrap(), "rad");

    Ok(())
}
