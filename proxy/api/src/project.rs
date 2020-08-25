//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use serde::{Deserialize, Serialize};

use crate::error;
use crate::registry;

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
        }
    }
}

/// Radicle project for sharing and collaborating.
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: coco::Urn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Informs if the project is present in the Registry and under what top-level entity it can be
    /// found.
    pub registration: Option<Registration>,
    /// High-level statistics about the project
    pub stats: coco::Stats,
}

/// Construct a Project from its metadata and stats
impl<ST> From<(coco::Project<ST>, coco::Stats)> for Project
where
    ST: Clone,
{
    /// Create a `Project` given a `librad` defined [`coco::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn from((project, stats): (coco::Project<ST>, coco::Stats)) -> Self {
        let id = project.urn();

        Self {
            id: id.clone(),
            shareable_entity_identifier: format!("%{}", id),
            metadata: project.into(),
            registration: None,
            stats,
        }
    }
}

/// Variants for possible registration states of a project.
#[allow(dead_code)]
pub enum Registration {
    /// Project is registered under an Org.
    Org(registry::Id),
    /// Project is registered under a User.
    User(registry::Id),
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

/// Returns a list of `Project`s for your peer.
///
/// # Errors
///
///   * We couldn't get a project list.
///   * We couldn't get project stats.
pub fn list_projects(api: &coco::Api) -> Result<Vec<Project>, error::Error> {
    let project_meta = api.list_projects()?;

    project_meta
        .into_iter()
        .map(|project| {
            api.with_browser(&project.urn(), |browser| {
                let stats = browser.get_stats().map_err(coco::Error::from)?;
                Ok((project, stats).into())
            })
            .map_err(error::Error::from)
        })
        .collect()
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
    let all_projects = list_projects(api)?;

    // We’re using MapSet to deduplicate projects. This is a bug in librad.
    // https://github.com/radicle-dev/radicle-link/issues/266.
    let mut user_projects = std::collections::HashMap::new();

    for project in all_projects {
        if api
            .tracked(&project.id)?
            .into_iter()
            .any(|(_, project_user)| project_user.urn() == *user)
        {
            user_projects.insert(project.id.clone(), project);
        }
    }
    Ok(user_projects
        .into_iter()
        .map(|(_, project)| project)
        .collect())
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
        coco::uri::Path::parse("")?,
    );

    let other_urn = coco::Urn::new(
        coco::Hash::hash(b"something_else"),
        coco::uri::Protocol::Git,
        coco::uri::Path::parse("")?,
    );

    let projects = vec![
            Project {
                id: urn,
                shareable_entity_identifier: "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe".to_string(),
                metadata: Metadata {
                    name: "radicle-upstream".to_string(),
                    description: "It is not the slumber of reason that engenders monsters, \
                        but vigilant and insomniac rationality.".to_string(),
                    default_branch: "main".to_string()
                },
                stats: coco::Stats {
                    contributors: 6,
                    branches: 36,
                    commits: 216
                },
                registration: None,
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
                    default_branch: "main".to_string()
                },
                stats: coco::Stats {
                    contributors: 7,
                    branches: 49,
                    commits: 343
                },
                registration: None,
            },
        ];

    Ok(projects)
}
