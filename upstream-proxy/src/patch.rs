// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! [`list`] all the [`Patch`]es for project.

use anyhow::Context as _;
use either::Either;
use radicle_git_ext::Oid;
use serde::Serialize;

use link_identities::git::Urn;

use crate::project;

const TAG_PREFIX: &str = "radicle-patch/";

/// A patch is a change set that a user wants the delegate to merge into a projects default
/// branch.
///
/// A patch is represented by an annotated tag, prefixed with `radicle-patch/`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Patch {
    /// ID of a patch. This is the portion of the tag name following the `radicle-patch/` prefix.
    pub id: String,
    /// Peer that the patch originated from
    pub peer: project::Peer,
    /// Message attached to the patch. This is the message of the annotated tag.
    pub message: Option<String>,
    /// Head commit that the author wants to merge with this patch.
    pub commit: Oid,
    /// The merge base of [`Patch::commit`] and the head commit of the first delegate's default
    /// branch.
    pub merge_base: Option<Oid>,
}

/// List all patches for the given project.
///
/// # Errors
/// * Cannot access the monorepo
/// * Cannot find references within the monorepo
pub async fn list(peer: &crate::peer::Peer, project_urn: Urn) -> anyhow::Result<Vec<Patch>> {
    let default_branch_head_commit_id = {
        let project = peer
            .librad_peer()
            .using_storage({
                let project_urn = project_urn.clone();
                move |store| librad::git::identities::project::get(store, &project_urn)
            })
            .await
            .context("failed to access storage")?
            .context("failed to get project")?
            .ok_or_else(|| anyhow::anyhow!("project {project_urn} not found"))?;
        let first_delegate = project
            .delegations()
            .iter()
            .flat_map(|either| match either {
                Either::Left(pk) => Either::Left(std::iter::once(pk)),
                Either::Right(indirect) => Either::Right(indirect.delegations().iter()),
            })
            .next()
            .context("project does not have any delegations")?;
        let first_delegate = librad::PeerId::from(*first_delegate);
        let remote = if first_delegate == peer.librad_peer().peer_id() {
            None
        } else {
            Some(first_delegate)
        };

        let default_branch_ref_name =
            if let Some(ref default_branch_name) = project.subject().default_branch {
                default_branch_name
                    .parse::<radicle_git_ext::RefLike>()
                    .context("invalid default branch name")?
            } else {
                librad::reflike!("main")
            };

        let reference = librad::git::types::Reference::head(
            librad::git::types::Namespace::from(project_urn.clone()),
            remote,
            default_branch_ref_name,
        );
        peer.monorepo_unblock(move |repo| Ok(reference.oid(&repo)?))
            .await
            .context("failed to resolve git reference")?
    };

    let mut patches = Vec::new();

    for project_peer in
        crate::daemon::state::list_project_peers(peer.librad_peer(), project_urn.clone()).await?
    {
        let namespace = project_urn.encode_id();
        let ref_glob = match &project_peer {
            crate::daemon::project::Peer::Local { .. } => {
                format!("refs/namespaces/{namespace}/refs/tags/{TAG_PREFIX}*")
            },
            crate::daemon::project::Peer::Remote { peer_id, .. } => {
                format!("refs/namespaces/{namespace}/refs/remotes/{peer_id}/tags/{TAG_PREFIX}*")
            },
        };

        let patch = peer
            .monorepo_unblock({
                move |repo| {
                    let mut patches = vec![];
                    for ref_result in repo
                        .references_glob(&ref_glob)
                        .context("failed to get references from glob")?
                    {
                        let reference = ref_result.context("failed to resolve reference")?;
                        let tag = reference
                            .peel_to_tag()
                            .context("failed to peel reference to tag")?;

                        let id = tag
                            .name()
                            .ok_or_else(|| anyhow::anyhow!("tag name is not valid UTF-8"))?
                            .strip_prefix(TAG_PREFIX)
                            .expect("tag name must have prefix")
                            .to_string();
                        let commit_id = tag.target_id();
                        let merge_base =
                            match repo.merge_base(commit_id, default_branch_head_commit_id) {
                                Ok(base) => Some(Oid::from(base)),
                                Err(err) if err.code() == git2::ErrorCode::NotFound => None,
                                Err(err) => {
                                    return Err(err)
                                        .context("failed to determine merge base for commits")
                                },
                            };
                        patches.push(Patch {
                            id,
                            peer: project_peer.clone().into(),
                            message: Some(
                                tag.message()
                                    .ok_or_else(|| {
                                        anyhow::anyhow!("tag message is not valid UTF-8")
                                    })?
                                    .to_string(),
                            ),
                            commit: Oid::from(commit_id),
                            merge_base,
                        })
                    }
                    Ok(patches)
                }
            })
            .await?;

        patches.extend(patch)
    }

    Ok(patches)
}
