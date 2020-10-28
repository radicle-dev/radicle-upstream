use std::time::{Duration, Instant};

use futures::{future, StreamExt as _};
use tokio::time::timeout;

use librad::{net::{gossip, peer::Rev, protocol::ProtocolEvent}, uri};
use radicle_surf::vcs::git::git2;

use coco::{config, seed::Seed, AnnounceConfig, RunConfig};

#[macro_use]
mod common;
use common::{
    assert_cloned, build_peer, build_peer_with_seeds, connected, init_logging, radicle_project,
    requested, shia_le_pathbuf,
};

#[tokio::test(core_threads = 2)]
async fn can_announce_new_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: AnnounceConfig {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_events = alice_peer.subscribe();

    tokio::spawn(alice_peer.into_running());

    let alice = alice_state.init_owner("alice").await?;
    alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await
        .expect("unable to init project");

    let announced = alice_events
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Announced(updates) if updates.len() == 1 => future::ready(Some(())),
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test(core_threads = 2)]
async fn can_observe_announcement_from_connected_peer() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(
        &alice_tmp_dir,
        RunConfig {
            announce: AnnounceConfig {
                interval: Duration::from_millis(100),
            },
            ..RunConfig::default()
        },
    )
    .await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();
    let alice = alice_state.init_owner("alice").await?;

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
    let _bob = bob_state.init_owner("bob").await?;
    let bob_connected = bob_peer.subscribe();
    let bob_events = bob_peer.subscribe();

    tokio::spawn(alice_peer.into_running());
    tokio::spawn(bob_peer.into_running());

    connected(bob_connected, &alice_peer_id).await?;

    let project = alice_state
        .init_project(&alice, shia_le_pathbuf(alice_repo_path))
        .await?;

    let announced = bob_events
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            coco::PeerEvent::Protocol(ProtocolEvent::Gossip(info)) => match info {
                librad::net::gossip::Info::Has(librad::net::gossip::Has {
                    provider,
                    val: librad::net::peer::Gossip { urn, .. },
                }) if provider.peer_id == alice_peer_id && urn.id == project.urn().id => {
                    future::ready(Some(()))
                },
                _ => future::ready(None),
            },
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(announced);
    timeout(Duration::from_secs(1), announced.next()).await?;

    Ok(())
}

#[tokio::test(core_threads = 2)]
async fn can_ask_and_clone_project() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_repo_path = alice_tmp_dir.path().join("radicle");
    let (alice_peer, alice_state) = build_peer(&alice_tmp_dir, RunConfig::default()).await?;
    let alice_addr = alice_state.listen_addr();
    let alice_peer_id = alice_state.peer_id();

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
    let bob_events = bob_peer.subscribe();
    let mut bob_control = bob_peer.control();
    let clone_listener = bob_peer.subscribe();
    let query_listener = bob_peer.subscribe();

    tokio::task::spawn(alice_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());

    connected(bob_events, &alice_peer_id).await?;

    bob_state.init_owner("bob").await?;

    let urn = {
        let alice = alice_state.init_owner("alice").await?;
        let project = radicle_project(alice_repo_path.clone());
        alice_state.init_project(&alice, project).await?.urn()
    };

    bob_control.request_project(&urn, Instant::now()).await;

    requested(query_listener, &urn).await?;
    assert_cloned(clone_listener, &urn.clone().into_rad_url(alice_peer_id)).await?;

    // TODO(finto): List projects
    let project = bob_state.get_project(urn, None).await;
    assert!(project.is_ok());

    Ok(())
}

#[tokio::test]
async fn can_hear_commits() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let run_config = RunConfig {
        announce: AnnounceConfig {
            interval: Duration::from_millis(100),
        },
        ..RunConfig::default()
    };

    let seed_tmp_dir = tempfile::tempdir()?;
    let (seed_peer, seed_state) = build_peer(&seed_tmp_dir, run_config.clone()).await?;
    let _seed = { seed_state.init_owner("seedling.xyz").await? };

    let bob_tmp_dir = tempfile::tempdir()?;
    let bob_repo_path = bob_tmp_dir.path().join("radicle");
    let (bob_peer, bob_state) = build_peer_with_seeds(
        &bob_tmp_dir,
        vec![Seed {
            addr: seed_state.listen_addr(),
            peer_id: seed_state.peer_id(),
        }],
        run_config.clone(),
    )
    .await?;
    let bob = bob_state.init_owner("bob").await?;

    /*
    let eve_tmp_dir = tempfile::tempdir()?;
    let (eve_peer, eve_state) = build_peer_with_seeds(
        &eve_tmp_dir,
        vec![Seed {
            addr: seed_state.listen_addr(),
            peer_id: seed_state.peer_id(),
        }],
        run_config.clone(),
    )
    .await?;
    let _eve = eve_state.init_owner("eve").await?;
    */

    println!("{:?}", seed_tmp_dir);
    println!("{:?}", bob_tmp_dir);
    // println!("{:?}", eve_tmp_dir);

    let seed_events = seed_peer.subscribe();
    let bob_events = bob_peer.subscribe();
    tokio::task::spawn(seed_peer.into_running());
    tokio::task::spawn(bob_peer.into_running());
    /*
    tokio::task::spawn(eve_peer.into_running());
    */

    let project = bob_state
        .init_project(&bob, shia_le_pathbuf(bob_repo_path.clone()))
        .await?;

    let _ = {
        let seed_peer_id = seed_state.peer_id();
        let seed_addr = seed_state.listen_addr();
        let bob_peer_id = bob_state.peer_id();
        let urn = seed_state
            .clone_project(
                project.urn().into_rad_url(bob_peer_id),
                vec![bob_state.listen_addr()].into_iter(),
            )
            .await
            .expect("unable to clone project");
        seed_state.track(urn.clone(), bob_peer_id.clone()).await?;
        /*
        let urn = eve_state
            .clone_project(urn.into_rad_url(seed_peer_id), vec![seed_addr].into_iter())
            .await
            .expect("unable to clone project");
        eve_state.track(urn.clone(), bob_peer_id).await?;
        eve_state.get_project(urn.clone(), None).await?
        */
    };

    let commit_id = {
        let repo = git2::Repository::open(bob_repo_path.join(project.name()))?;
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

        // TODO(finto): Wait on Results
        let mut rad = repo.find_remote(config::RAD_REMOTE)?;
        rad.push(&[&format!("refs/heads/{}", project.default_branch())], None)?;
        commit_id
    };
    println!("COMMIT ID: {}", commit_id);

    {
        let announced = bob_events.into_stream().take_while(|res| {
            let mut branch = project.urn();
            branch.path = librad::uri::Path::parse(format!("{}", project.default_branch()))
                .expect("failed to parse branch name");

            {
                let expected = (branch, commit_id.into());
                match res.as_ref().unwrap() {
                    coco::PeerEvent::Announced(updates) if updates.contains(&expected) => {
                        future::ready(false)
                    },
                    _ => future::ready(true),
                }
            }
        });
        let _ = timeout(Duration::from_secs(1), announced.collect::<Vec<_>>()).await?;
    }

    let seed_heard = seed_events
        .into_stream()
        .take_while(|res| match res.as_ref().unwrap() {
            coco::PeerEvent::Protocol(ProtocolEvent::Gossip(gossip::Info::Has(gossip)))
                if gossip.val.rev == Some(Rev::Git(commit_id)) =>
            {
                future::ready(false)
            },
            _ => future::ready(true),
        });
    println!("{:?}", timeout(Duration::from_secs(1), seed_heard.collect::<Vec<_>>()).await?);

    let branch = uri::RadUrn {
        path: uri::Path::parse(format!(
            "refs/remotes/{}/heads/{}",
            bob_state.peer_id(),
            project.default_branch()
        ))
        .expect("failed to parse branch name"),
        ..project.urn().clone()
    };

    assert!(
        seed_state.has_commit(branch.clone(), commit_id).await?,
        "seed is missing the commit"
    );
    /*
    assert!(
        eve_state.has_commit(branch, commit_id).await?,
        "eve is missing the commit"
    );
    */

    Ok(())
}
