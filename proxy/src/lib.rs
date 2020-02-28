//! Proxy to serve a specialised `GraphQL` API to radicle-upstream.

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
#![allow(
    clippy::implicit_return,
    clippy::let_underscore_must_use,
    clippy::missing_inline_in_public_items,
    clippy::option_expect_used,
    clippy::or_fun_call,
    clippy::result_expect_used,
    clippy::unseparated_literal_suffix
)]

#[macro_use]
extern crate juniper;

pub mod coco;
/// Utilities to manipulate the process environment.
pub mod env;
/// Error definitions and conversions.
pub mod error;
/// Defines the schema served to the application via `GraphQL`.
pub mod graphql;
/// Intergrations on the Regstriy.
mod registry;
