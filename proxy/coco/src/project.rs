//! Project creation data and functions.

use librad::{
    git::types::remote::Remote,
    git_ext::{is_not_found_err, OneLevel, Qualified},
    std_ext::result::ResultExt,
};
use radicle_surf::vcs::git::git2;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

pub mod peer;
pub use peer::Peer;

/// Set the upstream of the given branch to the given remote.
///
/// This writes to the `config` directly. The entry will look like the following:
///
/// ```text
/// [branch "main"]
///     remote = rad
///     merge = refs/heads/main
/// ```
fn set_upstream<Url>(
    repo: &git2::Repository,
    remote: &Remote<Url>,
    branch: OneLevel,
) -> Result<(), git2::Error> {
    let mut config = repo.config()?;
    let branch_remote = format!("branch.{}.remote", branch);
    let branch_merge = format!("branch.{}.merge", branch);
    config
        .remove_multivar(&branch_remote, ".*")
        .or_matches::<git2::Error, _, _>(is_not_found_err, || Ok(()))?;
    config
        .remove_multivar(&branch_merge, ".*")
        .or_matches::<git2::Error, _, _>(is_not_found_err, || Ok(()))?;
    config.set_multivar(&branch_remote, ".*", remote.name.as_str())?;
    config.set_multivar(&branch_merge, ".*", Qualified::from(branch).as_str())?;
    Ok(())
}
