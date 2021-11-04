// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Provide diagnostics information.

use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all diagnostics filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    // I couldn't figure out how to get past the type checker without having chaining.
    get_filter(ctx.clone()).or(get_filter(ctx)).boxed()
}

/// `GET /diagnostics`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get)
}

/// Diagnostics handlers for conversion between core domain and http request fullfilment.
mod handler {
    use serde_json::json;
    use walkdir::WalkDir;
    use warp::{reply, Rejection, Reply};

    use crate::context;

    /// Get diagnostics information.
    #[allow(clippy::unused_async)]
    pub async fn get(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let membership = ctx.peer.membership().await;
        let git_dir = ctx.rest.paths.git_dir();
        let refs_tree = WalkDir::new(git_dir.join("refs"))
            .into_iter()
            .filter_map(std::result::Result::ok)
            .map(|e| {
                e.path()
                    .strip_prefix(git_dir)
                    .expect("Couldn't strip prefix.")
                    .display()
                    .to_string()
            })
            .collect::<Vec<String>>();
        let reply = json!({
            "storage": {
                "gitDirPath": git_dir,
                "refsTree": refs_tree,
            },
            "peer": {
                "membership": {
                    "active": membership.active,
                    "passive": membership.passive
                }
            }
        });
        Ok(reply::json(&reply))
    }
}
