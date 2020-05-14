//! Abstractions and types to handle, persist and interact with transactions.

use async_trait::async_trait;
use hex::ToHex;
use kv::Codec as _;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use radicle_registry_client as protocol;

use crate::error;
use crate::registry;

/// A container to dissiminate and apply operations on the [`Registry`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Unique identifier, in actuality the Hash of the transaction. This handle should be used by
    /// the API consumer to query state changes of a transaction.
    pub id: registry::Hash,
    /// List of operations to be applied to the Registry. Currently limited to exactly one. We use
    /// a Vec here to accommodate future extensibility.
    pub messages: Vec<Message>,
    /// Current state of the transaction.
    pub state: State,
    /// Creation time.
    pub timestamp: SystemTime,
}

/// Possible messages a [`Transaction`] can carry.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Message {
    /// Issue a new org registration.
    OrgRegistration {
        /// The [`registry::Org`] id.
        id: registry::Id,
    },

    /// Issue an org unregistration with a given id.
    OrgUnregistration {
        /// The [`registry::Org`] id.
        id: registry::Id,
    },

    /// Issue a new project registration with a given name under a given org.
    ProjectRegistration {
        /// Actual project name, unique for org.
        project_name: registry::ProjectName,
        /// The Org in which to register the project.
        org_id: registry::Id,
    },

    /// Issue a user registration for a given handle storing the corresponding identity id.
    UserRegistration {
        /// Globally unique user handle.
        handle: registry::Id,
        /// Identity id originated from librad.
        id: Option<String>,
    },
}

/// Possible states a [`Transaction`] can have. Useful to reason about the lifecycle and
/// whereabouts of a given [`Transaction`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum State {
    /// [`Transaction`] has been applied to a block, carries the hash of the block.
    Applied {
        /// The hash of the block the transaction has been applied to.
        block: registry::Hash,
    },
}

/// Behaviour to manage and persist observed [`Transaction`].
pub trait Cache: Send + Sync {
    /// Caches a transaction locally in the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if access to the underlying [`kv::Store`] fails.
    fn cache_transaction(&mut self, tx: Transaction) -> Result<(), error::Error>;

    /// Returns all cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if access to the underlying [`kv::Store`] fails.
    fn list_transactions(
        &self,
        ids: Vec<protocol::TxHash>,
    ) -> Result<Vec<Transaction>, error::Error>;
}

/// Storage bucket description for [`kv::Store`].
type Transactions = kv::Bucket<'static, &'static str, kv::Json<Transaction>>;

/// Cacher persists and manages observed transactions.
#[derive(Clone)]
pub struct Cacher<C>
where
    C: registry::Client,
{
    /// The [`registry::Client`] to observe the transactions to be stored.
    client: C,
    /// Cached transactions.
    transactions: Transactions,
}

impl<C> Cacher<C>
where
    C: registry::Client,
{
    /// Cacher persists and manages observed transactions.
    pub fn new(client: C, store: &kv::Store) -> Self {
        Self {
            client,
            transactions: store
                .bucket::<&'static str, kv::Json<Transaction>>(Some("transactions"))
                .expect("unable to get 'transactions' bucket"),
        }
    }
}

impl<C> Cache for Cacher<C>
where
    C: registry::Client,
{
    /// Caches a transaction locally in the Registry.
    fn cache_transaction(&mut self, tx: Transaction) -> Result<(), error::Error> {
        let key = tx.id.0.encode_hex::<String>();
        self.transactions.set(key.as_str(), kv::Json(tx))?;

        Ok(())
    }

    /// Returns all cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    fn list_transactions(
        &self,
        ids: Vec<protocol::TxHash>,
    ) -> Result<Vec<Transaction>, error::Error> {
        let mut txs = Vec::new();

        for item in self.transactions.iter() {
            let tx = item?.value::<kv::Json<Transaction>>()?.to_inner();

            if ids.is_empty() || ids.contains(&tx.id.0) {
                txs.push(tx);
            }
        }

        Ok(txs)
    }
}

#[async_trait]
impl<C> registry::Client for Cacher<C>
where
    C: registry::Client,
{
    async fn get_org(&self, id: String) -> Result<Option<registry::Org>, error::Error> {
        self.client.get_org(id).await
    }

    async fn list_orgs(&self, handle: registry::Id) -> Result<Vec<registry::Org>, error::Error> {
        self.client.list_orgs(handle).await
    }

    async fn register_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.client.register_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn unregister_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.unregister_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn get_project(
        &self,
        id: String,
        project_name: String,
    ) -> Result<Option<registry::Project>, error::Error> {
        self.client.get_project(id, project_name).await
    }

    async fn list_org_projects(&self, id: String) -> Result<Vec<registry::Project>, error::Error> {
        self.client.list_org_projects(id).await
    }

    async fn list_projects(&self) -> Result<Vec<protocol::ProjectId>, error::Error> {
        self.client.list_projects().await
    }

    async fn register_project(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        project_name: String,
        maybe_project_id: Option<librad::project::ProjectId>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self
            .client
            .register_project(author, org_id, project_name, maybe_project_id, fee)
            .await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn get_user(&self, handle: registry::Id) -> Result<Option<registry::User>, error::Error> {
        self.client.get_user(handle).await
    }

    async fn register_user(
        &mut self,
        author: &protocol::ed25519::Pair,
        handle: registry::Id,
        id: Option<String>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.client.register_user(author, handle, id, fee).await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn prepay_account(
        &self,
        recipient: protocol::AccountId,
        balance: protocol::Balance,
    ) -> Result<(), error::Error> {
        self.client.prepay_account(recipient, balance).await
    }

    fn reset(&mut self, client: protocol::Client) {
        self.client.reset(client);
        // self.transactions = HashMap::new();
    }
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use radicle_registry_client as protocol;
    use std::time;

    use super::{Cache, Cacher, State, Transaction};
    use crate::registry;

    #[tokio::test]
    async fn list_transactions() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

        {
            let client = protocol::Client::new_emulator();
            let registry = registry::Registry::new(client);
            let mut cache = Cacher::new(registry, &store);

            let tx = Transaction {
                id: registry::Hash(protocol::TxHash::random()),
                messages: vec![],
                state: State::Applied {
                    block: registry::Hash(protocol::Hash::random()),
                },
                timestamp: time::SystemTime::now(),
            };

            cache.cache_transaction(tx.clone()).unwrap();

            for _ in 0..9 {
                let tx = Transaction {
                    id: registry::Hash(protocol::TxHash::random()),
                    messages: vec![],
                    state: State::Applied {
                        block: registry::Hash(protocol::Hash::random()),
                    },
                    timestamp: time::SystemTime::now(),
                };

                cache.cache_transaction(tx.clone()).unwrap();
            }

            // Get all transactions.
            {
                let txs = cache.list_transactions(Vec::new()).unwrap();
                assert_eq!(txs.len(), 10);
            }

            // Get single transaction.
            {
                let txs = cache.list_transactions(vec![tx.id.0]).unwrap();
                assert_eq!(txs.len(), 1);
            }

            // Filter and get none.
            {
                let txs = cache
                    .list_transactions(vec![protocol::TxHash::random()])
                    .unwrap();
                assert_eq!(txs.len(), 0);
            }
        }

        // Test persistance.
        {
            let client = protocol::Client::new_emulator();
            let registry = registry::Registry::new(client);
            let cache = Cacher::new(registry, &store);

            let txs = cache.list_transactions(Vec::new()).unwrap();
            assert_eq!(txs.len(), 10);
        }
    }
}
