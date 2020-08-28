//! Compute, track and announce noteworthy changes to the network.

use std::collections::HashSet;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use librad::net::peer::Rev;
use librad::uri::RadUrn;

use crate::oid::Oid;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("announcement failed")]
    AnnouncementFailed,
}

#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
struct Announcement {
    urn: RadUrn,
    head: String,
    hash: Oid,
}

type State = Vec<Announcement>;

/// Announces the list of given `updates` with the [`librad::net::protoco].
async fn announce(_updates: Vec<Announcement>) -> Result<(), Error> {
    Err(Error::AnnouncementFailed)
}

fn diff(old_state: State, new_state: State) -> Vec<Announcement> {
    let old: HashSet<_> = old_state.iter().collect();
    let new: HashSet<_> = new_state.iter().collect();

    // TODO(xla): Find a more elegant way to get a vec of values.
    new.difference(&old)
        .cloned()
        .map(|a| Announcement {
            urn: a.urn.clone(),
            head: a.head.clone(),
            hash: a.hash.clone(),
        })
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
