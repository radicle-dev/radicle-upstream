use hex::ToHex;
use std::convert::From;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::RwLock;

use librad::paths::Paths;
use librad::surf::git::git2;
use radicle_registry_client::ed25519;

use crate::coco;
use crate::error;
use crate::project;
use crate::registry;
use crate::session;

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
    /// Root on the filesystem for the librad config and storage paths.
    pub librad_paths: Arc<RwLock<Paths>>,
    /// Wrapper to interact with the Registry.
    registry: Arc<RwLock<registry::Registry>>,
    /// Handle to maintain local persistancce and caching.
    pub store: Arc<RwLock<kv::Store>>,
}

impl Context {
    /// Returns a new `Context`.
    #[must_use]
    pub fn new(
        librad_paths: Arc<RwLock<Paths>>,
        registry: Arc<RwLock<registry::Registry>>,
        store: Arc<RwLock<kv::Store>>,
    ) -> Self {
        Self {
            librad_paths,
            registry,
            store,
        }
    }
}

impl juniper::Context for Context {}

/// Encapsulates write path in API.
pub struct Mutation;

#[juniper::object(
    Context = Context,
    name = "UpstreamMutation",
)]
impl Mutation {}

/// Encapsulates read paths in API.
pub struct Query;

#[juniper::object(
    Context = Context,
    name = "UpstreamQuery",
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }
}

/// Bundles `Query` and `Mutation` used for controlling raw state.
pub type Control = juniper::RootNode<'static, ControlQuery, ControlMutation>;

/// Returns the [`Control`] schema used for controlling raw state.
#[must_use]
pub fn create_control() -> Control {
    Control::new(ControlQuery {}, ControlMutation {})
}

/// Control mutations.
pub struct ControlMutation;

#[juniper::object(
    Context = Context,
    name = "ControlMutation",
    description = "Mutations to control raw proxy state.",
)]
impl ControlMutation {
    fn create_project_with_fixture(
        ctx: &Context,
        metadata: ProjectMetadataInput,
    ) -> Result<project::Project, error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let paths = futures::executor::block_on(ctx.librad_paths.read());
        let (id, meta) = coco::replicate_platinum(
            &tmp_dir,
            &paths,
            &metadata.name,
            &metadata.description,
            &metadata.default_branch,
        )?;

        Ok(project::Project {
            id: id.into(),
            metadata: meta.into(),
            registration: None,
            stats: project::Stats {
                branches: 11,
                commits: 267,
                contributors: 8,
            },
        })
    }

    fn nuke_coco_state(ctx: &Context) -> Result<bool, error::Error> {
        let tmp_dir = tempfile::tempdir().expect("creating temporary directory for paths failed");
        let new_paths = Paths::from_root(tmp_dir.path()).expect("unable to get librad paths");

        let mut librad_paths = futures::executor::block_on(ctx.librad_paths.write());

        *librad_paths = new_paths;

        Ok(true)
    }

    fn nuke_registry_state(ctx: &Context) -> Result<bool, error::Error> {
        futures::executor::block_on(ctx.registry.write())
            .reset(radicle_registry_client::Client::new_emulator());

        Ok(true)
    }

    fn nuke_session_state(ctx: &Context) -> Result<bool, error::Error> {
        session::clear(&futures::executor::block_on(ctx.store.read()))?;

        Ok(true)
    }

    fn register_user(
        ctx: &Context,
        handle: juniper::ID,
        id: juniper::ID,
    ) -> Result<registry::Transaction, error::Error> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = ed25519::Pair::from_legacy_string("//Alice", None);
        // TODO(xla): Use real fee defined by the user.
        let fee = 100;

        futures::executor::block_on(
            futures::executor::block_on(ctx.registry.write()).register_user(
                &fake_pair,
                handle.to_string(),
                Some(id.to_string()),
                fee,
            ),
        )
    }
}

/// Control query endpoints.
pub struct ControlQuery;

#[juniper::object(
    Context = Context,
    name = "ControlQuery",
    description = "Queries to access raw proxy state.",
)]
impl ControlQuery {}

/// Input object capturing the fields we need to create project metadata.
#[derive(GraphQLInputObject)]
#[graphql(description = "Input object for project metadata")]
pub struct ProjectMetadataInput {
    /// Project name.
    pub name: String,
    /// High-level description of the project.
    pub description: String,
    /// Default branch for checkouts, often used as mainline as well.
    pub default_branch: String,
}

#[juniper::object]
impl project::Project {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(&self.id.to_string())
    }

    fn metadata(&self) -> &project::Metadata {
        &self.metadata
    }
    fn registered(&self) -> Option<ProjectRegistration> {
        if let Some(registration) = &self.registration {
            match registration {
                project::Registration::Org(org_id) => {
                    Some(ProjectRegistration::Org(OrgRegistration {
                        org_id: juniper::ID::new(org_id.to_string()),
                    }))
                },
                project::Registration::User(user_id) => {
                    Some(ProjectRegistration::User(UserRegistration {
                        user_id: juniper::ID::new(user_id.to_string()),
                    }))
                },
            }
        } else {
            None
        }
    }

    fn stats(&self) -> &project::Stats {
        &self.stats
    }
}

#[juniper::object(name = "ProjectMetadata")]
impl project::Metadata {
    fn default_branch(&self) -> &str {
        &self.default_branch
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Union to represent possible registration states of a project.
// TODO(xla): Remove attribute once integrated.
#[allow(dead_code)]
enum ProjectRegistration {
    /// Project is registered under an Org.
    Org(OrgRegistration),
    /// Project is registered under a User.
    User(UserRegistration),
}

juniper::graphql_union!(ProjectRegistration: () where Scalar = <S> |&self| {
instance_resolvers: |_| {
    &OrgRegistration => match *self {
        ProjectRegistration::Org(ref o) => Some(o),
        ProjectRegistration::User(..) => None,
    },
    &UserRegistration => match *self {
        ProjectRegistration::User(ref o) => Some(o),
        ProjectRegistration::Org(..) => None,
    },
}
});

/// Context data for a not registered project, there are none.
#[derive(juniper::GraphQLObject)]
struct NotRegistration {}

/// Context data for a project registered under an Org.
#[derive(juniper::GraphQLObject)]
struct OrgRegistration {
    /// The id of the Org.
    org_id: juniper::ID,
}

/// Context data for a proejct registered under a User.
#[derive(juniper::GraphQLObject)]
struct UserRegistration {
    /// The id of the User.
    user_id: juniper::ID,
}

#[juniper::object(name = "ProjectStats")]
impl project::Stats {
    fn branches(&self) -> i32 {
        i32::try_from(self.branches).expect("unable to convert branches number")
    }

    fn commits(&self) -> i32 {
        i32::try_from(self.commits).expect("unable to convert branches number")
    }

    fn contributors(&self) -> i32 {
        i32::try_from(self.contributors).expect("unable to convert branches number")
    }
}

/// Response wrapper for listTransactions query.
struct ListTransactions {
    /// The configured Registry thresholds for transaction acceptance stages.
    thresholds: registry::Thresholds,
    /// The known and cached transactions.
    transactions: Vec<registry::Transaction>,
}

#[juniper::object]
impl ListTransactions {
    fn transactions(&self) -> &Vec<registry::Transaction> {
        &self.transactions
    }

    fn thresholds(&self) -> &registry::Thresholds {
        &self.thresholds
    }
}

#[juniper::object]
impl registry::Thresholds {
    fn confirmation(&self) -> i32 {
        i32::try_from(self.confirmation).expect("conversion failed")
    }

    fn settlement(&self) -> i32 {
        i32::try_from(self.settlement).expect("conversion failed")
    }
}

#[juniper::object]
impl registry::Transaction {
    fn id(&self) -> juniper::ID {
        juniper::ID::new(self.id.encode_hex::<String>())
    }

    fn messages(&self) -> Vec<Message> {
        self.messages
            .iter()
            .map(|m| match m {
                registry::Message::OrgRegistration(org_id) => {
                    Message::OrgRegistration(OrgRegistrationMessage {
                        kind: MessageKind::OrgRegistration,
                        org_id: juniper::ID::new(org_id.to_string()),
                    })
                },
                registry::Message::OrgUnregistration(org_id) => {
                    Message::OrgUnregistration(OrgUnregistrationMessage {
                        kind: MessageKind::OrgUnregistration,
                        org_id: juniper::ID::new(org_id.to_string()),
                    })
                },
                registry::Message::ProjectRegistration {
                    project_name,
                    org_id,
                } => Message::ProjectRegistration(ProjectRegistrationMessage {
                    kind: MessageKind::ProjectRegistration,
                    project_name: juniper::ID::new(project_name.to_string()),
                    org_id: juniper::ID::new(org_id.to_string()),
                }),
                registry::Message::UserRegistration { handle, id } => {
                    Message::UserRegistration(UserRegistrationMessage {
                        kind: MessageKind::UserRegistration,
                        handle: juniper::ID::new(handle.to_string()),
                        id: juniper::ID::new(id.to_string()),
                    })
                },
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

/// Message types supproted in transactions.
enum Message {
    /// Registration of a new org.
    OrgRegistration(OrgRegistrationMessage),

    /// Registration of a new org.
    OrgUnregistration(OrgUnregistrationMessage),

    /// Registration of a new project.
    ProjectRegistration(ProjectRegistrationMessage),

    /// Registration of a new user.
    UserRegistration(UserRegistrationMessage),
}

/// Kind of the transaction message.
#[derive(juniper::GraphQLEnum)]
enum MessageKind {
    /// Registration of a new org.
    OrgRegistration,

    /// Registration of a new org.
    OrgUnregistration,

    /// Registration of a new project.
    ProjectRegistration,

    /// Registration of a new user.
    UserRegistration,
}

juniper::graphql_union!(Message: () where Scalar = <S> |&self| {
    instance_resolvers: |_| {
        &OrgRegistrationMessage => match *self {
            Message::OrgRegistration(ref o) => Some(o),
            Message::OrgUnregistration(..) | Message::ProjectRegistration(..) | Message::UserRegistration(..) => None,
        },
        &OrgUnregistrationMessage => match *self {
            Message::OrgUnregistration(ref o) => Some(o),
            Message::OrgRegistration(..) | Message::ProjectRegistration(..) | Message::UserRegistration(..) => None,
        },
        &ProjectRegistrationMessage => match *self {
            Message::ProjectRegistration(ref p) => Some(p),
            Message::OrgRegistration(..) | Message::OrgUnregistration(..) | Message::UserRegistration(..) => None,
        },
        &UserRegistrationMessage => match *self {
            Message::UserRegistration(ref o) => Some(o),
            Message::OrgRegistration(..) | Message::OrgUnregistration(..) | Message::ProjectRegistration(..) => None,
        }
    }
});

/// Contextual information for an org registration message.
#[derive(juniper::GraphQLObject)]
struct OrgRegistrationMessage {
    /// Field to distinguish [`Message`] types.
    kind: MessageKind,
    /// The ID of the org.
    org_id: juniper::ID,
}

/// Contextual information for an org unregistration message.
#[derive(juniper::GraphQLObject)]
struct OrgUnregistrationMessage {
    /// Field to distinguish [`Message`] types.
    kind: MessageKind,
    /// The ID of the org.
    org_id: juniper::ID,
}

/// Contextual information for a project registration message.
#[derive(juniper::GraphQLObject)]
struct ProjectRegistrationMessage {
    /// Field to distinguish [`Message`] types.
    kind: MessageKind,
    /// Actual project name, unique under org.
    project_name: juniper::ID,
    /// The org under which to register the project.
    org_id: juniper::ID,
}

/// Payload of a user registration message.
#[derive(juniper::GraphQLObject)]
struct UserRegistrationMessage {
    /// Field to distinguish [`Message`] types.
    kind: MessageKind,
    /// The chosen unique handle to be registered.
    handle: juniper::ID,
    /// The id of the coco identity.
    id: juniper::ID,
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
