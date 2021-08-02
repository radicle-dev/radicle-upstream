// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{context, http, notification::Notification};

/// SSE based notifications endpoint.
pub fn filters(
    ctx: context::Context,
    notifications: tokio::sync::broadcast::Sender<Notification>,
) -> BoxedFilter<(impl Reply,)> {
    local_peer_status_stream(ctx, notifications)
}

/// `GET /local_peer_events`
pub fn local_peer_status_stream(
    ctx: context::Context,
    notifications: tokio::sync::broadcast::Sender<Notification>,
) -> BoxedFilter<(impl Reply,)> {
    path!("local_peer_events")
        .and(http::with_context_unsealed(ctx))
        .and(warp::any().map(move || notifications.subscribe()))
        .and_then(handler::local_peer_events)
        .boxed()
}

/// Notification handlers to serve event streams.
mod handler {
    use futures::StreamExt as _;
    use warp::{sse, Rejection, Reply};

    use crate::{
        context,
        notification::{self, Notification},
    };

    /// Sets up local peer events notification stream.
    pub async fn local_peer_events(
        ctx: context::Unsealed,
        mut notifications: tokio::sync::broadcast::Receiver<Notification>,
    ) -> Result<impl Reply, Rejection> {
        let mut peer_control = ctx.peer_control;
        let current_status = peer_control.current_status().await;

        let initial = futures::stream::iter(vec![Notification::LocalPeer(
            notification::LocalPeer::StatusChanged {
                old: current_status.clone(),
                new: current_status,
            },
        )]);
        let filter = |notification: Notification| async move {
            match notification.clone() {
                Notification::LocalPeer(event) => Some(sse::Event::default().json_data(event)),
            }
        };
        let stream = async_stream::stream! {
            use tokio::sync::broadcast::error::RecvError;
            loop {
                match notifications.recv().await {
                    Ok(notification) => yield notification,
                    Err(RecvError::Lagged(_)) => {},
                    Err(RecvError::Closed) => break,

                }
            }
        };

        Ok(sse::reply(
            sse::keep_alive().stream(initial.chain(stream).filter_map(filter)),
        ))
    }
}
