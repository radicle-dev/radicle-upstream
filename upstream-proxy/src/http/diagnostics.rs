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
    pub async fn get(mut ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let listen_addrs = ctx.peer.daemon_control().listen_addrs().await;
        let protocol_config = ctx.peer.librad_peer().protocol_config();
        let membership = ctx.peer.librad_peer().membership().await;
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
                "listenAddresses": listen_addrs,
                "protocolConfig": {
                    "membership": {
                        "maxActive": protocol_config.membership.max_active,
                        "maxPassive": protocol_config.membership.max_passive,
                        "activeRandomWalkLength": protocol_config.membership.active_random_walk_length,
                        "passiveRandomWalkLength": protocol_config.membership.passive_random_walk_length,
                        "shuffleSampleSize": protocol_config.membership.shuffle_sample_size,
                        "shuffleInterval": protocol_config.membership.shuffle_interval,
                        "promoteInterval": protocol_config.membership.promote_interval
                    },
                    "network": protocol_config.network.to_string(),
                },
                "membership": {
                    "active": membership.active,
                    "passive": membership.passive
                }
            }
        });
        Ok(reply::json(&reply))
    }
}
