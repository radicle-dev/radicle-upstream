//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::notification::Subscriptions;

/// SSE based notifications endpoint.
pub fn filters(subscriptions: Subscriptions) -> BoxedFilter<(impl Reply,)> {
    stream_filter(subscriptions)
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

    use crate::notification::{Notification, Subscriptions};

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
            Notification::LocalPeerListening(addr) => {
                Ok((sse::event("LOCAL_PEER_LISTENING"), sse::json(addr)))
            },
        })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::test::request;

    use crate::error;
    use crate::notification;

    /// This test blocks as we don't have a termination condition for the stream. We need to find
    /// a way to test this properly. Warp does have test utility for websockets but not for SSE
    /// streams.
    #[ignore]
    #[tokio::test]
    async fn stream() -> Result<(), error::Error> {
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(subscriptions);

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
