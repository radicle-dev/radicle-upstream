//! Proxy serving a specialized API to the Upstream UI.

#![deny(missing_docs, unused_import_braces, unused_qualifications, warnings)]
#![deny(
    clippy::all,
    // clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
)]
// TODO(xla): Handle all Results properly and never panic outside of main.
// TODO(xla): Remove exception for or_fun_call lint.
// TODO(xla): Remove let_underscore_must_use once the issue is resolved: https://github.com/rust-lang/rust-clippy/issues/4980
// TODO(xla): Remove used_underscore_binding.
#![allow(
    clippy::implicit_return,
    clippy::let_underscore_must_use,
    clippy::missing_inline_in_public_items,
    clippy::option_expect_used,
    clippy::or_fun_call,
    clippy::result_expect_used,
    clippy::shadow_reuse,
    clippy::unseparated_literal_suffix,
    clippy::used_underscore_binding
)]
#![feature(result_flattening)]

pub mod avatar;
pub mod coco;
pub mod env;
pub mod error;
pub mod http;
pub mod identity;
mod notification;
mod project;
pub mod registry;
pub mod session;
