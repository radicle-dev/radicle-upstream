//! Unidirectional updates about significant state changes.

use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::notification::{Notification, Subscriptions};

/// SSE based notification endpoints.
pub fn filters(
    subscriptions: Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    test_html_filter()
        .or(test_js_filter())
        .or(stream_filter(subscriptions))
}

/// `GET /notifications`
fn stream_filter(
    subscriptions: Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("notifications")
        .and(warp::get())
        .and(document::document(document::description(
            "SSE stream of incoming notifications",
        )))
        .and(document::document(document::tag("Notification")))
        .and(document::document(
            document::response(
                200,
                document::body(Notification::document()).mime("text/event-stream"),
            )
            .description("Creation succeeded"),
        ))
        .and(super::with_subscriptions(subscriptions))
        .and_then(handler::stream)
}

/// `GET /notifications/test/index.html`
fn test_html_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("notifications" / "test" / "index.html")
        .and(warp::get())
        .and(document::document(document::description("Test HTML")))
        .and(document::document(document::tag("Notification")))
        .map(|| {
            warp::http::Response::builder()
                .header("content-type", "text/html; charset=utf-8")
                .body(TEST_HTML)
        })
}

/// `GET /notifications/test/index.js`
fn test_js_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("notifications" / "test" / "index.js")
        .and(warp::get())
        .and(document::document(document::description("Test JS")))
        .and(document::document(document::tag("Notification")))
        .map(|| {
            warp::http::Response::builder()
                .header("content-type", "text/javascript; charset=utf-8")
                .body(TEST_JS)
        })
}

/// Notification handlers to serve SSE based stream of updates.
mod handler {
    use futures::{Stream, StreamExt};
    use std::convert::Infallible;
    use tokio::sync::mpsc;
    use warp::sse::{event, json, ServerSentEvent};
    use warp::{Rejection, Reply};

    use crate::notification::{Notification, Subscriptions};

    /// Main handler to set up a stream of notifications.
    pub async fn stream(subscriptions: Subscriptions) -> Result<impl Reply, Rejection> {
        let subscriber = subscriptions.subscribe().await;

        Ok(warp::sse::reply(
            warp::sse::keep_alive().stream(map(subscriber)),
        ))
    }

    /// Maps incoming notifications to SSE replies.
    fn map(
        subscriber: mpsc::UnboundedReceiver<Notification>,
    ) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
        subscriber.map(|notification| match notification {
            Notification::Transaction(tx) => Ok((event("TRANSACTION"), json(tx))),
        })
    }
}

// TODO(xla): Document x-events properly.
impl ToDocumentedType for Notification {
    fn document() -> document::DocumentedType {
        document::string()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use warp::test::request;

    // TODO(xla): Implement a way to meaningfully test SSE streams.
    #[ignore]
    #[tokio::test]
    async fn stream() {
        let subs = super::Subscriptions::default();
        let api = super::filters(subs);

        let res = request()
            .method("GET")
            .path("/notifications")
            .header("Connection", "Keep-Alive")
            .reply(&api)
            .await
            .into_body();

        assert_eq!(
            res,
            r#"data:"foo"

"#
        );
    }
}

/// index.html for a simple test of the notifications.
const TEST_HTML: &str = r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Notification Test</title>
  </head>
  <body>
    <h1>notification test</h1>
    <button type="button" id="send">Register</button>
    <div id="notifications">
      <p><em>Connecting...</em></p>
    </div>
    <script
      defer
      src="/v1/notifications/test/index.js"
      type="application/javascript"
    ></script>
  </body>
</html>
"#;
/// index.js payload for a simple test of the notifications.
const TEST_JS: &str = r#"
const uri = `http://${location.host}/v1/notifications/`;

const notificationsEl = document.getElementById("notifications");
const registerButtonEl = document.getElementById("send");

registerButtonEl.onclick = function() {
  post("/v1/projects/register", {
    org_id: "radicle",
    project_name: "upstream"
  }).then(data => console.log(data));
};

const sse = new EventSource(uri);
sse.onopen = function() {
  console.log("open", notificationsEl);
  notificationsEl.innerHTML = "<p><em>Connected!</em></p>";
};
sse.onmessage = function(msg) {
  console.log("received messag", msg);
  notification(msg.data);
};
sse.addEventListener("TRANSACTION", tx => {
  console.log("received transaction", JSON.parse(tx.data));
  notification(tx.data);
});

function notification(data) {
  const line = document.createElement("p");
  line.innerText = JSON.stringify(data, 4);
  notificationsEl.appendChild(line);
}

async function post(url = "", data = {}) {
  const response = await fetch(url, {
    method: "POST",
    cache: "no-cache",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(data)
  });

  return await response.json();
}
"#;
