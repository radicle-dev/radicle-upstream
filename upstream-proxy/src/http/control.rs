// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints to manipulate app state in test mode.

use warp::{filters::BoxedFilter, path, Filter, Reply};

use crate::context;

/// GET /reset
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    path!("reset")
        .and(warp::get())
        .and(super::with_context(ctx))
        .and_then(handler::reset)
        .boxed()
}

/// Control handlers for conversion between core domain and http request fulfilment.
mod handler {
    use warp::{reply, Rejection, Reply};

    use crate::context;

    /// Abort the server task, which causes `main` to restart it.
    pub async fn reset(mut ctx: context::Context) -> Result<impl Reply, Rejection> {
        tracing::info!("reload requested");
        ctx.service_handle().reset();
        Ok(reply::json(&()))
    }
}
