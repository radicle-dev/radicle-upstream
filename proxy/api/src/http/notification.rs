//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::{filters::BoxedFilter, Filter, Reply};

use crate::{context, http, notification::Subscriptions};

/// SSE based notifications endpoint.
pub fn filters(ctx: context::Context, subscriptions: Subscriptions) -> BoxedFilter<(impl Reply,)> {
    local_peer_status_stream(ctx, subscriptions.clone())
        .or(stream_filter(subscriptions))
        .boxed()
}

/// `GET /local_peer_status`
pub fn local_peer_status_stream(
    ctx: context::Context,
    subscriptions: Subscriptions,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(http::with_context(ctx))
        .and(warp::any().map(move || subscriptions.clone()))
        .and_then(handler::local_peer_status)
        .boxed()
}

/// `GET /`
pub fn stream_filter(subscriptions: Subscriptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::any().map(move || subscriptions.clone()))
        .and_then(handler::stream)
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
                    // let res: Result<impl sse::ServerSentEvent, Infallible> =
                    //     Ok((sse::event("LOCAL_PEER_STATUS_CHANGED"), sse::json(new)));
                    Some(map_to_event(notification))
                },
                _ => None,
            }
        };

        Ok(sse::reply(
            sse::keep_alive().stream(initial.chain(subscriber).filter_map(filter)),
        ))
    }

    /// Provides an SSE events endpoint providing a stream of
    /// [`crate::notification::Notification`]s.
    pub async fn stream(subscriptions: Subscriptions) -> Result<impl Reply, Rejection> {
        let subscriber = subscriptions.subscribe().await;

        Ok(sse::reply(sse::keep_alive().stream(map(subscriber))))
    }

    /// Maps the [`crate::notification::Notification`] from a subscription to an
    /// [`sse::ServerSideEvent`].
    fn map(
        subscriber: mpsc::UnboundedReceiver<Notification>,
    ) -> impl Stream<Item = Result<impl sse::ServerSentEvent, Infallible>> {
        subscriber.map(|notification| match notification {
            Notification::LocalPeerStatusChanged(_old, new) => {
                Ok((sse::event("LOCAL_PEER_STATUS_CHANGED"), sse::json(new)))
            },
        })
    }

    fn map_to_event(notification: Notification) -> Result<impl sse::ServerSentEvent, Infallible> {
        match notification {
            Notification::LocalPeerStatusChanged(_old, new) => {
                Ok((sse::event("LOCAL_PEER_STATUS_CHANGED"), sse::json(new)))
            },
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::test::request;

    use crate::{context, notification};

    /// This test blocks as we don't have a termination condition for the stream. We need to find
    /// a way to test this properly. Warp does have test utility for websockets but not for SSE
    /// streams.
    #[ignore]
    #[tokio::test]
    async fn stream() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(ctx, subscriptions);

        let res = request()
            .method("GET")
            .path("/")
            .header("Connection", "Keep-Alive")
            .reply(&api)
            .await
            .into_body();

        assert_eq!(
            res,
            r#"data:"foo"
"#
        );

        Ok(())
    }
}
