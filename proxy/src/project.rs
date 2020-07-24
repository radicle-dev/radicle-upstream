//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta::project;
use serde::{Deserialize, Serialize};

use crate::coco;
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

impl<ST> From<project::Project<ST>> for Metadata
where
    ST: Clone,
{
    fn from(project_meta: project::Project<ST>) -> Self {
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
impl<ST> From<(project::Project<ST>, coco::Stats)> for Project
where
    ST: Clone,
{
    /// Create a `Project` given a `librad` defined [`project::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn from((project, stats): (project::Project<ST>, coco::Stats)) -> Self {
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
pub fn get(api: &coco::Api, project_urn: &coco::Urn) -> Result<Project, error::Error> {
    let project = api.get_project(project_urn)?;
    let stats = api.with_browser(project_urn, |browser| Ok(browser.get_stats()?))?;

    Ok((project, stats).into())
}

/// Returns a list of `Project`s for your peer.
pub fn list_projects(api: &coco::Api) -> Result<Vec<Project>, error::Error> {
    let project_meta = api.list_projects()?;

    project_meta
        .into_iter()
        .map(|project| {
            api.with_browser(&project.urn(), |browser| {
                let stats = browser.get_stats()?;
                Ok((project, stats).into())
            })
        })
        .collect()
}

/// Returns a stubbed feed of `DiscoveryItem`s
pub fn discover() -> Result<Vec<DiscoveryItem>, error::Error> {
    let projects = vec![
            DiscoveryItem {
                id: "rad@12345".to_string(),
                shareable_entity_identifier: "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe".to_string(),
                metadata: Metadata {
                    name: "radicle-upstream".to_string(),
                    description: "It is not the slumber of reason that engenders monsters, but vigilant and insomniac rationality.".to_string(),
                    default_branch: "main".to_string()
                },
                stats: coco::Stats {
                    contributors: 6,
                    branches: 36,
                    commits: 216
                },
                registration: None,
            }
        ];

    Ok(projects)
}

/// Controversial placeholder struct for `EventStream` projects. It's cumbersome to create
/// a new `RadUrn` otherwise and the `EventStream` will likely include fields the `Project`
/// does not (e.g. maintainers)
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryItem {
    /// Unique identifier of the project in the network.
    pub id: String,
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
