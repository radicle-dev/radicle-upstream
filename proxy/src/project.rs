//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta;
use librad::project;
use radicle_registry_client as registry;

/// Object the API returns for project metadata.
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
