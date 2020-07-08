//! Endpoints and serialisaton for [`registry::User`] related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::document::{self, ToDocumentedType};
use warp::{path, Filter, Rejection, Reply};

use crate::http;
use crate::registry;

/// Prefixed filter
pub fn routes<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("users").and(
        list_orgs_filter(ctx.clone())
            .or(register_project_filter(ctx.clone()))
            .or(get_filter(ctx.clone()))
            .or(register_filter(ctx)),
    )
}

/// Combination of all user filters.
#[cfg(test)]
fn filters<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    list_orgs_filter(ctx.clone())
        .or(register_project_filter(ctx.clone()))
        .or(get_filter(ctx.clone()))
        .or(register_filter(ctx))
}

/// GET /<handle>
fn get_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_context(ctx))
        .and(document::param::<String>(
            "handle",
            "ID of the user to query for",
        ))
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
fn register_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(http::with_context(ctx))
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
fn list_orgs_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(http::with_context(ctx))
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
fn register_project_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
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

/// User handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::convert::TryFrom;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::error::Error;
    use crate::http;
    use crate::notification;
    use crate::registry;
    use crate::session;

    /// Get the [`registry::User`] for the given `handle`.
    pub async fn get<R>(ctx: http::Ctx<R>, handle: String) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
    {
        let ctx = ctx.lock().await;

        let handle = registry::Id::try_from(handle).map_err(Error::from)?;
        let user = ctx.registry.get_user(handle).await?;
        Ok(reply::json(&user))
    }

    /// List the orgs the user is a member of.
    pub async fn list_orgs<R>(ctx: http::Ctx<R>, handle: String) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
    {
        let ctx = ctx.lock().await;

        let handle = registry::Id::try_from(handle).map_err(Error::from)?;
        let orgs = ctx.registry.list_orgs(handle).await?;

        Ok(reply::json(&orgs))
    }

    /// Register a user on the Registry.
    pub async fn register<R>(
        ctx: http::Ctx<R>,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
    {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let ctx = ctx.lock().await;
        let handle = registry::Id::try_from(input.handle).map_err(Error::from)?;
        let tx = ctx
            .registry
            .register_user(
                &fake_pair,
                handle.clone(),
                input.maybe_entity_id,
                input.transaction_fee,
            )
            .await?;

        // TODO(xla): This should only happen once the corresponding tx is confirmed.
        // Store registered user in session.
        session::set_handle(&ctx.store, handle)?;

        ctx.subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Register a project in the Registry.
    pub async fn register_project<R>(
        ctx: http::Ctx<R>,
        handle: String,
        project_name: String,
        input: http::RegisterProjectInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
    {
        http::register_project(ctx, registry::DomainType::User, handle, project_name, input).await
    }
}

impl ToDocumentedType for registry::User {
    fn document() -> document::DocumentedType {
        let mut props = HashMap::with_capacity(2);
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

        document::DocumentedType::from(props).description("Input for Uesr registration")
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::convert::TryFrom;
    use warp::http::StatusCode;
    use warp::test::request;

    use radicle_registry_client as protocol;

    use crate::avatar;
    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::registry::{self, Cache as _, Client as _};

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

        let ctx = ctx.lock().await;

        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead").unwrap();
        let fee: registry::Balance = 100;

        let _tx = ctx
            .registry
            .register_user(&author, handle.clone(), None, fee)
            .await
            .unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/{}", handle))
            .reply(&api)
            .await;

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            have,
            json!({
                "handle": "cloudhead",
                "maybeEntityId": Value::Null,
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn list_orgs() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

        let ctx = ctx.lock().await;

        // Register the user
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("cloudhead").map_err(error::Error::from)?;
        let org_id = registry::Id::try_from("radicle").map_err(error::Error::from)?;
        let fee: registry::Balance = 100;

        ctx.registry
            .register_user(&author, handle.clone(), Some("123abcd.git".into()), fee)
            .await?;

        let user = ctx.registry.get_user(handle.clone()).await?.unwrap();

        // Register the org
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

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
                avatar_fallback: avatar::Avatar::from(&org_id.to_string(), avatar::Usage::Org),
                members: vec![user]
            }])
        );

        Ok(())
    }

    #[tokio::test]
    async fn register() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

        let ctx = ctx.lock().await;

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

        let txs = ctx.registry.list_transactions(vec![])?;
        let tx = txs.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, json!(tx));

        Ok(())
    }

    #[allow(clippy::panic)]
    #[tokio::test]
    async fn register_project() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(tmp_dir).await?;
        let api = super::filters(ctx);

        let ctx = ctx.lock().await;

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_user(key, "cloudhead")?;
        let owner = coco::verify_user(owner)?;
        let urn = coco::Urn::new(
            owner.root_hash().clone(),
            librad::uri::Protocol::Git,
            librad::uri::Path::new(),
        );

        // Register user
        let fee: registry::Balance = 10;
        ctx.registry
            .register_user(&author, handle.clone(), None, fee)
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

        let txs = ctx.registry.list_transactions(vec![])?;
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
}
