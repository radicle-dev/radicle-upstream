// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints and serialisation for source code browsing.

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use link_crypto::PeerId;
use link_identities::git::Urn;
use radicle_git_ext::Oid;

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
        .and(path::param::<Urn>())
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
        .and(path::param::<Urn>())
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
        .and(path::param::<Urn>())
        .and(path::param::<Oid>())
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
        .and(path::param::<Urn>())
        .and(http::with_qs::<CommitsQuery>())
        .and_then(handler::commits)
}

/// `GET /local-state?path=<path>`
fn local_state_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("local-state")
        .and(warp::get())
        .and(http::with_qs::<LocalStateQuery>())
        .and_then(handler::local_state)
}

/// `GET /tags/<project_urn>?peer_id=<peer_id>`
fn tags_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tags")
        .and(path::param::<Urn>())
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
        .and(path::param::<Urn>())
        .and(path::end())
        .and(warp::get())
        .and(http::with_qs::<TreeQuery>())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::tree)
}

/// Source handlers for conversion between core domain and http request fullfilment.
mod handler {
    use warp::{reply, Rejection, Reply};

    use link_identities::git::Urn;
    use radicle_git_ext::Oid;
    use radicle_source::surf::vcs::git::RefScope;

    use crate::{browser, context, error};

    /// Fetch a [`radicle_source::Blob`].
    pub async fn blob(
        project_urn: Urn,
        super::BlobQuery {
            path,
            peer_id,
            revision,
            highlight,
        }: super::BlobQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let revision = super::http::guard_self_revision(&ctx.peer, revision);

        let theme = highlight.map(|theme| match theme {
            super::HighlightTheme::Dark => "base16-ocean.dark",
            super::HighlightTheme::Light => "base16-ocean.light",
            super::HighlightTheme::H4x0r => "base16-ocean.h4x0r",
        });

        let branch =
            crate::daemon::state::get_branch(ctx.peer.librad_peer(), project_urn, peer_id, None)
                .await
                .map_err(error::Error::from)?;
        let blob = browser::using(&ctx.peer, branch, |browser| {
            radicle_source::blob::highlighting::blob(browser, revision, &path, theme)
        })
        .map_err(error::Error::from)?;

        Ok(reply::json(&blob))
    }

    /// Fetch the list [`radicle_source::Branch`].
    pub async fn branches(
        project_urn: Urn,
        super::BranchQuery { peer_id }: super::BranchQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let default_branch =
            crate::daemon::state::get_branch(ctx.peer.librad_peer(), project_urn, peer_id, None)
                .await
                .map_err(error::Error::from)?;
        let branches = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::branches(browser, RefScope::from(peer_id))
        })
        .map_err(error::Error::from)?;

        Ok(reply::json(&branches))
    }

    /// Fetch a [`radicle_source::Commit`].
    pub async fn commit(
        project_urn: Urn,
        sha1: Oid,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), project_urn)
                .await
                .map_err(error::Error::from)?;
        let commit = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::commit(browser, *sha1)
        })
        .map_err(error::Error::from)?;

        Ok(reply::json(&commit))
    }

    /// Fetch the list of [`radicle_source::Commit`] from a branch.
    pub async fn commits(
        ctx: context::Unsealed,
        project_urn: Urn,
        super::CommitsQuery { revision }: super::CommitsQuery,
    ) -> Result<impl Reply, Rejection> {
        let revision = super::http::guard_self_revision(&ctx.peer, revision);

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), project_urn)
                .await
                .map_err(error::Error::from)?;
        let commits = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::commits(browser, revision)
        })
        .map_err(error::Error::from)?;

        Ok(reply::json(&commits))
    }

    /// Fetch the list [`radicle_source::Branch`] for a local repository.
    #[allow(clippy::unused_async)]
    pub async fn local_state(
        commits_query: super::LocalStateQuery,
    ) -> Result<impl Reply, Rejection> {
        let state = radicle_source::local_state(&commits_query.path, "master")
            .map_err(error::Error::from)?;

        Ok(reply::json(&state))
    }

    /// Fetch the list [`radicle_source::Tag`].
    pub async fn tags(
        project_urn: Urn,
        _query: super::TagQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let branch = crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), project_urn)
            .await
            .map_err(error::Error::from)?;
        let tags = browser::using(&ctx.peer, branch, |browser| radicle_source::tags(browser))
            .map_err(error::Error::from)?;

        Ok(reply::json(&tags))
    }

    /// Fetch a [`radicle_source::Tree`].
    pub async fn tree(
        project_urn: Urn,
        super::TreeQuery {
            prefix,
            peer_id,
            revision,
        }: super::TreeQuery,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = super::http::guard_self_peer_id(&ctx.peer, peer_id);
        let revision = super::http::guard_self_revision(&ctx.peer, revision);
        let branch =
            crate::daemon::state::get_branch(ctx.peer.librad_peer(), project_urn, peer_id, None)
                .await
                .map_err(error::Error::from)?;
        let tree = browser::using(&ctx.peer, branch, |browser| {
            radicle_source::tree(browser, revision, prefix)
        })
        .map_err(error::Error::from)?;

        Ok(reply::json(&tree))
    }
}

/// Query parameters for [`handler::local_state`]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalStateQuery {
    /// Path to the repository to get the local state of
    path: String,
}

/// Bundled query params to pass to the commits handler.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitsQuery {
    /// Revision to query at.
    revision: Option<radicle_source::Revision<PeerId>>,
}

/// Bundled query params to pass to the blob handler.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobQuery {
    /// Location of the blob in tree.
    path: String,
    /// PeerId to scope the query by.
    peer_id: Option<PeerId>,
    /// Revision to query at.
    revision: Option<radicle_source::Revision<PeerId>>,
    /// Whether or not to syntax highlight the blob.
    highlight: Option<HighlightTheme>,
}

/// A query param for [`handler::branches`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    /// PeerId to scope the query by.
    peer_id: Option<PeerId>,
}

/// Bundled query params to pass to the tree handler.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeQuery {
    /// Path prefix to query the tree.
    prefix: Option<String>,
    /// PeerId to scope the query by.
    peer_id: Option<PeerId>,
    /// Revision to query at.
    revision: Option<radicle_source::Revision<PeerId>>,
}

/// A query param for [`handler::tags`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagQuery {
    /// PeerId to scope the query by.
    pub peer_id: Option<PeerId>,
}

/// Syntax highlighting theme
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HighlightTheme {
    /// A dark theme.
    Dark,
    /// A light theme.
    Light,
    /// A h4x0r theme.
    H4x0r,
}

#[allow(clippy::non_ascii_literal, clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{convert::TryFrom as _, env};

    use pretty_assertions::assert_eq;
    use serde_json::json;
    use warp::{http::StatusCode, test::request};

    use link_identities::git::Urn;
    use radicle_source::surf::vcs::git::RefScope;

    use crate::{browser, context, error, http};

    #[allow(clippy::too_many_lines)]
    #[tokio::test]
    async fn blob() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir()?;
        let (ctx, _) = context::Unsealed::tmp(&tmp_dir)?;
        let api = super::filters(ctx.clone().into());

        let urn = replicate_platinum(&ctx).await?;
        let revision = radicle_source::Revision::Branch {
            name: "master".to_string(),
            peer_id: None,
        };
        let arrows = "text/arrows.txt";
        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn.clone()).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::blob(browser, Some(revision.clone()), arrows)
        })?;

        let query = super::BlobQuery {
            path: arrows.to_string(),
            peer_id: None,
            revision: Some(revision.clone()),
            highlight: None,
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
        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn.clone()).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::blob(browser, Some(revision.clone()), ls)
        })?;

        let query = super::BlobQuery {
            path: ls.to_string(),
            peer_id: None,
            revision: Some(revision),
            highlight: None,
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
                    "content": want.content,
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
        let revision = radicle_source::Revision::Branch {
            name: "dev".to_string(),
            peer_id: None,
        };
        let path = "here-we-are-on-a-dev-branch.lol";

        let query = super::BlobQuery {
            path: path.to_string(),
            peer_id: None,
            revision: Some(revision.clone()),
            highlight: None,
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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::blob(browser, Some(revision), path)
        })?;

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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::branches(browser, RefScope::All)
        })?;

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
        let sha1 = radicle_git_ext::Oid::try_from("3873745c8f6ffb45c990eb23b491d4b4b6182f95")?;

        let res = request()
            .method("GET")
            .path(&format!("/commit/{}/{}", urn, sha1))
            .reply(&api)
            .await;

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::commit::header(browser, *sha1)
        })?;

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
        let revision = radicle_source::Revision::Branch {
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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::commits(browser, Some(revision.clone()))
        })?;

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
        path.push("../fixtures/git-platinum");

        let res = request()
            .method("GET")
            .path(&format!(
                "/local-state?{}",
                serde_qs::to_string(&super::LocalStateQuery {
                    path: path.to_str().unwrap().to_string()
                })
                .unwrap()
            ))
            .reply(&api)
            .await;

        let want = radicle_source::local_state(path.to_str().unwrap(), "master").unwrap();

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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::tags(browser)
        })?;
        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
            assert_eq!(
                have,
                json!(["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0", "v0.6.0"]),
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
        let revision = radicle_source::Revision::Branch {
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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::tree(browser, Some(revision), Some(prefix.to_string()))
        })?;

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

        let revision = radicle_source::Revision::Branch {
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

        let default_branch =
            crate::daemon::state::find_default_branch(ctx.peer.librad_peer(), urn).await?;
        let want = browser::using(&ctx.peer, default_branch, |browser| {
            radicle_source::tree(browser, Some(revision), None)
        })?;

        http::test::assert_response(&res, StatusCode::OK, |have| {
            assert_eq!(have, json!(want));
        });

        Ok(())
    }

    async fn replicate_platinum(ctx: &context::Unsealed) -> Result<Urn, error::Error> {
        let owner = crate::daemon::state::init_owner(
            ctx.peer.librad_peer(),
            link_identities::payload::Person {
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
