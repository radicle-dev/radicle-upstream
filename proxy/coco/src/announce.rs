//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;

use librad::uri::RadUrn;
use radicle_surf::git::git2;

use crate::oid::Oid;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("announcement failed")]
    AnnouncementFailed,

    #[error(transparent)]
    Git(#[from] git2::Error),

    #[error(transparent)]
    Parse(#[from] librad::uri::rad_urn::ParseError),
}

type Announcement = (RadUrn, String, Oid);

type State = Vec<Announcement>;

/// Announces the list of given `updates` with the [`librad::net::protocol`].
async fn announce(_updates: Vec<Announcement>) -> Result<(), Error> {
    Err(Error::AnnouncementFailed)
}

/// Computes the list of announcements based on the difference of the `new` and `old` state. An
/// [`Announcement`] will be included if an entry in `new` can't be found in `old`.
fn diff(old_state: State, new_state: State) -> Vec<Announcement> {
    let old: HashSet<_> = old_state.iter().collect();
    new_state.into_iter().filter(|a| !old.contains(a)).collect()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use librad::hash::Hash;
    use librad::uri;

    use crate::oid;

    #[tokio::test]
    async fn announce() -> Result<(), super::Error> {
        let res = super::announce(vec![]).await;

        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), super::Error> {
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
