//! Combine the domain `CoCo` and Registry domain specific understanding of a Project into a single
//! abstraction.

use librad::meta;
use librad::project;

/// Metadata key used to store an image url for a project.
const IMG_URL_LABEL: &str = "img_url";

/// Object the API returns for project metadata.
pub struct Metadata {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
    /// Image url for the project.
    pub img_url: String,
}

impl From<meta::Project> for Metadata {
    fn from(project_meta: meta::Project) -> Self {
        let img_url = project_meta
            .rel
            .into_iter()
            .filter_map(|r| {
                if let meta::Relation::Url(label, url) = r {
                    Some((label, url))
                } else {
                    None
                }
            })
            .find_map(|(label, url)| {
                if *label == *IMG_URL_LABEL {
                    Some(url.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "".to_string());

        Self {
            name: project_meta.name.unwrap_or_else(|| "name unknown".into()),
            description: project_meta.description.unwrap_or_else(|| "".into()),
            default_branch: project_meta.default_branch,
            img_url,
        }
    }
}

/// Radicle project for sharing and collaborating.
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: project::ProjectId,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Coarse set of statistics for the project source code.
    pub stats: Stats,
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
