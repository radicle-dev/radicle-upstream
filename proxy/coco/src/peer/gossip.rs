//! Emit `Have`s and `Want`s on the network.

use librad::{
    net::peer::{Gossip, Rev},
    peer::PeerId,
    uri::RadUrn,
};

use crate::{oid::Oid, State};

/// Announce a new rev for the `urn`.
pub async fn announce(state: &State, urn: &RadUrn, rev: Option<Oid>) {
    let protocol = state.api.protocol();
    protocol
        .announce(Gossip {
            urn: urn.clone(),
            rev: rev.map(|rev| Rev::Git(rev.into())),
            origin: None,
        })
        .await;
}

/// Emit a [`Gossip`] request for the given `urn`.
pub async fn query(state: &State, urn: RadUrn) {
    let protocol = state.api.protocol().clone();
    protocol
        .query(Gossip {
            urn,
            rev: None,
            origin: None,
        })
        .await;
}

/// Emit a [`Gossip`] request for the given `urn` from the given `peer_id`.
pub async fn query_with_origin(state: &State, urn: RadUrn, peer_id: PeerId) {
    let protocol = state.api.protocol().clone();
    protocol
        .query(Gossip {
            urn,
            rev: None,
            origin: Some(peer_id),
        })
        .await;
}
