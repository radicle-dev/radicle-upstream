use futures::future::Future;
use std::collections::HashMap;

use radicle_registry_client::{CryptoPair as _, H256};
use radicle_registry_runtime::registry::{ProjectDomain, ProjectName};

/// Newtype for the registry `oscoin_client::AccountId`.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AccountId(pub radicle_registry_client::AccountId);

/// Newtype for the registry `oscoin_client::ProjectId`.
#[derive(Clone, Eq, PartialEq)]
pub struct ProjectId(pub radicle_registry_client::ProjectId);

/// Metadata enriched user keypair.
/// TODO(xla): This overlaps with accounts on the registry, needs renaming.
#[derive(Clone, GraphQLObject)]
#[graphql(description = "Metadata enriched user keypair")]
struct Account {
    /// Reference to the `AccountId`.
    id: AccountId,
    /// User given name of the key.
    key_name: String,
    /// User given url for the avatar attached to the keypair.
    avatar_url: String,
}

/// Representation of a users project.
#[derive(Clone, GraphQLObject)]
#[graphql(description = "An open source coin project")]
pub struct Project {
    /// Reference to the `ProjectId` of the project.
    id: ProjectId,
    /// User given project name.
    name: String,
    /// Longer form description of the project.
    description: String,
    /// Image to be shown as the projects avatar.
    img_url: String,
    /// List of members with extended rights.
    members: Vec<Account>,
}

impl From<radicle_registry_client::Project> for Project {
    fn from(p: radicle_registry_client::Project) -> Self {
        let ms = p
            .members
            .into_iter()
            .map(|id| Account {
                id: AccountId(id),
                key_name: "".to_owned(),
                avatar_url: "".to_owned(),
            })
            .collect();

        Self {
            id: ProjectId(p.id.clone()),
            name: p.id.0.to_string(),
            description: p.description,
            img_url: p.img_url,
            members: ms,
        }
    }
}

/// Abstraction used to fetch information from the registry.
pub trait Source {
    /// Retrieve unfiltered list of projects.
    fn get_all_projects(&self) -> Vec<Project>;
    /// Retrieve a single proejct by `ProjectId`.
    fn get_project(&self, id: ProjectId) -> Option<Project>;
    /// Register a new project.
    fn register_project(&self, name: String, description: String, img_url: String) -> Project;
}

/// Container to store local view on accounts to match with metadata.
pub struct Ledger<R>
where
    R: radicle_registry_client::ClientT,
{
    /// Mapping of `AccountId`s to `Account`s for easier metadata enrichment.
    accounts: HashMap<AccountId, Account>,
    /// Ledger client.
    registry_client: R,
}

impl<R> Ledger<R>
where
    R: radicle_registry_client::ClientT,
{
    /// Returns a new `Ledger`.
    pub fn new(registry_client: R) -> Self {
        Self {
            accounts: HashMap::new(),
            registry_client,
        }
    }

    /// Returns the project with added account metadata if found.
    fn enrich_members(&self, p: Project) -> Project {
        let ms = p
            .members
            .into_iter()
            .map(|a| {
                self.accounts
                    .get(&a.id)
                    .unwrap_or(&Account {
                        id: a.id,
                        key_name: "anonymous".to_owned(),
                        avatar_url: "".to_owned(),
                    })
                    .clone()
            })
            .collect();

        Project {
            id: p.id,
            name: p.name,
            description: p.description,
            img_url: p.img_url,
            members: ms,
        }
    }
}

impl<R> Source for Ledger<R>
where
    R: radicle_registry_client::ClientT,
{
    fn get_all_projects(&self) -> Vec<Project> {
        // TODO(xla): Return proper error.
        self.registry_client
            .list_projects()
            .wait()
            .expect("osc client list projects failed")
            .into_iter()
            .take(10)
            .map(|id| {
                let maybe_project = self
                    .registry_client
                    .get_project(id)
                    .wait()
                    .expect("get_project failed");

                match maybe_project {
                    Some(project) => Some(self.enrich_members(Project::from(project))),
                    None => None,
                }
            })
            .flatten()
            .collect()
    }

    fn get_project(&self, id: ProjectId) -> Option<Project> {
        let maybe_project = self
            .registry_client
            .get_project(id.0)
            .wait()
            .expect("get project failed");

        match maybe_project {
            Some(p) => Some(self.enrich_members(Project::from(p))),
            None => None,
        }
    }

    fn register_project(&self, name: String, description: String, img_url: String) -> Project {
        let (sender, _, _) = radicle_registry_client::ed25519::Pair::generate_with_phrase(None);

        let project_name =
            ProjectName::from_string(name.to_string()).expect("project name creation failed");
        let project_domain =
            ProjectDomain::from_string("rad".to_string()).expect("project domain creation faile");
        let project_id = (project_name, project_domain);

        // TODO(xla): Proper error handling.
        self.registry_client
            .register_project(
                &sender,
                radicle_registry_client::RegisterProjectParams {
                    id: project_id.clone(),
                    description: description.to_owned(),
                    img_url: img_url.to_owned(),
                    checkpoint_id: H256::random(),
                },
            )
            .wait()
            .expect("osc project registration failed");

        Project {
            id: ProjectId(project_id),
            name,
            description,
            img_url,
            members: vec![],
        }
    }
}
