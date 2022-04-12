// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Service for fetching projects seeds peers via Git+HTTPS.

use anyhow::Context as _;
use futures::prelude::*;
use link_identities::git::Revision;
use std::sync::Arc;

pub async fn create(
    peer: crate::peer::Peer,
    seeds: Vec<rad_common::Url>,
    fetch_interval: std::time::Duration,
) -> anyhow::Result<(Handle, Runner)> {
    let (update_tx, update_rx) = async_broadcast::broadcast(32);
    let (identity_queue, identity_rx) = UniqueDelayQueue::new();
    let handle = Handle {
        update_rx: update_rx.deactivate(),
        identity_queue: identity_queue.clone(),
    };

    let projects = crate::project::list_link(&peer)
        .await
        .context("failed to list projects")?;

    for project_result in projects {
        let project = project_result.context("failed to get project")?;
        handle.add(project.urn().id).await;
    }

    let runner = Runner {
        peer,
        seeds,
        update_tx,
        identity_rx,
        identity_queue,
        fetch_interval,
    };
    Ok((handle, runner))
}

#[derive(Debug, Clone)]
pub struct Handle {
    update_rx: async_broadcast::InactiveReceiver<Revision>,
    identity_queue: UniqueDelayQueue,
}

impl Handle {
    /// Add an identity to continuously fetch from the configured seeds. The identity will be
    /// fetched immediately after calling this function, even if it has been added before.
    pub async fn add(&self, identity: Revision) {
        self.identity_queue
            .add(identity, std::time::Duration::new(0, 0))
            .await
    }

    /// Stream that emits the identifier of an identity whenever we’ve fetched new updates for the
    /// identity from a seed.
    pub fn updates(&self) -> async_broadcast::Receiver<Revision> {
        self.update_rx.activate_cloned()
    }
}

pub struct Runner {
    peer: crate::peer::Peer,
    /// List of seed URLs to try to fetch identities from if we don’t know the seed yet.
    seeds: Vec<rad_common::Url>,
    /// Inform subscribers that an identity has been updated
    update_tx: async_broadcast::Sender<Revision>,
    /// Stream of queued identities to fetch updates for
    identity_rx: futures_delay_queue::Receiver<Revision>,
    /// Queue of identities to fetch updates for
    identity_queue: UniqueDelayQueue,
    /// Time after which project updates are fetched again.
    fetch_interval: std::time::Duration,
}

impl Runner {
    pub async fn run(self, shutdown_signal: future::BoxFuture<'static, ()>) {
        let Self {
            peer,
            seeds,
            update_tx,
            identity_rx,
            identity_queue,
            fetch_interval,
        } = self;

        let identity_rx = identity_rx.into_stream().take_until(shutdown_signal);
        futures::pin_mut!(identity_rx);
        let mut identity_provider = std::collections::HashMap::new();

        let seeds = seeds.into_iter().map(Arc::new).collect::<Vec<_>>();

        while let Some(identity) = identity_rx.next().await {
            match fetch_project(&peer, &seeds, &mut identity_provider, identity).await {
                Ok(true) => {
                    if let Err(err) = update_tx.try_broadcast(identity) {
                        tracing::warn!(?err, "failed to broadcast Git fetch result")
                    };
                },
                Ok(false) => {},
                Err(errs) => {
                    tracing::warn!(?errs, ?identity, "failed to fetch project with git");
                },
            };
            identity_queue.add(identity, fetch_interval).await;
        }
    }
}

/// Queue for [`Revision`]s that will be emitted by a receiver after a delay.
///
/// This is a wrapper around [`futures_delay_queue::DelayQueue`] that guarantees that each
/// [`Revision`] is only queued once.
#[derive(Debug, Clone)]
struct UniqueDelayQueue {
    handles: Arc<dashmap::DashMap<Revision, Option<futures_delay_queue::DelayHandle>>>,
    queue: Arc<
        futures_delay_queue::DelayQueue<
            Revision,
            futures_intrusive::buffer::GrowingHeapBuf<Revision>,
        >,
    >,
}

impl UniqueDelayQueue {
    fn new() -> (Self, futures_delay_queue::Receiver<Revision>) {
        let (queue, receiver) = futures_delay_queue::delay_queue();
        (
            Self {
                handles: Arc::new(dashmap::DashMap::new()),
                queue: Arc::new(queue),
            },
            receiver,
        )
    }

    /// Add a new [`Revision`] to the queue to be emitted after `delay`.
    ///
    /// If `revision` is already queued we update its entry so that it’s queued after `delay`.
    async fn add(&self, revision: Revision, delay: std::time::Duration) {
        match self.handles.entry(revision) {
            dashmap::mapref::entry::Entry::Occupied(mut occupied) => {
                let handle = occupied
                    .insert(None)
                    .expect("handle is None only when entry is locked");
                let handle = match handle.reset(delay).await {
                    Ok(handle) => handle,
                    Err(_expired) => self.queue.insert(revision, delay),
                };
                occupied.insert(Some(handle));
            },
            dashmap::mapref::entry::Entry::Vacant(vacant) => {
                let handle = self.queue.insert(revision, delay);
                vacant.insert(Some(handle));
            },
        }
    }
}

/// Try to fetch a project from one or more seeds.
///
/// Returns `true` if the project refernces were updated and `false` otherwise. Also returns
/// `false` if the project was not found on any of the seeds tried.
///
/// If the Project URN is present in `identity_providers`, then we only fetch it from that seed.
/// Otherwise, we try to fetch the projects from each of the `seeds`. If we find the project, we
/// update `identity_providers`.
async fn fetch_project(
    peer: &crate::peer::Peer,
    seeds: &[Arc<rad_common::Url>],
    identity_providers: &mut std::collections::HashMap<Revision, Arc<rad_common::Url>>,
    identity: Revision,
) -> Result<bool, Vec<anyhow::Error>> {
    let mut errors = vec![];

    let seeds_to_try = match identity_providers.get(&identity) {
        Some(seed) => std::borrow::Cow::Owned(vec![seed.clone()]),
        None => std::borrow::Cow::Borrowed(seeds),
    };

    for seed in &*seeds_to_try {
        let result = fetch_project_from_seed(peer, identity, seed)
            .await
            .context(format!("failed to fetch project from seed {}", &seed));
        tracing::debug!(identity = %link_identities::Urn::new(identity), seed = %seed, ?result, "fetched identity from git seed");
        match result {
            Ok(FetchResult::NotFound) => {},
            Ok(FetchResult::UpToDate) => {
                identity_providers.insert(identity, seed.clone());
                return Ok(false);
            },
            Ok(FetchResult::Updated) => {
                identity_providers.insert(identity, seed.clone());
                return Ok(true);
            },
            Err(err) => errors.push(err),
        };
    }

    if errors.is_empty() {
        Ok(false)
    } else {
        Err(errors)
    }
}

/// Result of fetching a project from a Git seed.
#[derive(Debug, Copy, Clone)]
enum FetchResult {
    /// The identity was found but our data is up-to-date.
    UpToDate,
    /// Updates for the identity have been fetched from the seed.
    Updated,
    /// The seed does not provide the identity.
    NotFound,
}

/// Try to fetch a project and all references of all the delegates from the Git seed.
async fn fetch_project_from_seed(
    peer: &crate::peer::Peer,
    project_id: Revision,
    seed_url: &rad_common::Url,
) -> anyhow::Result<FetchResult> {
    let this_peer_id = peer.librad_peer().peer_id();
    let monorepo_path = peer.paths().git_dir().to_owned();
    let urn = link_identities::Urn::new(project_id);
    let id = urn.encode_id();
    let proj_seed_url = seed_url.join(&id).expect("invalid Project URN");
    peer.librad_peer()
        .using_storage(move |storage| {
            match rad_common::seed::fetch_identity(&monorepo_path, &proj_seed_url, &urn) {
                Ok(_) => {},
                Err(err) => {
                    if err.root_cause().to_string()
                        == "fatal: couldn't find remote ref refs/rad/id\n"
                    {
                        return Ok(FetchResult::NotFound);
                    } else {
                        return Err(err.context("failed to fetch project identity"));
                    }
                },
            };

            let proj = rad_common::project::get(storage, &urn)?.context("failed to get project")?;

            for delegate in &proj.delegates {
                rad_common::seed::fetch_identity(&monorepo_path, &proj_seed_url, delegate)
                    .context(format!(
                        "failed to fetch identity for delegate {}",
                        delegate
                    ))?;
            }

            let tracking_config = Default::default();
            let tracking_actions = proj
                .remotes
                .iter()
                .filter(|remote_peer_id| **remote_peer_id != this_peer_id)
                .map({
                    |remote_peer_id| librad::git::tracking::Action::Track {
                        urn: (&urn).into(),
                        peer: Some(*remote_peer_id),
                        config: &tracking_config,
                        policy: librad::git::tracking::policy::Track::Any,
                    }
                });
            librad::git::tracking::batch(storage, tracking_actions)
                .context("failed to track remotes")?;

            let tracked_remotes = librad::git::tracking::tracked_peers(storage, Some(&urn))
                .context("failed to get tracked peers")?
                .filter(|re| match re {
                    Ok(id) => *id != this_peer_id,
                    Err(_) => true,
                })
                .collect::<Result<Vec<_>, _>>()
                .context("failed to get tracked peer")?;

            let output = rad_common::seed::fetch_remotes(
                &monorepo_path,
                &proj_seed_url,
                &urn,
                tracked_remotes,
            )
            .context("failed to fetch remotes")?;

            if output.contains("POST git-upload-pack") {
                Ok(FetchResult::Updated)
            } else {
                Ok(FetchResult::UpToDate)
            }
        })
        .await
        .context("failed to access storage")?
}
