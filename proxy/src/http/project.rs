//! Endpoints and serialisation for [`project::Project`] related types.

use std::path::PathBuf;

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::coco;
use crate::http;
use crate::project;
use crate::registry;

/// Combination of all routes.
pub fn filters<R>(ctx: http::Ctx<R>) -> BoxedFilter<(impl Reply,)>
where
    R: registry::Client + 'static,
{
    list_filter(ctx.clone())
        .or(checkout_filter(ctx.clone()))
        .or(create_filter(ctx.clone()))
        .or(discover_filter(ctx.clone()))
        .or(get_filter(ctx))
        .boxed()
}

/// `POST /<id>/checkout`
fn checkout_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
{
    http::with_context(ctx)
        .and(warp::post())
        .and(document::param::<coco::Urn>("id", "Project id"))
        .and(warp::body::json())
        .and(document::document(document::description(
            "Create a new working copy for a project",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::body(CheckoutInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(201, None).description("Checkout succeeded"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Project not found"),
        ))
        .and_then(handler::checkout)
}

/// `POST /`
fn create_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
{
    http::with_context(ctx.clone())
        .and(http::with_owner_guard(ctx))
        .and(warp::post())
        .and(warp::body::json())
        .and(document::document(document::description(
            "Create a new project",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::body(CreateInput::document()).mime("application/json"),
        ))
        .and(document::document(
            document::response(
                201,
                document::body(project::Project::document()).mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::create)
}

/// `GET /<id>`
fn get_filter<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(document::param::<String>("id", "Project id"))
        .and(path::end())
        .and(document::document(document::description(
            "Find Project by ID",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::response(
                200,
                document::body(project::Project::document()).mime("application/json"),
            )
            .description("Project found"),
        ))
        .and(document::document(
            document::response(
                404,
                document::body(super::error::Error::document()).mime("application/json"),
            )
            .description("Project not found"),
        ))
        .and_then(handler::get)
}

/// `GET /`
fn list_filter<R>(ctx: http::Ctx<R>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
{
    http::with_context(ctx)
        .and(warp::get())
        .and(path::end())
        .and(document::document(document::description("List projects")))
        .and(document::document(document::tag("Project")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(project::Project::document()).description("List of projects"),
                )
                .mime("application/json"),
            )
            .description("Creation succeeded"),
        ))
        .and_then(handler::list)
}

/// `GET /discover`
fn discover_filter<R>(
    ctx: http::Ctx<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: registry::Client + 'static,
{
    path("discover")
        .and(warp::get())
        .and(http::with_context(ctx))
        .and(path::end())
        .and(document::document(document::description(
            "Fetch discovery feed",
        )))
        .and(document::document(document::tag("Project")))
        .and(document::document(document::response(
            200,
            document::body(
                document::array(project::Project::document())
                    .description("Feed of untracked projects"),
            )
            .mime("application/json"),
        )))
        .and_then(handler::discover)
}

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::coco;
    use crate::error::Error;
    use crate::http;
    use crate::project;

    /// Create a new [`project::Project`].
    pub async fn create<R>(
        ctx: http::Ctx<R>,
        owner: coco::User,
        input: super::CreateInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
    {
        let ctx = ctx.read().await;

        let key = ctx.keystore.get_librad_key().map_err(Error::from)?;

        let meta = ctx.peer_api.init_project(
            &key,
            &owner,
            &input.path,
            &input.metadata.name,
            &input.metadata.description,
            &input.metadata.default_branch,
        )?;
        let urn = meta.urn();

        let stats = ctx
            .peer_api
            .with_browser(&urn, |browser| Ok(browser.get_stats()?))?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Checkout a [`project::Project`]'s source code.
    pub async fn checkout<R>(
        ctx: http::Ctx<R>,
        urn: coco::Urn,
        super::CheckoutInput { path, peer_id }: super::CheckoutInput,
    ) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
    {
        let ctx = ctx.read().await;
        let default_branch = ctx
            .peer_api
            .get_project(&urn, peer_id)?
            .default_branch()
            .to_owned();
        let directory_name = &project::get(&ctx.peer_api, &urn)?.metadata.name;

        project::Checkout::new(urn, directory_name.to_string(), default_branch, path, None)
            .run()?;

        Ok(reply::with_status(reply::json(&true), StatusCode::CREATED))
    }

    /// Get the [`project::Project`] for the given `id`.
    pub async fn get<R>(ctx: http::Ctx<R>, urn: String) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
    {
        let urn = urn.parse().map_err(Error::from)?;
        let ctx = ctx.read().await;

        Ok(reply::json(&project::get(&ctx.peer_api, &urn)?))
    }

    /// List all known projects.
    pub async fn list<R>(ctx: http::Ctx<R>) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
    {
        let ctx = ctx.read().await;
        let projects = project::list_projects(&ctx.peer_api)?;

        Ok(reply::json(&projects))
    }

    /// Get a feed of untracked projects.
    pub async fn discover<R>(_ctx: http::Ctx<R>) -> Result<impl Reply, Rejection>
    where
        R: Send + Sync,
    {
        let feed = project::discover()?;

        Ok(reply::json(&feed))
    }
}

impl Serialize for project::Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Project", 4)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field(
            "shareableEntityIdentifier",
            &self.shareable_entity_identifier.to_string(),
        )?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("registration", &self.registration)?;
        state.serialize_field("stats", &self.stats)?;
        state.end()
    }
}

impl ToDocumentedType for project::Project {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(4);
        properties.insert(
            "id".into(),
            document::string()
                .description("ID of the project")
                .example("ac1cac587b49612fbac39775a07fb05c6e5de08d.git"),
        );
        properties.insert(
            "shareableEntityIdentifier".into(),
            document::string()
                .description("Unique identifier that can be shared and looked up")
                .example("%123abcd.git"),
        );
        properties.insert("metadata".into(), project::Metadata::document());
        properties.insert("registration".into(), project::Registration::document());
        properties.insert("stats".into(), DocumentStats::document());

        document::DocumentedType::from(properties)
            .description("Radicle project for sharing and collaborating")
    }
}

impl Serialize for project::Registration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Org(org_id) => {
                serializer.serialize_newtype_variant("Registration", 0, "Org", &org_id.to_string())
            },
            Self::User(user_id) => serializer.serialize_newtype_variant(
                "Registration",
                1,
                "User",
                &user_id.to_string(),
            ),
        }
    }
}

/// Documentation of project stats
struct DocumentStats;

impl ToDocumentedType for DocumentStats {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "branches".into(),
            document::string()
                .description("Amount of known branches")
                .example(7),
        );
        properties.insert(
            "commits".into(),
            document::string()
                .description("Number of commits in the default branch")
                .example(420),
        );
        properties.insert(
            "contributors".into(),
            document::string()
                .description("Number of unique contributors on the default branch")
                .example(11),
        );

        document::DocumentedType::from(properties)
            .description("Coarse statistics for the Project source code")
    }
}

impl ToDocumentedType for project::Registration {
    fn document() -> document::DocumentedType {
        let org = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert(
                "org".into(),
                document::string().description("Org id").example("monadic"),
            );
            document::DocumentedType::from(fields).description("Registered under an Org")
        };
        let user = {
            let mut fields = HashMap::with_capacity(1);
            fields.insert(
                "user".into(),
                document::string().description("User id").example("monadic"),
            );
            document::DocumentedType::from(fields).description("Registered under a User")
        };

        document::one_of(vec![org, user])
            .description("Variants for possible registration states of a Project on the Registry")
            .example(Self::Org(
                registry::Id::try_from("monadic").expect("unable to parse org id"),
            ))
    }
}

impl ToDocumentedType for project::Metadata {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Project name")
                .example("upstream"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("High-level description of the Project")
                .example("Desktop client for radicle"),
        );
        properties.insert(
            "defaultBranch".into(),
            document::string()
                .description("Default branch for checkouts, often used as mainline as well")
                .example("master"),
        );

        document::DocumentedType::from(properties).description("Project metadata")
    }
}

/// Bundled input data for project creation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    /// Location on the filesystem of the project, an empty directory means we set up a fresh git
    /// repo at the path before initialising the project.
    path: PathBuf,
    /// User provided metadata for the project.
    metadata: MetadataInput,
}

impl ToDocumentedType for CreateInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(2);
        properties.insert(
            "path".into(),
            document::string()
                .description("Filesystem location of the git repository")
                .example("/home/xla/dev/src/github.com/radicle-dev/radicle-upstream"),
        );
        properties.insert("metadata".into(), MetadataInput::document());

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

/// Bundled input data for project checkout.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutInput {
    /// Location on the filesystem where the working copy should be created.
    path: PathBuf,
    /// Which peer are we checking out from. If it's `None`, we're checking out our own project.
    peer_id: Option<coco::PeerId>,
}

impl ToDocumentedType for CheckoutInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "path".into(),
            document::string()
                .description("Filesystem location where the working copy should be created")
                .example("/Users/rudolfs/work/radicle-tests/upstream-checkout"),
        );

        document::DocumentedType::from(properties).description("Input for project checkout")
    }
}

/// User provided metadata for project manipulation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInput {
    /// Name of the proejct.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
}

impl ToDocumentedType for MetadataInput {
    fn document() -> document::DocumentedType {
        let mut properties = HashMap::with_capacity(3);
        properties.insert(
            "name".into(),
            document::string()
                .description("Name of the project")
                .example("upstream"),
        );
        properties.insert(
            "description".into(),
            document::string()
                .description("Long-form text describing the project")
                .example("Desktop client for radicle"),
        );
        properties.insert(
            "defaultBranch".into(),
            document::string()
                .description("Projects mainline branch")
                .example("stable"),
        );

        document::DocumentedType::from(properties).description("Input for project creation")
    }
}

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::project;
    use crate::session;

    #[tokio::test]
    async fn create() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let path = dir.path();
        let ctx = http::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let handle = "cloudhead";
        let key = ctx.keystore.get_librad_key()?;
        let id = identity::create(&ctx.peer_api, key, handle)?;

        session::set_identity(&ctx.store, id.clone())?;

        let res = request()
            .method("POST")
            .path("/")
            .json(&super::CreateInput {
                path: path.into(),
                metadata: super::MetadataInput {
                    name: "Upstream".into(),
                    description: "Desktop client for radicle.".into(),
                    default_branch: "master".into(),
                },
            })
            .reply(&api)
            .await;

        let projects = project::list_projects(&ctx.peer_api)?;
        let meta = projects.first().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": meta.id,
            "metadata": {
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "name": "Upstream",
            },
            "registration": Value::Null,
            "shareableEntityIdentifier": format!("%{}", meta.id.to_string()),
            "stats": {
                "branches": 1,
                "commits": 1,
                "contributors": 1,
            },
        });

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(key.clone(), "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let project = project::get(&ctx.peer_api, &urn)?;

        let res = request()
            .method("GET")
            .path(&format!("/{}", urn))
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(project));
        });

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(key.clone(), "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, key, &owner)?;

        let projects = project::list_projects(&ctx.peer_api)?;
        let res = request().method("GET").path("/").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(projects));
        });

        Ok(())
    }

    #[tokio::test]
    async fn discover() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = http::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(key.clone(), "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, key, &owner)?;

        let res = request().method("GET").path("/discover").reply(&api).await;
        let want = json!([
            {
                "id": "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c",
                "metadata": {
                    "defaultBranch": "main",
                    "description": "It is not the slumber of reason that engenders monsters, \
                    but vigilant and insomniac rationality.",
                    "name": "radicle-upstream"
                },
                "registration": serde_json::Value::Null,
                "shareableEntityIdentifier": "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4ouwe",
                "stats": {
                    "branches": 36,
                    "commits": 216,
                    "contributors": 6,
                },
            },
            {
                "id": "rad:git:hwd1yrefz6xkwb46xkt7dhmwsjendiaqsaynpjwweqrqjc8muaath4gsf7o",
                "metadata": {
                    "defaultBranch": "main",
                    "description": "The monstrous complexity of our reality, a reality cross-hatched with fibre-optic cables, \
                    radio and microwaves, oil and gas pipelines, aerial and shipping routes, and the unrelenting, simultaneous execution \
                    of millions of communication protocols with every passing millisecond.",
                    "name": "radicle-link"
                },
                "registration": serde_json::Value::Null,
                "shareableEntityIdentifier": "rad:git:hwd1yre85ddm5ruz4kgqppdtdgqgqr4wjy3fmskgebhpzwcxshei7d4fd",
                "stats": {
                    "branches": 49,
                    "commits": 343,
                    "contributors": 7,
                },
            },
        ]);

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, want);
        });

        Ok(())
    }
}
