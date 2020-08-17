//! Project creation data and functions.

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

/// The default name for a user's remote, which is `"rad"`.
const RAD_REMOTE: &str = "rad";
