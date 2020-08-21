//! Project creation data and functions.

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

use radicle_surf::vcs::git::git2;

use crate::config;

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &str) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch, git2::BranchType::Local)?;
    branch.set_upstream(Some(&format!("{}/{}", config::RAD_REMOTE, default_branch)))
}
