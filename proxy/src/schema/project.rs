use librad::meta;

const IMG_URL_LABEL: &str = "img_url";

#[derive(GraphQLInputObject)]
#[graphql(description = "Input object for project metadata")]
pub struct MetadataInput {
    pub name: String,
    pub description: String,
    pub default_branch: String,
    pub img_url: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Project metadata")]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub default_branch: String,
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
            .filter_map(|(label, url)| {
                if *label == *IMG_URL_LABEL {
                    Some(url.to_string())
                } else {
                    None
                }
            })
            .nth(0)
            .unwrap_or_else(|| "".to_string());

        Self {
            name: project_meta.name.unwrap_or_else(|| "name unknown".into()),
            description: project_meta.description.unwrap_or_else(|| "".into()),
            default_branch: project_meta.default_branch,
            img_url,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Radicle project")]
pub struct Project {
    pub id: juniper::ID,
    pub metadata: Metadata,
}
