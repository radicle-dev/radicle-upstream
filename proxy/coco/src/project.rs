//! Project creation data and functions.

use serde::Serialize;

use librad::peer::PeerId;
use radicle_surf::vcs::git::git2;

use crate::config;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

/// Set the upstream of the default branch to the rad remote branch.
fn set_rad_upstream(repo: &git2::Repository, default_branch: &str) -> Result<(), git2::Error> {
    let mut branch = repo.find_branch(default_branch, git2::BranchType::Local)?;
    branch.set_upstream(Some(&format!("{}/{}", config::RAD_REMOTE, default_branch)))
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Contributer,
    Maintainer,
    Tracker,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Peer<U> {
    #[serde(rename_all = "camelCase")]
    Local {
        peer_id: PeerId,
        status: ReplicationStatus<U>,
    },
    #[serde(rename_all = "camelCase")]
    Remote {
        peer_id: PeerId,
        status: ReplicationStatus<U>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ReplicationStatus<U> {
    NotReplicated,
    #[serde(rename_all = "camelCase")]
    Replicated {
        role: Role,
        user: U,
    },
}
