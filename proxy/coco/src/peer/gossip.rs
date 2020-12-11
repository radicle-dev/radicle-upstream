//! Emit `Have`s and `Want`s on the network.

use librad::{
    identities::Urn,
    net::peer::{Gossip, Rev},
    peer::PeerId,
};
use radicle_git_ext::Oid;

use crate::State;

/// Announce a new rev for the `urn`.
pub async fn announce(state: &State, urn: &Urn, rev: Option<Oid>) {
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
pub async fn query(state: &State, urn: Urn, origin: Option<PeerId>) {
    state
        .api
        .protocol()
        .query(Gossip {
            urn,
            rev: None,
            origin,
        })
        .await;
}
