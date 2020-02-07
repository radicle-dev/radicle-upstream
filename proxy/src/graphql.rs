/// Server infrastructure used to power the API.
pub mod api;
/// Error definitions and type casting logic.
mod error;
/// Git objects as `GraphQL` objects.
pub mod git;
/// Radicle projects as `GraphQL` objects.
mod project;
/// Logic for registering new projects.
mod registry;
/// Query and mutation surface of the `GraphQL` API.
mod schema;
