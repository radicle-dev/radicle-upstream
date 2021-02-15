use std::{convert::TryFrom, time::Duration};

use assert_matches::assert_matches;
use futures::{future, StreamExt as _};
use pretty_assertions::assert_eq;
use tokio::time::timeout;

use librad::git::{
    local::url::LocalUrl,
    types::{remote::LocalPushspec, Fetchspec, Force, Remote},
};
use radicle_git_ext::RefLike;

use coco::{
    peer::run_config,
    project::{peer, Peer},
    seed::Seed,
    RunConfig,
};

#[macro_use]
mod common;
use common::{build_peer, build_peer_with_seeds, connected, init_logging, shia_le_pathbuf};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = bob_state.init_owner("bob".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    {
        let alice_peer_id = alice_state.peer_id();
        let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();
        bob_state
            .clone_project(project.urn(), alice_peer_id, alice_addrs, None)
            .await?;
    }

    let have = bob_state
        .list_projects()
        .await?
        .into_iter()
        .map(|project| project.urn())
        .collect::<Vec<_>>();
    let want = vec![project.urn()];

    assert_eq!(have, want);

    {
        let another_peer = librad::peer::PeerId::from(librad::keys::SecretKey::new());
        bob_state.track(project.urn(), another_peer).await?;
        let mut have = bob_state
            .tracked(project.urn())
            .await?
            .into_iter()
            .map(|peer| peer.map(|status| status.map(|user| user.subject().name.to_string())))
            .collect::<Vec<_>>();
        have.sort_by(|p1, p2| p1.status().cmp(p2.status()));
        let want: Vec<_> = vec![
            coco::project::Peer::Remote {
                peer_id: another_peer,
                status: peer::Status::NotReplicated,
            },
            coco::project::Peer::Remote {
                peer_id: alice_state.peer_id(),
                status: peer::Status::replicated(
                    peer::Role::Maintainer,
                    alice.subject().name.to_string(),
                ),
            },
        ];
        assert_eq!(have, want);
    }

    Ok(())
}

#[tokio::test]
async fn can_clone_user() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    {
        let alice_peer_id = alice_state.peer_id();
        let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();

        bob_state
            .clone_user(alice.urn(), alice_peer_id, alice_addrs, None)
            .await?;
    }

    let want = bob_state
        .list_users()
        .await?
        .into_iter()
        .map(|user| user.urn())
        .collect::<Vec<_>>();
    let have = vec![alice.urn()];

    assert_eq!(want, have);

    Ok(())
}

#[tokio::test]
async fn can_fetch_project_changes() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = bob_state.init_owner("bob".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path.clone()))
        .await?;

    {
        let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();
        let alice_peer_id = alice_state.peer_id();

        bob_state
            .clone_project(project.urn(), alice_peer_id, alice_addrs, None)
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
        vec![project.urn()]
    );

    let commit_id = {
        let repo =
            git2::Repository::open(alice_repo_path.join(project.subject().name.to_string()))?;
        let oid = repo
            .find_reference(&format!(
                "refs/heads/{}",
                project.subject().default_branch.clone().unwrap()
            ))?
            .target()
            .expect("Missing first commit");
        let commit = repo.find_commit(oid)?;
        let commit_id = {
            let empty_tree = {
                let mut index = repo.index()?;
                let oid = index.write_tree()?;
                repo.find_tree(oid)?
            };

            let author =
                git2::Signature::now(&alice.subject().name.to_string(), "alice@example.com")?;
            repo.commit(
                Some(&format!(
                    "refs/heads/{}",
                    project.subject().default_branch.clone().unwrap()
                )),
                &author,
                &author,
                "Successor commit",
                &empty_tree,
                &[&commit],
            )?
        };

        {
            let mut rad =
                Remote::<LocalUrl>::rad_remote::<_, Fetchspec>(LocalUrl::from(project.urn()), None);
            let _ = rad.push(
                alice_state.settings(),
                &repo,
                LocalPushspec::Matching {
                    pattern: RefLike::try_from(format!(
                        "refs/heads/{}",
                        project.subject().default_branch.clone().unwrap()
                    ))
                    .unwrap()
                    .into(),
                    force: Force::False,
                },
            )?;
        }

        commit_id
    };

    {
        let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();
        let alice_peer_id = alice_state.peer_id();

        bob_state
            .fetch(project.urn(), alice_peer_id, alice_addrs, None)
            .await?;
    };

    let alice_peer_id = alice_state.peer_id();
    let has_commit = bob_state
        .has_commit(
            project.urn().with_path(Some(
                RefLike::try_from(format!(
                    "refs/remotes/{}/heads/{}",
                    alice_peer_id,
                    project.subject().default_branch.clone().unwrap(),
                ))
                .unwrap(),
            )),
            coco::git_ext::Oid::from(commit_id),
        )
        .await?;
    assert!(has_commit);

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
            sync: run_config::Sync {
                on_startup: true,
                ..run_config::Sync::default()
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_addrs = alice_state.listen_addrs().collect();
    let alice_peer_id = alice_state.peer_id();
    let mut alice_events = alice_peer.subscribe();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs,
            peer_id: alice_peer_id,
        }],
        RunConfig::default(),
    )
    .await?;
    let bob_peer_id = bob_state.peer_id();

    let alice = alice_state.init_owner("alice".to_string()).await?;
    let _bob = bob_state.init_owner("bob".to_string()).await?;
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
    let alice = alice_state.init_owner("alice".to_string()).await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_repo_path = bob_tmp_dir.path().join("radicle");
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let bob = bob_state.init_owner("bob".to_string()).await?;

    let eve_tmp_dir = tempfile::tempdir()?;
    let eve_repo_path = eve_tmp_dir.path().join("radicle");
    let (eve_peer, eve_state) = build_peer(&eve_tmp_dir, RunConfig::default()).await?;
    let _eve = eve_state.init_owner("eve".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());
    tokio::task::spawn(eve_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let project = {
        let alice_peer_id = alice_state.peer_id();
        let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();
        let bob_peer_id = bob_state.peer_id();
        let bob_addrs = bob_state.listen_addrs().collect::<Vec<_>>();
        bob_state
            .clone_project(project.urn(), alice_peer_id, alice_addrs, None)
            .await
            .expect("unable to clone project");
        eve_state
            .clone_project(project.urn(), bob_peer_id, bob_addrs, None)
            .await
            .expect("unable to clone project");
        eve_state.get_project(project.urn()).await?.unwrap()
    };

    let commit_id = {
        let alice_peer_id = alice_state.peer_id();
        log::debug!("CHECKING OUT");
        let path = bob_state
            .checkout(project.urn(), alice_peer_id, bob_repo_path)
            .await?;
        log::debug!("CHECKED OUT");

        let repo = git2::Repository::open(path)?;
        let oid = repo
            .find_reference(&format!(
                "refs/heads/{}",
                project.subject().default_branch.clone().unwrap()
            ))?
            .target()
            .expect("Missing first commit");
        let commit = repo.find_commit(oid)?;
        let commit_id = {
            let empty_tree = {
                let mut index = repo.index()?;
                let oid = index.write_tree()?;
                repo.find_tree(oid)?
            };

            let author = git2::Signature::now(
                bob.subject().name.as_str(),
                &format!("{}@example.com", bob.subject().name),
            )?;
            repo.commit(
                Some(&format!(
                    "refs/heads/{}",
                    project.subject().default_branch.clone().unwrap()
                )),
                &author,
                &author,
                "Successor commit",
                &empty_tree,
                &[&commit],
            )?
        };

        {
            let mut rad = Remote::rad_remote::<_, Fetchspec>(LocalUrl::from(project.urn()), None);
            let _ = rad.push(
                bob_state.settings(),
                &repo,
                LocalPushspec::Matching {
                    pattern: RefLike::try_from(format!(
                        "refs/heads/{}",
                        project.subject().default_branch.clone().unwrap()
                    ))
                    .unwrap()
                    .into(),
                    force: Force::False,
                },
            )?;
        }

        commit_id
    };

    {
        let bob_addrs = bob_state.listen_addrs().collect::<Vec<_>>();
        let bob_peer_id = bob_state.peer_id();
        eve_state
            .fetch(project.urn(), bob_peer_id, bob_addrs, None)
            .await?;
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

#[allow(clippy::needless_collect)]
#[tokio::test]
async fn track_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = alice_state.init_owner("alice".to_string()).await?;
    let alice_addrs = alice_state.listen_addrs().collect::<Vec<_>>();
    let mut alice_events = alice_peer.subscribe();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_state) = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let _bob = bob_state.init_owner("bob".to_string()).await?;

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    bob_state
        .clone_project(
            project.urn(),
            alice_state.peer_id(),
            alice_addrs.into_iter(),
            None,
        )
        .await?;

    alice_state
        .track(project.urn(), bob_state.peer_id())
        .await?;

    assert_event!(alice_events, coco::PeerEvent::GossipFetched { .. })?;

    let tracked = alice_state.tracked(project.urn()).await?;
    assert!(tracked.iter().any(|peer| match peer {
        Peer::Remote { peer_id, status } =>
            *peer_id == bob_state.peer_id() && matches!(status, peer::Status::Replicated(_)),
        _ => false,
    }));

    Ok(())
}
