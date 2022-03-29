// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Service for watching the local monorepo for updates.

use anyhow::Context;
use link_identities::git::Revision;
use std::collections::HashSet;

pub fn create(peer: crate::peer::Peer) -> (Handle, Runner) {
    let (update_tx, update_rx) = async_broadcast::broadcast(32);
    let handle = Handle {
        update_rx: update_rx.deactivate(),
    };

    let runner = Runner { peer, update_tx };
    (handle, runner)
}

#[derive(Clone)]
pub struct Handle {
    update_rx: async_broadcast::InactiveReceiver<link_identities::Urn<Revision>>,
}

impl Handle {
    pub fn updates(&self) -> async_broadcast::Receiver<link_identities::Urn<Revision>> {
        self.update_rx.activate_cloned()
    }
}

pub struct Runner {
    peer: crate::peer::Peer,
    update_tx: async_broadcast::Sender<link_identities::Urn<Revision>>,
}

impl Runner {
    pub async fn run(self) {
        let Self { peer, update_tx } = self;

        let mut old = match get_refs(&peer).await {
            Ok(list) => list,
            Err(err) => {
                tracing::warn!(?err, "could not get the refs for the initial project list");
                HashSet::new()
            },
        };

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let new = match get_refs(&peer).await {
                Ok(new_list) => new_list,
                Err(err) => {
                    tracing::warn!(?err, "could not get the refs for the new project list");
                    continue;
                },
            };

            let updates = new.symmetric_difference(&old);
            let deduped_updates = updates.map(|(urn, _)| urn).collect::<HashSet<_>>();

            for urn in deduped_updates {
                if let Err(err) = update_tx.try_broadcast(urn.clone()) {
                    tracing::warn!(?err, "failed to broadcast")
                };
            }

            old = new;
        }
    }
}

async fn get_refs(
    peer: &crate::peer::Peer,
) -> Result<HashSet<(librad::git::Urn, radicle_git_ext::Oid)>, anyhow::Error> {
    let peer = peer.librad_peer();

    let identities = peer
        .using_storage(move |store| {
            let identities = librad::git::identities::any::list(store)?
                .filter_map(Result::ok)
                .collect::<Vec<_>>();
            Ok::<_, anyhow::Error>(identities)
        })
        .await
        .context("failed to use storage")?
        .context("failed to list identities")?;

    let mut updates = HashSet::new();

    for identity in identities {
        let urn = match identity {
            link_identities::SomeIdentity::Project(project) => project.urn(),
            _ => continue,
        };

        let refs = peer
            .using_storage({
                let urn = urn.clone();
                move |store| librad::git::refs::Refs::load(store, &urn, None)
            })
            .await
            .context("failed to use storage")?
            .context("failed to load refs")?;

        let refs = match refs {
            Some(refs) => refs,
            None => continue,
        };

        for ((one_level, oid), category) in refs.iter_categorised() {
            let path =
                radicle_git_ext::RefLike::from(one_level.clone().into_qualified(category.into()));
            let urn = urn.clone().with_path(path);
            updates.insert((urn, *oid));
        }
    }
    Ok(updates)
}
