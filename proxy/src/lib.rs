//! Proxy serving a specialized API to the Upstream UI.

#![warn(
    missing_docs,
    unused_import_braces,
    unused_qualifications,
    clippy::all,
    // clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::unwrap_used,
)]
// TODO(xla): Handle all Results properly and never panic outside of main.
// TODO(xla): Remove exception for or_fun_call lint.
// TODO(xla): Remove let_underscore_must_use once the issue is resolved: https://github.com/rust-lang/rust-clippy/issues/4980
// TODO(xla): Remove used_underscore_binding.
#![allow(
    clippy::clone_on_ref_ptr,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::let_underscore_must_use,
    clippy::missing_inline_in_public_items,
    clippy::or_fun_call,
    clippy::shadow_reuse,
    clippy::unseparated_literal_suffix,
    clippy::used_underscore_binding
)]
#![feature(result_flattening)]

pub mod avatar;
pub mod coco;
pub mod config;
pub mod env;
pub mod error;
pub mod http;
pub mod identity;
mod notification;
pub mod project;
pub mod registry;
pub mod seed;
pub mod session;
