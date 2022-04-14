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
