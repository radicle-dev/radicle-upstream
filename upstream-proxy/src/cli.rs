// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[derive(Debug, Clone, clap::Parser)]
#[allow(clippy::struct_excessive_bools)]
#[clap(disable_version_flag = true)]
pub struct Args {
    /// Put proxy in test mode to use certain fixtures
    #[clap(long)]
    pub test: bool,

    /// run HTTP API on a specified address:port
    #[clap(
        long,
        env = "RADICLE_PROXY_HTTP_LISTEN",
        default_value = "127.0.0.1:17246"
    )]
    pub http_listen: std::net::SocketAddr,

    /// Run the peer on a specified address:port
    #[clap(long, env = "RADICLE_PROXY_PEER_LISTEN", default_value = "0.0.0.0:0")]
    pub peer_listen: std::net::SocketAddr,

    /// Path to the secret key for the identity. Uses `LNK_HOME` if not provided.
    #[clap(long)]
    pub identity_key: Option<std::path::PathBuf>,

    /// Passphrase to unlock the keystore. If not provided the keystore must be unlocked via the
    /// HTTP API.
    #[clap(long, env = "RADICLE_PROXY_KEY_PASSPHRASE")]
    pub key_passphrase: Option<String>,

    /// Enables fast but unsafe encryption of the keystore for development builds
    #[clap(long)]
    pub unsafe_fast_keystore: bool,

    /// Enables more verbose logging for development
    #[clap(long)]
    pub dev_log: bool,

    /// URL of a seed to fetch identities via Git over HTTP. Can be specified multiple times.
    #[clap(
        long,
        env = "RADICLE_PROXY_GIT_SEEDS",
        long = "git-seed",
        use_value_delimiter = true
    )]
    pub git_seeds: Option<Vec<rad_common::Url>>,
}

impl Args {
    #[must_use]
    pub fn from_args() -> Self {
        <Self as clap::Parser>::parse()
    }
}
