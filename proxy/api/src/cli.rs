// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

#[derive(Debug, Clone, clap::Parser)]
#[allow(clippy::struct_excessive_bools)]
#[clap(setting(clap::AppSettings::DisableVersionFlag))]
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

    /// Populate the seed configuration with these seeds when the app is started for the first
    /// time.
    #[clap(
        long,
        env = "RADICLE_PROXY_DEFAULT_SEEDS",
        long = "default-seed",
        use_delimiter = true
    )]
    pub default_seeds: Vec<String>,

    /// Path to the secret key for the identity. Uses `RAD_HOME` if not provided.
    #[clap(long)]
    pub identity_key: Option<std::path::PathBuf>,

    /// Connect to the given seeds and not to those previously set and stored through the API. Does
    /// not override the stored seeds.
    #[clap(long, env = "RADICLE_PROXY_SEEDS", long = "seed", use_delimiter = true)]
    pub seeds: Option<Vec<String>>,

    /// Don’t install the git-remote-rad binary
    #[clap(long)]
    pub skip_remote_helper_install: bool,

    /// Passphrase to unlock the keystore. If not provided the keystore must be unlocked via the
    /// HTTP API.
    #[clap(long, env = "RADICLE_PROXY_KEY_PASSPHRASE")]
    pub key_passphrase: Option<String>,

    /// Enables fast but unsafe encryption of the keystore for development builds
    #[clap(long)]
    pub unsafe_fast_keystore: bool,

    /// If `true`, the HTTP api will accept any request without checking the auth token.
    #[clap(long, env = "RADICLE_PROXY_INSECURE_HTTP_API")]
    pub insecure_http_api: bool,

    /// Enables more verbose logging for development
    #[clap(long)]
    pub dev_log: bool,
}

impl Args {
    #[must_use]
    pub fn from_args() -> Self {
        <Self as clap::Parser>::parse()
    }
}
