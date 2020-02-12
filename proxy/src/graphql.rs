/// Server infrastructure used to power the API.
pub mod api;
/// Mappings to juniper errors.
mod error;
/// Radicle projects as `GraphQL` objects.
mod project;
/// Query and mutation surface of the `GraphQL` API.
pub mod schema;
