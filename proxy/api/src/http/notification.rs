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
        let mut subscriber = subscriptions.subscribe().await;

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
        let stream = async_stream::stream! { while let Some(notification) = subscriber.recv().await { yield notification } };

        Ok(sse::reply(
            sse::keep_alive().stream(initial.chain(stream).filter_map(filter)),
        ))
    }
}
