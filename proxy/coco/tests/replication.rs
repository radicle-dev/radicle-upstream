use std::{convert::TryFrom, time::Duration};

use assert_matches::assert_matches;
use futures::{future, StreamExt as _};
use pretty_assertions::assert_eq;
use tokio::time::timeout;

use librad::{
    git::{
        local::url::LocalUrl,
        types::{remote::LocalPushspec, Fetchspec, Force, Remote},
    },
    reflike,
};
use radicle_git_ext::RefLike;

use coco::{
    identities::payload::Person,
    project::{peer, Peer},
    seed::Seed,
    state,
    state::init_owner,
    RunConfig,
};

#[macro_use]
mod common;
use common::{
    build_peer, build_peer_with_seeds, connected, init_logging, shia_le_pathbuf, started,
};

#[tokio::test]
async fn can_clone_project() -> Result<(), Box<dyn std::error::Error>> {
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
        state::clone_project(&bob_peer, project.urn(), alice_peer_id, alice_addrs, None).await?;
    }

    let have = state::list_projects(&bob_peer)
        .await?
        .into_iter()
        .map(|project| project.urn())
        .collect::<Vec<_>>();
    let want = vec![project.urn()];

    assert_eq!(have, want);

    {
        let another_peer = librad::peer::PeerId::from(librad::keys::SecretKey::new());
        state::track(&bob_peer, project.urn(), another_peer).await?;
        let mut have = state::tracked(&bob_peer, project.urn())
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
                peer_id: alice_peer.peer_id(),
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

    {
        let alice_peer_id = alice_peer.peer_id();
        state::clone_user(&bob_peer, alice.urn(), alice_peer_id, alice_addrs, None).await?;
    }

    let want = state::list_users(&bob_peer)
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

    let project = state::init_project(
        &alice_peer,
        &alice,
        shia_le_pathbuf(alice_repo_path.clone()),
    )
    .await?;

    {
        let alice_peer_id = alice_peer.peer_id();
        state::clone_project(
            &bob_peer,
            project.urn(),
            alice_peer_id,
            alice_addrs.clone(),
            None,
        )
        .await
        .expect("unable to clone project")
    };

    assert_eq!(
        state::list_projects(&bob_peer)
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
            let branch =
                RefLike::try_from(project.subject().default_branch.as_ref().unwrap().as_str())
                    .unwrap();
            let _ = rad.push(
                state::settings(&alice_peer),
                &repo,
                LocalPushspec::Matching {
                    pattern: reflike!("refs/heads").join(branch).into(),
                    force: Force::False,
                },
            )?;
        }

        commit_id
    };

    {
        let alice_peer_id = alice_peer.peer_id();
        state::fetch(&bob_peer, project.urn(), alice_peer_id, alice_addrs, None).await?;
    };

    let alice_peer_id = alice_peer.peer_id();
    let has_commit = state::has_commit(
        &bob_peer,
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
    let config = RunConfig {
        sync: coco::peer::run_config::Sync {
            interval: Duration::from_millis(500),
        },
        ..RunConfig::default()
    };
    let alice_peer = build_peer(&alice_tmp_dir, config.clone()).await?;
    let mut alice_events = alice_peer.subscribe();
    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
        (peer, listen_addrs)
    };
    let alice_peer_id = alice_peer.peer_id();

    let bob_tmp_dir = tempfile::tempdir()?;
    let (bob_peer, bob_events) = {
        let peer = build_peer_with_seeds(
            &bob_tmp_dir,
            vec![Seed {
                addrs: alice_addrs,
                peer_id: alice_peer_id,
            }],
            config,
        )
        .await?;
        let bob_events = peer.subscribe();
        let bob_peer = peer.peer.clone();
        tokio::task::spawn(peer.run());
        (bob_peer, bob_events)
    };
    let bob_peer_id = bob_peer.peer_id();

    let alice = init_owner(
        &alice_peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;
    let _bob = init_owner(&bob_peer, Person { name: "bob".into() }).await?;
    state::init_project(
        &alice_peer,
        &alice,
        shia_le_pathbuf(alice_repo_path.clone()),
    )
    .await?;

    connected(bob_events, 1).await?;

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
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice = init_owner(
        &alice_peer.peer,
        Person {
            name: "alice".into(),
        },
    )
    .await?;

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_repo_path = bob_tmp_dir.path().join("radicle");
    let bob_peer = build_peer(&bob_tmp_dir, RunConfig::default()).await?;
    let bob = init_owner(&bob_peer.peer, Person { name: "bob".into() }).await?;

    let eve_tmp_dir = tempfile::tempdir()?;
    let eve_repo_path = eve_tmp_dir.path().join("radicle");
    let eve_peer = build_peer(&eve_tmp_dir, RunConfig::default()).await?;
    let _eve = init_owner(&eve_peer.peer, Person { name: "eve".into() }).await?;

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
        (peer, listen_addrs)
    };
    let (bob_peer, bob_addrs) = {
        let peer = bob_peer.peer.clone();
        let events = bob_peer.subscribe();
        let mut peer_control = bob_peer.control();
        tokio::task::spawn(bob_peer.run());
        started(events).await?;

        (peer, peer_control.listen_addrs().await)
    };
    let eve_peer = {
        let peer = eve_peer.peer.clone();
        let events = eve_peer.subscribe();
        tokio::task::spawn(eve_peer.run());
        started(events).await?;

        peer
    };

    let project =
        state::init_project(&alice_peer, &alice, shia_le_pathbuf(alice_repo_path)).await?;

    let project = {
        let alice_peer_id = alice_peer.peer_id();
        let bob_peer_id = bob_peer.peer_id();
        state::clone_project(&bob_peer, project.urn(), alice_peer_id, alice_addrs, None)
            .await
            .expect("unable to clone project");
        state::clone_project(
            &eve_peer,
            project.urn(),
            bob_peer_id,
            bob_addrs.clone(),
            None,
        )
        .await
        .expect("unable to clone project");
        state::get_project(&eve_peer, project.urn()).await?.unwrap()
    };

    let commit_id = {
        let alice_peer_id = alice_peer.peer_id();
        let path = state::checkout(&bob_peer, project.urn(), alice_peer_id, bob_repo_path).await?;

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
                state::settings(&bob_peer),
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
        let bob_peer_id = bob_peer.peer_id();
        state::fetch(&eve_peer, project.urn(), bob_peer_id, bob_addrs, None).await?;
    }

    let path = {
        let alice_peer_id = alice_peer.peer_id();
        state::checkout(&eve_peer, project.urn(), alice_peer_id, eve_repo_path).await?
    };

    let repo = git2::Repository::open(path)?;
    assert_matches!(repo.find_commit(commit_id), Err(_));

    Ok(())
}

#[tokio::test]
async fn track_peer() -> Result<(), Box<dyn std::error::Error>> {
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
    let mut alice_events = alice_peer.subscribe();

    let (alice_peer, alice_addrs) = {
        let peer = alice_peer.peer.clone();
        let events = alice_peer.subscribe();
        let mut peer_control = alice_peer.control();
        tokio::task::spawn(alice_peer.run());
        started(events).await?;

        let listen_addrs = peer_control.listen_addrs().await;
        (peer, listen_addrs)
    };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_peer = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addrs: alice_addrs.clone(),
            peer_id: alice_peer.peer_id(),
        }],
        RunConfig::default(),
    )
    .await?;
    let _bob = init_owner(&bob_peer.peer, Person { name: "bob".into() }).await?;

    let bob_peer = {
        let peer = bob_peer.peer.clone();
        let events = bob_peer.subscribe();
        tokio::task::spawn(bob_peer.run());
        started(events).await?;

        peer
    };

    let project =
        state::init_project(&alice_peer, &alice, shia_le_pathbuf(alice_repo_path)).await?;

    state::clone_project(
        &bob_peer,
        project.urn(),
        alice_peer.peer_id(),
        alice_addrs.into_iter(),
        None,
    )
    .await?;

    state::track(&alice_peer, project.urn(), bob_peer.peer_id()).await?;

    assert_event!(alice_events, coco::PeerEvent::GossipFetched { .. })?;

    let tracked = state::tracked(&alice_peer, project.urn()).await?;
    assert!(tracked.iter().any(|peer| match peer {
        Peer::Remote { peer_id, status } =>
            *peer_id == bob_peer.peer_id() && matches!(status, peer::Status::Replicated(_)),
        _ => false,
    }));

    Ok(())
}
