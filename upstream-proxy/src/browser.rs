// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use std::convert::TryFrom as _;

use librad::git::types::{Reference, Single};
use radicle_source::{error, surf::vcs::git};

use crate::error::Error;

/// Provide a repo [`git::Browser`] where the `Browser` is initialised with the provided
/// `reference`.
///
/// See [`radicle_daemon::state::find_default_branch`] and [`radicle_daemon::state::get_branch`] for
/// obtaining a [`Reference`].
///
/// # Errors
///   * If the namespace of the reference could not be converted to a [`git::Namespace`].
///   * If we could not open the backing storage.
///   * If we could not initialise the `Browser`.
///   * If the callback provided returned an error.
pub fn using<T, F>(
    peer: &crate::peer::Peer,
    reference: Reference<Single>,
    callback: F,
) -> Result<T, Error>
where
    F: FnOnce(&mut git::Browser) -> Result<T, radicle_source::Error> + Send,
{
    let namespace = git::namespace::Namespace::try_from(
        reference
            .namespace
            .ok_or(radicle_daemon::state::Error::MissingNamespace)?
            .to_string()
            .as_str(),
    )
    .map_err(error::Error::from)?;

    let branch = match reference.remote {
        None => git::Branch::local(reference.name.as_str()),
        Some(peer) => git::Branch::remote(
            &format!("heads/{}", reference.name.as_str()),
            &peer.to_string(),
        ),
    };

    let monorepo = radicle_daemon::state::monorepo(peer.librad_peer());
    let repo = git::Repository::new(monorepo).map_err(error::Error::from)?;
    let mut browser =
        git::Browser::new_with_namespace(&repo, &namespace, branch).map_err(error::Error::from)?;

    Ok(callback(&mut browser)?)
}
