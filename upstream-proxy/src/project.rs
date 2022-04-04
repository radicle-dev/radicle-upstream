// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Combine the domain `CoCo` domain specific understanding of a Project into a single
//! abstraction.

use std::{collections::HashSet, convert::TryFrom, ops::Deref};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use link_identities::{git::Urn, Person, Project as LinkProject};
use radicle_source::surf::vcs::git::Stats;

use crate::{browser, error, identity};

/// Object encapsulating project metadata.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
    /// List of delegates.
    pub delegates: HashSet<Urn>,
}

impl TryFrom<LinkProject> for Metadata {
    type Error = error::Error;

    #[allow(clippy::redundant_closure_for_method_calls)]
    fn try_from(project: LinkProject) -> Result<Self, Self::Error> {
        let subject = project.subject();
        // TODO(finto): Some delegates may be directly delegating, i.e. only supply their
        // PublicKey. Should we display these?
        let delegates = project
            .delegations()
            .iter()
            .indirect()
            .map(|indirect| indirect.urn())
            .collect();
        let default_branch = subject
            .default_branch
            .clone()
            .ok_or(error::Error::MissingDefaultBranch)?
            .to_string();

        Ok(Self {
            name: subject.name.to_string(),
            description: subject
                .description
                .clone()
                .map_or_else(|| "".into(), |desc| desc.to_string()),
            default_branch,
            delegates,
        })
    }
}

/// Radicle project for sharing and collaborating.
///
/// See [`Projects`] for a detailed breakdown of both kinds of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Unique identifier of the project in the network.
    pub urn: Urn,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// High-level statistics about the project
    pub stats: Stats,
}

/// Construct a Project from its metadata and stats
impl TryFrom<(LinkProject, Stats)> for Project {
    type Error = error::Error;

    /// Create a `Project` given a [`LinkProject`] and the [`Stats`]
    /// for the repository.
    fn try_from((project, stats): (LinkProject, Stats)) -> Result<Self, Self::Error> {
        let urn = project.urn();
        let metadata = Metadata::try_from(project)?;

        Ok(Self {
            urn,
            metadata,
            stats,
        })
    }
}

/// Codified relation in form of roles and availability of project views.
#[derive(Debug, Clone, Serialize)]
pub struct Peer(
    crate::daemon::project::peer::Peer<crate::daemon::project::peer::Status<identity::Identity>>,
);

impl Deref for Peer {
    type Target = crate::daemon::project::peer::Peer<
        crate::daemon::project::peer::Status<identity::Identity>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<crate::daemon::project::peer::Peer<crate::daemon::project::peer::Status<Person>>>
    for Peer
{
    fn from(
        peer: crate::daemon::project::peer::Peer<crate::daemon::project::peer::Status<Person>>,
    ) -> Self {
        let peer_id = peer.peer_id();
        Self(peer.map(|status| status.map(|user| (peer_id, user).into())))
    }
}

/// Partial failures that occur when getting the list of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Failure {
    /// We couldn't get a default branch for the project.
    DefaultBranch { urn: Urn, metadata: Metadata },
    /// We couldn't get the stats for the project.
    Stats { urn: Urn, metadata: Metadata },
    /// We couldn't get the signed refs of the project, and so we can't determine if it's tracked
    /// or contributed.
    SignedRefs(Project),
}

/// All projects contained in a user's monorepo.
#[derive(Serialize)]
pub struct Projects {
    /// A project that is tracked is one that the user has replicated onto their device but has not
    /// made any changes to. A project is still considered tracked if they checked out a working
    /// copy but have not performed any commits to the references.
    pub tracked: Vec<Project>,

    /// A project that has been *contributed* to is one that the user has either:
    ///     a. Created themselves using the application.
    ///     b. Has replicated (see tracked above), checked out a working copy, and pushed changes
    ///     to references.
    ///
    /// The conditions imply that a project is "contributed" if I am the delegate or I have
    /// contributed to the project.
    pub contributed: Vec<Project>,

    /// A project that failed partially when trying to retrieve metadata for it.
    pub failures: Vec<Failure>,
}

pub async fn list_link(
    peer: &crate::peer::Peer,
) -> anyhow::Result<Vec<Result<LinkProject, librad::git::identities::Error>>> {
    peer.librad_peer()
        .using_read_only(|storage| {
            lnk_identities::project::list(storage).map(|projects| projects.collect::<Vec<_>>())
        })
        .await
        .context("failed to open read-only storage")?
        .context("failed to list project identities")
}

impl Projects {
    /// List all the projects that are located on your device. These projects could either be
    /// "tracked" or "contributed".
    ///
    /// See [`Projects`] for a detailed breakdown of both kinds of projects.
    ///
    /// # Errors
    ///
    ///   * We couldn't get the list of projects
    ///   * We couldn't inspect the `signed_refs` of the project
    ///   * We couldn't get stats for a project
    pub async fn list(peer: &crate::peer::Peer) -> Result<Self, error::Error> {
        let mut projects = Self {
            tracked: vec![],
            contributed: vec![],
            failures: vec![],
        };
        let link_projects = list_link(peer).await?;
        for link_project_result in link_projects {
            let link_project = link_project_result.context("failed to load project")?;

            let urn = link_project.urn();
            let metadata = Metadata::try_from(link_project)?;
            let default_branch =
                match crate::daemon::state::find_default_branch(peer.librad_peer(), urn.clone())
                    .await
                {
                    Err(err) => {
                        tracing::warn!(project_urn = %urn, ?err, "cannot find default branch");
                        projects
                            .failures
                            .push(Failure::DefaultBranch { urn, metadata });
                        continue;
                    },
                    Ok(branch) => branch,
                };

            let stats =
                match browser::using(peer, default_branch, |browser| Ok(browser.get_stats()?)) {
                    Err(err) => {
                        tracing::warn!(project_urn = %urn, ?err, "cannot get project stats");
                        projects.failures.push(Failure::Stats { urn, metadata });
                        continue;
                    },
                    Ok(stats) => stats,
                };

            let project = Project {
                urn,
                metadata,
                stats,
            };

            let refs = match crate::daemon::state::load_refs(
                peer.librad_peer(),
                project.urn.clone(),
            )
            .await
            {
                Err(err) => {
                    tracing::warn!(project_urn = %project.urn, ?err, "cannot load refs");
                    projects.failures.push(Failure::SignedRefs(project));
                    continue;
                },
                Ok(refs) => refs,
            };

            match refs {
                None => projects.tracked.push(project),
                Some(refs) => {
                    if refs.heads().next().is_none() {
                        projects.tracked.push(project);
                    } else {
                        projects.contributed.push(project);
                    }
                },
            }
        }

        Ok(projects)
    }
}

/// Fetch the project with a given urn from a peer
///
/// # Errors
///
///   * Failed to get the project.
///   * Failed to get the stats of the project.
pub async fn get(peer: &crate::peer::Peer, project_urn: Urn) -> Result<Project, error::Error> {
    let project = crate::daemon::state::get_project(peer.librad_peer(), project_urn.clone())
        .await?
        .ok_or(crate::error::Error::ProjectNotFound)?;

    let branch =
        crate::daemon::state::find_default_branch(peer.librad_peer(), project_urn.clone()).await?;
    let project_stats = browser::using(peer, branch, |browser| Ok(browser.get_stats()?))?;

    Project::try_from((project, project_stats))
}

/// This lists all the projects for a given `user`. This `user` should not be your particular
/// `user` (i.e. the "default user"), but rather should be another user that you are tracking.
///
/// The resulting list of projects will be a subset of the projects that you track or contribute
/// to. This is because we can only know our projects (local-first) and the users that we track
/// for those projects.
///
/// TODO(finto): We would like to also differentiate whether these are tracked or contributed to
/// for this given user. See <https://github.com/radicle-dev/radicle-upstream/issues/915>
///
/// # Errors
///
/// * We couldn't get a project list.
/// * We couldn't get project stats.
/// * We couldn't determine the tracking peers of a project.
pub async fn list_for_user(
    peer: &crate::peer::Peer,
    user: &Urn,
) -> Result<Vec<Project>, error::Error> {
    let mut projects = vec![];

    for project in crate::daemon::state::list_projects(peer.librad_peer()).await? {
        let tracked = crate::daemon::state::tracked(peer.librad_peer(), project.urn())
            .await?
            .into_iter()
            .filter_map(crate::daemon::project::Peer::replicated_remote)
            .find(|(_, project_user)| project_user.urn() == *user);
        if let Some((peer_id, _)) = tracked {
            let subject = project.subject();
            let branch = crate::daemon::state::get_branch(
                peer.librad_peer(),
                project.urn(),
                peer_id,
                subject.default_branch.clone(),
            )
            .await?;
            let stats = browser::using(peer, branch, |browser| Ok(browser.get_stats()?))?;
            let full = Project::try_from((project, stats))?;

            projects.push(full);
        }
    }
    Ok(projects)
}
