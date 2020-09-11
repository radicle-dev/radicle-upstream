//! Endpoints and serialisation for [`project::Project`] related types.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use crate::context;
use crate::http;

/// Combination of all routes.
pub fn filters(ctx: context::Ctx) -> BoxedFilter<(impl Reply,)> {
    tracked_filter(ctx.clone())
        .or(contributed_filter(ctx.clone()))
        .or(checkout_filter(ctx.clone()))
        .or(create_filter(ctx.clone()))
        .or(discover_filter(ctx.clone()))
        .or(get_filter(ctx))
        .boxed()
}

/// `POST /<id>/checkout`
fn checkout_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::post())
        .and(path::param::<coco::Urn>())
        .and(warp::body::json())
        .and_then(handler::checkout)
}

/// `POST /`
fn create_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path::end())
        .and(http::with_context(ctx.clone()))
        .and(http::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `GET /<id>`
fn get_filter(ctx: context::Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context(ctx)
        .and(warp::get())
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and_then(handler::get)
}

/// `GET /tracked`
fn tracked_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tracked")
        .and(warp::get())
        .and(http::with_context(ctx))
        .and(path::end())
        .and_then(handler::list_tracked)
}

/// `GET /contributed`
fn contributed_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("contributed")
        .and(warp::get())
        .and(http::with_context(ctx))
        .and(path::end())
        .and_then(handler::list_contributed)
}

/// `GET /discover`
fn discover_filter(
    ctx: context::Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("discover")
        .and(warp::get())
        .and(http::with_context(ctx))
        .and(path::end())
        .and_then(handler::discover)
}

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use std::path::PathBuf;

    use warp::http::StatusCode;
    use warp::{reply, Rejection, Reply};

    use crate::context;
    use crate::error::Error;
    use crate::project;

    /// Create a new [`project::Project`].
    pub async fn create(
        ctx: context::Ctx,
        owner: coco::User,
        input: coco::project::Create<PathBuf>,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let meta = ctx
            .peer_api
            .init_project(&ctx.signer, &owner, &input)
            .map_err(Error::from)?;
        let urn = meta.urn();

        let stats = ctx
            .peer_api
            .with_browser(&urn, |browser| Ok(browser.get_stats()?))
            .map_err(Error::from)?;
        let project: project::Project = (meta, stats).into();

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Checkout a [`project::Project`]'s source code.
    pub async fn checkout(
        ctx: context::Ctx,
        urn: coco::Urn,
        super::CheckoutInput { path, peer_id }: super::CheckoutInput,
    ) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;
        let project = ctx
            .peer_api
            .get_project(&urn, peer_id)
            .map_err(Error::from)?;

        let path = coco::project::Checkout::new(project, path)
            .run(ctx.peer_api.peer_id())
            .map_err(Error::from)?;

        Ok(reply::with_status(reply::json(&path), StatusCode::CREATED))
    }

    /// Get the [`project::Project`] for the given `id`.
    pub async fn get(ctx: context::Ctx, urn: coco::Urn) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        Ok(reply::json(&project::get(&ctx.peer_api, &urn)?))
    }

    /// List all projects the current user is tracking.
    pub async fn list_tracked(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let projects = project::Projects::list(&ctx.peer_api)?;

        Ok(reply::json(&projects.tracked))
    }

    /// List all projects the current user has contributed to.
    pub async fn list_contributed(ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let ctx = ctx.read().await;

        let projects = project::Projects::list(&ctx.peer_api)?;

        Ok(reply::json(&projects.contributed))
    }

    /// Get a feed of untracked projects.
    pub async fn discover(_ctx: context::Ctx) -> Result<impl Reply, Rejection> {
        let feed = project::discover()?;

        Ok(reply::json(&feed))
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

/// Bundled input data for project checkout.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutInput {
    /// Location on the filesystem where the working copy should be created.
    path: PathBuf,
    /// Which peer are we checking out from. If it's `None`, we're checking out our own project.
    peer_id: Option<coco::PeerId>,
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

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use radicle_surf::vcs::git::git2;

    use crate::context;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::project;
    use crate::session;

    #[tokio::test]
    async fn checkout() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let handle = "cloudhead";

        let owner = ctx.peer_api.init_owner(&ctx.signer, handle)?;
        session::set_identity(&ctx.store, (ctx.peer_api.peer_id(), owner.clone()).into())?;

        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let input = super::CheckoutInput {
            path: dir.path().to_path_buf(),
            peer_id: None,
        };
        let res = request()
            .method("POST")
            .path(&format!("/{}/checkout", urn))
            .json(&input)
            .reply(&api)
            .await;

        http::test::assert_response(&res, StatusCode::CREATED, |_| {});
        assert!(dir.path().exists());

        let repo =
            git2::Repository::open(dir.path().join("git-platinum")).map_err(coco::Error::from)?;
        let refs = repo
            .branches(None)
            .map_err(coco::Error::from)?
            .map(|branch| {
                branch
                    .expect("failed to get branch")
                    .0
                    .name()
                    .expect("failed to get name")
                    .expect("utf-8 error")
                    .to_string()
            })
            .collect::<Vec<_>>();
        let remote = repo
            .find_remote(coco::config::RAD_REMOTE)
            .map_err(coco::Error::from)?;
        assert_eq!(
            remote.url(),
            Some(
                coco::LocalUrl::from_urn(urn, ctx.peer_api.peer_id())
                    .to_string()
                    .as_str()
            )
        );
        assert_eq!(refs, vec!["master", "rad/dev", "rad/master"]);

        Ok(())
    }

    #[tokio::test]
    async fn create_new() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let handle = "cloudhead";
        let id = identity::create(&ctx.peer_api, &ctx.signer, handle)?;

        session::set_identity(&ctx.store, id.clone())?;

        let project = coco::project::Create {
            repo: coco::project::Repo::New {
                path: dir.path(),
                name: "Upstream".to_string(),
            },
            description: "Desktop client for radicle.".into(),
            default_branch: "master".into(),
        };

        let res = request()
            .method("POST")
            .path("/")
            .json(&project)
            .reply(&api)
            .await;

        let projects = project::Projects::list(&ctx.peer_api)?;
        let meta = projects.into_iter().next().unwrap();
        let maintainer = meta.metadata.maintainers.iter().next().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": meta.id,
            "metadata": {
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "maintainers": [
                    maintainer
                ],
                "name": "Upstream",
            },
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
    async fn create_existing() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let repo_path = dir.path().join("Upstream");
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let handle = "cloudhead";
        let id = identity::create(&ctx.peer_api, &ctx.signer, handle)?;

        session::set_identity(&ctx.store, id.clone())?;

        let project = coco::project::Create {
            repo: coco::project::Repo::Existing {
                path: repo_path.clone(),
            },
            description: "Desktop client for radicle.".into(),
            default_branch: "master".into(),
        };

        // Create the repository for which we'll create a project for
        coco::control::clone_platinum(repo_path)?;

        let res = request()
            .method("POST")
            .path("/")
            .json(&project)
            .reply(&api)
            .await;

        let projects = project::Projects::list(&ctx.peer_api)?;
        let meta = projects.into_iter().next().unwrap();
        let maintainer = meta.metadata.maintainers.iter().next().unwrap();

        let have: Value = serde_json::from_slice(res.body()).unwrap();
        let want = json!({
            "id": meta.id,
            "metadata": {
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "name": "Upstream",
                "maintainers": [
                    maintainer
                ],
            },
            "shareableEntityIdentifier": format!("%{}", meta.id.to_string()),
            "stats": {
                "branches": 1,
                "commits": 14,
                "contributors": 4,
            },
        });

        assert_eq!(res.status(), StatusCode::CREATED);
        assert_eq!(have, want);

        Ok(())
    }

    #[tokio::test]
    async fn create_existing_after_reset() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let repos_dir = tempfile::tempdir_in(tmp_dir.path())?;
        let dir = tempfile::tempdir_in(repos_dir.path())?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        {
            let ctx = ctx.read().await;
            let handle = "cloudhead";
            let id = identity::create(&ctx.peer_api, &ctx.signer, handle)?;

            session::set_identity(&ctx.store, id)?;
        }

        let project = coco::project::Create {
            repo: coco::project::Repo::New {
                path: dir.path(),
                name: "Upstream".to_string(),
            },
            description: "Desktop client for radicle.".into(),
            default_branch: "master".into(),
        };

        let _res = request()
            .method("POST")
            .path("/")
            .json(&project)
            .reply(&api)
            .await;

        context::reset_ctx_peer(ctx.clone()).await?;

        {
            let ctx = ctx.read().await;
            let handle = "cloudhead";
            let id = identity::create(&ctx.peer_api, &ctx.signer, handle)?;

            session::set_identity(&ctx.store, id)?;
        }

        let res = request()
            .method("POST")
            .path("/")
            .json(&project.into_existing())
            .reply(&api)
            .await;

        let projects = {
            let ctx = ctx.read().await;
            project::Projects::list(&ctx.peer_api)
        }?;

        let meta = projects.into_iter().next().unwrap();
        let maintainer = meta.metadata.maintainers.iter().next().unwrap();

        let want = json!({
            "id": meta.id,
            "metadata": {
                "defaultBranch": "master",
                "description": "Desktop client for radicle.",
                "name": "Upstream",
                "maintainers": [
                    maintainer
                ],
            },
            "shareableEntityIdentifier": format!("%{}", meta.id.to_string()),
            "stats": {
                "branches": 1,
                "commits": 1,
                "contributors": 1,
            },
        });

        http::test::assert_response(&res, StatusCode::CREATED, |have| {
            assert_eq!(have, want);
        });

        Ok(())
    }

    #[tokio::test]
    async fn get() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &ctx.signer,
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
    async fn list_tracked() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, &ctx.signer, &owner)?;
        let projects = project::Projects::list(&ctx.peer_api)?;

        let res = request().method("GET").path("/tracked").reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(projects.tracked));
        });

        Ok(())
    }

    #[tokio::test]
    async fn list_my_projects() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, &ctx.signer, &owner)?;

        let res = request()
            .method("GET")
            .path("/contributed")
            .reply(&api)
            .await;

        let projects = project::Projects::list(&ctx.peer_api)?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(projects.contributed));
        });

        Ok(())
    }

    #[tokio::test]
    async fn discover() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = context::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let owner = ctx.peer_api.init_owner(&ctx.signer, "cloudhead")?;

        coco::control::setup_fixtures(&ctx.peer_api, &ctx.signer, &owner)?;

        let res = request().method("GET").path("/discover").reply(&api).await;
        let want = json!([
            {
                "id": "rad:git:hwd1yrerz7sig1smr8yjs5ue1oij61bfhyx41couxqj61qn5joox5pu4o4c",
                "metadata": {
                    "defaultBranch": "main",
                    "description": "It is not the slumber of reason that engenders monsters, \
                    but vigilant and insomniac rationality.",
                    "name": "radicle-upstream",
                    "maintainers": [],
                },
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
                    "name": "radicle-link",
                    "maintainers": [],
                },
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
