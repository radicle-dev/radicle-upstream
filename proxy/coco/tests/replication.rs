use librad::uri;
use radicle_surf::vcs::git::git2;

use coco::config;

mod common;
use common::{build_peer, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store));

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;
    let project = alice_state.lock().await.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path),
    )?;

    let project_urn = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        let alice_addr = alice_state.lock().await.listen_addr();
        let bobby = bob_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || {
            bobby
                .clone_project(
                    project.urn().into_rad_url(alice_peer_id),
                    vec![alice_addr].into_iter(),
                )
                .expect("unable to clone project")
        })
        .await?
    };

    let have = bob_state
        .lock()
        .await
        .list_projects()?
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
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, _bob_signer) = build_peer(&bob_tmp_dir).await?;

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store));
    tokio::task::spawn(bob_peer.run(alice_state.clone(), bob_store));

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;
    let cloned_urn = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        let alice_addr = alice_state.lock().await.listen_addr();
        let url = alice.urn().into_rad_url(alice_peer_id);

        let bobby = bob_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || {
            bobby
                .clone_user(url, vec![alice_addr].into_iter())
                .expect("unable to clone project")
        })
        .await?
    };

    let want = bob_state
        .lock()
        .await
        .list_users()?
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
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store));

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;
    let project = alice_state.lock().await.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path.clone()),
    )?;

    let project_urn = {
        let alice_addr = alice_state.lock().await.listen_addr();
        let alice_peer_id = alice_state.lock().await.peer_id().clone();
        let clone_url = project.urn().into_rad_url(alice_peer_id.clone());

        let bobby = bob_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || {
            bobby
                .clone_project(clone_url, vec![alice_addr].into_iter())
                .expect("unable to clone project")
        })
        .await?
    };

    assert_eq!(
        bob_state
            .lock()
            .await
            .list_projects()?
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
        let alice_addr = alice_state.lock().await.listen_addr();
        let alice_peer_id = alice_state.lock().await.peer_id().clone();
        let fetch_url = project.urn().into_rad_url(alice_peer_id.clone());

        let bobby = bob_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || {
            bobby
                .fetch(fetch_url, vec![alice_addr])
                .expect("unable to fetch")
        })
        .await?;
    };

    let alice_peer_id = alice_state.lock().await.peer_id();
    assert!(bob_state.lock().await.has_commit(
        &uri::RadUrn {
            path: uri::Path::parse(format!(
                "refs/remotes/{}/heads/{}",
                alice_peer_id,
                project.default_branch()
            ))
            .expect("failed to parse uri::Path"),
            ..project.urn()
        },
        commit_id
    )?);

    Ok(())
}
