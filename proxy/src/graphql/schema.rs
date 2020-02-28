use std::convert::From;
use std::convert::TryFrom;
use std::str::FromStr;

use librad::paths::Paths;
use librad::surf;
use librad::surf::git::git2;
use radicle_registry_client::ed25519;

use super::project;
use crate::coco;
use crate::error;
use crate::registry;

/// Glue to bundle our read and write APIs together.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

/// Returns a `Schema` with the default parameterised `Query` and `Mutation`.
#[must_use]
pub fn create() -> Schema {
    Schema::new(Query {}, Mutation {})
}

/// Container for data access from handlers.
#[derive(Clone)]
pub struct Context {
    /// Intermediate repo used to serve dummy data to be presented to the API consumer.
    dummy_repo_path: String,
    /// Root on the filesystem for the librad config and storage paths.
    librad_paths: Paths,
    /// Wrapper to interact with the Registry.
    registry: registry::Registry,
}

impl Context {
    /// Returns a new `Context`.
    #[must_use]
    pub const fn new(
        dummy_repo_path: String,
        librad_paths: Paths,
        registry_client: radicle_registry_client::Client,
    ) -> Self {
        Self {
            dummy_repo_path,
            librad_paths,
            registry: registry::Registry::new(registry_client),
        }
    }
}

impl juniper::Context for Context {}

#[derive(GraphQLObject)]
struct Identity {
    pub id: juniper::ID,
    pub shareable_entity_identifier: juniper::ID,
    pub metadata: IdentityMetadata,
}

#[derive(GraphQLObject)]
struct IdentityMetadata {
    pub handle: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_identity(
        _ctx: &Context,
        handle: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Identity, error::Error> {
        Ok(Identity {
            id: juniper::ID::new("123abcd.git"),
            shareable_entity_identifier: juniper::ID::new(format!("{}@123abcd.git", handle)),
            metadata: IdentityMetadata {
                handle,
                display_name,
                avatar_url,
            },
        })
    }

    fn create_project(
        ctx: &Context,
        metadata: project::MetadataInput,
        path: String,
        publish: bool,
    ) -> Result<project::Project, error::Error> {
        if surf::git::git2::Repository::open(path.clone()).is_err() {
            coco::init_repo(path.clone())?;
        };

        let (id, meta) = coco::init_project(
            &ctx.librad_paths,
            &path,
            &metadata.name,
            &metadata.description,
            &metadata.default_branch,
            &metadata.img_url,
        )?;

        Ok(project::Project {
            id: id.to_string().into(),
            metadata: meta.into(),
            registered: project::Registered::Not,
            stats: project::Stats {
                branches: 11,
                commits: 267,
                contributors: 8,
            },
        })
    }

    fn register_project(
        ctx: &Context,
        project_name: String,
        org_id: String,
        maybe_librad_id_input: Option<juniper::ID>,
    ) -> Result<registry::Transaction, error::Error> {
        let maybe_librad_id = maybe_librad_id_input.map(|id| {
            librad::project::ProjectId::from_str(&id.to_string())
                .expect("unable to parse project id")
        });

        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = ed25519::Pair::from_legacy_string("//Robot", None);
        // TODO(xla): Remove single-threaded executor once async/await lands in juniper:
        // https://github.com/graphql-rust/juniper/pull/497
        futures::executor::block_on(ctx.registry.register_project(
            &fake_pair,
            project_name,
            org_id,
            maybe_librad_id,
        ))
    }
}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn blob(
        ctx: &Context,
        id: juniper::ID,
        revision: String,
        path: String,
    ) -> Result<coco::Blob, error::Error> {
        coco::blob(&ctx.librad_paths, &id.to_string(), &revision, &path)
    }

    fn commit(ctx: &Context, id: juniper::ID, sha1: String) -> Result<coco::Commit, error::Error> {
        coco::commit(&ctx.librad_paths, &id.to_string(), &sha1)
    }

    fn branches(ctx: &Context, id: juniper::ID) -> Result<Vec<String>, error::Error> {
        Ok(coco::branches(&ctx.librad_paths, &id.to_string())?
            .into_iter()
            .map(|t| t.to_string())
            .collect())
    }

    fn local_branches(ctx: &Context, path: String) -> Result<Vec<String>, error::Error> {
        Ok(coco::local_branches(&path)?
            .into_iter()
            .map(|t| t.to_string())
            .collect())
    }

    fn tags(ctx: &Context, id: juniper::ID) -> Result<Vec<String>, error::Error> {
        Ok(coco::tags(&ctx.librad_paths, &id.to_string())?
            .into_iter()
            .map(|t| t.to_string())
            .collect())
    }

    fn tree(
        ctx: &Context,
        id: juniper::ID,
        revision: String,
        prefix: String,
    ) -> Result<coco::Tree, error::Error> {
        coco::tree(&ctx.librad_paths, &id, &revision, &prefix)
    }

    fn project(ctx: &Context, id: juniper::ID) -> Result<project::Project, error::Error> {
        let meta = coco::get_project_meta(&ctx.librad_paths, &id.to_string())?;

        Ok(project::Project {
            id,
            metadata: meta.into(),
            registered: project::Registered::Not,
            stats: project::Stats {
                branches: 11,
                commits: 267,
                contributors: 8,
            },
        })
    }

    fn projects(ctx: &Context) -> Result<Vec<project::Project>, error::Error> {
        let projects = coco::list_projects(&ctx.librad_paths)
            .into_iter()
            .map(|(id, meta)| project::Project {
                id: juniper::ID::new(id.to_string()),
                metadata: meta.into(),
                registered: project::Registered::Not,
                stats: project::Stats {
                    branches: 11,
                    commits: 267,
                    contributors: 8,
                },
            })
            .collect::<Vec<project::Project>>();

        Ok(projects)
    }

    fn list_registry_projects(ctx: &Context) -> Result<Vec<juniper::ID>, error::Error> {
        let ids = futures::executor::block_on(ctx.registry.list_projects())?;

        Ok(ids
            .iter()
            .map(|id| juniper::ID::from(id.0.to_string()))
            .collect::<Vec<juniper::ID>>())
    }
}

#[juniper::object]
impl coco::Blob {
    fn binary(&self) -> bool {
        match &self.content {
            coco::BlobContent::Ascii(_content) => false,
            coco::BlobContent::Binary => true,
        }
    }

    fn content(&self) -> Option<String> {
        match &self.content {
            coco::BlobContent::Ascii(content) => Some(content.clone()),
            coco::BlobContent::Binary => None,
        }
    }

    fn info(&self) -> &coco::Info {
        &self.info
    }
}

#[juniper::object]
impl coco::Commit {
    fn sha1(&self) -> String {
        self.sha1.to_string()
    }

    fn author(&self) -> &coco::Person {
        &self.author
    }

    fn summary(&self) -> &str {
        &self.summary
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn committer_time(&self) -> String {
        self.committer_time.seconds().to_string()
    }
}

#[juniper::object]
impl coco::Info {
    fn name(&self) -> &str {
        &self.name
    }

    fn object_type(&self) -> ObjectType {
        match self.object_type {
            coco::ObjectType::Blob => ObjectType::Blob,
            coco::ObjectType::Tree => ObjectType::Tree,
        }
    }

    fn last_commit(&self) -> Option<&coco::Commit> {
        self.last_commit.as_ref()
    }
}

/// Git object types.
///
/// <https://git-scm.com/book/en/v2/Git-Internals-Git-Objects>
#[derive(GraphQLEnum)]
enum ObjectType {
    /// Directory tree.
    Tree,
    /// Text or binary blob of a file.
    Blob,
}

/// Contextual information for an org registration message.
#[derive(juniper::GraphQLObject)]
struct OrgRegistration {
    /// The ID of the org.
    org_id: String,
}

/// Contextual information for an org unregistration message.
#[derive(juniper::GraphQLObject)]
struct OrgUnregistration {
    /// The ID of the org.
    org_id: String,
}

/// Contextual information for a project registration message.
#[derive(juniper::GraphQLObject)]
struct ProjectRegistration {
    /// Actual project name, unique under org.
    project_name: String,
    /// The org under which to register the project.
    org_id: String,
}

/// Message types supproted in transactions.
enum Message {
    /// Registration of a new org.
    OrgRegistration(OrgRegistration),

    /// Registration of a new org.
    OrgUnregistration(OrgUnregistration),

    /// Registration of a new project.
    ProjectRegistration(ProjectRegistration),
}

juniper::graphql_union!(Message: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &ProjectRegistration => match *self {
            Message::ProjectRegistration(ref p) => Some(p),
            _ => None
        },
        &OrgRegistration => match *self {
            Message::OrgRegistration(ref o) => Some(o),
            _ => None
        },
        &OrgUnregistration => match *self {
            Message::OrgUnregistration(ref o) => Some(o),
            _ => None
        },
    }
});

#[juniper::object]
impl coco::Person {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn avatar(&self) -> &str {
        &self.avatar
    }
}

#[juniper::object]
impl registry::Transaction {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.id.to_string())
    }

    fn messages(&self) -> Vec<Message> {
        self.messages
            .iter()
            .map(|m| match m {
                registry::Message::OrgRegistration(org_id) => {
                    Message::OrgRegistration(OrgRegistration {
                        org_id: org_id.to_string(),
                    })
                },
                registry::Message::OrgUnregistration(org_id) => {
                    Message::OrgUnregistration(OrgUnregistration {
                        org_id: org_id.to_string(),
                    })
                },
                registry::Message::ProjectRegistration {
                    project_name,
                    org_id,
                } => Message::ProjectRegistration(ProjectRegistration {
                    project_name: project_name.to_string(),
                    org_id: org_id.to_string(),
                }),
            })
            .collect()
    }

    fn state(&self) -> TransactionState {
        match self.state {
            registry::TransactionState::Applied(block_hash) => TransactionState::Applied(Applied {
                block: juniper::ID::new(block_hash.to_string()),
            }),
        }
    }

    fn timestamp(&self) -> juniper::FieldResult<String> {
        let since_epoch = i64::try_from(
            self.timestamp
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        )?;
        let git_time = git2::Time::new(since_epoch, 0).seconds().to_string();

        Ok(git_time)
    }
}

/// States a transaction can go through.
enum TransactionState {
    /// The transaction has been applied to a block.
    Applied(Applied),
}

/// Context for a chain applied transaction.
#[derive(GraphQLObject)]
struct Applied {
    /// Block hash the transaction was included in.
    block: juniper::ID,
}

juniper::graphql_union!(TransactionState: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &Applied => match *self { TransactionState::Applied(ref a) => Some(a) },
    }
});

#[juniper::object]
impl coco::Tree {
    fn path(&self) -> &str {
        &self.path
    }

    fn entries(&self) -> &Vec<coco::TreeEntry> {
        self.entries.as_ref()
    }

    fn info(&self) -> &coco::Info {
        &self.info
    }
}

#[juniper::object]
impl coco::TreeEntry {
    fn info(&self) -> &coco::Info {
        &self.info
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}
