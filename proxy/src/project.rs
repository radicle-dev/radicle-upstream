//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta::project;
use librad::uri;
use serde::{Deserialize, Serialize};

use crate::avatar;
use crate::coco;
use crate::error;
use crate::identity;
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
    /// High-level statistics about the project
    pub stats: coco::Stats,
    /// The user to which all source browsing queries are scoped to.
    pub default_user: identity::Identity,
}

/// Construct a Project from its metadata and stats
impl<ST> From<(project::Project<ST>, coco::Stats, identity::Identity)> for Project
where
    ST: Clone,
{
    /// Create a `Project` given a `librad` defined [`project::Project`] and the [`coco::Stats`]
    /// for the repository.
    fn from(
        (project, stats, default_user): (project::Project<ST>, coco::Stats, identity::Identity),
    ) -> Self {
        let id = project.urn();

        Self {
            id: id.clone(),
            shareable_entity_identifier: format!("%{}", id),
            default_user,
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

pub fn get_fake_default_user() -> identity::Identity {
    let fake_handle = "rudolfs";
    let fake_urn = "rad:git:hwd1yrereyss6pihzu3f3k4783boykpwr1uzdn3cwugmmxwrpsay5ycyuro";

    identity::Identity {
        id: fake_urn.parse().expect("failed to parse hardcoded URN"),
        metadata: identity::Metadata {
            handle: fake_handle.to_string(),
        },
        avatar_fallback: avatar::Avatar::from(&fake_handle, avatar::Usage::Identity),
        registered: None,
        shareable_entity_identifier: identity::SharedIdentifier {
            handle: fake_handle.to_string(),
            urn: fake_urn.parse().expect("failed to parse hardcoded URN"),
        },
    }
}

/// Fetch the project with a given urn from a peer
pub fn get(peer: &coco::PeerApi, project_urn: &uri::RadUrn) -> Result<Project, error::Error> {
    let project = coco::get_project(peer, project_urn)?;
    let stats = coco::with_browser(peer, project_urn, |browser| Ok(browser.get_stats()?))?;

    Ok((project, stats, get_fake_default_user()).into())
}
