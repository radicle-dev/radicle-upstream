//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use std::collections::HashSet;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::error;

/// Object the API returns for project metadata.
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

impl<ST> From<coco::Project<ST>> for Metadata
where
    ST: Clone,
{
    fn from(project_meta: coco::Project<ST>) -> Self {
        Self {
            name: project_meta.name().to_string(),
            description: project_meta
                .description()
                .clone()
                .unwrap_or_else(|| "".into()),
            default_branch: project_meta.default_branch().to_string(),
            maintainers: project_meta.maintainers().clone(),
        }
    }
}

/// Radicle project for sharing and collaborating.
///
/// See [`Projects`] for a detailed breakdown of both kinds of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// High-level statistics about the project
    pub stats: coco::Stats,
}

/// Construct a Project from its metadata and stats
impl<ST> From<(coco::Project<ST>, coco::Stats)> for Project
where
    ST: Clone,
{
    /// Create a `Project` given a [`coco::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn from((project, stats): (coco::Project<ST>, coco::Stats)) -> Self {
        let id = project.urn();

        Self {
            id: id.clone(),
            shareable_entity_identifier: format!("%{}", id),
            metadata: project.into(),
            stats,
        }
    }
}

/// A Radicle project that you're interested in but haven't contributed to.
///
/// See [`Projects`] for a detailed breakdown of both kinds of projects.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracked(Project);

impl Deref for Tracked {
    type Target = Project;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
    /// to     references.
    ///
    /// The conditions imply that a project is "contributed" if I am the maintainer or I have
    /// contributed to the project.
    pub mine: Vec<Project>,
}

impl Projects {
    /// List all the projects that are located on your device. These projects could either be
    /// "tracked" or "mine".
    ///
    /// See [`Projects`] for a detailed breakdown of both kinds of projects.
    ///
    /// # Errors
    ///
    ///   * We couldn't get the list of projects
    ///   * We couldn't inspect the `signed_refs` of the project
    ///   * We couldn't get stats for a project
    pub fn list(api: &coco::Api) -> Result<Self, error::Error> {
        let mut projects = Self {
            tracked: vec![],
            mine: vec![],
        };
        for project in api.list_projects()? {
            let refs = api.list_project_refs(&project.urn())?;
            let project = api.with_browser(&project.urn(), |browser| {
                let stats = browser.get_stats().map_err(coco::Error::from)?;
                Ok((project, stats).into())
            })?;
            if refs.heads.is_empty() {
                projects.tracked.push(Tracked(project))
            } else {
                projects.mine.push(project)
            }
        }

        Ok(projects)
    }
}

impl IntoIterator for Projects {
    type Item = Project;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    // Give back an iterator over all the [`Project`]s.
    fn into_iter(self) -> Self::IntoIter {
        // TODO(sos): chain two iterators together here
        //
        // e.g.
        //
        // let mine = self.mine.into_iter();
        // let tracked = self.tracked.into_iter().map(|tracked| tracked.0);
        // mine.chain(tracked)

        self.mine.into_iter()
    }
}

/// Fetch the project with a given urn from a peer
///
/// # Errors
///
///   * Failed to get the project.
///   * Failed to get the stats of the project.
pub fn get(api: &coco::Api, project_urn: &coco::Urn) -> Result<Project, error::Error> {
    let project = api.get_project(project_urn, None)?;
    let stats = api.with_browser(project_urn, |browser| Ok(browser.get_stats()?))?;

    Ok((project, stats).into())
}

/// List all projects tracked by the given user.
///
/// # Errors
///
/// * We couldn't get a project list.
/// * We couldn't get project stats.
/// * We couldn't determine the tracking peers of a project.
pub fn list_projects_for_user(
    api: &coco::Api,
    user: &coco::Urn,
) -> Result<Vec<Project>, error::Error> {
    let mut projects = vec![];

    for project in Projects::list(api)?.into_iter() {
        if api
            .tracked(&project.id)?
            .into_iter()
            .any(|(_, project_user)| project_user.urn() == *user)
        {
            projects.push(project);
        }
    }
    Ok(projects)
}

/// Returns a stubbed feed of `Project`s
///
/// # Errors
///
///   * Parsing an empty path fails (it shouldn't really).
pub fn discover() -> Result<Vec<Project>, error::Error> {
    let urn = coco::Urn::new(
        coco::Hash::hash(b"hash"),
        coco::uri::Protocol::Git,
        coco::uri::Path::parse("").map_err(coco::Error::from)?,
    );

    let other_urn = coco::Urn::new(
        coco::Hash::hash(b"something_else"),
        coco::uri::Protocol::Git,
        coco::uri::Path::parse("").map_err(coco::Error::from)?,
    );

    let projects = vec![
            Project {
                id: urn,
                shareable_entity_identifier: "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe".to_string(),
                metadata: Metadata {
                    name: "radicle-upstream".to_string(),
                    description: "It is not the slumber of reason that engenders monsters, \
                        but vigilant and insomniac rationality.".to_string(),
                    default_branch: "main".to_string(),
                    maintainers: HashSet::new(),
                },
                stats: coco::Stats {
                    contributors: 6,
                    branches: 36,
                    commits: 216
                },
            },
            Project {
                id: other_urn,
                shareable_entity_identifier: "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4fd".to_string(),
                metadata: Metadata {
                    name: "radicle-link".to_string(),
                    description: "The monstrous complexity of our reality, a reality \
                    cross-hatched with fibre-optic cables, radio and microwaves, \
                    oil and gas pipelines, aerial and shipping routes, and the unrelenting, \
                    simultaneous execution of millions of communication protocols with every passing millisecond.".to_string(),
                    default_branch: "main".to_string(),
                    maintainers: HashSet::new(),
                },
                stats: coco::Stats {
                    contributors: 7,
                    branches: 49,
                    commits: 343
                },
            },
        ];

    Ok(projects)
}
