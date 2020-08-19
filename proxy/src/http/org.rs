//! Endpoints for Org.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::avatar;
use crate::coco;
use crate::coco::signer;
use crate::http;
use crate::project;
use crate::registry;

/// Combination of all org routes.
pub fn filters<R, S>(ctx: http::Ctx<R, S>) -> BoxedFilter<(impl Reply,)>
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    get_filter(ctx.clone())
        .or(register_project_filter(ctx.clone()))
        .or(get_project_filter(ctx.clone()))
        .or(get_projects_filter(ctx.clone()))
        .or(register_filter(ctx.clone()))
        .or(register_member_filter(ctx.clone()))
        .or(transfer_filter(ctx))
        .boxed()
}

/// `GET /<id>`
fn get_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(document::param::<String>("id", "Unique ID of the Org"))
        .and(path::end())
        .and(document::document(document::description("Find Org by ID")))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("Org not found"),
        ))
        .and_then(handler::get)
}

/// `POST /<id>/projects/<name>`
fn register_project_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::post())
        .and(document::param::<registry::Id>(
            "org_id",
            "Unique ID of the Org",
        ))
        .and(path("projects"))
        .and(document::param::<registry::ProjectName>(
            "project_name",
            "Name of the project",
        ))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new project under the org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(http::RegisterProjectInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Registration succeeded"),
        ))
        .and_then(handler::register_project)
}

/// `GET /<id>/projects/<project_name>`
fn get_project_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(document::param::<String>("org_id", "Unique ID of the Org"))
        .and(path("projects"))
        .and(document::param::<String>(
            "project_name",
            "Name of the project",
        ))
        .and(path::end())
        .and(document::document(document::description(
            "Find Project for Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Project::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(http::error::Error::document()).mime("application/json"),
            )
            .description("Project not found"),
        ))
        .and_then(handler::get_project)
}

/// `GET /<id>/projects`
fn get_projects_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(document::param::<String>("org_id", "Unique ID of the Org"))
        .and(path("projects"))
        .and(path::end())
        .and(document::document(document::description(
            "Lists all Projects of the Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::response(
                200,
                document::body(registry::Project::document()).mime("application/json"),
            )
            .description("Successful retrieval"),
        ))
        .and_then(handler::get_projects)
}

/// `POST /`
fn register_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::post())
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a new unique Org",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(RegisterInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register)
}

/// `POST /<id>/members`
fn register_member_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::post())
        .and(document::param::<String>("id", "Unique ID of the Org"))
        .and(path("members"))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Register a member",
        )))
        .and(document::document(document::tag("Org")))
        .and(document::document(
            document::body(RegisterMemberInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(registry::Org::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::register_member)
}

/// `POST /<id>/transfer`
fn transfer_filter<R, S>(
    ctx: http::Ctx<R, S>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
    S: signer::Signer,
    S::Error: coco::SignError,
{
    http::with_context(ctx)
        .and(warp::post())
        .and(document::param::<registry::Id>(
            "id",
            "Unique ID of the Org",
        ))
        .and(path("transfer"))
        .and(path::end())
        .and(warp::body::json())
        .and(document::document(document::description("Transfer funds")))
        .and(document::document(document::tag("Org")))
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

/// Org handlers for conversion between core domain and http request fullfilment.
mod handler {
    use std::convert::TryFrom;
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::coco::signer;
    use crate::error;
    use crate::http;
    use crate::notification;
    use crate::project;
    use crate::registry;

    /// Get the Org for the given `id`.
    pub async fn get<R, S>(ctx: http::Ctx<R, S>, org_id: String) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(error::Error::from)?;
        let org = ctx.registry.get_org(org_id).await?;

        Ok(reply::json(&org))
    }

    /// Register a project in the Registry.
    pub async fn register_project<R, S>(
        ctx: http::Ctx<R, S>,
        org_id: registry::Id,
        project_name: registry::ProjectName,
        input: http::RegisterProjectInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        http::register_project(ctx, registry::DomainType::Org, org_id, project_name, input).await
    }

    /// Get the [`registry::Project`] under the given org id.
    pub async fn get_project<R, S>(
        ctx: http::Ctx<R, S>,
        org_id: String,
        project_name: String,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(error::Error::from)?;
        let project_domain = registry::ProjectDomain::Org(org_id);
        let project_name =
            registry::ProjectName::try_from(project_name).map_err(error::Error::from)?;
        let project = ctx
            .registry
            .get_project(project_domain, project_name)
            .await?;

        Ok(reply::json(&project))
    }

    /// Get all projects under the given org id.
    pub async fn get_projects<R, S>(
        ctx: http::Ctx<R, S>,
        org_id: String,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        let ctx = ctx.read().await;
        let org_id = registry::Id::try_from(org_id).map_err(error::Error::from)?;
        let projects = ctx.registry.list_org_projects(org_id).await?;
        let mut mapped_projects = Vec::new();
        for p in &projects {
            let maybe_project = if let Some(urn) = &p.maybe_project_id {
                Some(project::get(&ctx.peer_api, urn).expect("Project not found"))
            } else {
                None
            };

            let org_project = super::Project {
                name: p.name.to_string(),
                org_id: p.domain.id().to_string(),
                shareable_entity_identifier: format!(
                    "%{}/{}",
                    p.domain.id().to_string(),
                    p.name.to_string()
                ),
                maybe_project,
            };
            mapped_projects.push(org_project);
        }

        Ok(reply::json(&mapped_projects))
    }

    /// Register an org on the Registry.
    pub async fn register<R, S>(
        ctx: http::Ctx<R, S>,
        input: super::RegisterInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let ctx = ctx.read().await;
        let org_id = registry::Id::try_from(input.id).map_err(error::Error::from)?;
        let tx = ctx
            .registry
            .register_org(&fake_pair, org_id, input.transaction_fee)
            .await?;

        ctx.subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Register a member under an org on the Registry.
    pub async fn register_member<R, S>(
        ctx: http::Ctx<R, S>,
        id: String,
        input: super::RegisterMemberInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let ctx = ctx.read().await;
        let org_id = registry::Id::try_from(id).map_err(error::Error::from)?;
        let handle = registry::Id::try_from(input.handle).map_err(error::Error::from)?;
        let tx = ctx
            .registry
            .register_member(&fake_pair, org_id, handle, input.transaction_fee)
            .await?;

        ctx.subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }

    /// Transfer funds to the given `recipient`.
    pub async fn transfer<R, S>(
        ctx: http::Ctx<R, S>,
        id: registry::Id,
        input: super::TransferInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: registry::Client,
        S: signer::Signer,
        S::Error: coco::SignError,
    {
        // TODO(xla): Get keypair from persistent storage.
        let fake_pair = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);

        let ctx = ctx.read().await;
        let tx = ctx
            .registry
            .transfer_from_org(
                &fake_pair,
                id,
                input.recipient,
                input.amount,
                input.transaction_fee,
            )
            .await?;

        ctx.subscriptions
            .broadcast(notification::Notification::Transaction(tx.clone()))
            .await;

        Ok(reply::with_status(reply::json(&tx), StatusCode::CREATED))
    }
}

impl ToDocumentedType for registry::Org {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(4);
        properties.insert("avatarFallback".into(), avatar::Avatar::document());
        properties.insert(
            "id".into(),
            document::string()
                .description("The id of the org")
                .example("monadic"),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%monadic"),
        );
        properties.insert(
            "accountId".into(),
            document::string()
                .description("Public key of the account associated with the org")
                .example("5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu"),
        );
        properties.insert(
            "members".into(),
            document::array(registry::User::document()),
        );

        document::DocumentedType::from(properties).description("Org")
    }
}

impl Serialize for registry::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 3)?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("orgId", &self.domain.id().to_string())?;
        state.serialize_field("maybeProjectId", &self.maybe_project_id)?;

        state.end()
    }
}

impl ToDocumentedType for registry::Project {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the project")
                .example("upstream"),
        );
        properties.insert(
            "orgId".into(),
            document::string()
                .description("The id of the org")
                .example("radicle"),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%monadic/radicle-link"),
        );
        properties.insert(
            "maybeProjectId".into(),
            document::string()
                .description("The id project attested in coco")
                .example("123abdcd.git")
                .nullable(true),
        );

        document::DocumentedType::from(properties).description("Project")
    }
}

/// Object the API returns for a project that is registered under an org.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Id of the Org.
    org_id: String,
    /// Unambiguous identifier pointing at this identity.
    shareable_entity_identifier: String,
    /// Name of the project.
    name: String,
    /// Associated CoCo project.
    maybe_project: Option<project::Project>,
}

/// Bundled input data for org registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    /// Id of the Org.
    id: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for RegisterInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "id".into(),
            document::string()
                .description("ID of the org")
                .example("monadic"),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for org registration")
    }
}

/// Bundled input data for member registration.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMemberInput {
    /// Id of the User.
    handle: String,
    /// User specified transaction fee.
    transaction_fee: registry::Balance,
}

impl ToDocumentedType for RegisterMemberInput {
    fn document() -> document::DocumentedType {
        let mut properties = std::collections::HashMap::with_capacity(1);
        properties.insert(
            "handle".into(),
            document::string()
                .description("Handle of the user")
                .example("cloudhead"),
        );
        properties.insert(
            "transactionFee".into(),
            document::string()
                .description("User specified transaction fee")
                .example(100),
        );

        document::DocumentedType::from(properties).description("Input for member registration")
    }
}

/// Bundled input data for transfer.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInput {
    /// Account id of the recipient.
    recipient: radicle_registry_client::ed25519::Public,
    /// Amount that is transferred.
    amount: registry::Balance,
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
            "amount".into(),
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

#[allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use radicle_registry_client::CryptoPair;
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
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        let ctx = ctx.read().await;
        // Register the user
        let fee: registry::Balance = 10;
        ctx.registry
            .register_user(&author, handle.clone(), None, fee)
            .await?;

        let user = ctx.registry.get_user(handle).await?.unwrap();

        // Register the org
        let fee: registry::Balance = 100;
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        let org = ctx.registry.get_org(org_id.clone()).await?.unwrap();

        let res = request()
            .method("GET")
            .path(&format!("/{}", org_id.to_string()))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!(registry::Org {
                    id: org_id.clone(),
                    shareable_entity_identifier: format!("%{}", org_id.to_string()),
                    account_id: org.account_id,
                    avatar_fallback: avatar::Avatar::from(&org_id.to_string(), avatar::Usage::Org),
                    members: vec![user]
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn register_project() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let urn = coco::Urn::new(
            owner.root_hash().clone(),
            librad::uri::Protocol::Git,
            librad::uri::Path::new(),
        );

        // Register user.
        let fee: registry::Balance = 10;
        ctx.registry
            .register_user(&author, handle, None, fee)
            .await?;

        // Register org.
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Register project
        let project_name = "upstream";

        let res = request()
            .method("POST")
            .path(&format!("/{}/projects/{}", org_id, project_name))
            .json(&http::RegisterProjectInput {
                maybe_coco_id: Some(urn),
                transaction_fee: registry::MINIMUM_TX_FEE,
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
                assert_eq!(domain_type.clone(), registry::DomainType::Org);
                assert_eq!(domain_id.clone(), org_id);
            },
            _ => panic!("The tx message is an unexpected variant."),
        }

        Ok(())
    }

    #[tokio::test]
    async fn get_project() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let project_name = registry::ProjectName::try_from("upstream")?;
        let project_domain = registry::ProjectDomain::Org(org_id.clone());

        let ctx = ctx.read().await;

        // Register the user
        let fee: registry::Balance = 10;
        ctx.registry
            .register_user(&author, handle, None, fee)
            .await?;

        // Register the org.
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Register the project.
        ctx.registry
            .register_project(
                &author,
                project_domain.clone(),
                project_name.clone(),
                None,
                fee,
            )
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/projects/{}", org_id, project_name))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!(registry::Project {
                    name: project_name,
                    domain: project_domain,
                    maybe_project_id: None,
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn get_projects() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;
        let project_name = "upstream";
        let project_description = "desktop client for radicle";
        let default_branch = "master";

        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
            &owner,
            project_name,
            project_description,
            default_branch,
        )?;
        let urn = platinum_project.urn();

        // Register the user
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let project_name = registry::ProjectName::try_from(project_name)?;
        let project_domain = registry::ProjectDomain::Org(org_id.clone());
        let fee: registry::Balance = 10;

        ctx.registry
            .register_user(&author, handle, None, fee)
            .await?;

        // Register the org.
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Register the project.
        ctx.registry
            .register_project(
                &author,
                project_domain,
                project_name.clone(),
                Some(urn.clone()),
                fee,
            )
            .await?;

        let res = request()
            .method("GET")
            .path(&format!("/{}/projects", org_id.to_string()))
            .reply(&api)
            .await;

        let want = json!([{
            "name": project_name.to_string(),
            "orgId": org_id.to_string(),
            "shareableEntityIdentifier": format!("%{}/{}", org_id.to_string(), project_name.to_string()),
            "maybeProject": {
                "id": urn.to_string(),
                "metadata": {
                    "defaultBranch": default_branch.to_string(),
                    "description": project_description.to_string(),
                    "name": project_name.to_string(),
                },
                "registration": Value::Null,
                "shareableEntityIdentifier": format!("%{}", urn),
                "stats": {
                  "branches": 2,
                  "commits": 14,
                  "contributors": 4
                }
            },
        }]);

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, want);
        });

        Ok(())
    }

    #[tokio::test]
    async fn register() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let fee: registry::Balance = 10;

        // Register the user
        ctx.registry
            .register_user(&author, handle, None, fee)
            .await?;

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::RegisterInput {
                id: org_id.to_string(),
                transaction_fee: registry::MINIMUM_TX_FEE,
            })
            .reply(&api)
            .await;

        let txs = ctx.registry.list_transactions(vec![])?;

        // Get the registered org
        let org = ctx.registry.get_org(org_id.clone()).await?.unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(txs.len(), 2);
        assert_eq!(org.id, org_id);

        Ok(())
    }

    #[tokio::test]
    async fn register_member() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let author = protocol::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;
        let fee: registry::Balance = 10;

        // Register the user
        ctx.registry
            .register_user(&author, handle.clone(), None, fee)
            .await?;

        // Register the org
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Register a second user
        let author2 = protocol::ed25519::Pair::from_legacy_string("//Bob", None);
        let handle2 = registry::Id::try_from("bob")?;
        ctx.registry
            .register_user(&author2, handle2.clone(), None, fee)
            .await?;

        // Register the second user as a member of the org
        let res = request()
            .method("POST")
            .path(&format!("/{}/members", org_id.clone()))
            .json(&super::RegisterMemberInput {
                handle: handle2.clone().to_string(),
                transaction_fee: registry::MINIMUM_TX_FEE,
            })
            .reply(&api)
            .await;

        let txs = ctx.registry.list_transactions(vec![])?;

        // Get the org and its members
        let org = ctx.registry.get_org(org_id).await?.unwrap();
        let member_handles: Vec<registry::Id> =
            org.members.iter().map(|user| user.handle.clone()).collect();

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(txs.len(), 4);
        assert_eq!(org.members.len(), 2);
        assert!(member_handles.contains(&handle));
        assert!(member_handles.contains(&handle2));

        Ok(())
    }

    #[tokio::test]
    async fn transfer() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Ctx::from(http::Context::tmp(&tmp_dir).await?);
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let author = radicle_registry_client::ed25519::Pair::from_legacy_string("//Alice", None);
        let handle = registry::Id::try_from("alice")?;
        let org_id = registry::Id::try_from("radicle")?;

        // Register the user
        ctx.registry
            .register_user(&author, handle.clone(), None, 10)
            .await?;

        // Register the org
        let fee: registry::Balance = 100;
        ctx.registry
            .register_org(&author, org_id.clone(), fee)
            .await?;

        // Transfer tokens from the org to the user
        let amount: registry::Balance = 10;
        let res = request()
            .method("POST")
            .path(&format!("/{}/transfer", org_id))
            .json(&super::TransferInput {
                recipient: author.public(),
                amount,
                transaction_fee: registry::MINIMUM_TX_FEE,
            })
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
