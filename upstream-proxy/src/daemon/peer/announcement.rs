// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;

use librad::{
    git::Urn,
    git_ext::{Oid, RefLike},
    identities::{urn, SomeIdentity},
    net::peer::Peer,
    Signer,
};
use tokio::task::spawn_blocking;

use crate::daemon::{peer::gossip, state};

/// Name for the bucket used in [`kv::Store`].
const BUCKET_NAME: &str = "announcements";
/// Key for the single value used as cache.
const KEY_NAME: &str = "latest";

/// Announcement errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failures from [`kv`].
    #[error(transparent)]
    Kv(#[from] kv::Error),

    /// Failures parsing.
    #[error(transparent)]
    Parse(#[from] urn::error::FromStr<git2::Error>),

    /// Error occurred when interacting with [`Peer`].
    #[error(transparent)]
    State(#[from] Box<state::Error>),

    /// Error in spawned task.
    #[error(transparent)]
    Task(#[from] tokio::task::JoinError),
}

impl From<state::Error> for Error {
    fn from(e: state::Error) -> Self {
        Self::from(Box::new(e))
    }
}

/// An update and all the required information that can be announced on the
/// network.
pub type Announcement = (Urn, Oid);

/// Unique list of [`Announcement`]s.
pub type Updates = HashSet<Announcement>;

/// Announces the list of given `updates` with the [`librad::net::protocol`].
///
/// # Errors
///
/// * if the announcemnet of one of the project heads failed
fn announce<'a, S>(peer: &Peer<S>, updates: impl Iterator<Item = &'a Announcement> + Send)
where
    S: Clone + Signer,
{
    for (urn, hash) in updates {
        gossip::announce(peer, urn, Some(*hash));
    }
}

/// Builds the latest list of [`Announcement`]s for the current state of the
/// peer.
///
/// # Errors
///
/// * if listing of the projects fails
/// * if listing of the Refs for a project fails
async fn build<S>(peer: &Peer<S>) -> Result<Updates, Error>
where
    S: Clone + Signer,
{
    let identities = state::list_identities(peer).await?;
    let mut updates: Updates = HashSet::new();
    for identity in identities {
        let urn = match identity {
            SomeIdentity::Person(person) => person.urn(),
            SomeIdentity::Project(project) => project.urn(),
            _ => continue,
        };
        let refs = match state::load_refs(peer, urn.clone()).await? {
            Some(refs) => refs,
            None => continue,
        };
        for ((one_level, oid), category) in refs.iter_categorised() {
            let path = RefLike::from(one_level.clone().into_qualified(category.into()));
            let urn = urn.clone().with_path(path);
            updates.insert((urn, *oid));
        }
    }
    Ok(updates)
}

/// Computes the list of announcements based on the difference of the `new` and
/// `old` state. An [`Announcement`] will be included if an entry in `new` can't
/// be found in `old`.
#[must_use]
fn diff<'a>(old_state: &'a Updates, new_state: &'a Updates) -> Updates {
    new_state.difference(old_state).cloned().collect()
}

/// Load the cached announcements from the [`kv::Store`].
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the access of the key in the [`kv::Bucket`] fails
fn load(store: &kv::Store) -> Result<Updates, Error> {
    let bucket = store.bucket::<&'static str, kv::Json<Updates>>(Some(BUCKET_NAME))?;
    let value = bucket.get(KEY_NAME)?.map_or(HashSet::new(), |json| json.0);

    Ok(value)
}

/// Runs the entire announcement procedure.
///
/// # Errors
///
/// * if it can't build the new list of updates
/// * access to the storage fails
pub async fn run<S>(peer: &Peer<S>, store: kv::Store) -> Result<Updates, Error>
where
    S: Clone + Signer,
{
    let old = spawn_blocking({
        let store = store.clone();
        move || load(&store)
    })
    .await??;
    let new = build(peer).await?;
    let updates = diff(&old, &new);

    announce(peer, updates.iter());

    if !updates.is_empty() {
        spawn_blocking(move || save(&store, new.clone())).await??;
    }

    Ok(updates)
}

/// Update the cache with the latest announcements.
///
/// # Errors
///
/// * if the [`kv::Bucket`] can't be accessed
/// * if the storage of the new updates fails
fn save(store: &kv::Store, updates: Updates) -> Result<(), Error> {
    let bucket = store.bucket::<&'static str, kv::Json<Updates>>(Some(BUCKET_NAME))?;
    bucket.set(KEY_NAME, kv::Json(updates)).map_err(Error::from)
}
