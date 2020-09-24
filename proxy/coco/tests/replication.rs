use std::time::Duration;

use assert_matches::assert_matches;
use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::uri;
use radicle_surf::vcs::git::git2;

use coco::{config, seed::Seed, RunConfig, SyncConfig, SyncEvent};

#[macro_use]
mod common;
use common::{build_peer, build_peer_with_seeds, connected, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice = {
        let alice_signer = alice_signer.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ally.init_owner(&alice_signer.clone(), "alice"))
            .await??
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let _bob = {
        let bob_state = bob_state.clone();
        let bobby = bob_state.lock_owned().await;
        tokio::task::spawn_blocking(move || bobby.init_owner(&bob_signer, "bob")).await??
    };

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store, RunConfig::default()));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store, RunConfig::default()));

    let project = {
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(&alice_signer, &alice, &shia_le_pathbuf(alice_repo_path))
        })
        .await??
    };

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
    let alice = {
        let alice_signer = alice_signer.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ally.init_owner(&alice_signer.clone(), "alice"))
            .await??
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, _bob_signer) = build_peer(&bob_tmp_dir).await?;

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store, RunConfig::default()));
    tokio::task::spawn(bob_peer.run(alice_state.clone(), bob_store, RunConfig::default()));

    let cloned_urn = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        let alice_addr = alice_state.lock().await.listen_addr();
        let url = alice.urn().into_rad_url(alice_peer_id);

        let bobby = bob_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || bobby.clone_user(url, vec![alice_addr].into_iter()))
            .await??
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
    let alice = {
        let alice_signer = alice_signer.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ally.init_owner(&alice_signer.clone(), "alice"))
            .await??
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let _bob = {
        let bob_state = bob_state.clone();
        let bobby = bob_state.lock_owned().await;
        tokio::task::spawn_blocking(move || bobby.init_owner(&bob_signer, "bob")).await??
    };

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store, RunConfig::default()));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store, RunConfig::default()));

    let project = {
        let alice = alice.clone();
        let alice_repo_path = alice_repo_path.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(
                &alice_signer,
                &alice,
                &shia_le_pathbuf(alice_repo_path.clone()),
            )
        })
        .await??
    };

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
        tokio::task::spawn_blocking(move || bobby.fetch(fetch_url, vec![alice_addr])).await??;
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

#[tokio::test]
async fn can_sync_on_startup() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice_addr = alice_state.lock().await.listen_addr();
    let alice_peer_id = alice_state.lock().await.peer_id();
    let alice_events = alice_peer.subscribe();

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: alice_addr,
            peer_id: alice_peer_id.clone(),
        }],
    )
    .await?;
    let bob_peer_id = bob_state.lock().await.peer_id();

    let alice = {
        let alice_signer = alice_signer.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ally.init_owner(&alice_signer.clone(), "alice"))
            .await??
    };
    let _bob = {
        let bob_state = bob_state.clone();
        let bobby = bob_state.lock_owned().await;
        tokio::task::spawn_blocking(move || bobby.init_owner(&bob_signer, "bob")).await??
    };
    {
        let alice = alice.clone();
        let alice_repo_path = alice_repo_path.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(
                &alice_signer,
                &alice,
                &shia_le_pathbuf(alice_repo_path.clone()),
            )
        })
        .await??;
    };

    {
        let bob_events = bob_peer.subscribe();
        tokio::task::spawn(alice_peer.run(
            alice_state.clone(),
            alice_store,
            RunConfig {
                sync: SyncConfig {
                    on_startup: true,
                    ..SyncConfig::default()
                },
                ..RunConfig::default()
            },
        ));
        tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store, RunConfig::default()));
        connected(bob_events, &alice_peer_id).await?;
    };

    assert_event!(
        alice_events,
        coco::PeerEvent::PeerSync(SyncEvent::Succeeded(peer_id)) if peer_id == bob_peer_id
    )?;

    Ok(())
}

#[tokio::test]
async fn can_create_working_copy_of_peer() -> Result<(), Box<dyn std::error::Error + 'static>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_store = kv::Store::new(kv::Config::new(alice_tmp_dir.path().join("store")))?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(&alice_tmp_dir).await?;
    let alice = {
        let alice_signer = alice_signer.clone();
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ally.init_owner(&alice_signer.clone(), "alice"))
            .await??
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_repo_path = bob_tmp_dir.path().join("radicle");
    let bob_store = kv::Store::new(kv::Config::new(bob_tmp_dir.path().join("store")))?;
    let (bob_peer, bob_state, bob_signer) = build_peer(&bob_tmp_dir).await?;
    let bob = {
        let bob_state = bob_state.clone();
        let bobby = bob_state.lock_owned().await;
        tokio::task::spawn_blocking(move || bobby.init_owner(&bob_signer, "bob")).await??
    };

    let eve_tmp_dir = tempfile::tempdir()?;
    let eve_repo_path = eve_tmp_dir.path().join("radicle");
    let eve_store = kv::Store::new(kv::Config::new(eve_tmp_dir.path().join("store")))?;
    let (eve_peer, eve_state, eve_signer) = build_peer(&eve_tmp_dir).await?;
    let _eve = {
        let eve_state = eve_state.clone();
        let ev = eve_state.lock_owned().await;
        tokio::task::spawn_blocking(move || ev.init_owner(&eve_signer, "eve")).await??
    };

    tokio::task::spawn(alice_peer.run(alice_state.clone(), alice_store, RunConfig::default()));
    tokio::task::spawn(bob_peer.run(bob_state.clone(), bob_store, RunConfig::default()));
    tokio::task::spawn(eve_peer.run(eve_state.clone(), eve_store, RunConfig::default()));

    let project = {
        let alice_state = alice_state.clone();
        let ally = alice_state.lock_owned().await;
        tokio::task::spawn_blocking(move || {
            ally.init_project(&alice_signer, &alice, &shia_le_pathbuf(alice_repo_path))
        })
        .await??
    };

    let project = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        let alice_addr = alice_state.lock().await.listen_addr();
        let bob_peer_id = bob_state.lock().await.peer_id();
        let bob_addr = bob_state.lock().await.listen_addr();
        let bobby = bob_state.clone().lock_owned().await;
        let ev = eve_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || {
            let urn = bobby
                .clone_project(
                    project.urn().into_rad_url(alice_peer_id),
                    vec![alice_addr].into_iter(),
                )
                .expect("unable to clone project");
            let urn = ev.clone_project(urn.into_rad_url(bob_peer_id), vec![bob_addr].into_iter()).expect("unable to clone project");
            ev.get_project(&urn, None).expect("failed to get project just cloned")
        })
        .await?
    };

    println!("GIT HERE");
    let commit_id = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        let path = bob_state.lock().await.checkout(&project.urn(), alice_peer_id, bob_repo_path)?;
        println!("PATH: {:?}", path);
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
        let bob_addr = bob_state.lock().await.listen_addr();
        let bob_peer_id = bob_state.lock().await.peer_id().clone();
        let fetch_url = project.urn().into_rad_url(bob_peer_id.clone());

        let ev = eve_state.clone().lock_owned().await;
        tokio::task::spawn_blocking(move || ev.fetch(fetch_url, vec![bob_addr])).await??;
    }

    println!("ALICE: {:?}", alice_tmp_dir.path());
    println!("BOB: {:?}", bob_tmp_dir.path());
    println!("EVE: {:?}", eve_tmp_dir.path());

    println!("GIT HERE");
    let path = {
        let alice_peer_id = alice_state.lock().await.peer_id();
        eve_state.lock().await.checkout(&project.urn(), alice_peer_id, eve_repo_path)?
    };

    std::thread::sleep(std::time::Duration::from_secs(60));
    println!("GIT HERE");
    let repo = git2::Repository::open(path)?;
    assert_matches!(repo.find_commit(commit_id), Err(_));

    Ok(())
}
