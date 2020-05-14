//! Integrations with the radicle Registry.

#![allow(clippy::empty_line_after_outer_attr)]

use async_trait::async_trait;
use hex::ToHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_cbor::from_reader;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

use radicle_registry_client::{self as protocol, ClientT, CryptoPair};

use crate::avatar;
use crate::error;

mod transaction;
pub use transaction::{Cache, Cacher, Message, State, Transaction};

/// Wrapper for [`protocol::Id`] to add serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct Id(protocol::Id);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl TryFrom<String> for Id {
    type Error = error::Error;

    fn try_from(input: String) -> Result<Self, error::Error> {
        Ok(Self(protocol::Id::try_from(input)?))
    }
}

impl TryFrom<&str> for Id {
    type Error = error::Error;

    fn try_from(input: &str) -> Result<Self, error::Error> {
        Ok(Self(protocol::Id::try_from(input)?))
    }
}

// TODO(xla): This should go into the radicle-registry.
impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(|_err| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &"a Registry Id")
        })
    }
}

// TODO(xla): This should go into the radicle-registry.
impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

/// Wrapper for [`protocol::ProjectName`] to add serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectName(protocol::ProjectName);

impl fmt::Display for ProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for ProjectName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(|_err| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"a Registry ProjectName",
            )
        })
    }
}

impl Serialize for ProjectName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl TryFrom<String> for ProjectName {
    type Error = error::Error;

    fn try_from(input: String) -> Result<Self, error::Error> {
        Ok(Self(protocol::ProjectName::try_from(input)?))
    }
}

impl TryFrom<&str> for ProjectName {
    type Error = error::Error;

    fn try_from(input: &str) -> Result<Self, error::Error> {
        Ok(Self(protocol::ProjectName::try_from(input)?))
    }
}

/// Wrapper for [`protocol::Hash`] to add serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct Hash(pub protocol::Hash);

// TODO(xla): This should go into the radicle-registry.
impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        let hash = protocol::TxHash::from_str(s).map_err(|err| {
            serde::de::Error::custom(err)
            // serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &"a TxHash")
        })?;

        Ok(Self(hash))
    }
}

// TODO(xla): This should go into the radicle-registry.
impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.encode_hex::<String>())
    }
}

/// `ProjectID` wrapper for serde de/serialization
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Librad project ID.
    pub id: String,
    /// Metadata version.
    pub version: u8,
}

/// Configured thresholds for acceptance criteria of transaction progress.
pub struct Thresholds {
    /// Number of blocks after which a [`Transaction`] is assumed to be confirmed.
    pub confirmation: u64,
    /// Number of blocks after which a [`Transaction`] is assumed to be settled.
    pub settlement: u64,
}

/// The registered org with identifier and avatar
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    /// The unique identifier of the org
    pub id: String,
    /// Generated fallback avatar
    pub avatar_fallback: avatar::Avatar,
    /// List of members of the org
    pub members: Vec<User>,
}

/// A project registered under an [`Org`] or [`User`] on the Registry.
pub struct Project {
    /// Name of the project, unique under the top-level entity.
    pub name: ProjectName,
    /// Id of the top-level entity.
    pub org_id: Id,
    /// Optionally associated project id for attestation in other systems.
    pub maybe_project_id: Option<String>,
}

/// The registered user with associated coco id.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Unique handle regsistered on the Regisry.
    pub handle: Id,
    /// Associated entity id for attestion.
    pub maybe_entity_id: Option<String>,
}

/// Methods to interact with the Registry in a uniform way.
#[async_trait]
pub trait Client: Clone + Send + Sync {
    /// Try to retrieve org from the Registry by id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn get_org(&self, id: String) -> Result<Option<Org>, error::Error>;

    /// List orgs of the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn list_orgs(&self, handle: Id) -> Result<Vec<Org>, error::Error>;

    /// Create a new unique Org on the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn register_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error>;

    /// Remove a registered Org from the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn unregister_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error>;

    /// Try to retrieve project from the Registry by name for an id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn get_project(
        &self,
        id: String,
        project_name: String,
    ) -> Result<Option<Project>, error::Error>;

    /// List all projects of the Registry for an org.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn list_org_projects(&self, id: String) -> Result<Vec<Project>, error::Error>;

    /// List projects of the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn list_projects(&self) -> Result<Vec<protocol::ProjectId>, error::Error>;

    /// Register a new project on the chain.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn register_project(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        project_name: String,
        maybe_project_id: Option<librad::project::ProjectId>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error>;

    /// Try to retrieve user from the Registry by handle.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn get_user(&self, handle: Id) -> Result<Option<User>, error::Error>;

    /// Create a new unique user on the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn register_user(
        &mut self,
        author: &protocol::ed25519::Pair,
        handle: Id,
        id: Option<String>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error>;

    /// Graciously pay some tokens to the recipient out of Alices pocket.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn prepay_account(
        &self,
        recipient: protocol::AccountId,
        balance: protocol::Balance,
    ) -> Result<(), error::Error>;

    /// Replaces the underlying client. Useful to reset the state of an emulator client, or connect
    /// to a different nework.
    fn reset(&mut self, client: protocol::Client);
}

/// Registry client wrapper.
#[derive(Clone)]
pub struct Registry {
    /// Registry client, whether an emulator or otherwise.
    client: protocol::Client,
}

/// Registry client wrapper methods
impl Registry {
    /// Wraps a registry client.
    #[must_use]
    pub const fn new(client: protocol::Client) -> Self {
        Self { client }
    }

    /// Returns the configured thresholds for [`Transaction`] acceptance stages.
    #[must_use]
    pub const fn thresholds() -> Thresholds {
        Thresholds {
            confirmation: 3,
            settlement: 9,
        }
    }
}

#[async_trait]
impl Client for Registry {
    async fn get_org(&self, id: String) -> Result<Option<Org>, error::Error> {
        let org_id = protocol::Id::try_from(id.clone())?;
        if let Some(org) = self.client.get_org(org_id).await? {
            let mut members = Vec::new();
            for member in org.members.clone() {
                members.push(
                    self.get_user(Id(member))
                        .await?
                        .expect("Couldn't retrieve org member"),
                );
            }
            Ok(Some(Org {
                id: id.clone(),
                avatar_fallback: avatar::Avatar::from(&id, avatar::Usage::Org),
                members,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_orgs(&self, handle: Id) -> Result<Vec<Org>, error::Error> {
        let mut orgs = Vec::new();
        for org_id in &self.client.list_orgs().await? {
            let org = self
                .get_org(org_id.to_string())
                .await?
                .expect("org missing for id");
            if org.members.iter().any(|m| m.handle == handle) {
                orgs.push(org);
            }
        }

        Ok(orgs)
    }

    async fn register_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let org_id = protocol::Id::try_from(org_id.clone())?;

        // Prepare and submit org registration transaction.
        let register_message = protocol::message::RegisterOrg {
            org_id: org_id.clone(),
        };
        let register_tx = protocol::Transaction::new_signed(
            author,
            register_message,
            protocol::TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;
        let tx = Transaction {
            id: Hash(register_applied.tx_hash),
            messages: vec![Message::OrgRegistration {
                id: Id(org_id.clone()),
            }],
            state: transaction::State::Applied {
                block: Hash(register_applied.block),
            },
            timestamp: SystemTime::now(),
        };

        // TODO(xla): Remove automatic prepayment once we have proper balances.
        let org = self.client.get_org(org_id).await?.expect("org not present");
        self.prepay_account(org.account_id, 1000).await?;

        Ok(tx)
    }

    async fn unregister_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let org_id = protocol::Id::try_from(org_id.clone())?;

        // Prepare and submit org unregistration transaction.
        let unregister_message = protocol::message::UnregisterOrg {
            org_id: org_id.clone(),
        };
        let register_tx = protocol::Transaction::new_signed(
            author,
            unregister_message,
            protocol::TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let unregister_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: Hash(unregister_applied.tx_hash),
            messages: vec![Message::OrgUnregistration { id: Id(org_id) }],
            state: transaction::State::Applied {
                block: Hash(unregister_applied.block),
            },
            timestamp: SystemTime::now(),
        })
    }

    async fn get_project(
        &self,
        id: String,
        project_name: String,
    ) -> Result<Option<Project>, error::Error> {
        let org_id = protocol::Id::try_from(id.clone())?;
        let project_name = protocol::ProjectName::try_from(project_name)?;

        Ok(self
            .client
            .get_project(project_name, org_id)
            .await?
            .map(|project| {
                let metadata_vec: Vec<u8> = project.metadata.into();
                Project {
                    name: ProjectName(project.name),
                    org_id: Id(project.org_id),
                    maybe_project_id: if metadata_vec[..].is_empty() {
                        None
                    } else {
                        let maybe_metadata: Result<Metadata, serde_cbor::error::Error> =
                            from_reader(&metadata_vec[..]);
                        Some(maybe_metadata.expect("Could not read Metadata").id)
                    },
                }
            }))
    }

    async fn list_org_projects(&self, id: String) -> Result<Vec<Project>, error::Error> {
        let org_id = protocol::Id::try_from(id.clone())?;
        let project_ids = self.client.list_projects().await?.into_iter();
        let mut projects = Vec::new();
        for project_id in project_ids {
            if project_id.1 == org_id {
                projects.push(
                    self.get_project(org_id.to_string(), project_id.0.to_string())
                        .await?
                        .expect("Get project"),
                );
            }
        }
        Ok(projects)
    }

    async fn list_projects(&self) -> Result<Vec<protocol::ProjectId>, error::Error> {
        self.client.list_projects().await.map_err(|e| e.into())
    }

    async fn register_project(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        project_name: String,
        maybe_project_id: Option<librad::project::ProjectId>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        // Verify that inputs are valid.
        let org_id = protocol::Id::try_from(org_id.clone())?;
        let project_name = protocol::ProjectName::try_from(project_name.clone())?;

        // Prepare and submit checkpoint transaction.
        let checkpoint_message = protocol::message::CreateCheckpoint {
            project_hash: protocol::H256::random(),
            previous_checkpoint_id: None,
        };
        let checkpoint_tx = protocol::Transaction::new_signed(
            author,
            checkpoint_message,
            protocol::TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        let checkpoint_id = self
            .client
            .submit_transaction(checkpoint_tx)
            .await?
            .await?
            .result?;

        let register_metadata_vec = if let Some(pid_string) = maybe_project_id {
            let pid_cbor = Metadata {
                id: pid_string.to_string(),
                version: 1,
            };
            // TODO(garbados): unpanic
            serde_cbor::to_vec(&pid_cbor).expect("unable to serialize project metadata")
        } else {
            vec![]
        };

        // TODO: remove .expect() call, see: https://github.com/radicle-dev/radicle-registry/issues/185
        let register_metadata =
            protocol::Bytes128::from_vec(register_metadata_vec).expect("unable construct metadata");

        // Prepare and submit project registration transaction.
        let register_message = protocol::message::RegisterProject {
            project_name: project_name.clone(),
            org_id: org_id.clone(),
            checkpoint_id,
            metadata: register_metadata,
        };
        let register_tx = protocol::Transaction::new_signed(
            author,
            register_message,
            protocol::TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: Hash(register_applied.tx_hash),
            messages: vec![Message::ProjectRegistration {
                project_name: ProjectName(project_name),
                org_id: Id(org_id),
            }],
            state: transaction::State::Applied {
                block: Hash(register_applied.block),
            },
            timestamp: SystemTime::now(),
        })
    }

    async fn get_user(&self, handle: Id) -> Result<Option<User>, error::Error> {
        Ok(self
            .client
            .get_user(handle.0.clone())
            .await?
            .map(|_user| User {
                handle,
                maybe_entity_id: None,
            }))
    }

    async fn register_user(
        &mut self,
        author: &protocol::ed25519::Pair,
        handle: Id,
        id: Option<String>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        // Prepare and submit user registration transaction.
        let register_message = protocol::message::RegisterUser {
            user_id: handle.0.clone(),
        };
        let register_tx = protocol::Transaction::new_signed(
            author,
            register_message,
            protocol::TransactionExtra {
                genesis_hash: self.client.genesis_hash(),
                nonce: self.client.account_nonce(&author.public()).await?,
                fee,
            },
        );
        // TODO(xla): Unpack the result to find out if the application of the transaction failed.
        let register_applied = self.client.submit_transaction(register_tx).await?.await?;

        Ok(Transaction {
            id: Hash(register_applied.tx_hash),
            messages: vec![Message::UserRegistration { handle, id: id }],
            state: transaction::State::Applied {
                block: Hash(register_applied.block),
            },
            timestamp: SystemTime::now(),
        })
    }

    async fn prepay_account(
        &self,
        recipient: protocol::AccountId,
        balance: protocol::Balance,
    ) -> Result<(), error::Error> {
        let alice = protocol::ed25519::Pair::from_legacy_string("//Alice", None);

        let _tx_applied = self
            .client
            .sign_and_submit_message(
                &alice,
                protocol::message::Transfer { recipient, balance },
                1,
            )
            .await?;

        Ok(())
    }

    fn reset(&mut self, client: protocol::Client) {
        self.client = client;
    }
}

#[allow(
    clippy::indexing_slicing,
    clippy::panic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#[cfg(test)]
mod test {
    use radicle_registry_client::{self as protocol, ClientT};
    use serde_cbor::from_reader;
    use std::convert::TryFrom as _;

    use super::{Client, Id, Metadata, ProjectName, Registry};
    use crate::avatar;

    #[tokio::test]
    async fn test_register_org() {
        // Test that org registration submits valid transactions and they succeed.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        let result =
            futures::executor::block_on(registry.register_org(&author, "monadic".into(), 10));
        assert!(result.is_ok());

        let org_id = protocol::Id::try_from("monadic").unwrap();
        let maybe_org = client.get_org(org_id.clone()).await.unwrap();
        assert!(maybe_org.is_some());
        let org = maybe_org.unwrap();
        assert_eq!(org.id, org_id);
        assert_eq!(org.members[0], protocol::Id::try_from("alice").unwrap());
    }

    #[tokio::test]
    async fn test_unregister_org() {
        // Test that org unregistration submits valid transactions and they succeed.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        // Register the org
        let org_id = protocol::Id::try_from("monadic").unwrap();
        let registration = registry
            .register_org(&author, org_id.clone().into(), 10)
            .await;
        assert!(registration.is_ok());

        // Unregister the org
        let unregistration =
            futures::executor::block_on(registry.unregister_org(&author, "monadic".into(), 10));
        assert!(unregistration.is_ok());
    }

    #[tokio::test]
    async fn test_get_org() {
        // Test that a registered org can be retrieved.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        // Register the org
        let org_id = protocol::Id::try_from("monadic").unwrap();
        let registration = registry
            .register_org(&author, org_id.clone().into(), 10)
            .await;
        assert!(registration.is_ok());

        // Query the org
        let org = registry.get_org("monadic".into()).await.unwrap().unwrap();
        assert_eq!(org.id, "monadic");
        assert_eq!(
            org.avatar_fallback,
            avatar::Avatar::from("monadic", avatar::Usage::Org)
        );
    }

    #[tokio::test]
    async fn test_list_org() {
        // Test that a registered org can be retrieved.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle.clone(), Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        // Register the org
        let org_id = protocol::Id::try_from("monadic").unwrap();
        let org_registration = registry
            .register_org(&author, org_id.clone().into(), 10)
            .await;
        assert!(org_registration.is_ok());

        // List the orgs
        let orgs = registry.list_orgs(handle).await.unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].id, "monadic");
    }

    #[tokio::test]
    async fn test_list_org_projects() {
        // Test that a registered project is included in the list of org projects.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        // Register the org
        let org_id = protocol::Id::try_from("monadic").unwrap();
        let org_registration = registry
            .register_org(&author, org_id.clone().into(), 10)
            .await;
        assert!(org_registration.is_ok());

        // Register the project
        let result = registry
            .register_project(
                &author,
                org_id.into(),
                "radicle".into(),
                Some(librad::git::ProjectId::new(librad::surf::git::git2::Oid::zero()).into()),
                10,
            )
            .await;
        assert!(result.is_ok());

        // List the projects
        let projects = registry
            .list_org_projects("monadic".to_string())
            .await
            .unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, ProjectName::try_from("radicle").unwrap());
        assert_eq!(
            projects[0].maybe_project_id,
            Some("0000000000000000000000000000000000000000.git".to_string())
        );
    }

    #[tokio::test]
    async fn test_register_project() {
        // Test that project registration submits valid transactions and they succeed.
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client.clone());
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("alice").unwrap();

        // Register the user
        let user_registration = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(user_registration.is_ok());

        // Register the org
        let org_id = protocol::Id::try_from("monadic").unwrap();
        let org_result = registry
            .register_org(&author, org_id.clone().into(), 10)
            .await;
        assert!(org_result.is_ok());

        // Register the project
        let result = registry
            .register_project(
                &author,
                org_id.into(),
                "radicle".into(),
                Some(librad::git::ProjectId::new(librad::surf::git::git2::Oid::zero()).into()),
                10,
            )
            .await;
        assert!(result.is_ok());

        let org_id = protocol::Id::try_from("monadic").unwrap();
        let project_name = protocol::ProjectName::try_from("radicle").unwrap();
        let maybe_project = client
            .get_project(project_name.clone(), org_id.clone())
            .await
            .unwrap();

        assert!(maybe_project.is_some());

        let project = maybe_project.unwrap();
        assert_eq!(project.name, project_name);
        assert_eq!(project.org_id, org_id);
        let metadata_vec: Vec<u8> = project.metadata.into();
        let metadata: Metadata = from_reader(&metadata_vec[..]).unwrap();
        assert_eq!(metadata.version, 1);
    }

    #[tokio::test]
    async fn register_user() {
        let client = protocol::Client::new_emulator();
        let mut registry = Registry::new(client);
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = Id::try_from("cloudhead").unwrap();

        let res = registry
            .register_user(&author, handle, Some("123abcd.git".into()), 100)
            .await;
        assert!(res.is_ok());
    }
}
