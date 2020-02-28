//! Abstractions for projects returned by the API.

use librad::meta;

/// Metadata key used to store an image url for a project.
const IMG_URL_LABEL: &str = "img_url";

/// Input object capturing the fields we need to create project metadata.
#[derive(GraphQLInputObject)]
#[graphql(description = "Input object for project metadata")]
pub struct MetadataInput {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
    /// Image url for the project.
    pub img_url: String,
}

/// Object the API returns for project metadata.
#[derive(GraphQLObject)]
#[graphql(description = "Project metadata")]
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

#[derive(GraphQLEnum)]
pub enum Registered {
    Not,
    Org,
    User,
}

/// Radicle project for sharing and collaborating.
#[derive(GraphQLObject)]
#[graphql(description = "Project")]
pub struct Project {
    /// Unique identifier of the project in the network.
    pub id: juniper::ID,
    /// Attached metadata, mostly for human pleasure.
    pub metadata: Metadata,
    /// Signals if a project is on the Registry.
    pub registered: Registered,
    /// Coarse set of statistics for the project source code.
    pub stats: Stats,
}

#[derive(GraphQLObject)]
#[graphql(name = "ProjectStats")]
pub struct Stats {
    pub branches: i32,
    pub commits: i32,
    pub contributors: i32,
}
