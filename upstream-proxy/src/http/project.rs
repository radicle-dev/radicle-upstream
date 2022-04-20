// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Endpoints and serialisation for [`crate::project::Project`] related types.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use link_crypto::PeerId;
use link_identities::git::Urn;

use crate::{context, http};

mod request;

/// Combination of all routes.
pub fn filters(ctx: context::Context) -> BoxedFilter<(impl Reply,)> {
    checkout_filter(ctx.clone())
        .or(create_filter(ctx.clone()))
        .or(failed_filter(ctx.clone()))
        .or(get_filter(ctx.clone()))
        .or(owner_contributed_filter(ctx.clone()))
        .or(owner_tracked_filter(ctx.clone()))
        .or(peers_filter(ctx.clone()))
        .or(path("requests").and(request::filters(ctx.clone())))
        .or(track_filter(ctx.clone()))
        .or(patches_filter(ctx.clone()))
        .or(untrack_filter(ctx.clone()))
        .or(user_filter(ctx))
        .boxed()
}

/// `POST /<urn>/checkout`
fn checkout_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path("checkout"))
        .and(path::end())
        .and(warp::post())
        .and(http::with_context_unsealed(ctx))
        .and(warp::body::json())
        .and_then(handler::checkout)
}

/// `POST /`
fn create_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end()
        .and(warp::post())
        .and(http::with_context_unsealed(ctx.clone()))
        .and(http::with_owner_guard(ctx))
        .and(warp::body::json())
        .and_then(handler::create)
}

/// `GET /failed`
fn failed_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("failed")
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::list_failed)
}

/// `GET /<urn>`
fn get_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::get)
}

/// `GET /contributed`
fn owner_contributed_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("contributed")
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and(path::end())
        .and_then(handler::list_owner_contributed)
}

/// `GET /tracked`
fn owner_tracked_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("tracked")
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and(path::end())
        .and_then(handler::list_owner_tracked)
}

/// `GET /<urn>/peers`
fn peers_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    http::with_context_unsealed(ctx)
        .and(warp::get())
        .and(path::param::<Urn>())
        .and(path("peers"))
        .and(path::end())
        .and_then(handler::peers)
}

/// `PUT /<urn>/track/<peer_id>`
fn track_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path("track"))
        .and(path::param::<PeerId>())
        .and(path::end())
        .and(warp::put())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::track)
}

/// `PUT /<urn>/untrack/<peer_id>`
fn untrack_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path("untrack"))
        .and(path::param::<PeerId>())
        .and(path::end())
        .and(http::with_context_unsealed(ctx))
        .and(warp::put())
        .and_then(handler::untrack)
}

/// `GET /user/<urn>`
fn user_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("user")
        .and(path::param::<Urn>())
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::list_user)
}

/// `GET /<urn>/patches`
///
/// Get the list of patches for the project.
fn patches_filter(
    ctx: context::Context,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::param::<Urn>()
        .and(path("patches"))
        .and(path::end())
        .and(warp::get())
        .and(http::with_context_unsealed(ctx))
        .and_then(handler::patches)
}

/// Project handlers to implement conversion and translation between core domain and http request
/// fullfilment.
mod handler {
    use std::convert::TryFrom;

    use warp::{http::StatusCode, reply, Rejection, Reply};

    use link_crypto::PeerId;
    use link_identities::git::Urn;

    use crate::{browser, context, error::Error, http, patch, project};

    /// Checkout a [`project::Project`]'s source code.
    pub async fn checkout(
        urn: Urn,
        ctx: context::Unsealed,
        super::CheckoutInput { path, peer_id }: super::CheckoutInput,
    ) -> Result<impl Reply, Rejection> {
        let peer_id = http::guard_self_peer_id(&ctx.peer, peer_id);
        let path = crate::daemon::state::checkout(ctx.peer.librad_peer(), urn, peer_id, path)
            .await
            .map_err(Error::from)?;
        Ok(reply::with_status(reply::json(&path), StatusCode::CREATED))
    }

    /// Create a new [`project::Project`].
    pub async fn create(
        ctx: context::Unsealed,
        owner: crate::daemon::LocalIdentity,
        input: crate::daemon::project::Create,
    ) -> Result<impl Reply, Rejection> {
        let project = crate::daemon::state::init_project(ctx.peer.librad_peer(), &owner, input)
            .await
            .map_err(Error::from)?;
        let urn = project.urn();

        let branch = crate::daemon::state::get_branch(
            ctx.peer.librad_peer(),
            urn,
            None,
            project.subject().default_branch.clone(),
        )
        .await
        .map_err(Error::from)?;
        let stats = browser::using(&ctx.peer, branch, |browser| {
            browser.get_stats().map_err(radicle_source::Error::from)
        })
        .map_err(Error::from)?;
        let project = project::Project::try_from((project, stats, None))?;

        Ok(reply::with_status(
            reply::json(&project),
            StatusCode::CREATED,
        ))
    }

    /// Get the [`project::Project`] for the given `id`.
    pub async fn get(urn: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let seed = ctx.git_fetch.get_seed(urn.id);
        Ok(reply::json(&project::get(&ctx.peer, urn, seed).await?))
    }

    /// List all failed projects.
    pub async fn list_failed(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let projects = project::Projects::list(&ctx.peer).await?;

        Ok(reply::json(&projects.failures))
    }

    /// List all projects the current user has contributed to.
    pub async fn list_owner_contributed(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let projects = project::Projects::list(&ctx.peer).await?;

        Ok(reply::json(&projects.contributed))
    }

    /// List all projects tracked by the current user.
    pub async fn list_owner_tracked(ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let projects = project::Projects::list(&ctx.peer).await?.tracked;

        Ok(reply::json(&projects))
    }

    /// This lists all the projects for a given `user`. This `user` should not be your particular
    /// `user` (i.e. the "default user"), but rather should be another user that you are tracking.
    ///
    /// See [`project::list_for_user`] for more information.
    pub async fn list_user(user_id: Urn, ctx: context::Unsealed) -> Result<impl Reply, Rejection> {
        let projects = project::list_for_user(&ctx.peer, &user_id).await?;

        Ok(reply::json(&projects))
    }

    /// List the remote peers for a project.
    pub async fn peers(ctx: context::Unsealed, urn: Urn) -> Result<impl Reply, Rejection> {
        let peers: Vec<project::Peer> =
            crate::daemon::state::list_project_peers(ctx.peer.librad_peer(), urn)
                .await
                .map_err(Error::from)?
                .into_iter()
                .map(project::Peer::from)
                .collect::<Vec<_>>();

        Ok(reply::json(&peers))
    }

    /// Track the peer for the provided project.
    pub async fn track(
        urn: Urn,
        peer_id: PeerId,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        crate::daemon::state::track(ctx.peer.librad_peer(), urn.clone(), peer_id)
            .await
            .map_err(Error::from)?;
        ctx.git_fetch.add(urn.id).await;
        Ok(reply::json(&true))
    }

    /// Untrack the peer for the provided project.
    pub async fn untrack(
        urn: Urn,
        peer_id: PeerId,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        crate::daemon::state::untrack(ctx.peer.librad_peer(), urn, peer_id)
            .await
            .map_err(Error::from)?;
        Ok(reply::json(&true))
    }

    /// Get the list of patches for a project
    pub async fn patches(
        project_urn: Urn,
        ctx: context::Unsealed,
    ) -> Result<impl Reply, Rejection> {
        let patches = patch::list(&ctx.peer, project_urn)
            .await
            .map_err(Error::from)?;

        Ok(reply::json(&patches))
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
    peer_id: Option<PeerId>,
}

/// User provided metadata for project manipulation.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInput {
    /// Name of the project.
    name: String,
    /// Long form outline.
    description: String,
    /// Configured default branch.
    default_branch: String,
}
