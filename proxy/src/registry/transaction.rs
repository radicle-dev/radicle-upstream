//! Abstractions and types to handle, persist and interact with transactions.

use async_trait::async_trait;
use hex::ToHex;
use kv::Codec as _;
use serde::{Deserialize, Serialize};
use std::time::{self, Duration, SystemTime};

use radicle_registry_client as protocol;

use crate::error;
use crate::registry;

/// Amount of blocks we assume to have been mined before a transaction is
/// considered to have settled.
pub const MIN_CONFIRMATIONS: u32 = 6;

/// Wrapper for [`SystemTime`] carrying the time since epoch.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    /// Seconds since unix epoch.
    secs: u64,
    /// Sub-second nanos part.
    nanos: u32,
}

impl Timestamp {
    /// Creates a new [`Timestamp`] at the current time.
    #[must_use]
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now
            .duration_since(time::UNIX_EPOCH)
            .expect("time should be after unix epoch");

        Self {
            nanos: duration.subsec_nanos(),
            secs: duration.as_secs(),
        }
    }
}

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
    pub timestamp: Timestamp,
}

impl Transaction {
    /// Constructs a new confirmed [`Transaction`] with a single [`Message`].
    #[must_use]
    pub fn confirmed(id: registry::Hash, block: protocol::BlockNumber, message: Message) -> Self {
        let now = Timestamp::now();

        Self {
            id,
            messages: vec![message],
            state: State::Confirmed {
                block,
                confirmations: 1,
                min_confirmations: MIN_CONFIRMATIONS,
                timestamp: now,
            },
            timestamp: now,
        }
    }
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
    /// [`Transaction`] has been applied to a block, carries the height of the block.
    #[serde(rename_all = "camelCase")]
    Confirmed {
        /// The height of the block the transaction has been applied to.
        block: protocol::BlockNumber,
        /// Amount of progress made towards settlement. We assume height+5 to
        /// be mathematically impossible to be reverted.
        confirmations: u32,
        /// Amount of blocks we assume to have been mined before a transaction is
        /// considered to have settled.
        min_confirmations: u32,
        /// Time when it was applied.
        timestamp: Timestamp,
    },

    /// [`Transaction`] failed to be applied or processed.
    // TODO(xla): Embbed original [`protocol::TransactionError`] and serialize it.
    #[serde(rename_all = "camelCase")]
    Failed {
        /// Description of the error that occurred.
        error: String,
        /// Time when it failed.
        timestamp: Timestamp,
    },

    /// [`Transaction`] has been send but not yet applied to a block.
    #[serde(rename_all = "camelCase")]
    Pending {
        /// Time when it was applied.
        timestamp: Timestamp,
    },

    /// [`Transaction`] has been settled on the network and is unlikely to be reverted.
    #[serde(rename_all = "camelCase")]
    Settled {
        /// Amount of blocks we assume to have been mined before a transaction is
        /// considered to have settled.
        min_confirmations: u32,
        /// Time when it settled.
        timestamp: Timestamp,
    },
}

/// Behaviour to manage and persist observed [`Transaction`].
pub trait Cache: Send + Sync {
    /// Clears the cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if access to the underlying [`kv::Store`] fails.
    fn clear(&self) -> Result<(), error::Error>;

    /// Caches a transaction locally in the Registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if access to the underlying [`kv::Store`] fails.
    fn cache_transaction(&self, tx: Transaction) -> Result<(), error::Error>;

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

    /// Starts up the machinery to check for latest block information and advance the state of
    /// cached transactions.
    ///
    /// # Errors
    ///
    /// Will return `Err` if a protocol error occurs or access to [`kv::Store`] fails.
    pub async fn run(&self) -> Result<(), error::Error> {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;

            self.advance(self.client.best_height().await?)?;
        }
    }

    /// Updates cached transactions progress given the latest height.
    fn advance(&self, best_height: protocol::BlockNumber) -> Result<(), error::Error> {
        let mut txs = self.list_transactions(vec![])?;

        for tx in &mut txs {
            match tx.state {
                State::Confirmed {
                    block, timestamp, ..
                } => {
                    let target = block
                        .checked_add(MIN_CONFIRMATIONS - 1)
                        .unwrap_or(MIN_CONFIRMATIONS);

                    if best_height >= target {
                        tx.state = State::Settled {
                            min_confirmations: MIN_CONFIRMATIONS,
                            timestamp: Timestamp::now(),
                        };
                    } else {
                        let offset = best_height
                            .checked_add(MIN_CONFIRMATIONS - 1)
                            .unwrap_or(MIN_CONFIRMATIONS);
                        let confirmations = offset.saturating_sub(target) + 1;

                        tx.state = State::Confirmed {
                            block,
                            confirmations,
                            min_confirmations: MIN_CONFIRMATIONS,
                            timestamp,
                        };
                    }

                    self.cache_transaction(tx.clone())?;
                },
                State::Failed { .. } | State::Pending { .. } | State::Settled { .. } => continue,
            }
        }

        Ok(())
    }
}

impl<C> Cache for Cacher<C>
where
    C: registry::Client,
{
    /// Clears the cached transactions.
    fn clear(&self) -> Result<(), error::Error> {
        Ok(self.transactions.clear()?)
    }

    /// Caches a transaction locally in the Registry.
    fn cache_transaction(&self, tx: Transaction) -> Result<(), error::Error> {
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
        txs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(txs)
    }
}

#[async_trait]
impl<C> registry::Client for Cacher<C>
where
    C: registry::Client,
{
    async fn best_height(&self) -> Result<u32, error::Error> {
        self.client.best_height().await
    }

    async fn get_org(&self, id: registry::Id) -> Result<Option<registry::Org>, error::Error> {
        self.client.get_org(id).await
    }

    async fn list_orgs(&self, handle: registry::Id) -> Result<Vec<registry::Org>, error::Error> {
        self.client.list_orgs(handle).await
    }

    async fn register_org(
        &self,
        author: &protocol::ed25519::Pair,
        org_id: registry::Id,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.client.register_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn unregister_org(
        &self,
        author: &protocol::ed25519::Pair,
        org_id: registry::Id,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self.unregister_org(author, org_id, fee).await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn register_member(
        &self,
        author: &protocol::ed25519::Pair,
        org_id: registry::Id,
        user_id: registry::Id,
        fee: protocol::Balance,
    ) -> Result<Transaction, error::Error> {
        let tx = self
            .client
            .register_member(author, org_id, user_id, fee)
            .await?;

        self.cache_transaction(tx.clone())?;

        Ok(tx)
    }

    async fn get_project(
        &self,
        org_id: registry::Id,
        project_name: registry::ProjectName,
    ) -> Result<Option<registry::Project>, error::Error> {
        self.client.get_project(org_id, project_name).await
    }

    async fn list_org_projects(
        &self,
        org_id: registry::Id,
    ) -> Result<Vec<registry::Project>, error::Error> {
        self.client.list_org_projects(org_id).await
    }

    async fn list_projects(&self) -> Result<Vec<protocol::ProjectId>, error::Error> {
        self.client.list_projects().await
    }

    async fn register_project(
        &self,
        author: &protocol::ed25519::Pair,
        org_id: registry::Id,
        project_name: registry::ProjectName,
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
        &self,
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
    }
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod test {
    use radicle_registry_client as protocol;

    use super::{Cache, Cacher, State, Timestamp, Transaction, MIN_CONFIRMATIONS};
    use crate::registry;

    #[tokio::test]
    async fn list_transactions() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();

        {
            let (client, _) = protocol::Client::new_emulator();
            let registry = registry::Registry::new(client);
            let cache = Cacher::new(registry, &store);
            let now = Timestamp::now();

            let tx = Transaction {
                id: registry::Hash(protocol::TxHash::random()),
                messages: vec![],
                state: State::Confirmed {
                    block: 1,
                    confirmations: 3,
                    min_confirmations: MIN_CONFIRMATIONS,
                    timestamp: now,
                },
                timestamp: now,
            };

            cache.cache_transaction(tx.clone()).unwrap();

            for height in 0..9 {
                let tx = Transaction {
                    id: registry::Hash(protocol::TxHash::random()),
                    messages: vec![],
                    state: State::Confirmed {
                        block: height,
                        confirmations: 2,
                        min_confirmations: MIN_CONFIRMATIONS,
                        timestamp: now,
                    },
                    timestamp: now,
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
            let (client, _) = protocol::Client::new_emulator();
            let registry = registry::Registry::new(client);
            let cache = Cacher::new(registry, &store);

            let txs = cache.list_transactions(Vec::new()).unwrap();
            assert_eq!(txs.len(), 10);
        }
    }
}
