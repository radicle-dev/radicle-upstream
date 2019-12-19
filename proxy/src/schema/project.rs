use librad::meta;

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
    name: String,
    description: String,
    default_branch: String,
    img_url: String,
}

impl From<meta::Project> for Metadata {
    fn from(project_meta: meta::Project) -> Self {
        Self {
            name: project_meta.name.unwrap_or("name unknown".into()),
            description: project_meta.description.unwrap_or("".into()),
            default_branch: project_meta.default_branch,
            img_url: "".into(),
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Radicle project")]
pub struct Project {
    pub id: juniper::ID,
    pub metadata: Metadata,
}
