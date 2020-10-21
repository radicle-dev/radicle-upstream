use std::time::Duration;

use assert_matches::assert_matches;
use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::uri;
use radicle_surf::vcs::git::git2;

use coco::{config, seed::Seed, RunConfig, SyncConfig};

#[macro_use]
mod common;
use common::{build_peer, build_peer_with_seeds, connected, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = bob_state.init_owner("bob").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let project_urn = {
        let alice_peer_id = alice_state.peer_id();
        let alice_addr = alice_state.listen_addr();
        bob_state
            .clone_project(
                project.urn().into_rad_url(alice_peer_id),
                vec![alice_addr].into_iter(),
            )
            .await
            .expect("unable to clone project")
    };

    let have = bob_state
        .list_projects()
        .await?
        .into_iter()
        .map(|project| project.urn())
        .collect::<Vec<_>>();
    let want = vec![project_urn];

    assert_eq!(have, want);

    Ok(())
}

#[tokio::test]
async fn can_clone_user() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let cloned_urn = {
        let alice_peer_id = alice_state.peer_id();
        let alice_addr = alice_state.listen_addr();
        let url = alice.urn().into_rad_url(alice_peer_id);

        bob_state
            .clone_user(url, vec![alice_addr].into_iter())
            .await
            .expect("unable to clone project")
    };

    let want = bob_state
        .list_users()
        .await?
        .into_iter()
        .map(|user| user.urn())
        .collect::<Vec<_>>();
    let have = vec![cloned_urn];

    assert_eq!(want, have);

    Ok(())
}

#[tokio::test]
async fn can_fetch_project_changes() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = bob_state.init_owner("bob").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path.clone()))
        .await?;

    let project_urn = {
        let alice_addr = alice_state.listen_addr();
        let alice_peer_id = alice_state.peer_id();
        let clone_url = project.urn().into_rad_url(alice_peer_id);

        bob_state
            .clone_project(clone_url, vec![alice_addr].into_iter())
            .await
            .expect("unable to clone project")
    };

    assert_eq!(
        bob_state
            .list_projects()
            .await?
            .into_iter()
            .map(|project| project.urn())
            .collect::<Vec<_>>(),
        vec![project_urn.clone()]
    );

    let commit_id = {
        let repo = git2::Repository::open(alice_repo_path.join(project.name()))?;
        let oid = repo
            .find_reference(&format!("refs/heads/{}", project.default_branch()))?
            .target()
            .expect("Missing first commit");
        let commit = repo.find_commit(oid)?;
        let commit_id = {
            let empty_tree = {
                let mut index = repo.index()?;
                let oid = index.write_tree()?;
                repo.find_tree(oid)?
            };

            let author = git2::Signature::now(alice.name(), "alice@example.com")?;
            repo.commit(
                Some(&format!("refs/heads/{}", project.default_branch())),
                &author,
                &author,
                "Successor commit",
                &empty_tree,
                &[&commit],
            )?
        };

        let mut rad = repo.find_remote(config::RAD_REMOTE)?;
        rad.push(&[&format!("refs/heads/{}", project.default_branch())], None)?;
        commit_id
    };

    {
        let alice_addr = alice_state.listen_addr();
        let alice_peer_id = alice_state.peer_id();
        let fetch_url = project.urn().into_rad_url(alice_peer_id);

        println!("GOT THIS FAR");
        bob_state
            .fetch(fetch_url, vec![alice_addr])
            .await
            .expect("unable to fetch");
    };

    let alice_peer_id = alice_state.peer_id();
    assert!(
        bob_state
            .has_commit(
                uri::RadUrn {
                    path: uri::Path::parse(format!(
                        "refs/remotes/{}/heads/{}",
                        alice_peer_id,
                        project.default_branch()
                    ))
                    .expect("failed to parse uri::Path"),
                    ..project.urn()
                },
                commit_id
            )
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn can_sync_on_startup() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            sync: SyncConfig {
                on_startup: true,
                ..SyncConfig::default()
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();
    let alice_events = alice_peer.subscribe();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    let bob_peer_id = bob_state.peer_id();

    let alice = alice_state.init_owner("alice").await?;
    let _bob = bob_state.init_owner("bob").await?;
    alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path.clone()))
        .await?;

    let bob_events = bob_peer.subscribe();
    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());
    connected(bob_events, &alice_peer_id).await?;

    assert_event!(
        alice_events,
        coco::PeerEvent::PeerSynced(peer_id) if peer_id == bob_peer_id
    )?;

    Ok(())
}

#[tokio::test]
async fn can_create_working_copy_of_peer() -> Result<(), Box<dyn std::error::Error + 'static>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_repo_path = bob_tmp_dir.path().join("radicle");
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let bob = bob_state.init_owner("bob").await?;

    let eve_tmp_dir = tempfile::tempdir()?;
    let eve_repo_path = eve_tmp_dir.path().join("radicle");
    let (eve_peer, eve_state) = build_peer(&eve_tmp_dir, RunConfig::default()).await?;
    let _eve = eve_state.init_owner("eve").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());
    tokio::task::spawn(eve_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let project = {
        let alice_peer_id = alice_state.peer_id();
        let alice_addr = alice_state.listen_addr();
        let bob_peer_id = bob_state.peer_id();
        let bob_addr = bob_state.listen_addr();
        let urn = bob_state
            .clone_project(
                project.urn().into_rad_url(alice_peer_id),
                vec![alice_addr].into_iter(),
            )
            .await
            .expect("unable to clone project");
        let urn = eve_state
            .clone_project(urn.into_rad_url(bob_peer_id), vec![bob_addr].into_iter())
            .await
            .expect("unable to clone project");
        eve_state.get_project(urn.clone(), None).await?
    };

    let commit_id = {
        let alice_peer_id = alice_state.peer_id();
        let path = bob_state
            .checkout(project.urn(), alice_peer_id, bob_repo_path)
            .await?;
        let repo = git2::Repository::open(path)?;
        let oid = repo
            .find_reference(&format!("refs/heads/{}", project.default_branch()))?
            .target()
            .expect("Missing first commit");
        let commit = repo.find_commit(oid)?;
        let commit_id = {
            let empty_tree = {
                let mut index = repo.index()?;
                let oid = index.write_tree()?;
                repo.find_tree(oid)?
            };

            let author = git2::Signature::now(bob.name(), &format!("{}@example.com", bob.name()))?;
            repo.commit(
                Some(&format!("refs/heads/{}", project.default_branch())),
                &author,
                &author,
                "Successor commit",
                &empty_tree,
                &[&commit],
            )?
        };

        let mut rad = repo.find_remote(config::RAD_REMOTE)?;
        rad.push(&[&format!("refs/heads/{}", project.default_branch())], None)?;
        commit_id
    };
    {
        let bob_addr = bob_state.listen_addr();
        let bob_peer_id = bob_state.peer_id();
        let fetch_url = project.urn().into_rad_url(bob_peer_id);

        eve_state.fetch(fetch_url, vec![bob_addr]).await?;
    }

    let path = {
        let alice_peer_id = alice_state.peer_id();
        eve_state
            .checkout(project.urn(), alice_peer_id, eve_repo_path)
            .await?
    };

    let repo = git2::Repository::open(path)?;
    assert_matches!(repo.find_commit(commit_id), Err(_));

    Ok(())
}
