//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta;
use librad::project;
use radicle_registry_client as registry;
use std::str::FromStr;

use crate::coco;
use crate::error;

/// Object the API returns for project metadata.
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
}

impl From<meta::Project> for Metadata {
    fn from(project_meta: meta::Project) -> Self {
        Self {
            name: project_meta.name.unwrap_or_else(|| "name unknown".into()),
            description: project_meta.description.unwrap_or_else(|| "".into()),
            default_branch: project_meta.default_branch,
        }
    }
}

/// Radicle project for sharing and collaborating.
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: project::ProjectId,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Informs if the project is present in the Registry and under what top-level entity it can be
    /// found.
    pub registration: Option<Registration>,
    /// High-level statistics about the project.
    pub stats: Stats,
}

/// Variants for possible registration states of a project.
// TODO(xla): Remove once properly integrated.
#[allow(dead_code)]
pub enum Registration {
    /// Project is registered under an Org.
    Org(registry::OrgId),
    /// Project is registered under a User.
    User(registry::UserId),
}

/// Coarse statistics for the Project source code.
pub struct Stats {
    /// Amount of known branches.
    pub branches: u32,
    /// Number of commits on the default branch.
    pub commits: u32,
    /// Amount of unique commiters on the default branch.
    pub contributors: u32,
}

/// TODO(xla): Add documentation.
pub async fn get(paths: &librad::paths::Paths, id: &str) -> Result<Project, error::Error> {
    let meta = coco::get_project_meta(paths, id)?;

    Ok(Project {
        id: librad::project::ProjectId::from_str(id)?,
        metadata: meta.into(),
        registration: None,
        stats: Stats {
            branches: 11,
            commits: 267,
            contributors: 8,
        },
    })
}
