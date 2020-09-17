//! Combine the domain `CoCo` domain specific understanding of a Project into a single
//! abstraction.

use std::{collections::HashSet, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::error;

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
    ///     to references.
    ///
    /// The conditions imply that a project is "contributed" if I am the maintainer or I have
    /// contributed to the project.
    pub contributed: Vec<Project>,
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
    pub fn list(state: &coco::State) -> Result<Self, error::Error> {
        let mut projects = Self {
            tracked: vec![],
            contributed: vec![],
        };
        for project in state.list_projects()? {
            let refs = state.list_owner_project_refs(&project.urn())?;
            let project = state.with_browser(&project.urn(), |browser| {
                let project_stats = browser.get_stats().map_err(coco::Error::from)?;
                Ok((project, project_stats).into())
            })?;
            if refs.heads.is_empty() {
                projects.tracked.push(Tracked(project))
            } else {
                projects.contributed.push(project)
            }
        }

        Ok(projects)
    }

    /// Give back an `Iter` that can be used to iterate over the projects. It first yields
    /// contributed projects and then tracked projects.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            contributed: self.contributed.iter(),
            tracked: self.tracked.iter(),
        }
    }
}

/// An iterator over [`Projects`] that first yields contributed projects and then tracked projects.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    /// Iterator over contributed projects.
    contributed: std::slice::Iter<'a, Project>,

    /// Iterator over tracked projects.
    tracked: std::slice::Iter<'a, Tracked>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Project;

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
    type Item = Project;
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
    contributed: std::vec::IntoIter<Project>,

    /// Iterator over tracked projects.
    tracked: std::vec::IntoIter<Tracked>,
}

impl Iterator for IntoIter {
    type Item = Project;

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
pub fn get(state: &coco::State, project_urn: &coco::Urn) -> Result<Project, error::Error> {
    let project = state.get_project(project_urn, None)?;
    let project_stats = state.with_browser(project_urn, |browser| Ok(browser.get_stats()?))?;

    Ok((project, project_stats).into())
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
pub fn list_for_user(state: &coco::State, user: &coco::Urn) -> Result<Vec<Project>, error::Error> {
    let mut projects = vec![];

    for project in state.list_projects()? {
        if state
            .tracked(&project.urn())?
            .into_iter()
            .any(|(_, project_user)| project_user.urn() == *user)
        {
            let proj = state.with_browser(&project.urn(), |browser| {
                let project_stats = browser.get_stats().map_err(coco::Error::from)?;
                Ok((project, project_stats).into())
            })?;

            projects.push(proj);
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
