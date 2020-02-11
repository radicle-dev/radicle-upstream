/// Server infrastructure used to power the API.
pub mod api;
/// Mappings to juniper errors.
mod error;
/// Git objects as `GraphQL` objects.
pub mod git;
/// Radicle projects as `GraphQL` objects.
mod project;
/// Query and mutation surface of the `GraphQL` API.
mod schema;
