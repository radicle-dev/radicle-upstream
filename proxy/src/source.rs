use futures::Future;
use std::collections::HashMap;

/// Newtype for the registry `oscoin_client::AccountId`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct AccountId(pub oscoin_client::AccountId);

/// Newtype for the registry `oscoin_client::ProjectId`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ProjectId(pub oscoin_client::ProjectId);

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

impl From<oscoin_ledger::interface::Project> for Project {
    fn from(p: oscoin_ledger::interface::Project) -> Self {
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
            id: ProjectId(p.id),
            name: p.name,
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
pub struct Ledger {
    /// Ledger client.
    client: oscoin_client::Client,
    /// Mapping of `AccountId`s to `Account`s for easier metadata enrichment.
    accounts: HashMap<AccountId, Account>,
}

impl Ledger {
    /// Returns a new `Ledger`.
    pub fn new(client: oscoin_client::Client) -> Self {
        Self {
            client,
            accounts: HashMap::new(),
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

impl Source for Ledger {
    fn get_all_projects(&self) -> Vec<Project> {
        // TODO(xla): Return proper error.
        self.client
            .list_projects()
            .wait()
            .expect("osc client list projects failed")
            .into_iter()
            .map(Project::from)
            .map(|p| self.enrich_members(p))
            .collect()
    }

    fn get_project(&self, id: ProjectId) -> Option<Project> {
        // TODO(xla): Bubble up errors from QueryResult.
        match self.client.get_project(id.0).wait() {
            Ok(maybe_project) => match maybe_project {
                Some(p) => Some(self.enrich_members(Project::from(p))),
                None => None,
            },
            Err(_err) => None,
        }
    }

    fn register_project(&self, name: String, description: String, img_url: String) -> Project {
        let sender = self
            .client
            .new_account()
            .wait()
            .expect("osc new account failed");

        // TODO(xla): Proper error handling.
        let project_id = self
            .client
            .register_project(
                sender,
                name.to_owned(),
                description.to_owned(),
                img_url.to_owned(),
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

pub mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    use crate::source::{Account, AccountId, Project, ProjectId, Source};

    pub struct Local {
        projects: Arc<RwLock<HashMap<ProjectId, Project>>>,
    }

    impl Local {
        pub fn new() -> Self {
            let mut projects = HashMap::new();

            let id = ProjectId(*oscoin_client::Address::random().as_fixed_bytes());
            projects.insert(id, Project{
            id,
            name: "monokel".to_owned(),
            description: "A looking glass into the future".to_owned(),
            img_url: "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg".to_owned(),
            members: vec![
                Account{
                    id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                    key_name: "xla".to_owned(),
                    avatar_url: "https://avatars0.githubusercontent.com/u/1585".to_owned(),
                },
            ],
        });

            let id = ProjectId(*oscoin_client::Address::random().as_fixed_bytes());
            projects.insert(id, Project{
            id,
            name: "Monadic".to_owned(),
            description: "Open source organization of amazing things".to_owned(),
            img_url: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg".to_owned(),
            members: vec![
                Account{
                    id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                    key_name: "cloudhead".to_owned(),
                    avatar_url: "https://avatars1.githubusercontent.com/u/40774".to_owned(),
                },
                Account{
                    id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                    key_name: "lftherios".to_owned(),
                    avatar_url: "https://avatars3.githubusercontent.com/u/853825".to_owned(),
                },
                Account{
                    id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                    key_name: "juliendonck".to_owned(),
                    avatar_url: "https://avatars2.githubusercontent.com/u/2326909".to_owned(),
                },
            ],
        });

            let id = ProjectId(*oscoin_client::Address::random().as_fixed_bytes());
            projects.insert(
                id,
                Project {
                    id,
                    name: "open source coin".to_owned(),
                    description: "Infrastructure for the open source communit".to_owned(),
                    img_url: "https://avatars0.githubusercontent.com/u/31632242".to_owned(),
                    members: vec![
                        Account {
                            id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                            key_name: "geigerzaehler".to_owned(),
                            avatar_url: "https://avatars2.githubusercontent.com/u/3919579"
                                .to_owned(),
                        },
                        Account {
                            id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                            key_name: "rockbmb".to_owned(),
                            avatar_url: "https://avatars2.githubusercontent.com/u/16455833"
                                .to_owned(),
                        },
                        Account {
                            id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                            key_name: "rudolfs".to_owned(),
                            avatar_url: "https://avatars1.githubusercontent.com/u/158411"
                                .to_owned(),
                        },
                    ],
                },
            );

            let id = ProjectId(*oscoin_client::Address::random().as_fixed_bytes());
            projects.insert(
                id,
                Project {
                    id,
                    name: "radicle".to_owned(),
                    description: "Decentralized open source collaboration".to_owned(),
                    img_url: "https://avatars0.githubusercontent.com/u/48290027".to_owned(),
                    members: vec![Account {
                        id: AccountId(*oscoin_client::Address::random().as_fixed_bytes()),
                        key_name: "jkarni".to_owned(),
                        avatar_url: "https://avatars3.githubusercontent.com/u/1657498".to_owned(),
                    }],
                },
            );

            Self {
                projects: Arc::new(RwLock::new(projects)),
            }
        }
    }

    impl Source for Local {
        fn get_all_projects(&self) -> Vec<Project> {
            let projects = self
                .projects
                .read()
                .expect("acquiring read lock on projects failed");

            let mut ps: Vec<Project> = projects.iter().map(|(_k, v)| v.clone()).collect();

            ps.sort_by(|a, b| a.name.partial_cmp(&b.name).expect("project compare failed"));

            ps.to_vec()
        }

        fn get_project(&self, id: ProjectId) -> Option<Project> {
            let projects = self
                .projects
                .read()
                .expect("acquiring read lock for projects failed");
            match projects.get(&id) {
                Some(p) => Some(p.clone()),
                None => None,
            }
        }

        fn register_project(&self, name: String, description: String, img_url: String) -> Project {
            let addr = oscoin_client::Address::random();
            let id = addr.as_fixed_bytes();

            let mut projects = self
                .projects
                .write()
                .expect("claiming projects write lock failed");
            let p = Project {
                id: ProjectId(*id),
                name,
                description,
                img_url,
                members: vec![],
            };
            projects.insert(ProjectId(*id), p.clone());

            p
        }
    }
}
