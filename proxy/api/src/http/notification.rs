//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{context, http, notification::Subscriptions};

/// SSE based notifications endpoint.
pub fn filters(ctx: context::Context, subscriptions: Subscriptions) -> BoxedFilter<(impl Reply,)> {
    local_peer_status_stream(ctx, subscriptions.clone())
}

/// `GET /local_peer_status`
pub fn local_peer_status_stream(
    ctx: context::Context,
    subscriptions: Subscriptions,
) -> BoxedFilter<(impl Reply,)> {
    path!("local_peer_status")
        .and(http::with_context(ctx))
        .and(warp::any().map(move || subscriptions.clone()))
        .and_then(handler::local_peer_status)
        .boxed()
}

/// Notification handlers to serve event streams.
mod handler {
    use std::convert::Infallible;

    use futures::{Stream, StreamExt as _};
    use tokio::sync::mpsc;
    use warp::{sse, Rejection, Reply};

    use crate::{
        context,
        notification::{Notification, Subscriptions},
    };

    pub async fn local_peer_status(
        ctx: context::Context,
        subscriptions: Subscriptions,
    ) -> Result<impl Reply, Rejection> {
        let mut peer_control = ctx.peer_control;
        let current_status = peer_control.current_status().await;
        let subscriber = subscriptions.subscribe().await;

        let initial = futures::stream::iter(vec![Notification::LocalPeerStatusChanged(
            current_status.clone(),
            current_status,
        )]);
        let filter = |notification: Notification| async move {
            match notification.clone() {
                Notification::LocalPeerStatusChanged(_old, _new) => {
                    Some(map_to_event(notification))
                }
                _ => None,
            }
        };

        Ok(sse::reply(
            sse::keep_alive().stream(initial.chain(subscriber).filter_map(filter)),
        ))
    }

    fn map_to_event(notification: Notification) -> Result<impl sse::ServerSentEvent, Infallible> {
        match notification {
            Notification::LocalPeerStatusChanged(_old, new) => {
                Ok((sse::event("LOCAL_PEER_STATUS_CHANGED"), sse::json(new)))
            }
        }
    }

    #[cfg(test)]
    mod test {
        use warp::filters::sse;

        use crate::notification;

        #[test]
        fn json_serialize() -> Result<(), Box<dyn std::error::Error>> {
            sse::json(coco::PeerStatus::Started);

            super::map_to_event(notification::Notification::LocalPeerStatusChanged(
                coco::PeerStatus::Stopped,
                coco::PeerStatus::Started,
            ))
            .unwrap();

            Ok(())
        }
    }
}
