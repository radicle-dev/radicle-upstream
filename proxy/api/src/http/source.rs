//! Endpoints and serialisation for source code browsing.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::{context, http};

/// Combination of all source filters.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    blob_filter(ctx.clone())
        .or(branches_filter(ctx.clone()))
        .or(commit_filter(ctx.clone()))
        .or(commits_filter(ctx.clone()))
        .or(local_state_filter())
        .or(tags_filter(ctx.clone()))
        .or(tree_filter(ctx))
        .boxed()
}

/// `GET /blob/<project_urn>?revision=<revision>&path=<path>`
fn blob_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("blob")
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and(http::with_qs::<BlobQuery>())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::blob)
}

/// `GET /branches/<project_urn>?peerId=<peer_id>`
fn branches_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("branches")
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and(warp::query::<BranchQuery>())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::branches)
}

/// `GET /commit/<project_urn>/<sha1>`
fn commit_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("commit")
        .and(path::param::<coco::Urn>())
        .and(path::param::<coco::git_ext::Oid>())
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::commit)
}

/// `GET /commits/<project_urn>?revision=<revision>`
fn commits_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("commits")
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and(path::param::<coco::Urn>())
        .and(http::with_qs::<CommitsQuery>())
        .and_then(handler::commits)
}

/// `GET /local-state/<path>`
fn local_state_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("local-state")
        .and(path::tail())
        .and(warp::get())
        .and_then(handler::local_state)
}

/// `GET /tags/<project_urn>?peer_id=<peer_id>`
fn tags_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tags")
        .and(path::param::<coco::Urn>())
        .and(warp::query::<TagQuery>())
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::tags)
}

/// `GET /tree/<project_urn>?peerId=<peer_id>&prefix=<prefix>*revision=<revision>`
fn tree_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tree")
        .and(path::param::<coco::Urn>())
        .and(path::end())
        .and(warp::get())
        .and(http::with_qs::<TreeQuery>())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::tree)
}

/// Source handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{path::Tail, reply, Rejection, Reply};

    use coco::git_ext::Oid;

    use crate::{context, error, session, session::settings};

    /// Fetch a [`coco::source::Blob`].
    pub async fn blob(
        project_urn: coco::Urn,
        super::BlobQuery {
            path,
            peer_id,
            revision,
            highlight,
        }: super::BlobQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let settings = session::get_current(&ctx.store)?
            .map(|session| session.settings)
            .unwrap_or_default();
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let revision = super::http::guard_self_revision(&ctx.peer, revision);

        let theme = if let Some(true) = highlight {
            Some(match settings.appearance.theme {
                settings::Theme::Dark => "base16-ocean.dark",
                settings::Theme::Light => "base16-ocean.light",
                settings::Theme::H4x0r => "base16-ocean.h4x0r",
            })
        } else {
            None
        };

        let branch = coco::state::get_branch(&ctx.peer, project_urn, peer_id, None)
            .await
            .map_err(error::Error::from)?;
        let blob = coco::state::with_browser(&ctx.peer, branch, |mut browser| {
            coco::source::blob(&mut browser, revision, &path, theme)
        })
        .await
        .map_err(error::Error::from)?;

        Ok(reply::json(&blob))
    }

    /// Fetch the list [`coco::source::Branch`].
    pub async fn branches(
        project_urn: coco::Urn,
        super::BranchQuery { peer_id }: super::BranchQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let default_branch = coco::state::get_branch(&ctx.peer, project_urn, peer_id, None)
            .await
            .map_err(error::Error::from)?;
        let branches = coco::state::with_browser(&ctx.peer, default_branch, |browser| {
            coco::source::branches(browser, Some(coco::source::into_branch_type(peer_id)))
        })
        .await
        .map_err(error::Error::from)?;

        Ok(reply::json(&branches))
    }

    /// Fetch a [`coco::source::Commit`].
    pub async fn commit(
        project_urn: coco::Urn,
        sha1: Oid,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let default_branch = coco::state::find_default_branch(&ctx.peer, project_urn)
            .await
            .map_err(error::Error::from)?;
        let commit = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::commit(&mut browser, sha1)
        })
        .await
        .map_err(error::Error::from)?;

        Ok(reply::json(&commit))
    }

    /// Fetch the list of [`coco::source::Commit`] from a branch.
    pub async fn commits(
        ctx: context::Unsealed,
        project_urn: coco::Urn,
        super::CommitsQuery { revision }: super::CommitsQuery,
    ) -> Result<impl Reply, Rejection> {
        let revision = super::http::guard_self_revision(&ctx.peer, revision);

        let default_branch = coco::state::find_default_branch(&ctx.peer, project_urn)
            .await
            .map_err(error::Error::from)?;
        let commits = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::commits(&mut browser, revision)
        })
        .await
        .map_err(error::Error::from)?;

        Ok(reply::json(&commits))
    }

    /// Fetch the list [`coco::source::Branch`] for a local repository.
    pub async fn local_state(path: Tail) -> Result<impl Reply, Rejection> {
        let state = coco::source::local_state(path.as_str())
            .map_err(coco::state::Error::from)
            .map_err(error::Error::from)?;

        Ok(reply::json(&state))
    }

    /// Fetch the list [`coco::source::Tag`].
    pub async fn tags(
        project_urn: coco::Urn,
        _query: super::TagQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let branch = coco::state::find_default_branch(&ctx.peer, project_urn)
            .await
            .map_err(error::Error::from)?;
        let tags =
            coco::state::with_browser(&ctx.peer, branch, |browser| coco::source::tags(browser))
                .await
                .map_err(error::Error::from)?;

        Ok(reply::json(&tags))
    }

    /// Fetch a [`coco::source::Tree`].
    pub async fn tree(
        project_urn: coco::Urn,
        super::TreeQuery {
            prefix,
            peer_id,
            revision,
        }: super::TreeQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let revision = super::http::guard_self_revision(&ctx.peer, revision);
        let branch = coco::state::get_branch(&ctx.peer, project_urn, peer_id, None)
            .await
            .map_err(error::Error::from)?;
        let tree = coco::state::with_browser(&ctx.peer, branch, |mut browser| {
            coco::source::tree(&mut browser, revision, prefix)
        })
        .await
        .map_err(error::Error::from)?;

        Ok(reply::json(&tree))
    }
}

/// Bundled query params to pass to the commits handler.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitsQuery {
    /// Revision to query at.
    revision: Option<coco::source::Revision<coco::PeerId>>,
}

/// Bundled query params to pass to the blob handler.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobQuery {
    /// Location of the blob in tree.
    path: String,
    /// PeerId to scope the query by.
    peer_id: Option<coco::PeerId>,
    /// Revision to query at.
    revision: Option<coco::source::Revision<coco::PeerId>>,
    /// Whether or not to syntax highlight the blob.
    highlight: Option<bool>,
}

/// A query param for [`handler::branches`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    /// PeerId to scope the query by.
    peer_id: Option<coco::PeerId>,
}

/// Bundled query params to pass to the tree handler.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeQuery {
    /// Path prefix to query the tree.
    prefix: Option<String>,
    /// PeerId to scope the query by.
    peer_id: Option<coco::PeerId>,
    /// Revision to query at.
    revision: Option<coco::source::Revision<coco::PeerId>>,
}

/// A query param for [`handler::tags`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagQuery {
    /// PeerId to scope the query by.
    peer_id: Option<coco::PeerId>,
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{convert::TryFrom, env};

    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use warp::{http::StatusCode, test::request};

    use crate::{context, error, http};

    #[tokio::test]
    async fn blob() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;
        let revision = coco::source::Revision::Branch {
            name: "master".to_string(),
            peer_id: None,
        };
        let arrows = "text/arrows.txt";
        let default_branch = coco::state::find_default_branch(&ctx.peer, urn.clone()).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::blob(&mut browser, Some(revision.clone()), arrows, None)
        })
        .await?;

        let query = super::BlobQuery {
            path: arrows.to_string(),
            peer_id: None,
            revision: Some(revision.clone()),
            highlight: Some(false),
        };

        let path = format!("/blob/{}?{}", urn, serde_qs::to_string(&query).unwrap());

        // Get ASCII blob.
        let res = request().method("GET").path(&path).reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
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
        let ls = "bin/ls";
        let default_branch = coco::state::find_default_branch(&ctx.peer, urn.clone()).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |browser| {
            coco::source::blob(browser, Some(revision.clone()), ls, None)
        })
        .await?;

        let query = super::BlobQuery {
            path: ls.to_string(),
            peer_id: None,
            revision: Some(revision),
            highlight: Some(false),
        };

        let path = format!("/blob/{}?{}", urn, serde_qs::to_string(&query).unwrap());

        let res = request().method("GET").path(&path).reply(&api).await;

        http::test::assert_response(&res, StatusCode::OK, |have| {
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
    async fn blob_dev_branch() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;
        let revision = coco::source::Revision::Branch {
            name: "dev".to_string(),
            peer_id: None,
        };
        let path = "here-we-are-on-a-dev-branch.lol";

        let query = super::BlobQuery {
            path: path.to_string(),
            peer_id: None,
            revision: Some(revision.clone()),
            highlight: Some(false),
        };

        // Get ASCII blob.
        let res = request()
            .method("GET")
            .path(&format!(
                "/blob/{}?{}",
                urn,
                serde_qs::to_string(&query).unwrap()
            ))
            .reply(&api)
            .await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::blob(&mut browser, Some(revision), path, None)
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    #[tokio::test]
    async fn branches() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        let urn = replicate_platinum(&ctx).await?;

        let res = request()
            .method("GET")
            .path(&format!("/branches/{}", urn))
            .reply(&api)
            .await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |browser| {
            coco::source::branches(browser, None)
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(have, json!(["dev", "master"]));
        });

        Ok(())
    }

    #[tokio::test]
    #[allow(clippy::indexing_slicing)]
    async fn commit() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;
        let sha1 = coco::git_ext::Oid::try_from("3873745c8f6ffb45c990eb23b491d4b4b6182f95")?;

        let res = request()
            .method("GET")
            .path(&format!("/commit/{}/{}", urn, sha1))
            .reply(&api)
            .await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::commit_header(&mut browser, sha1)
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
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
    async fn commits() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;

        let branch_name = "dev";
        let revision = coco::source::Revision::Branch {
            name: branch_name.to_string(),
            peer_id: None,
        };
        let query = super::CommitsQuery {
            revision: Some(revision.clone()),
        };
        let res = request()
            .method("GET")
            .path(&format!(
                "/commits/{}?{}",
                urn.clone(),
                serde_qs::to_string(&query).unwrap(),
            ))
            .reply(&api)
            .await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::commits(&mut browser, Some(revision.clone()))
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    #[tokio::test]
    async fn local_state() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let mut path = env::current_dir()?;
        path.push("../../fixtures/git-platinum");

        let res = request()
            .method("GET")
            .path(&format!("/local-state/{}", path.to_str().unwrap()))
            .reply(&api)
            .await;

        let want = coco::source::local_state(path.to_str().unwrap()).unwrap();

        http::test::assert_response(&res, StatusCode::OK, |have| {
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
    async fn tags() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;

        let res = request()
            .method("GET")
            .path(&format!("/tags/{}", urn))
            .reply(&api)
            .await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |browser| {
            coco::source::tags(browser)
        })
        .await?;
        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!(["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"]),
            );
        });

        Ok(())
    }

    #[tokio::test]
    async fn tree() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        let urn = replicate_platinum(&ctx).await?;

        let prefix = "src";
        let revision = coco::source::Revision::Branch {
            name: "master".to_string(),
            peer_id: None,
        };
        let query = super::TreeQuery {
            prefix: Some(prefix.to_string()),
            peer_id: None,
            revision: Some(revision.clone()),
        };
        let path = format!("/tree/{}?{}", urn, serde_qs::to_string(&query).unwrap());
        let res = request().method("GET").path(&path).reply(&api).await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::tree(&mut browser, Some(revision), Some(prefix.to_string()))
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
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
    async fn tree_dev_branch() -> Result<(), Box<dyn std::error::Error>> {
        // Testing that the endpoint works with URL encoding
        const FRAGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'[')
            .add(b']')
            .add(b'=');

        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());
        let urn = replicate_platinum(&ctx).await?;

        let revision = coco::source::Revision::Branch {
            name: "dev".to_string(),
            peer_id: None,
        };
        let query = super::TreeQuery {
            prefix: None,
            peer_id: None,
            revision: Some(revision.clone()),
        };
        let path = format!(
            "/tree/{}?{}",
            urn,
            percent_encoding::utf8_percent_encode(&serde_qs::to_string(&query).unwrap(), FRAGMENT)
        );
        let res = request().method("GET").path(&path).reply(&api).await;

        let default_branch = coco::state::find_default_branch(&ctx.peer, urn).await?;
        let want = coco::state::with_browser(&ctx.peer, default_branch, |mut browser| {
            coco::source::tree(&mut browser, Some(revision), None)
        })
        .await?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    async fn replicate_platinum(ctx: &context::Unsealed) -> Result<coco::Urn, error::Error> {
        let owner = coco::state::init_owner(
            &ctx.peer,
            coco::identities::payload::Person {
                name: "cloudhead".into(),
            },
        )
        .await?;
        let platinum_project = crate::control::replicate_platinum(
            &ctx.peer,
            &owner,
            "git-platinum",
            "fixture data",
            crate::control::default_branch(),
        )
        .await?;
        Ok(platinum_project.urn())
    }
}
