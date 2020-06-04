//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta::project;
use librad::uri;
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
    pub id: uri::RadUrn,
    /// Unambiguous identifier pointing at this identity.
    pub shareable_entity_identifier: String,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Informs if the project is present in the Registry and under what top-level entity it can be
    /// found.
    pub registration: Option<Registration>,
    /// High-level statistics about the project.
    pub stats: Stats,
}

/// Variants for possible registration states of a project.
#[allow(dead_code)]
pub enum Registration {
    /// Project is registered under an Org.
    Org(registry::Id),
    /// Project is registered under a User.
    User(registry::Id),
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
pub async fn get(peer: &coco::Peer, project_urn: &str) -> Result<Project, error::Error> {
    let meta = peer.get_project(&project_urn).await?;

    Ok(Project {
        id: meta.urn().clone(),
        shareable_entity_identifier: project_urn.to_string(),
        metadata: meta.into(),
        registration: None,
        stats: Stats {
            branches: 11,
            commits: 267,
            contributors: 8,
        },
    })
}
