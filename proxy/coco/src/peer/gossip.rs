//! Emit `Have`s and `Want`s on the network.

use librad::{
    identities::Urn,
    net::{
        peer::Peer,
        protocol::gossip::{Payload, Rev},
    },
    peer::PeerId,
    signer::BoxedSigner,
};
use radicle_git_ext::Oid;

/// Announce a new rev for the `urn`.
pub fn announce(peer: &Peer<BoxedSigner>, urn: &Urn, rev: Option<Oid>) {
    peer.announce(Payload {
        urn: urn.clone(),
        rev: rev.map(|rev| Rev::Git(rev.into())),
        origin: None,
    });
}

/// Emit a [`Gossip`] request for the given `urn`.
pub fn query(peer: &Peer<BoxedSigner>, urn: Urn, origin: Option<PeerId>) {
    peer.query(Payload {
        urn,
        rev: None,
        origin,
    });
}
