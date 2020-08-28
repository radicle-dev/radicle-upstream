//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;
use std::ops::Deref as _;

use librad::uri::RadUrn;

use crate::error::Error;
use crate::oid::Oid;
use crate::peer;

/// An update and all the required information that can be announced on the network.
type Announcement = (RadUrn, String, Oid);

/// Announces the list of given `updates` with the [`librad::net::protocol`].
async fn announce(api: &peer::Api, updates: Vec<Announcement>) -> Result<(), Error> {
    for update in &updates {
        api.announce_project_head(&update.0, update.1.clone(), update.2)
            .await?;
    }

    Ok(())
}

/// Builds the latset list of [`Announcement`]s for the current state of the peer.
fn build(api: &peer::Api) -> Result<Vec<Announcement>, Error> {
    let projects = api.list_projects()?;
    let mut list: Vec<Announcement> = vec![];

    for project in &projects {
        let refs = api.list_project_refs(&project.urn())?;

        for (head, hash) in &refs.heads {
            list.push((project.urn(), head.to_string(), Oid::from(*hash.deref())));
        }
    }

    Ok(list)
}

/// Computes the list of announcements based on the difference of the `new` and `old` state. An
/// [`Announcement`] will be included if an entry in `new` can't be found in `old`.
fn diff(old_state: Vec<Announcement>, new_state: Vec<Announcement>) -> Vec<Announcement> {
    let old: HashSet<_> = old_state.iter().collect();
    new_state.into_iter().filter(|a| !old.contains(a)).collect()
}

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
        let res = super::announce(&api, updates).await;

        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), Error> {
        let project0 = || uri::RadUrn {
            id: Hash::hash("project0".as_bytes()),
            proto: uri::Protocol::Git,
            path: uri::Path::empty(),
        };
        let project1 = || uri::RadUrn {
            id: Hash::hash("project1".as_bytes()),
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
        let announcements = super::diff(left, right);

        assert_eq!(announcements, new);

        Ok(())
    }
}
