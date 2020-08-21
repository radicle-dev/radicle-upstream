//! Endpoints and serialisation for source code browsing.

use serde::ser::SerializeStruct as _;
use serde::{Deserialize, Serialize, Serializer};
use warp::document::{self, ToDocumentedType};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection, Reply};

use librad::meta::user;
use librad::peer;
use radicle_surf::vcs::git;

use coco;
use core::identity;

use crate::{Ctx, with_context, with_qs, with_owner_guard};

/// Combination of all source filters.
pub fn filters(ctx: Ctx) -> BoxedFilter<(impl Reply,)>
{
    blob_filter(ctx.clone())
        .or(branches_filter(ctx.clone()))
        .or(commit_filter(ctx.clone()))
        .or(commits_filter(ctx.clone()))
        .or(local_state_filter())
        .or(revisions_filter(ctx.clone()))
        .or(tags_filter(ctx.clone()))
        .or(tree_filter(ctx))
        .boxed()
}

/// `GET /blob/<project_id>?revision=<revision>&path=<path>`
fn blob_filter(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("blob")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(with_qs::<BlobQuery>())
        .and(document::document(
            document::query("revision", document::string()).description("Git revision"),
        ))
        .and(document::document(
            document::query("path", document::string())
                .description("Location of the file in the repo tree"),
        ))
        .and(document::document(document::description("Fetch a Blob")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Blob::document()).mime("application/json"),
            )
            .description("Blob for path found"),
        ))
        .and_then(handler::blob)
}

/// `GET /branches/<project_id>`
fn branches_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("branches")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(warp::filters::query::query::<BranchQuery>())
        .and(document::document(
            document::query("peerId", document::string()).description("The peer identifier"),
        ))
        .and(document::document(document::description("List Branches")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(coco::Branch::document()).description("List of branches"),
                )
                .mime("application/json"),
            )
            .description("List of branches"),
        ))
        .and_then(handler::branches)
}

/// `GET /commit/<project_id>/<sha1>`
fn commit_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("commit")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::param::<String>("sha1", "Git object id"))
        .and(document::document(document::description("Fetch a Commit")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Commit::document()).mime("application/json"),
            )
            .description("Commit for SHA1 found"),
        ))
        .and_then(handler::commit)
}

/// `GET /commits/<project_id>?branch=<branch>`
fn commits_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("commits")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(warp::filters::query::query::<CommitsQuery>())
        .and(document::document(
            document::query("branch", document::string()).description("Git branch"),
        ))
        .and(document::document(document::description(
            "Fetch Commits from a Branch",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(coco::Commit::document())).mime("application/json"),
            )
            .description("Branch found"),
        ))
        .and_then(handler::commits)
}

/// `GET /branches/<project_id>`
fn local_state_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("local-state")
        .and(warp::get())
        .and(document::tail(
            "path",
            "Location of the repository on the filesystem",
        ))
        .and(document::document(document::description(
            "List Branches, Remotes and if it is managed by coco for a local Repository",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(coco::Branch::document()).description("List of branches"),
                )
                .mime("application/json"),
            )
            .description("List of branches"),
        ))
        .and_then(handler::local_state)
}

/// `GET /revisions/<project_id>`
fn revisions_filter(
    ctx: Ctx,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("revisions")
        .and(warp::get())
        .and(with_context(ctx.clone()))
        .and(with_owner_guard(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::document(document::description(
            "List both branches and tags",
        )))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(
                    document::array(coco::Revisions::<(), ()>::document())
                        .description("List of revisions per repo"),
                )
                .mime("application/json"),
            )
            .description("List of branches and tags"),
        ))
        .and_then(handler::revisions)
}

/// `GET /tags/<project_id>`
fn tags_filter(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("tags")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(document::document(document::description("List Tags")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(document::array(coco::Tag::document()).description("List of tags"))
                    .mime("application/json"),
            )
            .description("List of tags"),
        ))
        .and_then(handler::tags)
}

/// `GET /tree/<project_id>/<revision>/<prefix>`
fn tree_filter(ctx: Ctx) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    path("tree")
        .and(warp::get())
        .and(with_context(ctx))
        .and(document::param::<coco::Urn>(
            "project_id",
            "ID of the project the blob is part of",
        ))
        .and(with_qs::<TreeQuery>())
        .and(document::document(
            document::query("revision", document::string()).description("Git revision"),
        ))
        .and(document::document(
            document::query("prefix", document::string())
                .description("Prefix to filter files and folders by"),
        ))
        .and(document::document(document::description("Fetch a Tree")))
        .and(document::document(document::tag("Source")))
        .and(document::document(
            document::response(
                200,
                document::body(coco::Tree::document()).mime("application/json"),
            )
            .description("Tree for path found"),
        ))
        .and_then(handler::tree)
}

/// Source handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::path::Tail;
    use warp::{reply, Rejection, Reply};

    use radicle_surf::vcs::git;

    use coco;
    use core::session;
    use core::error::Error;

    use crate::Ctx;
    use crate::HttpError;

    /// Fetch a [`coco::Blob`].
    pub async fn blob(
        ctx: Ctx,
        project_urn: coco::Urn,
        super::BlobQuery {
            path,
            peer_id,
            revision,
            highlight,
        }: super::BlobQuery,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;

        let session = session::current(&ctx.peer_api, &ctx.store).await.map_err(HttpError::into)?;

        let project = ctx.peer_api.get_project(&project_urn, None).map_err(HttpError::into)?;

        let default_branch = match peer_id {
            Some(peer_id) if peer_id != ctx.peer_api.peer_id() => {
                git::Branch::remote(project.default_branch(), &peer_id.to_string())
            },
            Some(_) | None => git::Branch::local(project.default_branch()),
        };

        let theme = if let Some(true) = highlight {
            Some(&session.settings.appearance.theme)
        } else {
            None
        };
        let blob = ctx.peer_api.with_browser(&project_urn, |mut browser| {
            coco::blob(&mut browser, default_branch, revision, &path, theme)
        }).map_err(HttpError::into)?;

        Ok(reply::json(&blob))
    }

    /// Fetch the list [`coco::Branch`].
    pub async fn branches(
        ctx: Ctx,
        project_urn: coco::Urn,
        super::BranchQuery { peer_id }: super::BranchQuery,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        let branches = ctx.peer_api.with_browser(&project_urn, |browser| {
            coco::branches(browser, Some(coco::into_branch_type(peer_id)))
        })?;

        Ok(reply::json(&branches))
    }

    /// Fetch a [`coco::Commit`].
    pub async fn commit(
        ctx: Ctx,
        project_urn: coco::Urn,
        sha1: String,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        let commit = ctx.peer_api.with_browser(&project_urn, |mut browser| {
            coco::commit(&mut browser, &sha1)
        })?;

        Ok(reply::json(&commit))
    }

    /// Fetch the list of [`coco::Commit`] from a branch.
    pub async fn commits(
        ctx: Ctx,
        project_urn: coco::Urn,
        query: super::CommitsQuery,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        let commits = ctx.peer_api.with_browser(&project_urn, |mut browser| {
            coco::commits(&mut browser, query.into())
        })?;

        Ok(reply::json(&commits))
    }

    /// Fetch the list [`coco::Branch`] for a local repository.
    pub async fn local_state(path: Tail) -> Result<impl Reply, Rejection> {
        let state = coco::local_state(path.as_str())?;

        Ok(reply::json(&state))
    }

    /// Fetch the list [`coco::Branch`] and [`coco::Tag`].
    pub async fn revisions(
        ctx: Ctx,
        owner: coco::User,
        project_urn: coco::Urn,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        let peers = ctx.peer_api.tracked(&project_urn).map_err(HttpError::into)?;
        let peer_id = ctx.peer_api.peer_id();
        let revisions: Vec<super::Revisions> =
            ctx.peer_api.with_browser(&project_urn, |browser| {
                // TODO(finto): downgraded verified user, which should not be needed.
                let owner = owner.to_data().build()?;
                Ok(coco::revisions(browser, peer_id, owner, peers)?
                    .into_iter()
                    .map(|revision| revision.into())
                    .collect())
            }).map_err(HttpError::into)?;

        Ok(reply::json(&revisions))
    }

    /// Fetch the list [`coco::Tag`].
    pub async fn tags(ctx: Ctx, project_urn: coco::Urn) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;
        let tags = ctx
            .peer_api
            .with_browser(&project_urn, |browser| coco::tags(browser)).map_err(HttpError::into)?;

        Ok(reply::json(&tags))
    }

    /// Fetch a [`coco::Tree`].
    pub async fn tree(
        ctx: Ctx,
        project_urn: coco::Urn,
        super::TreeQuery {
            prefix,
            peer_id,
            revision,
        }: super::TreeQuery,
    ) -> Result<impl Reply, Rejection>
    {
        let ctx = ctx.read().await;

        let project = ctx.peer_api.get_project(&project_urn, None)?;
        let default_branch = match peer_id {
            Some(peer_id) if peer_id != ctx.peer_api.peer_id() => {
                git::Branch::remote(project.default_branch(), &peer_id.to_string())
            },
            Some(_) | None => git::Branch::local(project.default_branch()),
        };

        let tree = ctx.peer_api.with_browser(&project_urn, |mut browser| {
            coco::tree(&mut browser, default_branch, revision, prefix)
        }).map_err(HttpError::into)?;

        Ok(reply::json(&tree))
    }
}

/// Bundled query params to pass to the commits handler.
#[derive(Debug, Deserialize)]
pub struct CommitsQuery {
    /// PeerId to scope the query by.
    peer_id: Option<peer::PeerId>,
    /// Branch to get the commit history for.
    branch: String,
}

impl From<CommitsQuery> for git::Branch {
    fn from(CommitsQuery { peer_id, branch }: CommitsQuery) -> Self {
        match peer_id {
            None => Self::local(&branch),
            Some(peer_id) => Self::remote(&branch, &peer_id.to_string()),
        }
    }
}

/// Bundled query params to pass to the blob handler.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobQuery {
    /// Location of the blob in tree.
    path: String,
    /// PeerId to scope the query by.
    peer_id: Option<peer::PeerId>,
    /// Revision to query at.
    revision: Option<coco::Revision<peer::PeerId>>,
    /// Whether or not to syntax highlight the blob.
    highlight: Option<bool>,
}

/// A query param for [`handler::branches`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    /// PeerId to scope the query by.
    peer_id: Option<peer::PeerId>,
}

/// Bundled query params to pass to the tree handler.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreeQuery {
    /// Path prefix to query the tree.
    prefix: Option<String>,
    /// PeerId to scope the query by.
    peer_id: Option<peer::PeerId>,
    /// Revision to query at.
    revision: Option<coco::Revision<peer::PeerId>>,
}

/// The output structure when calling the `/revisions` endpoint.
#[derive(Serialize)]
struct Revisions {
    /// The [`identity::Identity`] that owns these revisions.
    identity: identity::Identity,
    /// The branches for this project.
    branches: Vec<coco::Branch>,
    /// The branches for this project.
    tags: Vec<coco::Tag>,
}

impl<S> From<coco::Revisions<peer::PeerId, user::User<S>>> for Revisions {
    fn from(other: coco::Revisions<peer::PeerId, user::User<S>>) -> Self {
        Self {
            identity: (other.peer_id, other.user).into(),
            branches: other.branches,
            tags: other.tags,
        }
    }
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::http::StatusCode;
    use warp::test::request;

    use radicle_surf::vcs::git;

    use crate::coco;
    use crate::error;
    use crate::http;
    use crate::identity;
    use crate::session;

    #[tokio::test]
    async fn blob() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let revision = coco::Revision::Branch {
            name: "master".to_string(),
            peer_id: None,
        };
        let default_branch = git::Branch::local(platinum_project.default_branch());
        let path = "text/arrows.txt";
        let want = ctx.peer_api.with_browser(&urn, |mut browser| {
            coco::blob(
                &mut browser,
                default_branch.clone(),
                Some(revision.clone()),
                path,
                None,
            )
        })?;

        let query = super::BlobQuery {
            path: path.to_string(),
            peer_id: None,
            revision: Some(revision.clone()),
            highlight: Some(false),
        };

        let path = format!("/blob/{}?{}", urn, serde_qs::to_string(&query).unwrap());

        // Get ASCII blob.
        let res = request().method("GET").path(&path).reply(&api).await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!({
                    "binary": false,
                    "html": false,
                    "content": "  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
  ;;;;;        ;;;;;        ;;;;;
..;;;;;..    ..;;;;;..    ..;;;;;..
 ':::::'      ':::::'      ':::::'
   ':`          ':`          ':`
",
                    "info": {
                        "name": "arrows.txt",
                        "objectType": "BLOB",
                        "lastCommit": {
                            "sha1": "1e0206da8571ca71c51c91154e2fee376e09b4e7",
                            "author": {
                                "name": "Rūdolfs Ošiņš",
                                "email": "rudolfs@osins.org",
                            },
                            "committer": {
                                "name": "Rūdolfs Ošiņš",
                                "email": "rudolfs@osins.org",
                            },
                            "summary": "Add text files",
                            "description": "",
                            "committerTime": 1_575_283_425,
                        },
                    },
                    "path": "text/arrows.txt",
                })
            );
        });

        // Get binary blob.
        let path = "bin/ls";
        let want = ctx.peer_api.with_browser(&urn, |browser| {
            coco::blob(browser, default_branch, Some(revision.clone()), path, None)
        })?;

        let query = super::BlobQuery {
            path: path.to_string(),
            peer_id: None,
            revision: Some(revision),
            highlight: Some(false),
        };

        let path = format!("/blob/{}?{}", urn, serde_qs::to_string(&query).unwrap());

        let res = request().method("GET").path(&path).reply(&api).await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!({
                    "binary": true,
                    "html": false,
                    "content": Value::Null,
                    "info": {
                        "name": "ls",
                        "objectType": "BLOB",
                        "lastCommit": {
                            "sha1": "19bec071db6474af89c866a1bd0e4b1ff76e2b97",
                            "author": {
                                "name": "Rūdolfs Ošiņš",
                                "email": "rudolfs@osins.org",
                            },
                            "committer": {
                                "name": "Rūdolfs Ošiņš",
                                "email": "rudolfs@osins.org",
                            },
                            "summary": "Add some binary files",
                            "description": "",
                            "committerTime": 1_575_282_964, },
                    },
                    "path": "bin/ls",
                })
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn blob_dev_branch() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let revision = coco::Revision::Branch {
            name: "dev".to_string(),
            peer_id: None,
        };
        let default_branch = git::Branch::local(platinum_project.default_branch());
        let path = "here-we-are-on-a-dev-branch.lol";
        let want = ctx.peer_api.with_browser(&urn, |mut browser| {
            coco::blob(
                &mut browser,
                default_branch.clone(),
                Some(revision.clone()),
                path,
                None,
            )
        })?;

        let query = super::BlobQuery {
            path: path.to_string(),
            peer_id: None,
            revision: Some(revision),
            highlight: Some(false),
        };

        let path = format!("/blob/{}?{}", urn, serde_qs::to_string(&query).unwrap());

        // Get ASCII blob.
        let res = request().method("GET").path(&path).reply(&api).await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    #[tokio::test]
    async fn branches() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let want = ctx
            .peer_api
            .with_browser(&urn, |browser| coco::branches(browser, None))?;

        let res = request()
            .method("GET")
            .path(&format!("/branches/{}", urn))
            .reply(&api)
            .await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(have, json!(["dev", "master"]));
        });

        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::indexing_slicing)]
    async fn commit() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let sha1 = "3873745c8f6ffb45c990eb23b491d4b4b6182f95";
        let want = ctx
            .peer_api
            .with_browser(&urn, |mut browser| coco::commit_header(&mut browser, sha1))?;

        let res = request()
            .method("GET")
            .path(&format!("/commit/{}/{}", urn, sha1))
            .reply(&api)
            .await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have["header"], json!(want));
            assert_eq!(
                have["header"],
                json!({
                    "sha1": sha1,
                    "author": {
                        "name": "Fintan Halpenny",
                        "email": "fintan.halpenny@gmail.com",
                    },
                    "committer": {
                        "email": "noreply@github.com",
                        "name": "GitHub",
                    },
                    "summary": "Extend the docs (#2)",
                    "description": "I want to have files under src that have separate commits.\r\nThat way src\'s latest commit isn\'t the same as all its files, instead it\'s the file that was touched last.",
                    "committerTime": 1_578_309_972,
                }),
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn commits() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let branch = git::Branch::local("master");
        let head = "223aaf87d6ea62eef0014857640fd7c8dd0f80b5";
        let (want, head_commit) = ctx.peer_api.with_browser(&urn, |mut browser| {
            let want = coco::commits(&mut browser, branch.clone())?;
            let head_commit = coco::commit_header(&mut browser, head)?;
            Ok((want, head_commit))
        })?;

        let res = request()
            .method("GET")
            .path(&format!("/commits/{}?branch={}", urn, branch.name))
            .reply(&api)
            .await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(have.as_array().unwrap().len(), 14);
            assert_eq!(
                have.as_array().unwrap().first().unwrap(),
                &serde_json::to_value(&head_commit).unwrap(),
                "the first commit is the head of the branch"
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn local_state() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let path = "../fixtures/git-platinum";
        let res = request()
            .method("GET")
            .path(&format!("/local-state/{}", path))
            .reply(&api)
            .await;

        let want = coco::local_state(path).unwrap();

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!({
                    "branches": [
                        "dev",
                        "master",
                    ],
                }),
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn revisions() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let peer_id = ctx.peer_api.peer_id();

        let id = identity::create(&ctx.peer_api, &key, "cloudhead")?;

        let owner = ctx.peer_api.get_user(&id.clone().urn)?;
        let owner = coco::verify_user(owner)?;

        session::set_identity(&ctx.store, id)?;

        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let (remote, fintohaps) =
            coco::control::track_fake_peer(&ctx.peer_api, &key, &platinum_project, "fintohaps");

        let res = request()
            .method("GET")
            .path(&format!("/revisions/{}", urn))
            .reply(&api)
            .await;

        let owner = owner.to_data().build()?; // TODO(finto): Unverify owner, unfortunately
        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(
                have,
                json!([
                    super::Revisions {
                        identity: (peer_id, owner).into(),
                        branches: vec![
                            coco::Branch("dev".to_string()),
                            coco::Branch("master".to_string())
                        ],
                        tags: vec![
                            coco::Tag("v0.1.0".to_string()),
                            coco::Tag("v0.2.0".to_string()),
                            coco::Tag("v0.3.0".to_string()),
                            coco::Tag("v0.4.0".to_string()),
                            coco::Tag("v0.5.0".to_string())
                        ]
                    },
                    super::Revisions {
                        identity: (remote.clone(), fintohaps).into(),
                        branches: vec![coco::Branch("master".to_string())],
                        tags: vec![]
                    },
                ])
            )
        });

        let res = request()
            .method("GET")
            .path(&format!("/branches/{}?peerId={}", urn, remote))
            .reply(&api)
            .await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!([coco::Branch("master".to_string())]));
        });

        Ok(())
    }

    #[tokio::test]
    async fn tags() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let want = ctx
            .peer_api
            .with_browser(&urn, |browser| coco::tags(browser))?;

        let res = request()
            .method("GET")
            .path(&format!("/tags/{}", urn))
            .reply(&api)
            .await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!(["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"]),
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn tree() -> Result<(), error::Error> {
        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let revision = coco::Revision::Branch {
            name: "master".to_string(),
            peer_id: None,
        };
        let prefix = "src";

        let default_branch = git::Branch::local(platinum_project.default_branch());
        let want = ctx.peer_api.with_browser(&urn, |mut browser| {
            coco::tree(
                &mut browser,
                default_branch,
                Some(revision.clone()),
                Some(prefix.to_string()),
            )
        })?;

        let query = super::TreeQuery {
            prefix: Some(prefix.to_string()),
            peer_id: None,
            revision: Some(revision),
        };

        let path = format!("/tree/{}?{}", urn, serde_qs::to_string(&query).unwrap());
        let res = request().method("GET").path(&path).reply(&api).await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!({
                    "path": "src",
                    "info": {
                        "name": "src",
                        "objectType": "TREE",
                        "lastCommit": null,                },
                        "entries": [
                        {
                            "path": "src/Eval.hs",
                            "info": {
                                "name": "Eval.hs",
                                "objectType": "BLOB",
                                "lastCommit": null,
                            },
                        },
                        {
                            "path": "src/memory.rs",
                            "info": {
                                "name": "memory.rs",
                                "objectType": "BLOB",
                                "lastCommit": null,
                            },
                        },
                    ],
                }),
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn tree_dev_branch() -> Result<(), error::Error> {
        // Testing that the endpoint works with URL encoding
        const FRAGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'[')
            .add(b']')
            .add(b'=');

        let tmp_dir = tempfile::tempdir()?;
        let ctx = super::Context::tmp(&tmp_dir).await?;
        let api = super::filters(ctx.clone());

        let ctx = ctx.read().await;
        let key = ctx.keystore.get_librad_key()?;
        let owner = ctx.peer_api.init_owner(&key, "cloudhead")?;
        let platinum_project = coco::control::replicate_platinum(
            &ctx.peer_api,
            &key,
            &owner,
            "git-platinum",
            "fixture data",
            "master",
        )?;
        let urn = platinum_project.urn();

        let revision = coco::Revision::Branch {
            name: "dev".to_string(),
            peer_id: None,
        };

        let default_branch = git::Branch::local(platinum_project.default_branch());
        let want = ctx.peer_api.with_browser(&urn, |mut browser| {
            coco::tree(&mut browser, default_branch, Some(revision.clone()), None)
        })?;

        let query = super::TreeQuery {
            prefix: None,
            peer_id: None,
            revision: Some(revision),
        };

        let path = format!(
            "/tree/{}?{}",
            urn,
            percent_encoding::utf8_percent_encode(&serde_qs::to_string(&query).unwrap(), FRAGMENT)
        );
        let res = request().method("GET").path(&path).reply(&api).await;

        super::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }
}
