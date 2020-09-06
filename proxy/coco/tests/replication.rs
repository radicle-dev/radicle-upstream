use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use librad::keys::SecretKey;
use librad::signer;
use librad::uri;
use radicle_surf::vcs::git::git2;

use coco::config;
use coco::project;
use coco::Peer;
use coco::{Lock, State};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state, alice_signer) = build_peer(alice_tmp_dir).await?;

    tokio::task::spawn(alice_peer.run());

    let alice = alice_state
        .lock()
        .await
        .init_owner(&alice_signer, "alice")?;
    let project = alice_state.lock().await.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path),
    )?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state, bob_signer) = build_peer(bob_tmp_dir).await?;
    let _bob = bob_state.lock().await.init_owner(&bob_signer, "bob")?;

    tokio::task::spawn(bob_peer.run());

    let bobby = bob_state.clone();
    let project_urn = tokio::task::spawn_blocking(|| async move {
        let alice_state = alice_state.lock().await;
        let bob_state = bobby.lock().await;
        bob_state
            .clone_project(
                project.urn().into_rad_url(alice_state.peer_id()),
                vec![alice_state.listen_addr()].into_iter(),
            )
            .expect("unable to clone project")
    })
    .await
    .expect("failed to join thread");

    let left: Vec<librad::uri::RadUrn> = bob_state
        .lock()
        .await
        .list_projects()?
        .into_iter()
        .map(|project| project.urn())
        .collect::<Vec<_>>();
    let right: Vec<librad::uri::RadUrn> = vec![project_urn.await];

    assert_eq!(left, right,);

    Ok(())
}

#[tokio::test]
async fn can_clone_user() -> Result<(), Box<dyn std::error::Error>> {
    let alice_key = SecretKey::new();
    let alice_signer = signer::BoxedSigner::from(alice_key.clone());
    let alice_tmp_dir = tempfile::tempdir().expect("failed to create temdir");
    let alice_config = config::default(alice_key, alice_tmp_dir.path())?;
    let (alice_api, _run_loop) = alice_config.try_into_peer().await?.accept()?;
    let alice_state = State::new(alice_api, alice_signer.clone());

    let bob_key = SecretKey::new();
    let bob_signer = signer::BoxedSigner::from(bob_key.clone());
    let bob_tmp_dir = tempfile::tempdir().expect("failed to create temdir");
    let bob_config = config::default(bob_key, bob_tmp_dir.path())?;
    let (bob_api, _run_loop) = bob_config.try_into_peer().await?.accept()?;
    let bob_state = State::new(bob_api, bob_signer.clone());

    let alice = alice_state.init_user(&alice_signer, "alice")?;
    let bob_state = std::sync::Arc::new(std::sync::Mutex::new(bob_state));
    let bobby = bob_state.clone();
    let user_urn = tokio::task::spawn_blocking(move || {
        bobby
            .lock()
            .expect("unable to acquite lock")
            .clone_user(
                alice.urn().into_rad_url(alice_state.peer_id()),
                vec![alice_state.listen_addr()].into_iter(),
            )
            .expect("unable to clone project")
    })
    .await?;

    assert_eq!(
        bob_state
            .lock()
            .expect("unable to acquire lock")
            .list_users()?
            .into_iter()
            .map(|user| user.urn())
            .collect::<Vec<_>>(),
        vec![user_urn]
    );

    Ok(())
}

#[tokio::test]
async fn can_fetch_project_changes() -> Result<(), Box<dyn std::error::Error>> {
    let alice_key = SecretKey::new();
    let alice_signer = signer::BoxedSigner::from(alice_key.clone());

    let alice_tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let alice_config = config::default(alice_key, alice_tmp_dir.path())?;

    let (alice_api, _run_loop) = alice_config.try_into_peer().await?.accept()?;
    let alice_state = State::new(alice_api, alice_signer.clone());

    let alice_peer_id = alice_state.peer_id().clone();
    let alice_addr = alice_state.listen_addr();

    let alice = alice_state.init_owner(&alice_signer, "alice")?;
    let project = alice_state.init_project(
        &alice_signer,
        &alice,
        &shia_le_pathbuf(alice_repo_path.clone()),
    )?;
    let urn = project.urn();

    let bob_key = SecretKey::new();
    let bob_signer = signer::BoxedSigner::from(signer::SomeSigner {
        signer: bob_key.clone(),
    });

    let bob_tmp_dir = tempfile::tempdir().expect("failed to create tempdir");

    let bob_config = config::default(bob_key, bob_tmp_dir.path())?;
    let (bob_api, _run_loop) = bob_config.try_into_peer().await?.accept()?;
    let bob_state = State::new(bob_api, alice_signer);
    let _bob = bob_state.init_owner(&bob_signer, "bob")?;

    let url = urn.into_rad_url(alice_peer_id.clone());
    let bob_state = Arc::new(Mutex::new(bob_state));
    let bobby = bob_state.clone();
    let project_urn = tokio::task::spawn_blocking(move || {
        bobby
            .lock()
            .expect("unable to acquire lock")
            .clone_project(url, vec![alice_state.listen_addr()].into_iter())
            .expect("unable to clone project")
    })
    .await
    .expect("failed to join thread");

    assert_eq!(
        bob_state
            .lock()
            .expect("unable to acquire lock")
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

    let url = project_urn.into_rad_url(alice_peer_id.clone());
    let bobby = bob_state.clone();
    tokio::task::spawn_blocking(move || {
        bobby
            .lock()
            .expect("unable to acquire lock")
            .fetch(url, vec![alice_addr])
            .expect("unable to fetch")
    })
    .await
    .expect("failed to join thread");

    assert!(bob_state
        .lock()
        .expect("unable to acquire lock")
        .has_commit(
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

fn shia_le_pathbuf(path: PathBuf) -> project::Create<PathBuf> {
    project::Create {
        repo: project::Repo::New {
            path,
            name: "just".to_string(),
        },
        description: "do".to_string(),
        default_branch: "it".to_string(),
    }
}

async fn build_peer(
    tmp_dir: tempfile::TempDir,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key.clone());

    let conf = config::default(key, tmp_dir.path())?;
    let (api, run_loop) = conf.try_into_peer().await?.accept()?;
    let state = State::new(api, signer.clone());
    let state = Lock::from(state);
    let peer = Peer::new(run_loop, state.clone());

    Ok((peer, state, signer))
}
