// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;
use axum::extract::Path;

/// Provides the following endpoints:
/// * `GET /projects-v2/:urn/patches/:patch_id/events`
/// * `PUT /projects-v2/:urn/patches/:patch_id/events`
pub fn router() -> axum::Router {
    axum::Router::new().route(
        // TODO change the prefix to just `/projects` once we figure out how to make this
        // compatible with the old warp routes.
        "/projects-v2/:urn/events/:topic",
        axum::routing::get(get_event).put(publish_event),
    )
}

async fn get_event(
    Path((urn, topic)): Path<(librad::git::Urn, String)>,
    super::extract::UnsealedContext(ctx): super::extract::UnsealedContext,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    let events = ctx
        .event_log
        .get(urn.id, topic)
        .await
        .context("failed to get event log")?;

    Ok(axum::response::Json(events))
}

async fn publish_event(
    Path((urn, topic)): Path<(librad::git::Urn, String)>,
    event: axum::extract::Json<crate::events::Event>,
    super::extract::UnsealedContext(ctx): super::extract::UnsealedContext,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    ctx.event_log
        .publish(urn.id, &topic, event.0)
        .await
        .context("failed to publish event")?;
    Ok(http::StatusCode::CREATED)
}
