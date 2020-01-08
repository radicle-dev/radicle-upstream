use futures::future::Future;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use radicle_registry_client::{CryptoPair as _, ProjectDomain, ProjectName, H256};

/// Newtype for the registry `oscoin_client::AccountId`.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AccountId(pub radicle_registry_client::AccountId);

// /// Newtype for the registry `oscoin_client::ProjectId`.
// #[derive(Clone, Eq, PartialEq)]
// pub struct ProjectId(pub radicle_registry_client::ProjectId);

/// Metadata enriched user keypair.
/// TODO(xla): This overlaps with accounts on the registry, needs renaming.
#[derive(Clone)]
struct Account {
    /// Reference to the `AccountId`.
    id: AccountId,
    /// User given name of the key.
    key_name: String,
    /// User given url for the avatar attached to the keypair.
    avatar_url: String,
}

/// Registry identification tuple comprised of name and domain.
#[derive(Clone)]
pub struct ProjectId {
    /// Name part, assumed to be unique in the domain.
    pub name: String,
    /// Domain part.
    pub domain: String,
}

impl TryFrom<radicle_registry_client::ProjectId> for ProjectId {
    type Error = &'static str;
    fn try_from(id: radicle_registry_client::ProjectId) -> Result<Self, Self::Error> {
        Ok(Self {
            name: id.0.to_string(),
            domain: id.1.to_string(),
        })
    }
}

impl TryInto<radicle_registry_client::ProjectId> for ProjectId {
    type Error = &'static str;
    fn try_into(self) -> Result<radicle_registry_client::ProjectId, Self::Error> {
        Ok((
            ProjectName::from_string(self.name).expect("project name creation failed"),
            ProjectDomain::from_string(self.domain).expect("project domain creation faile"),
        ))
    }
}

/// Representation of a users project.
#[derive(Clone)]
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

impl TryFrom<radicle_registry_client::Project> for Project {
    type Error = &'static str;
    fn try_from(p: radicle_registry_client::Project) -> Result<Self, Self::Error> {
        let ms = p
            .members
            .into_iter()
            .map(|id| Account {
                id: AccountId(id),
                key_name: "".to_owned(),
                avatar_url: "".to_owned(),
            })
            .collect();

        Ok(Self {
            id: p.id.clone().try_into().unwrap(),
            name: p.id.0.to_string(),
            description: p.description,
            img_url: p.img_url,
            members: ms,
        })
    }
}

/// Abstraction used to fetch information from the registry.
pub trait Source {
    /// Create a new keypair with an associated avatar.
    fn create_account(&mut self, key_name: String, avatar_url: String) -> AccountId;
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
    #[allow(dead_code)]
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
    fn create_account(&mut self, key_name: String, avatar_url: String) -> AccountId {
        let id = AccountId(
            radicle_registry_client::ed25519::Pair::generate()
                .0
                .public(),
        );

        self.accounts.insert(
            id.clone(),
            Account {
                id: id.clone(),
                avatar_url,
                key_name,
            },
        );

        id
    }

    fn get_all_projects(&self) -> Vec<Project> {
        // TODO(xla): Return proper error.
        self.registry_client
            .list_projects()
            .wait()
            .expect("osc client list projects failed")
            .into_iter()
            .take(10)
            .flat_map(|id| {
                let maybe_project = self
                    .registry_client
                    .get_project(id)
                    .wait()
                    .expect("get_project failed");

                match maybe_project {
                    Some(project) => Some(self.enrich_members(Project::try_from(project).unwrap())),
                    None => None,
                }
            })
            .collect()
    }

    fn get_project(&self, id: ProjectId) -> Option<Project> {
        let maybe_project = self
            .registry_client
            .get_project(id.try_into().unwrap())
            .wait()
            .expect("get project failed");

        match maybe_project {
            Some(p) => Some(self.enrich_members(Project::try_from(p).unwrap())),
            None => None,
        }
    }

    fn register_project(&self, name: String, description: String, img_url: String) -> Project {
        let (sender, _, _) = radicle_registry_client::ed25519::Pair::generate_with_phrase(None);

        let project_name =
            ProjectName::from_string(name.to_string()).expect("project name creation failed");
        let project_domain =
            ProjectDomain::from_string("rad".to_string()).expect("project domain creation faile");
        let registry_id = (project_name, project_domain);

        // TODO(xla): Proper error handling.
        self.registry_client
            .register_project(
                &sender,
                radicle_registry_client::RegisterProjectParams {
                    id: registry_id.clone(),
                    description: description.to_owned(),
                    img_url: img_url.to_owned(),
                    checkpoint_id: H256::random(),
                },
            )
            .wait()
            .expect("osc project registration failed");

        Project {
            id: registry_id.try_into().unwrap(),
            name,
            description,
            img_url,
            members: vec![],
        }
    }
}

#[allow(dead_code)]
/// Populate a `Source` with a set of initial projects.
pub fn setup_fixtures<S: Source + Send + Sync>(source: &mut S) {
    let _ = source.create_account(
        "xla".into(),
        "https://avatars0.githubusercontent.com/u/1585".into(),
    );
    let _ = source.register_project(
        "monokel".into(),
        "A looking glass into the future".into(),
        "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg".into(),
    );

    let _ = source.register_project(
        "Monadic".into(),
        "Open source organization of amazing things.".into(),
        "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg"
            .into(),
    );

    let _ = source.register_project(
        "open source coin".into(),
        "Research for the open source community.".into(),
        "https://avatars0.githubusercontent.com/u/31632242".into(),
    );

    let _ = source.register_project(
        "radicle".into(),
        "Decentralized open source collaboration".into(),
        "https://avatars0.githubusercontent.com/u/48290027".into(),
    );
}
