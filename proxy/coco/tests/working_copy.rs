use coco::RunConfig;

#[macro_use]
mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_create_working_copy() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path.clone()))
        .await?;

    println!("{:?}", alice_repo_path.join("just"));
    // std::thread::sleep(std::time::Duration::from_secs(1000000));
    let repo = git2::Repository::open(alice_repo_path.join("just"))
        .map_err(radicle_surf::vcs::git::error::Error::from)?;
    let main = repo.find_branch("it", git2::BranchType::Local)?;

    dbg!(main.upstream().unwrap().name().unwrap());

    Ok(())
}
