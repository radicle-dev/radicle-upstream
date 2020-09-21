//! Project creation data and functions.

use librad::git::include;
use radicle_surf::vcs::git::git2;

use crate::config;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

fn set_include_path<P>(
    repo: &git2::Repository,
    include: &include::Include<P>,
) -> Result<(), git2::Error>
where
    P: AsRef<std::path::Path>,
{
    let mut config = repo.config()?;

    config.set_str("include.path", &include.file_path().display().to_string())
}

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &str) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch, git2::BranchType::Local)?;
    branch.set_upstream(Some(&format!("{}/{}", config::RAD_REMOTE, default_branch)))
}
