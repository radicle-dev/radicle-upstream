//! Defines the schema served to the application via `GraphQL`.

/// Server infrastructure used to power the API.
pub mod api;
/// Mappings to juniper errors.
pub mod error;
/// Query and mutation surface of the `GraphQL` API.
pub mod schema;
