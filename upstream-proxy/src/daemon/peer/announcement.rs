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
#[allow(clippy::implicit_hasher)]
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
#[allow(clippy::implicit_hasher)]
fn save(store: &kv::Store, updates: Updates) -> Result<(), Error> {
    let bucket = store.bucket::<&'static str, kv::Json<Updates>>(Some(BUCKET_NAME))?;
    bucket.set(KEY_NAME, kv::Json(updates)).map_err(Error::from)
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, convert::TryFrom as _};

    use pretty_assertions::assert_eq;

    use librad::{
        crypto::{BoxedSigner, SomeSigner},
        git::Urn,
        net, SecretKey,
    };
    use radicle_git_ext::{oid, RefLike};

    use crate::daemon::{config, identities::payload::Person};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[allow(clippy::unwrap_used)] // XXX: clippy sees an unwrap where there is none
    async fn announce() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = BoxedSigner::new(SomeSigner {
            signer: key.clone(),
        });
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

        let _owner = crate::daemon::state::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;

        // TODO(xla): Build up proper testnet to assert that haves are announced.
        let updates = super::build(&peer).await?;
        super::announce(&peer, updates.iter());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), Box<dyn std::error::Error>> {
        let both = vec![
            (project0("dev"), "68986574".parse::<oid::Oid>()?),
            (project0("master"), "c8d2ad44".parse::<oid::Oid>()?),
            (project0("stable"), "2d2e1408".parse::<oid::Oid>()?),
            (
                project0("cloudhead/cool-feature"),
                "68986574".parse::<oid::Oid>()?,
            ),
            (
                project0("fintohaps/doc-tests"),
                "f90353ba".parse::<oid::Oid>()?,
            ),
            (project1("dev"), "c8d2ad44".parse::<oid::Oid>()?),
            (project0("master"), "2d2e1408".parse::<oid::Oid>()?),
            (project1("stable"), "a3403e2d".parse::<oid::Oid>()?),
        ];
        let old = vec![
            (
                project0("igor/zero-assertions"),
                "72a78226".parse::<oid::Oid>()?,
            ),
            (project0("thoshol/remove"), "7c69d71a".parse::<oid::Oid>()?),
            (project1("rudolfs/release"), "8c085d58".parse::<oid::Oid>()?),
        ];
        let new = vec![
            (
                project0("cloudhead/new-language"),
                "7dec3269".parse::<oid::Oid>()?,
            ),
            (
                project0("fintohaps/notations"),
                "b4d3276d".parse::<oid::Oid>()?,
            ),
            (
                project0("kalt/eat-my-impls"),
                "2206e5dc".parse::<oid::Oid>()?,
            ),
            (project1("backport"), "869e5740".parse::<oid::Oid>()?),
        ];

        let left: HashSet<_> = [&both[..], &old[..]].concat().iter().cloned().collect();
        let right: HashSet<_> = [&both[..], &new[..]].concat().iter().cloned().collect();
        let announcements = super::diff(&left, &right);

        assert_eq!(announcements, new.iter().cloned().collect::<HashSet<_>>());

        Ok(())
    }

    #[test]
    fn save_and_load() -> Result<(), Box<dyn std::error::Error>> {
        let updates: HashSet<_> = vec![
            (
                project0("cloudead/new-language"),
                "7dec3269".parse::<oid::Oid>()?,
            ),
            (
                project0("fintohaps/notations"),
                "b4d3276d".parse::<oid::Oid>()?,
            ),
            (project0("kalt/loops"), "2206e5dc".parse::<oid::Oid>()?),
            (project1("backport"), "869e5740".parse::<oid::Oid>()?),
        ]
        .iter()
        .cloned()
        .collect();
        let dir = tempfile::tempdir()?;
        let store = kv::Store::new(kv::Config::new(dir.path().join("store")))?;

        super::save(&store, updates.clone())?;

        assert_eq!(super::load(&store)?, updates);

        Ok(())
    }

    fn project0(head: &str) -> Urn {
        Urn {
            id: "7ab8629dd6da14dcacde7f65b3d58cd291d7e235"
                .parse::<radicle_git_ext::Oid>()
                .expect("oid parse failed"),
            path: Some(RefLike::try_from(head).expect("head was not reflike")),
        }
    }
    fn project1(head: &str) -> Urn {
        Urn {
            id: "7ab8629dd6da14dcacde7f65b3d58cd291d7e234"
                .parse::<radicle_git_ext::Oid>()
                .expect("oid parse failed"),
            path: Some(RefLike::try_from(head).expect("head was not reflike")),
        }
    }
}
