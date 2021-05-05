//! Proxy serving a specialized API to the Upstream UI.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_used,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
// TODO(xla): Handle all Results properly and never panic outside of main.
// TODO(xla): Remove exception for or_fun_call lint.
#![allow(
    clippy::clone_on_ref_ptr,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::missing_inline_in_public_items,
    clippy::multiple_crate_versions,
    clippy::or_fun_call,
    clippy::shadow_reuse,
    clippy::clippy::option_if_let_else,
    clippy::similar_names,
    clippy::large_types_passed_by_value
)]

mod browser;
mod config;
mod context;
mod control;
pub mod env;
mod error;
mod ethereum {
    pub mod address;
    pub mod claim_ext;
}
mod git_helper;
mod http;
mod identifier;
mod identity;
mod notification;
mod process;
mod project;
mod service;
mod session;

pub use process::{run, Args};
