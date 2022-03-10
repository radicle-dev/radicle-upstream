// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Proxy serving a specialized API to the Upstream UI.

#![warn(
    clippy::all,
    clippy::cargo,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::multiple_crate_versions)]

mod browser;
mod cli;
mod config;
mod context;
mod control;
mod daemon;
pub mod env;
mod error;
mod peer;
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

pub use cli::Args;
pub use process::run;
