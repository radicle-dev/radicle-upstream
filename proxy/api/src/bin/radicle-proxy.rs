// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    api::env::set_if_unset("RUST_BACKTRACE", "full");
    api::env::set_if_unset("RUST_LOG", "info,quinn=warn");
    pretty_env_logger::init_timed();

    api::run(argh::from_env()).await
}
