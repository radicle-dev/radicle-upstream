// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Handling of include files

use librad::{git::Urn, net::peer::Peer, Signer};

use crate::state;

/// Update the include file for the given `RadUrn` and log the result.
pub async fn update<S>(peer: Peer<S>, urn: Urn)
where
    S: Clone + Signer,
{
    if let Err(err) = state::update_include(&peer, urn.clone()).await {
        tracing::error!(%urn, error = ?err, "Failed to update include file");
    }
}
