// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Unidirectional stream of events happening in the proxy. This enables exposing tailing logs to
//! users, or widgets which show topology information like how many and what peers are connected.

use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::{context, http};

/// SSE based notifications endpoint.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    local_peer_status_stream(ctx)
}

/// `GET /local_peer_events`
pub fn local_peer_status_stream(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    path!("local_peer_events")
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::local_peer_events)
        .boxed()
}

/// Notification handlers to serve event streams.
mod handler {
    use futures::prelude::*;
    use warp::{sse, Rejection, Reply};

    use crate::{context, notification::Notification};

    /// Sets up local peer events notification stream.
    pub async fn local_peer_events(mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let current_status = ctx.peer.daemon_control().current_status().await;

        let initial = futures::stream::iter([Notification::StatusChanged {
            old: current_status.clone(),
            new: current_status,
        }]);

        let peer_notifications = ctx
            .peer_events()
            .filter_map(|event| future::ready(crate::notification::from_peer_event(event)));

        let git_fetch_notifications =
            ctx.git_fetch
                .updates()
                .map(|id| Notification::ProjectUpdated {
                    urn: link_identities::Urn::new(id),
                });

        let monorepo_local_update_notifications = ctx
            .watch_monorepo
            .updates()
            .map(|id| Notification::ProjectUpdated { urn: id });

        let notifications = futures::stream::select_all(vec![
            peer_notifications.boxed(),
            git_fetch_notifications.boxed(),
            monorepo_local_update_notifications.boxed(),
        ]);

        Ok(sse::reply(
            sse::keep_alive().stream(
                initial
                    .chain(notifications)
                    .map(|event| sse::Event::default().json_data(event)),
            ),
        ))
    }
}
