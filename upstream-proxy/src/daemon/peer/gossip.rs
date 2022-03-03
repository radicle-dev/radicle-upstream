// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Emit `Have`s and `Want`s on the network.

use librad::{
    git::Urn,
    git_ext::Oid,
    net::{
        peer::Peer,
        protocol::gossip::{Payload, Rev},
    },
    PeerId, Signer,
};

/// Announce a new rev for the `urn`.
pub fn announce<S>(peer: &Peer<S>, urn: &Urn, rev: Option<Oid>)
where
    S: Clone + Signer,
{
    match peer.announce(Payload {
        urn: urn.clone(),
        rev: rev.map(|rev| Rev::Git(rev.into())),
        origin: None,
    }) {
        Ok(()) => tracing::trace!(%urn, ?rev, "successfully announced URN"),
        Err(_payload) => tracing::warn!(%urn, ?rev, "failed to announce URN"),
    }
}

/// Emit a [`Payload`] request for the given `urn`.
pub fn query<S>(peer: &Peer<S>, urn: &Urn, origin: Option<PeerId>)
where
    S: Clone + Signer,
{
    match peer.query(Payload {
        urn: urn.clone(),
        rev: None,
        origin,
    }) {
        Ok(()) => tracing::trace!(%urn, ?origin, "successfully queried URN"),
        Err(_payload) => tracing::warn!(%urn, "failed to query URN"),
    };
}
