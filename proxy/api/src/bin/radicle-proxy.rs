// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    api::env::set_if_unset("RUST_BACKTRACE", "full");
    api::env::set_if_unset("RUST_LOG", "info,quinn=warn");

    let builder = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    match std::env::var("TRACING_FMT").as_deref() {
        Ok("pretty") => builder.pretty().init(),
        Ok("compact") => builder.compact().init(),
        Ok("json") => builder.json().init(),
        _ => builder.init(),
    };

    api::run(argh::from_env()).await
}
