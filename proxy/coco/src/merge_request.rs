//! TODO

use either::Either;
use librad::{peer::PeerId, signer::BoxedSigner};

use crate::state;

/// TODO
#[derive(Debug, Clone)]
pub struct MergeRequest {
    /// TODO
    pub id: String,
    /// TODO
    pub merged: bool,
    /// TODO
    pub peer: crate::project::Peer<crate::project::peer::Status<crate::Person>>,
    /// TODO
    pub message: Option<String>,
    /// TODO
    pub commit: git2::Oid,
}

/// TODO
///
/// # Errors
pub async fn list(
    peer: &crate::net::peer::Peer<BoxedSigner>,
    project_urn: crate::Urn,
) -> Result<Vec<MergeRequest>, crate::state::Error> {
    let mut merge_requests = Vec::new();
    let monorepo_path = crate::state::monorepo(peer);
    let monorepo = git2::Repository::open(monorepo_path)?;
    let namespace = librad::git::types::namespace::Namespace::from(project_urn.clone());
    let default_branch_head_commit = {
        let project = crate::state::get_project(peer, project_urn.clone())
            .await?
            .ok_or_else(|| crate::state::Error::ProjectNotFound(project_urn.clone()))?;
        let maintainer = project
            .delegations()
            .iter()
            .flat_map(|either| match either {
                Either::Left(pk) => Either::Left(std::iter::once(PeerId::from(*pk))),
                Either::Right(indirect) => {
                    Either::Right(indirect.delegations().iter().map(|pk| PeerId::from(*pk)))
                },
            })
            .next()
            .expect("missing delegation");
        // the `remote` for `get_branch` is set to the first maintainer, if the current `peer`
        // is that maintainer, `get_branch` will catch that and search the local peers directories.
        // The `branch` is set to `None` as `get_branch` will then fall back to the default branch.
        let default_branch =
            state::get_branch(peer, project_urn.clone(), Some(maintainer), None).await?;
        monorepo
            .find_reference(&default_branch.to_string())?
            .peel_to_commit()?
            .id()
    };

    for project_peer in crate::state::list_project_peers(peer, project_urn.clone()).await? {
        let remote = match project_peer {
            crate::project::Peer::Local { .. } => None,
            crate::project::Peer::Remote { peer_id, .. } => Some(peer_id),
        };
        let ref_pattern = librad::git::types::Reference {
            remote,
            category: librad::git::types::RefsCategory::Tags,
            name: librad::refspec_pattern!("merge-request/*"),
            namespace: Some(namespace.clone()),
        };
        let refs = ref_pattern.references(&monorepo)?;
        for r in refs {
            let r = r?;
            let tag = match r.peel_to_tag() {
                Ok(tag) => tag,
                Err(err) => {
                    tracing::warn!(?err, "merge request ref cannot be peeled");
                    continue;
                },
            };

            let merged = default_branch_head_commit == tag.target_id()
                || monorepo.graph_descendant_of(default_branch_head_commit, tag.target_id())?;

            let tag_name = match std::str::from_utf8(tag.name_bytes()) {
                Ok(tag_name) => tag_name,
                Err(err) => {
                    tracing::warn!(tag_id = ?tag.id(), ?err, "merge request tag name is not valid UTF-8");
                    continue;
                },
            };

            let id = tag_name
                .strip_prefix("merge-request/")
                .expect("invalid prefix");

            if tag.target_type() != Some(git2::ObjectType::Commit) {
                tracing::warn!(tag_id = ?tag.id(), "merge request tag target object is not a commit");
                continue;
            }

            merge_requests.push(MergeRequest {
                id: id.to_owned(),
                merged,
                peer: project_peer.clone(),
                message: tag.message().map(String::from),
                commit: tag.target_id(),
            })
        }
    }
    Ok(merge_requests)
}
