use structopt::{clap::AppSettings, StructOpt};

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

/// Upstream seed node.
#[derive(Debug, StructOpt)]
pub struct Args {
    /// Listen on the following address for peer messages.
    #[structopt(long, default_value = "0.0.0.0:8776")]
    pub listen: std::net::SocketAddr,

    /// Path to store radicle profile data including the monorepo.
    #[structopt(long)]
    pub rad_home: std::path::PathBuf,

    /// Path to the secret key for the identity. Uses `--rad-home` if not provided. Creates an
    /// identity if the file does not exist.
    #[structopt(long)]
    pub identity_key: Option<std::path::PathBuf>,

    /// List of bootstrap peers in the format `f00...@seed1.example.com:12345`. May be specified
    /// multiple times.
    #[structopt(long, parse(try_from_str = parse_bootstrap))]
    pub bootstrap: Option<Vec<(librad::PeerId, std::net::SocketAddr)>>,

    /// URNs of projects to replicate. May be specified multiple times or as a comma separated
    /// list.
    #[structopt(long, use_delimiter = true)]
    pub project: Vec<link_identities::git::Urn>,

    /// Output logs as JSON.
    #[structopt(long)]
    pub log_json: bool,
}

pub fn from_args() -> Args {
    let version = format!(
        "{}-{}.{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HEAD"),
        env!("PROFILE")
    );
    let app = Args::clap()
        .version(version.as_ref())
        .settings(&[AppSettings::UnifiedHelpMessage]);
    Args::from_clap(&app.get_matches())
}

fn parse_bootstrap(value: &str) -> Result<(librad::PeerId, std::net::SocketAddr), String> {
    use std::{net::ToSocketAddrs as _, str::FromStr as _};

    let parts = value.splitn(2, '@').collect::<Vec<_>>();
    let id = librad::PeerId::from_str(parts[0]).map_err(|e| e.to_string())?;
    let addr = parts[1]
        .to_socket_addrs()
        .map(|mut a| a.next())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Could not resolve peer address".to_owned())?;

    Ok((id, addr))
}
