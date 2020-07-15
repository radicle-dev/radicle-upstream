//! Endpoints and serialisaton for [`registry::User`] related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::notification;
use crate::registry;

/// Prefixed filter
pub fn routes<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("users").and(
        list_orgs_filter(Arc::clone(&registry))
            .or(register_project_filter(
                Arc::clone(&registry),
                subscriptions.clone(),
            ))
            .or(get_filter(Arc::clone(&registry)))
            .or(register_filter(
                Arc::clone(&registry),
                store,
                subscriptions.clone(),
            ))
            .or(transfer_filter(registry, subscriptions)),
    )
}

/// Combination of all user filters.
#[cfg(test)]
fn filters<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_orgs_filter(Arc::clone(&registry))
        .or(register_project_filter(
            Arc::clone(&registry),
            subscriptions.clone(),
        ))
        .or(get_filter(Arc::clone(&registry)))
        .or(register_filter(
            Arc::clone(&registry),
            store,
            subscriptions.clone(),
        ))
        .or(transfer_filter(registry, subscriptions))
}

/// GET /<handle>
fn get_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_shared(registry))
        .and(document::param::<String>(
            "handle",
            "ID of the user to query for",
        ))
        .and(path::end())
        .and(document::document(document::description("Fetch a User")))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::User::document()).mime("application/json"),
            )
            .description("User with the given id"),
        ))
        .and_then(handler::get)
}

/// POST /
fn register_filter<R: registry::Client>(
    registry: http::Shared<R>,
    store: Arc<RwLock<kv::Store>>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path::end())
        .and(http::with_shared(registry))
        .and(http::with_store(store))
        .and(http::with_subscriptions(subscriptions))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a handle on the Registry",
        )))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(document::response(
            201,
            document::body(registry::Transaction::document()).mime("application/json"),
        )))
        .and_then(handler::register)
}

/// `GET /<handle>/orgs`
fn list_orgs_filter<R: registry::Client>(
    registry: http::Shared<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_shared(registry))
        .and(document::param::<String>(
            "handle",
            "ID of the user to query for",
        ))
        .and(path("orgs"))
        .and(document::document(document::description(
            "List all orgs the user is a member of",
        )))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(registry::Org::document())).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::list_orgs)
}

/// `POST /<id>/projects/<name>`
fn register_project_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_shared(registry)
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(document::param::<String>(
            "handle",
            "ID of the user under which to register the project",
        ))
        .and(path("projects"))
        .and(document::param::<String>(
            "project_name",
            "Name of the project",
        ))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new project under the user",
        )))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::body(http::RegisterProjectInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::User::document()).mime("application/json"),
            )
            .description("Registration succeeded"),
        ))
        .and_then(handler::register_project)
}

/// `POST /<id>/transfer`
fn transfer_filter<R: registry::Client>(
    registry: http::Shared<R>,
    subscriptions: notification::Subscriptions,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client,
{
    http::with_shared(registry)
        .and(http::with_subscriptions(subscriptions))
        .and(warp::post())
        .and(document::param::<registry::Id>(
            "handle",
            "ID of the user to transfer funds from",
        ))
        .and(path("transfer"))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description("Transfer funds")))
        .and(document::document(document::tag("User")))
        .and(document::document(
            document::body(TransferInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Transaction::document()).mime("application/json"),
            )
            .description("Transfer succeeded"),
        ))
        .and_then(handler::transfer)
}

/// User handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::error::Error;
    use crate::http;
    use crate::notification;
    use crate::registry;
    use crate::session;

    /// Get the [`registry::User`] for the given `handle`.
    pub async fn get<R: registry::Client>(
        registry: http::Shared<R>,
        handle: String,
    ) -> Result<impl Reply, Rejection> {
        let handle = registry::Id::try_from(handle).map_err(Error::from)?;
        let user = registry.read().await.get_user(handle).await?;
        Ok(reply::json(&user))
    }

    /// List the orgs the user is a member of.
    pub async fn list_orgs<R: registry::Client>(
        registry: http::Shared<R>,
        handle: String,
    ) -> Result<impl Reply, Rejection> {
        let reg = registry.read().await;
        let handle = registry::Id::try_from(handle).map_err(Error::from)?;
        let orgs = reg.list_orgs(handle).await?;

        Ok(reply::json(&orgs))
    }

    /// Register a user on the Registry.
    pub async fn register<R: registry::Client>(
        registry: http::Shared<R>,
        store: Arc<RwLock<kv::Store>>,
        subscriptions: notification::Subscriptions,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let handle = registry::Id::try_from(input.handle).map_err(Error::from)?;
        let reg = registry.write().await;
        let tx = reg
            .register_user(
                &fake_pair,
                handle.clone(),
                input.maybe_entity_id,
                input.transaction_fee,
            )
            .await?;

        // TODO(xla): This should only happen once the corresponding tx is confirmed.
        // Store registered user in session.
        let store = store.read().await;
        session::set_handle(&store, handle)?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Register a project in the Registry.
    pub async fn register_project<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        handle: String,
        project_name: String,
        input: http::RegisterProjectInput,
    ) -> Result<impl Reply, Rejection> {
        http::register_project(
            registry,
            subscriptions,
            registry::DomainType::User,
            handle,
            project_name,
            input,
        )
        .await
    }

    /// Transfer funds to the given `recipient`.
    pub async fn transfer<R: registry::Client>(
        registry: http::Shared<R>,
        subscriptions: notification::Subscriptions,
        _handle: registry::Id,
        input: super::TransferInput,
    ) -> Result<impl Reply, Rejection> {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let reg = registry.write().await;
        let tx = reg
            .transfer_from_user(
                &fake_pair,
                input.recipient,
                input.balance,
                input.transaction_fee,
            )
            .await?;

        subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

impl ToDocumentedType for registry::User {
    fn document() -> document::DocumentedType {
        let mut props = HashMap::with_capacity(3);
        props.insert(
            "handle".into(),
            document::string()
                .description("Handle/ID of the User to be registered under")
                .example("cloudhead"),
        );
        props.insert(
            "maybeEntityId".into(),
            document::string()
                .description("Exisiting entity id for attestion")
                .example("cloudhead@123abcd.git")
                .nullable(true),
        );
        props.insert(
            "accountId".into(),
            document::string()
                .description("Public key of the account associated with the user")
                .example("5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"),
        );

        document::DocumentedType::from(props)
            .description("Input for User registration")
            .nullable(true)
    }
}

/// Bundled input data for user registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Handle the User registered under.
    handle: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
    /// Optionally passed entity id to store for attestion.
    maybe_entity_id: Option<String>,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut props = HashMap::with_capacity(2);
        props.insert(
            "handle".into(),
            document::string()
                .description("Handle/ID of the User to be registered under")
                .example("cloudhead"),
        );
        props.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );
        props.insert(
            "maybeEntityId".into(),
            document::string()
                .description("Exisiting project id for attestion")
                .example("cloudhead@123abcd.git")
                .nullable(true),
        );

        document::DocumentedType::from(props).description("Input for User registration")
    }
}

/// Bundled input data for transfer.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInput {
    /// Account id of the recipient.
    recipient: radicle_registry_client::ed25519::Public,
    /// Amount that is transferred.
    balance: registry::Balance,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for TransferInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "recipient".into(),
            document::string()
                .description("Account id of the recipient")
                .example("5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"),
        );
        properties.insert(
            "balance".into(),
            document::string()
                .description("Amount that is transferred")
                .example(100),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for transferring funds")
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use radicle_registry_client::CryptoPair;
    use serde_json::{json, Value};
    use std::convert::TryFrom;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use warp::http::StatusCode;
    use warp::test::request;

    use librad::keys::SecretKey;
    use radicle_registry_client as protocol;

    use crate::avatar;
    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::notification;
    use crate::registry::{self, Cache as _, Client as _};

    #[tokio::test]
    async fn get() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let subscriptions = notification::Subscriptions::default();

        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead").unwrap();

        let _tx = registry
            .write()
            .await
            .register_user(&author, handle.clone(), None, 100)
            .await
            .unwrap();

        let api = super::filters(Arc::clone(&registry), store, subscriptions);
        let res = request()
            .method("GET")
            .path(&format!("/{}", handle.clone()))
            .reply(&api)
            .await;

        let user = registry
            .read()
            .await
            .get_user(handle.clone())
            .await?
            .unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "accountId": user.account_id,
                "handle": "cloudhead",
                "maybeEntityId": Value::Null,
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_orgs() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir()?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            Arc::new(RwLock::new(registry::Registry::new(client)))
        };
        let store = Arc::new(RwLock::new(
            kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap(),
        ));
        let subscriptions = notification::Subscriptions::default();
        let api = super::filters(Arc::clone(&registry), store, subscriptions);

        // Register the user
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead").map_err(Error::from)?;
        let org_id = registry::Id::try_from("radicle").map_err(Error::from)?;

        registry
            .write()
            .await
            .register_user(&author, handle.clone(), Some("123abcd.git".into()), 100)
            .await?;

        let user = registry
            .read()
            .await
            .get_user(handle.clone())
            .await?
            .unwrap();

        // Register the org
        registry
            .write()
            .await
            .register_org(&author, org_id.clone(), 100)
            .await?;

        let org = registry
            .read()
            .await
            .get_org(org_id.clone())
            .await?
            .unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/{}/orgs", handle))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!([registry::Org {
                id: org_id.clone(),
                shareable_entity_identifier: format!("%{}", org_id.to_string()),
                account_id: org.account_id,
                avatar_fallback: avatar::Avatar::from(&org_id.to_string(), avatar::Usage::Org),
                members: vec![user]
            }])
        );

        Ok(())
    }

    #[tokio::test]
    async fn register() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::clone(&cache),
            Arc::new(RwLock::new(store)),
            subscriptions,
        );

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::RegisterInput {
                handle: "cloudhead".into(),
                maybe_entity_id: Some("cloudhead@123abcd.git".into()),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        let txs = cache.read().await.list_transactions(vec![]).unwrap();
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));
    }

    #[allow(clippy::panic)]
    #[tokio::test]
    async fn register_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir()?;
        let key = SecretKey::new();
        let config = coco::config::default(key.clone(), tmp_dir.path())?;
        let peer = coco::create_peer_api(config).await?;
        let owner = coco::init_user(&peer, key, "cloudhead")?;
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::clone(&cache),
            Arc::new(RwLock::new(store)),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let urn = coco::Urn::new(
            owner.root_hash().clone(),
            librad::uri::Protocol::Git,
            librad::uri::Path::new(),
        );

        // Register user
        cache
            .read()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register project
        let project_name = "upstream";

        let res = request()
            .method("POST")
            .path(&format!("/{}/projects/{}", handle, project_name))
            .json(&http::RegisterProjectInput {
                maybe_coco_id: Some(urn.to_string()),
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let txs = cache.read().await.list_transactions(vec![])?;
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(have, json!(tx));

        let tx_msg = tx.messages.first().unwrap();
        match tx_msg {
            registry::Message::ProjectRegistration {
                project_name,
                domain_type,
                domain_id,
            } => {
                assert_eq!(
                    project_name.clone(),
                    registry::ProjectName::try_from("upstream").unwrap()
                );
                assert_eq!(domain_type.clone(), registry::DomainType::User);
                assert_eq!(domain_id.clone(), handle);
            },
            _ => panic!("The tx message is an unexpected variant."),
        }

        Ok(())
    }

    #[tokio::test]
    async fn transfer() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().unwrap();
        let registry = {
            let (client, _) = radicle_registry_client::Client::new_emulator();
            registry::Registry::new(client)
        };
        let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store"))).unwrap();
        let cache = Arc::new(RwLock::new(registry::Cacher::new(registry, &store)));
        let subscriptions = notification::Subscriptions::default();

        let api = super::filters(
            Arc::clone(&cache),
            Arc::new(RwLock::new(store)),
            subscriptions,
        );
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;

        // Register the user
        cache
            .write()
            .await
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register a second user
        let author2 = radicle_registry_client::ed25519::Pair::from_legacy_string("//Bob", None);
        let handle2 = registry::Id::try_from("bob")?;
        cache
            .write()
            .await
            .register_user(&author2, handle2.clone(), None, 10)
            .await?;

        // Transfer tokens from alice to bob
        let balance: registry::Balance = 10;
        let res = request()
            .method("POST")
            .path(&format!("/{}/transfer", handle))
            .json(&super::TransferInput {
                recipient: author2.public(),
                balance,
                transaction_fee: registry::MINIMUM_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
