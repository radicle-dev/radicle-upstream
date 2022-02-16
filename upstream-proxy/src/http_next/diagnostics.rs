// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

/// Provides `GET /diagnostics` endpoint that returns diagnostics data.
pub fn router() -> axum::Router {
    axum::Router::new().route("/diagnostics", axum::routing::get(get))
}

async fn get(
    super::ExtractUnsealedContext(mut ctx): super::ExtractUnsealedContext,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    let listen_addrs = ctx.peer.daemon_control().listen_addrs().await;
    let protocol_config = ctx.peer.librad_peer().protocol_config();
    let membership = ctx.peer.librad_peer().membership().await;
    let git_dir = ctx.rest.paths.git_dir();
    let refs_tree = walkdir::WalkDir::new(git_dir.join("refs"))
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
    let reply = serde_json::json!({
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
    Ok(axum::response::Json(reply))
}
