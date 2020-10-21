//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{context, http, notification::Subscriptions};

/// SSE based notifications endpoint.
pub fn filters(ctx: context::Context, subscriptions: Subscriptions) -> BoxedFilter<(impl Reply,)> {
    local_peer_status_stream(ctx, subscriptions)
}

/// `GET /local_peer_events`
pub fn local_peer_status_stream(
    ctx: context::Context,
    subscriptions: Subscriptions,
) -> BoxedFilter<(impl Reply,)> {
    path!("local_peer_events")
        .and(http::with_context_unsealed(ctx))
        .and(warp::any().map(move || subscriptions.clone()))
        .and_then(handler::local_peer_events)
        .boxed()
}

/// Notification handlers to serve event streams.
mod handler {
    use std::convert::Infallible;

    use futures::StreamExt as _;
    use warp::{sse, Rejection, Reply};

    use crate::{
        context,
        notification::{self, Notification, Subscriptions},
    };

    /// Sets up local peer events notification stream.
    pub async fn local_peer_events(
        ctx: context::Unsealed,
        subscriptions: Subscriptions,
    ) -> Result<impl Reply, Rejection> {
        let mut peer_control = ctx.peer_control;
        let current_status = peer_control.current_status().await;
        let subscriber = subscriptions.subscribe().await;

        let initial = futures::stream::iter(vec![Notification::LocalPeer(
            notification::LocalPeer::StatusChanged {
                old: current_status.clone(),
                new: current_status,
            },
        )]);
        let filter = |notification: Notification| async move {
            match notification.clone() {
                Notification::LocalPeer(event) => Some(map_to_event(event)),
            }
        };

        Ok(sse::reply(
            sse::keep_alive().stream(initial.chain(subscriber).filter_map(filter)),
        ))
    }

    /// Helper for mapping [`Notification::LocalPeerStatusChanged`] events onto
    /// [`sse::ServerSentEvent`]s.
    fn map_to_event(
        event: notification::LocalPeer,
    ) -> Result<impl sse::ServerSentEvent, Infallible> {
        Ok((sse::event(event.to_string()), sse::json(event)))
    }
}
