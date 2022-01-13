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

    /// Populate the seed configuration with these seeds when the app is started for the first
    /// time.
    #[structopt(
        long,
        env = "RADICLE_PROXY_DEFAULT_SEEDS",
        long = "default-seed",
        use_delimiter = true
    )]
    pub default_seeds: Vec<String>,

    /// Path to the secret key for the identity. Uses `RAD_HOME` if not provided.
    #[structopt(long)]
    pub identity_key: Option<std::path::PathBuf>,

    /// Connect to the given seeds and not to those previously set and stored through the API. Does
    /// not override the stored seeds.
    #[structopt(long, env = "RADICLE_PROXY_SEEDS", long = "seed", use_delimiter = true)]
    pub seeds: Option<Vec<String>>,

    /// Don’t install the git-remote-rad binary
    #[structopt(long)]
    pub skip_remote_helper_install: bool,

    /// Passphrase to unlock the keystore. If not provided the keystore must be unlocked via the
    /// HTTP API.
    #[structopt(long, env = "RADICLE_PROXY_KEY_PASSPHRASE")]
    pub key_passphrase: Option<String>,

    /// Enables fast but unsafe encryption of the keystore for development builds
    #[structopt(long)]
    pub unsafe_fast_keystore: bool,

    /// If `true`, the HTTP api will accept any request without checking the auth token.
    #[structopt(long, env = "RADICLE_PROXY_INSECURE_HTTP_API")]
    pub insecure_http_api: bool,

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
