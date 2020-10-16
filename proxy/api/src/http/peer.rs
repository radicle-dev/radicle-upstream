//! Inspect and perform actions on the locally running peer.

use std::net::SocketAddr;

use serde::Serialize;
use warp::{filters::BoxedFilter, Filter, Reply};

use crate::{context, http};

/// Combination of all endpoints.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    connected_peers(ctx)
}

/// `GET /connected_peers`
fn connected_peers(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("connected_peers")
        .and(warp::get())
        .and(http::with_context(ctx))
        .and_then(handler::connected_peers)
        .boxed()
}

/// Peer handlers.
mod handler {
    use warp::{reply, Rejection, Reply};

    use crate::context;

    /// List of currently connected peers.
    pub async fn connected_peers(ctx: context::Context) -> Result<impl Reply, Rejection> {
        let peers = ctx
            .state
            .connected_peers()
            .await
            .into_iter()
            .map(|(peer_id, addr)| super::ConnectedPeer { addr, peer_id })
            .collect::<Vec<_>>();

        Ok(reply::json(&peers))
    }
}

/// Pair of [`PeerId`] and [`SocketAddr`] of a remote peer.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConnectedPeer {
    /// Address of the conencted peer.
    addr: SocketAddr,
    /// PeerId of the connected peer.
    peer_id: coco::PeerId,
}
