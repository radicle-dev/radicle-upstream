//! Abstractions and types to handle, persist and interact with transactions.

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::SystemTime;

use radicle_registry_client as protocol;

use crate::error;
use crate::registry;

/// A container to dissiminate and apply operations on the [`Registry`].
#[derive(Clone, Debug)]
pub struct Transaction {
    /// Unique identifier, in actuality the Hash of the transaction. This handle should be used by
    /// the API consumer to query state changes of a transaction.
    pub id: protocol::TxHash,
    /// List of operations to be applied to the Registry. Currently limited to exactly one. We use
    /// a Vec here to accommodate future extensibility.
    pub messages: Vec<Message>,
    /// Current state of the transaction.
    pub state: State,
    /// Creation time.
    pub timestamp: SystemTime,
}

/// Possible messages a [`Transaction`] can carry.
#[derive(Clone, Debug)]
pub enum Message {
    /// Issue a new org registration with a given id.
    OrgRegistration(registry::Id),

    /// Issue an org unregistration with a given id.
    OrgUnregistration(registry::Id),

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
#[derive(Clone, Debug)]
pub enum State {
    /// [`Transaction`] has been applied to a block, carries the hash of the block.
    Applied(protocol::Hash),
}

/// Behaviour to manage and persist observed [`Transaction`].
#[async_trait]
pub trait Cache: Send + Sync {
    /// Caches a transaction locally in the Registry.
    async fn cache_transaction(&mut self, tx: Transaction) -> Result<(), error::Error>;

    /// Returns all cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn list_transactions(
        &self,
        ids: Vec<protocol::TxHash>,
    ) -> Result<Vec<Transaction>, error::Error>;
}

/// Cacher persists and manages observed transactions.
#[derive(Clone)]
pub struct Cacher<C>
where
    C: registry::Client,
{
    /// The [`registry::Client`] to store the returned transactions.
    client: C,
    /// Cached transactions.
    transactions: HashMap<protocol::TxHash, Transaction>,
}

impl<C> Cacher<C>
where
    C: registry::Client,
{
    /// Cacher persists and manages observed transactions.
    pub fn new(client: C) -> Self {
        Self {
            client,
            transactions: HashMap::new(),
        }
    }
}

#[async_trait]
impl<C> Cache for Cacher<C>
where
    C: registry::Client,
{
    /// Caches a transaction locally in the Registry.
    async fn cache_transaction(&mut self, tx: Transaction) -> Result<(), error::Error> {
        self.transactions.insert(tx.id, tx);

        Ok(())
    }

    /// Returns all cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs.
    async fn list_transactions(
        &self,
        ids: Vec<protocol::TxHash>,
    ) -> Result<Vec<Transaction>, error::Error> {
        Ok(self
            .transactions
            .values()
            .cloned()
            .filter(|tx| {
                if ids.is_empty() {
                    true
                } else {
                    ids.contains(&tx.id)
                }
            })
            .collect::<Vec<Transaction>>())
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

    async fn list_orgs(&self, user_id: String) -> Result<Vec<registry::Org>, error::Error> {
        self.client.list_orgs(user_id).await
    }

    async fn register_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.client.register_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone()).await?;

        Ok(tx)
    }

    async fn unregister_org(
        &mut self,
        author: &protocol::ed25519::Pair,
        org_id: String,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.unregister_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone()).await?;

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

        self.cache_transaction(tx.clone()).await?;

        Ok(tx)
    }

    async fn get_user(&self, handle: String) -> Result<Option<registry::User>, error::Error> {
        self.client.get_user(handle).await
    }

    async fn register_user(
        &mut self,
        author: &protocol::ed25519::Pair,
        handle: String,
        id: Option<String>,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.client.register_user(author, handle, id, fee).await?;

        self.cache_transaction(tx.clone()).await?;

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
        self.transactions = HashMap::new();
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
        let client = protocol::Client::new_emulator();
        let registry = registry::Registry::new(client);
        let mut cache = Cacher::new(registry);

        let tx = Transaction {
            id: protocol::TxHash::random(),
            messages: vec![],
            state: State::Applied(protocol::Hash::random()),
            timestamp: time::SystemTime::now(),
        };

        cache.cache_transaction(tx.clone()).await.unwrap();

        for _ in 0..9 {
            let tx = Transaction {
                id: protocol::TxHash::random(),
                messages: vec![],
                state: State::Applied(protocol::Hash::random()),
                timestamp: time::SystemTime::now(),
            };

            cache.cache_transaction(tx.clone()).await.unwrap();
        }

        // Get all transactions.
        {
            let txs = cache.list_transactions(Vec::new()).await.unwrap();
            assert_eq!(txs.len(), 10);
        }

        // Get single transaction.
        {
            let txs = cache.list_transactions(vec![tx.id]).await.unwrap();
            assert_eq!(txs.len(), 1);
        }

        // Filter and get none.
        {
            let txs = cache
                .list_transactions(vec![protocol::TxHash::random()])
                .await
                .unwrap();
            assert_eq!(txs.len(), 0);
        }
    }
}
