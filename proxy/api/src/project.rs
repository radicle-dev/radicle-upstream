//! Combine the domain `CoCo` domain specific understanding of a Project into a single
//! abstraction.

use std::{collections::HashSet, convert::TryFrom, ops::Deref};

use serde::{Deserialize, Serialize};

use coco::{project::peer, signer::BoxedSigner};

use crate::{error, identity};

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
    /// List of maintainers.
    pub maintainers: HashSet<coco::Urn>,
}

impl TryFrom<coco::Project> for Metadata {
    type Error = error::Error;

    #[allow(clippy::redundant_closure_for_method_calls)]
    fn try_from(project: coco::Project) -> Result<Self, Self::Error> {
        let subject = project.subject();
        // TODO(finto): Some maintainers may be directly delegating, i.e. only supply their
        // PublicKey. Should we display these?
        let maintainers = project
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
            maintainers,
        })
    }
}

/// Radicle project for sharing and collaborating.
///
/// See [`Projects`] for a detailed breakdown of both kinds of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project<S> {
    /// Unique identifier of the project in the network.
    pub urn: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// High-level statistics about the project
    pub stats: S,
}

/// A `Partial` project is one where we _weren't_ able to fetch the [`coco::Stats`] for it.
pub type Partial = Project<()>;

/// A `Full` project is one where we _were_ able to fetch the [`coco::Stats`] for it.
pub type Full = Project<coco::Stats>;

impl Partial {
    /// Convert a `Partial` project into a `Full` one by providing the `stats` for the project.
    #[allow(clippy::missing_const_for_fn)]
    pub fn fulfill(self, stats: coco::Stats) -> Full {
        Project {
            urn: self.urn,
            shareable_entity_identifier: self.shareable_entity_identifier,
            metadata: self.metadata,
            stats,
        }
    }
}

/// Construct a Project from its metadata and stats
impl TryFrom<coco::Project> for Partial {
    type Error = error::Error;

    /// Create a `Project` given a [`coco::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn try_from(project: coco::Project) -> Result<Self, Self::Error> {
        let urn = project.urn();
        let metadata = Metadata::try_from(project)?;

        Ok(Self {
            urn: urn.clone(),
            shareable_entity_identifier: format!("%{}", urn),
            metadata,
            stats: (),
        })
    }
}

/// Construct a Project from its metadata and stats
impl TryFrom<(coco::Project, coco::Stats)> for Full {
    type Error = error::Error;

    /// Create a `Project` given a [`coco::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn try_from((project, stats): (coco::Project, coco::Stats)) -> Result<Self, Self::Error> {
        let urn = project.urn();
        let metadata = Metadata::try_from(project)?;

        Ok(Self {
            urn: urn.clone(),
            shareable_entity_identifier: format!("%{}", urn),
            metadata,
            stats,
        })
    }
}

/// Codified relation in form of roles and availability of project views.
#[derive(Serialize)]
pub struct Peer(peer::Peer<peer::Status<identity::Identity>>);

impl Deref for Peer {
    type Target = peer::Peer<peer::Status<identity::Identity>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<peer::Peer<peer::Status<coco::Person>>> for Peer {
    fn from(peer: peer::Peer<peer::Status<coco::Person>>) -> Self {
        let peer_id = peer.peer_id();
        Self(peer.map(|status| status.map(|user| (peer_id, user).into())))
    }
}

/// A Radicle project that you're interested in but haven't contributed to.
///
/// See [`Projects`] for a detailed breakdown of both kinds of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracked(Full);

impl Deref for Tracked {
    type Target = Full;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Partial failures that occur when getting the list of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Failure {
    /// We couldn't get a default branch for the project.
    DefaultBranch(Partial),
    /// We couldn't get the stats for the project.
    Stats(Partial),
    /// We couldn't get the signed refs of the project, and so we can't determine if it's tracked
    /// or contributed.
    SignedRefs(Full),
}

/// All projects contained in a user's monorepo.
#[derive(Serialize)]
pub struct Projects {
    /// A project that is tracked is one that the user has replicated onto their device but has not
    /// made any changes to. A project is still considered tracked if they checked out a working
    /// copy but have not performed any commits to the references.
    pub tracked: Vec<Tracked>,

    /// A project that has been *contributed* to is one that the user has either:
    ///     a. Created themselves using the application.
    ///     b. Has replicated (see tracked above), checked out a working copy, and pushed changes
    ///     to references.
    ///
    /// The conditions imply that a project is "contributed" if I am the maintainer or I have
    /// contributed to the project.
    pub contributed: Vec<Full>,

    /// A project that failed partially when trying to retrieve metadata for it.
    pub failures: Vec<Failure>,
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
    pub async fn list(peer: &coco::net::peer::Peer<BoxedSigner>) -> Result<Self, error::Error> {
        let mut projects = Self {
            tracked: vec![],
            contributed: vec![],
            failures: vec![],
        };

        for project in coco::state::list_projects(peer).await? {
            let project = Project::try_from(project)?;
            let default_branch =
                match coco::state::find_default_branch(peer, project.urn.clone()).await {
                    Err(err) => {
                        log::warn!("Failure for '{}': {}", project.urn, err);
                        projects.failures.push(Failure::DefaultBranch(project));
                        continue;
                    },
                    Ok(branch) => branch,
                };

            let stats = match coco::state::with_browser(peer, default_branch, |browser| {
                Ok(browser.get_stats()?)
            })
            .await
            {
                Err(err) => {
                    log::warn!("Failure for '{}': {}", project.urn, err);
                    projects.failures.push(Failure::Stats(project));
                    continue;
                },
                Ok(stats) => stats,
            };

            let project = project.fulfill(stats);

            let refs = match coco::state::list_owner_project_refs(peer, project.urn.clone()).await {
                Err(err) => {
                    log::warn!("Failure for '{}': {}", project.urn, err);
                    projects.failures.push(Failure::SignedRefs(project));
                    continue;
                },
                Ok(refs) => refs,
            };

            match refs {
                None => projects.tracked.push(Tracked(project)),
                Some(refs) => {
                    if refs.heads.is_empty() {
                        projects.tracked.push(Tracked(project))
                    } else {
                        projects.contributed.push(project)
                    }
                },
            }
        }

        Ok(projects)
    }
}

/// An iterator over [`Projects`] that first yields contributed projects and then tracked projects.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    /// Iterator over contributed projects.
    contributed: std::slice::Iter<'a, Full>,

    /// Iterator over tracked projects.
    tracked: std::slice::Iter<'a, Tracked>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Full;

    fn next(&mut self) -> Option<Self::Item> {
        self.contributed
            .next()
            .or_else(|| match self.tracked.next() {
                Some(tracked) => Some(&tracked.0),
                None => None,
            })
    }
}

impl IntoIterator for Projects {
    type Item = Full;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            contributed: self.contributed.into_iter(),
            tracked: self.tracked.into_iter(),
        }
    }
}

/// An iterator over [`Projects`] that moves the values into the iterator.
/// It first yields contributed projects and then tracked projects.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoIter {
    /// Iterator over contributed projects.
    contributed: std::vec::IntoIter<Full>,

    /// Iterator over tracked projects.
    tracked: std::vec::IntoIter<Tracked>,
}

impl Iterator for IntoIter {
    type Item = Full;

    fn next(&mut self) -> Option<Self::Item> {
        self.contributed
            .next()
            .or_else(|| match self.tracked.next() {
                Some(tracked) => Some(tracked.0),
                None => None,
            })
    }
}

/// Fetch the project with a given urn from a peer
///
/// # Errors
///
///   * Failed to get the project.
///   * Failed to get the stats of the project.
pub async fn get(
    peer: &coco::net::peer::Peer<BoxedSigner>,
    project_urn: coco::Urn,
) -> Result<Full, error::Error> {
    let project = coco::state::get_project(peer, project_urn.clone())
        .await?
        .ok_or(crate::error::Error::ProjectNotFound)?;

    let branch = coco::state::find_default_branch(peer, project_urn.clone()).await?;
    let project_stats =
        coco::state::with_browser(peer, branch, |browser| Ok(browser.get_stats()?)).await?;

    Full::try_from((project, project_stats))
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
    peer: &coco::net::peer::Peer<BoxedSigner>,
    user: &coco::Urn,
) -> Result<Vec<Full>, error::Error> {
    let mut projects = vec![];

    for project in coco::state::list_projects(peer).await? {
        let tracked = coco::state::tracked(peer, project.urn())
            .await?
            .into_iter()
            .filter_map(coco::project::Peer::replicated_remote)
            .find(|(_, project_user)| project_user.urn() == *user);
        if let Some((peer_id, _)) = tracked {
            let subject = project.subject();
            let branch = coco::state::get_branch(
                peer,
                project.urn(),
                peer_id,
                subject.default_branch.to_owned(),
            )
            .await?;
            let stats =
                coco::state::with_browser(peer, branch, |browser| Ok(browser.get_stats()?)).await?;
            let full = Full::try_from((project, stats))?;

            projects.push(full);
        }
    }
    Ok(projects)
}
