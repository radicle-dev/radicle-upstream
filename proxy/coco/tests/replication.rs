use std::time::{Duration, Instant};

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::uri;
use radicle_surf::vcs::git::git2;

use coco::{
    config,
    request::waiting_room::{self, WaitingRoom},
    seed::Seed,
    shared::Shared,
    RunConfig, SyncConfig, SyncEvent,
};

#[macro_use]
mod common;
use common::{build_peer, build_peer_with_seeds, connected, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;
    let alice = alice_state.init_owner(&alice_signer, "alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, bob_signer) =
        build_peer(&bob_tmp_dir, waiting_room, RunConfig::default()).await?;
    let _bob = bob_state.init_owner(&bob_signer, "bob").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice_signer, &alice, shia_le_pathbuf(alice_repo_path))
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
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;
    let alice = alice_state.init_owner(&alice_signer, "alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, _bob_signer) =
        build_peer(&bob_tmp_dir, waiting_room, RunConfig::default()).await?;

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
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;
    let alice = alice_state.init_owner(&alice_signer, "alice").await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, bob_signer) =
        build_peer(&bob_tmp_dir, waiting_room, RunConfig::default()).await?;
    let _bob = bob_state.init_owner(&bob_signer, "bob").await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(
            &alice_signer,
            &alice,
            shia_le_pathbuf(alice_repo_path.clone()),
        )
        .await?;

    let project_urn = {
        let alice_addr = alice_state.listen_addr();
        let alice_peer_id = alice_state.peer_id().clone();
        let clone_url = project.urn().into_rad_url(alice_peer_id.clone());

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
        let alice_peer_id = alice_state.peer_id().clone();
        let fetch_url = project.urn().into_rad_url(alice_peer_id.clone());

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
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, alice_state, alice_signer) = build_peer(
        &alice_tmp_dir,
        waiting_room,
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
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
        waiting_room,
        RunConfig::default(),
    )
    .await?;
    let bob_peer_id = bob_state.peer_id();

    let alice = alice_state.init_owner(&alice_signer, "alice").await?;
    let _bob = bob_state.init_owner(&bob_signer, "bob").await?;
    alice_state
        .init_project(
            &alice_signer,
            &alice,
            shia_le_pathbuf(alice_repo_path.clone()),
        )
        .await?;

    let bob_events = bob_peer.subscribe();
    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());
    connected(bob_events, &alice_peer_id).await?;

    assert_event!(
        alice_events,
        coco::PeerEvent::PeerSync(SyncEvent::Succeeded(peer_id)) if peer_id == bob_peer_id
    )?;

    Ok(())
}
