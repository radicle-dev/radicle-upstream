// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Proxy serving a specialized API to the Upstream UI.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::unwrap_used,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

mod browser;
mod cli;
mod config;
mod context;
mod control;
mod daemon;
mod error;
mod git_fetch;
mod http_next;
mod peer;
mod semaphore_map;
mod shutdown_runner;
mod ethereum {
    pub mod address;
    pub mod claim_ext;
}
pub mod dev_cli;
mod http;
mod identifier;
mod identity;
mod keystore;
mod notification;
mod patch;
mod process;
mod project;
mod service;
mod session;
mod watch_monorepo;

pub use cli::Args;
pub use process::run;
