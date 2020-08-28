//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;

use librad::uri::RadUrn;

use crate::oid::Oid;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("announcement failed")]
    AnnouncementFailed,
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
    let new: HashSet<_> = new_state.iter().collect();

    // TODO(xla): Find a more elegant way to get a vec of values.
    new.difference(&old)
        .cloned()
        .map(|a| (a.0.clone(), a.1.clone(), a.2))
        .collect()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn announce() -> Result<(), super::Error> {
        let res = super::announce(vec![]).await;

        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn diff() -> Result<(), super::Error> {
        let left = vec![];
        let right = vec![];
        let announcements = super::diff(left, right);

        assert_eq!(announcements, vec![]);

        Ok(())
    }
}
