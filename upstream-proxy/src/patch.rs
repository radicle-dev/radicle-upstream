// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! [`list`] all the [`Patch`]es for project.

use either::Either;
use radicle_git_ext::Oid;
use radicle_source::surf::git::RefScope;
use serde::Serialize;

use link_crypto::PeerId;
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
pub async fn list(
    peer: &crate::peer::Peer,
    project_urn: Urn,
) -> Result<Vec<Patch>, crate::error::Error> {
    let mut patches = Vec::new();

    let default_branch_head_commit_id = {
        let project = crate::daemon::state::get_project(peer.librad_peer(), project_urn.clone())
            .await?
            .ok_or_else(|| crate::daemon::state::Error::ProjectNotFound(project_urn.clone()))?;
        let delegate = project
            .delegations()
            .iter()
            .flat_map(|either| match either {
                Either::Left(pk) => Either::Left(std::iter::once(pk)),
                Either::Right(indirect) => Either::Right(indirect.delegations().iter()),
            })
            .next()
            .expect("missing delegation");
        let default_branch = crate::daemon::state::get_branch(
            peer.librad_peer(),
            project_urn.clone(),
            Some(PeerId::from(*delegate)),
            None,
        )
        .await?;
        crate::browser::using(peer, default_branch, move |browser| {
            Ok(browser.get().first().clone())
        })?
        .id
    };

    for project_peer in
        crate::daemon::state::list_project_peers(peer.librad_peer(), project_urn.clone()).await?
    {
        let remote = match &project_peer {
            crate::daemon::project::Peer::Local { .. } => None,
            crate::daemon::project::Peer::Remote { peer_id, .. } => Some(*peer_id),
        };

        let ref_scope = match remote {
            Some(remote) => RefScope::Remote {
                name: Some(remote.to_string()),
            },
            None => RefScope::Local,
        };

        let branch = match crate::daemon::state::get_branch(
            peer.librad_peer(),
            project_urn.clone(),
            remote,
            None,
        )
        .await
        {
            Ok(branch) => branch,
            Err(crate::daemon::state::Error::MissingRef { .. }) => {
                // The peer hasn’t published any branches yet.
                continue;
            },
            Err(e) => return Err(e.into()),
        };

        crate::browser::using(peer, branch, {
            let patches = &mut patches;
            move |browser| {
                let tags = browser.list_tags(ref_scope)?;
                for tag in tags {
                    match tag {
                        radicle_source::surf::git::Tag::Light { .. } => {
                            continue;
                        },
                        radicle_source::surf::git::Tag::Annotated {
                            target_id,
                            name,
                            message,
                            ..
                        } => {
                            let id = match name.to_string().strip_prefix(TAG_PREFIX) {
                                Some(id) => id.to_string(),
                                None => continue,
                            };

                            let merge_base = browser
                                .merge_base(target_id, default_branch_head_commit_id)?
                                .map(Oid::from);
                            patches.push(Patch {
                                id,
                                peer: project_peer.clone().into(),
                                message,
                                commit: Oid::from(target_id),
                                merge_base,
                            });
                        },
                    }
                }

                Ok(())
            }
        })?;
    }

    Ok(patches)
}
