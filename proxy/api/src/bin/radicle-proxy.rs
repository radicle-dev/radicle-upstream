// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    api::run(api::Args::from_args()).await
}
