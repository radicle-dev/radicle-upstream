//! Compute, track and announce noteworthy changes to the network.

use std::{collections::HashSet, ops::Deref as _};

use librad::{keys, net, uri};

use crate::{error::Error, oid::Oid, peer};

/// An update and all the required information that can be announced on the network.
pub type Announcement = (uri::RadUrn, Oid);

/// Announces the list of given `updates` with the [`librad::net::protocol`].
///
/// # Errors
///
/// * if the announcemnet of one of the project heads failed
pub async fn announce(
    protocol: net::protocol::Protocol<net::peer::PeerStorage<keys::SecretKey>, net::peer::Gossip>,
    updates: impl Iterator<Item = &Announcement> + Send,
) -> Result<(), Error> {
    for (urn, hash) in updates {
        let have = net::peer::Gossip {
            urn: urn.clone(),
            rev: Some(net::peer::Rev::Git((*hash).into())),
            origin: None,
        };

        protocol.announce(have).await;
    }

    Ok(())
}

/// Builds the latest list of [`Announcement`]s for the current state of the peer.
///
/// # Errors
///
/// * if listing of the projects fails
/// * if listing of the Refs for a project fails
pub fn build(api: &peer::Api) -> Result<HashSet<Announcement>, Error> {
    let mut list: HashSet<Announcement> = HashSet::new();

    // TODO(xla): We need to avoid the case where there is no owner yet for the peer api, there
    // should be machinery to kick off these routines only if our app state is ready for it.
    match api.list_projects() {
        Err(Error::Storage(librad::git::storage::Error::Config(_err))) => Ok(list),
        Err(err) => Err(err),
        Ok(projects) => {
            for project in &projects {
                let refs = api.list_owner_project_refs(&project.urn())?;

                for (head, hash) in &refs.heads {
                    list.insert((
                        uri::RadUrn {
                            path: head.parse::<uri::Path>()?,
                            ..project.urn()
                        },
                        Oid::from(*hash.deref()),
                    ));
                }
            }

            Ok(list)
        },
    }
}

/// Computes the list of announcements based on the difference of the `new` and `old` state. An
/// [`Announcement`] will be included if an entry in `new` can't be found in `old`.
#[allow(clippy::implicit_hasher)]
#[must_use]
pub fn diff<'a>(
    old_state: &'a HashSet<Announcement>,
    new_state: &'a HashSet<Announcement>,
) -> HashSet<Announcement> {
    new_state.difference(old_state).cloned().collect()
}

#[allow(clippy::panic)]
#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use librad::{hash::Hash, keys::SecretKey, uri};

    use crate::{config, error::Error, oid, peer, signer};

    #[tokio::test]
    async fn announce() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::new(signer::SomeSigner {
            signer: key.clone(),
        });
        let config = config::default(key, tmp_dir.path())?;
        let api = peer::Api::new(config).await?;

        let _owner = api.init_owner(&signer, "cloudhead")?;

        // TODO(xla): Build up proper testnet to assert that haves are announced.
        let updates = super::build(&api)?;
        let res = super::announce(api.protocol(), updates.iter()).await;

        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), Error> {
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

    fn project0(head: &str) -> uri::RadUrn {
        uri::RadUrn {
            id: Hash::hash(b"project0"),
            proto: uri::Protocol::Git,
            path: head.parse::<uri::Path>().expect("unable to parse head"),
        }
    }
    fn project1(head: &str) -> uri::RadUrn {
        uri::RadUrn {
            id: Hash::hash(b"project1"),
            proto: uri::Protocol::Git,
            path: head.parse::<uri::Path>().expect("unable to parse head"),
        }
    }
}
