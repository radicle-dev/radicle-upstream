//! Project creation data and functions.

use librad::git_ext::{OneLevel, Qualified, is_not_found_err};
use librad::std_ext::result::ResultExt;
use librad::git::types::remote::Remote;
use radicle_surf::vcs::git::git2;

use crate::config;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

pub mod peer;
pub use peer::Peer;

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &OneLevel) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch.as_str(), git2::BranchType::Local)?;
    log::debug!("SETTING UPSTREAM: {}", repo.path().display());
    // std::thread::sleep(std::time::Duration::from_secs(60));
    branch.set_upstream(Some(&format!(
        "{}/{}",
        config::RAD_REMOTE,
        default_branch.as_str()
    )))
}

/// [branch "main"]
/// remote = rad
/// merge = refs/heads/main
fn set_upstream<Url>(repo: &git2::Repository, remote: &Remote<Url>, branch: OneLevel) -> Result<(), git2::Error> {
    let mut config = repo.config()?;
    let branch_remote = format!("branch.{}.remote", branch);
    let branch_merge = format!("branch.{}.merge", branch);
    config.remove_multivar(&branch_remote, ".*").or_matches::<git2::Error, _, _>(is_not_found_err, || Ok(()))?;
    config.remove_multivar(&branch_merge, ".*").or_matches::<git2::Error, _, _>(is_not_found_err, || Ok(()))?;
    config.set_multivar(&branch_remote, ".*", remote.name.as_str())?;
    config.set_multivar(&branch_merge, ".*", Qualified::from(branch).as_str())?;
    Ok(())
}
