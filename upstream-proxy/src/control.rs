// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Utility for fixture data in the monorepo.

use std::{env, io, path, str::FromStr};

use radicle_data::NonEmptyVec;

use radicle_source::surf::vcs::git::git2;

use crate::daemon::{
    librad::{
        git::{
            identities::local::LocalIdentity,
            local::{transport, url::LocalUrl},
            types::{
                remote::{LocalPushspec, Remote},
                Force, Pushspec, Refspec,
            },
        },
        git_ext::OneLevel,
        identities::Project,
        refspec_pattern,
    },
    project,
    state::{self, Error},
};

#[cfg(test)]
pub use test::*;

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`Error`] if any of the git interaction fail, or the initialisation of
/// the coco project.
pub async fn replicate_platinum(
    peer: &crate::peer::Peer,
    owner: &LocalIdentity,
    name: &str,
    description: &str,
    default_branch: OneLevel,
) -> Result<Project, Error> {
    let peer = peer.librad_peer();
    // Construct path for fixtures to clone into.
    let monorepo = state::monorepo(peer);
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_into)?;

    let project_creation = project::Create {
        description: description.to_string(),
        default_branch,
        repo: project::Repo::Existing {
            path: platinum_into.clone(),
        },
    };

    let meta = state::init_project(peer, owner, project_creation).await?;

    // Push branches and tags.
    {
        let repo = git2::Repository::open(platinum_into)?;
        let mut rad = Remote::rad_remote(
            LocalUrl::from(meta.urn()),
            Refspec {
                src: refspec_pattern!("refs/tags/*"),
                dst: refspec_pattern!("refs/tags/*"),
                force: Force::False,
            },
        );
        let storage = state::settings(peer);
        // Push all tags to rad remote.
        push_tags(&mut rad, storage, &repo)?;
    }

    // Init as rad project.
    Ok(meta)
}

/// Push any tags that are in the `repo` to the monorepo storage.
///
/// # Errors
///   * If we could not retrive the tag names from the repository.
pub fn push_tags(
    remote: &mut Remote<LocalUrl>,
    storage: transport::Settings,
    repo: &git2::Repository,
) -> Result<(), Error> {
    let tags = repo
        .tag_names(None)?
        .into_iter()
        .flatten()
        .filter_map(|tag| Pushspec::from_str(&format!("+refs/tags/{}:refs/tags/{}", tag, tag)).ok())
        .collect::<Vec<_>>();
    let tags = NonEmptyVec::from_vec(tags);

    match tags {
        None => {
            tracing::debug!("No tags to push to remote");
            Ok(())
        },
        Some(tags) => {
            let _refs = remote.push(storage, repo, LocalPushspec::Specs(tags));
            Ok(())
        },
    }
}

/// Return the canonicalized path to the `git-platinum` fixtures repo.
///
/// # Errors
///
///   * The path could not be canonicalized. This happens if the path does not exist.
fn platinum_directory() -> io::Result<path::PathBuf> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    path::Path::new(cargo_manifest_dir)
        .join("../fixtures/git-platinum")
        .canonicalize()
}

/// This function exists as a standalone because the logic does not play well with async in
/// `replicate_platinum`.
///
/// # Errors
///
///   * Cloning the repository failed
///   * We could not fetch branches
///
/// # Panics
///
///   * The platinum directory path was malformed
///   * Getting the branches fails
pub fn clone_platinum(platinum_into: impl AsRef<path::Path>) -> Result<(), Error> {
    let platinum_dir = platinum_directory().expect("failed to get platinum directory");
    let platinum_from = format!(
        "file://{}",
        platinum_dir
            .to_str()
            .expect("failed to get platinum directory")
    );
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(&platinum_from, platinum_into.as_ref())?;

    {
        let branches = platinum_repo.branches(Some(git2::BranchType::Remote))?;

        for branch in branches {
            let (branch, _branch_type) = branch?;
            let name = &branch
                .name()
                .expect("unable to get branch name")
                .expect("branch not present")
                .strip_prefix("origin/")
                .expect("unable to extract branch name");

            if *name != "master" && *name != "HEAD" {
                let oid = branch.get().target().expect("can't find OID");
                let commit = platinum_repo.find_commit(oid)?;
                platinum_repo.branch(name, &commit, false)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::daemon::{
        librad::{
            git::identities::local::LocalIdentity, git_ext::OneLevel, identities::Project, reflike,
            PeerId,
        },
        state::Error,
    };

    /// Generate a fresh `PeerId` for use in tests.
    #[must_use]
    pub fn generate_peer_id() -> PeerId {
        PeerId::from(link_crypto::SecretKey::new())
    }

    /// **Testing Only**
    ///
    /// Default reference name for testing purposes.
    #[must_use]
    pub fn default_branch() -> OneLevel {
        OneLevel::from(reflike!("master"))
    }

    /// Creates a small set of projects in your peer.
    ///
    /// # Errors
    ///
    /// Will error if filesystem access is not granted or broken for the configured
    /// [`librad::paths::Paths`].
    pub async fn setup_fixtures(
        peer: &crate::peer::Peer,
        owner: &LocalIdentity,
    ) -> Result<Vec<Project>, Error> {
        let infos = vec![
            (
                "monokel",
                "A looking glass into the future",
                default_branch(),
            ),
            (
                "Monadic",
                "Open source organization of amazing things.",
                default_branch(),
            ),
            (
                "open source coin",
                "Research for the sustainability of the open source community.",
                default_branch(),
            ),
            (
                "radicle",
                "Decentralized open source collaboration",
                default_branch(),
            ),
        ];

        let mut projects = Vec::with_capacity(infos.len());
        for info in infos {
            projects.push(super::replicate_platinum(peer, owner, info.0, info.1, info.2).await?);
        }
        Ok(projects)
    }
}
