// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[derive(Debug, Clone, structopt::StructOpt)]
#[allow(clippy::struct_excessive_bools)]
#[structopt(
    setting(structopt::clap::AppSettings::UnifiedHelpMessage),
    setting(structopt::clap::AppSettings::DisableVersion)
)]
pub struct Args {
    /// Put proxy in test mode to use certain fixtures
    #[structopt(long)]
    pub test: bool,

    /// run HTTP API on a specified address:port
    #[structopt(
        long,
        env = "RADICLE_PROXY_HTTP_LISTEN",
        default_value = "127.0.0.1:17246"
    )]
    pub http_listen: std::net::SocketAddr,

    /// Run the peer on a specified address:port
    #[structopt(long, env = "RADICLE_PROXY_PEER_LISTEN", default_value = "0.0.0.0:0")]
    pub peer_listen: std::net::SocketAddr,

    /// Add one or more default seed addresses to initialise the settings store
    #[structopt(long, env = "RADICLE_PROXY_DEFAULT_SEED", long = "default-seed")]
    pub default_seeds: Vec<String>,

    /// Don’t install the git-remote-rad binary
    #[structopt(long)]
    pub skip_remote_helper_install: bool,

    /// Passphrase to unlock the keystore. If not provided the keystore must be unlocked via the
    /// HTTP API.
    #[structopt(long)]
    pub key_passphrase: Option<String>,

    /// Enables fast but unsafe encryption of the keystore for development builds
    #[cfg(feature = "unsafe-fast-keystore")]
    #[structopt(long)]
    pub unsafe_fast_keystore: bool,

    /// Enables more verbose logging for development
    #[structopt(long)]
    pub dev_log: bool,
}

impl Args {
    #[must_use]
    pub fn from_args() -> Self {
        <Self as structopt::StructOpt>::from_args()
    }
}
