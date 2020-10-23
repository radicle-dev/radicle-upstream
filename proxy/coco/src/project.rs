//! Project creation data and functions.

use librad::git::ext::OneLevel;
use radicle_surf::vcs::git::git2;

use crate::config;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

/// The `peer` module models the data representing a peer's relation to a project.
///
/// A [`Peer`] can be `Local` or `Remote`, it can be `NotReplicated` or `Replicated`, and it can be
/// a `Tracker`, `Contributor`, or `Maintainer` of the project.
pub mod peer;
pub use peer::{Peer, Replicated, Role, Status as ReplicationStatus};

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &OneLevel) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch.as_str(), git2::BranchType::Local)?;
    branch.set_upstream(Some(&format!(
        "{}/{}",
        config::RAD_REMOTE,
        default_branch.as_str()
    )))
}
