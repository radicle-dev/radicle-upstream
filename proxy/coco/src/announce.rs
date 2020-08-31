//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;
use std::ops::Deref as _;

use librad::keys;
use librad::net;
use librad::uri;

use crate::error::Error;
use crate::oid::Oid;
use crate::peer;

/// An update and all the required information that can be announced on the network.
pub type Announcement = (uri::RadUrn, String, Oid);

/// Announces the list of given `updates` with the [`librad::net::protocol`].
///
/// # Errors
///
/// * if the announcemnet of one of the project heads failed
pub async fn announce(
    protocol: net::protocol::Protocol<net::peer::PeerStorage<keys::SecretKey>, net::peer::Gossip>,
    updates: Vec<Announcement>,
) -> Result<(), Error> {
    for (project_urn, head, hash) in &updates {
        let urn = uri::RadUrn::new(
            project_urn.id.clone(),
            uri::Protocol::Git,
            uri::Path::parse(head)?,
        );
        let have = net::peer::Gossip {
            urn,
            rev: Some(net::peer::Rev::Git((*hash).into())),
            origin: Some(protocol.peer_id().clone()),
        };

        protocol.announce(have).await;
    }

    Ok(())
}

/// Builds the latset list of [`Announcement`]s for the current state of the peer.
///
/// # Errors
///
/// * if listing of the projects fails
/// * if listing of the Refs for a project fails
pub fn build(api: &peer::Api) -> Result<Vec<Announcement>, Error> {
    let mut list: Vec<Announcement> = vec![];

    // TODO(xla): We need to avoid the case where there is no owner yet for the peer api, there
    // should be machinery to kick off these routines only if our app state is ready for it.
    match api.list_projects() {
        Err(Error::Storage(librad::git::storage::Error::Config(_err))) => Ok(list),
        Err(err) => Err(err),
        Ok(projects) => {
            for project in &projects {
                let refs = api.list_project_refs(&project.urn())?;

                for (head, hash) in &refs.heads {
                    list.push((project.urn(), head.to_string(), Oid::from(*hash.deref())));
                }
            }

            Ok(list)
        }
    }
}

/// Computes the list of announcements based on the difference of the `new` and `old` state. An
/// [`Announcement`] will be included if an entry in `new` can't be found in `old`.
#[must_use]
pub fn diff(old_state: &[Announcement], new_state: &[Announcement]) -> Vec<Announcement> {
    let old: HashSet<_> = old_state.iter().collect();
    new_state
        .iter()
        .filter(|a| !old.contains(a))
        .cloned()
        .collect()
}

#[allow(clippy::panic)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use librad::hash::Hash;
    use librad::keys::SecretKey;
    use librad::uri;

    use crate::config;
    use crate::error::Error;
    use crate::oid;
    use crate::peer;

    #[tokio::test]
    async fn announce() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = peer::Api::new(config).await?;

        let _owner = api.init_owner(&key, "cloudhead")?;

        // TODO(xla): Build up proper testnet to assert that haves are announced.
        let updates = super::build(&api)?;
        let res = api
            .with_protocol(|protocol| {
                Box::pin(async move { super::announce(protocol, updates).await })
            })
            .await;

        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), Error> {
        let project0 = || uri::RadUrn {
            id: Hash::hash(b"project0"),
            proto: uri::Protocol::Git,
            path: uri::Path::empty(),
        };
        let project1 = || uri::RadUrn {
            id: Hash::hash(b"project1"),
            proto: uri::Protocol::Git,
            path: uri::Path::empty(),
        };

        let both = vec![
            (
                project0(),
                "dev".to_string(),
                "68986574".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "master".to_string(),
                "c8d2ad44".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "stable".to_string(),
                "2d2e1408".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "cloudhead/cool-feature".to_string(),
                "68986574".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "fintohaps/doc-tests".to_string(),
                "f90353ba".parse::<oid::Oid>()?,
            ),
            (
                project1(),
                "dev".to_string(),
                "c8d2ad44".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "master".to_string(),
                "2d2e1408".parse::<oid::Oid>()?,
            ),
            (
                project1(),
                "stable".to_string(),
                "a3403e2d".parse::<oid::Oid>()?,
            ),
        ];
        let old = vec![
            (
                project0(),
                "igor/zero-assertions".to_string(),
                "72a78226".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "thoshol/remove".to_string(),
                "7c69d71a".parse::<oid::Oid>()?,
            ),
            (
                project1(),
                "rudolfs/release".to_string(),
                "8c085d58".parse::<oid::Oid>()?,
            ),
        ];
        let new = vec![
            (
                project0(),
                "cloudhead/new-language".to_string(),
                "7dec3269".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "fintohaps/notations".to_string(),
                "b4d3276d".parse::<oid::Oid>()?,
            ),
            (
                project0(),
                "kalt/eat-my-impls".to_string(),
                "2206e5dc".parse::<oid::Oid>()?,
            ),
            (
                project1(),
                "backport".to_string(),
                "869e5740".parse::<oid::Oid>()?,
            ),
        ];

        let left = [&both[..], &old[..]].concat();
        let right = [&both[..], &new[..]].concat();
        let announcements = super::diff(&left, &right);

        assert_eq!(announcements, new);

        Ok(())
    }
}
