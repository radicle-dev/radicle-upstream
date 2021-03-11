use either::Either;
use librad::{peer::PeerId, signer::BoxedSigner};
use radicle_surf::vcs::git::Tag;

use crate::state;

#[derive(Debug, Clone)]
pub struct MergeRequest {
    pub id: String,
    pub merged: bool,
    pub peer: crate::project::Peer<crate::project::peer::Status<crate::Person>>,
    pub message: Option<String>,
    pub commit: git2::Oid,
}

/// TODO
///
/// # Errors
pub async fn list(
    peer: &crate::net::peer::Peer<BoxedSigner>,
    project: crate::Urn,
) -> Result<Vec<MergeRequest>, crate::state::Error> {
    let mut merge_requests = Vec::new();
    let monorepo_path = crate::state::monorepo(peer);
    let monorepo = git2::Repository::open(monorepo_path)?;
    let namespace = librad::git::types::namespace::Namespace::from(project.clone());
    let projectproject = crate::state::get_project(peer, project.clone())
        .await?
        .unwrap();
    let maintainer = projectproject
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
    let default_branch = state::get_branch(peer, project.clone(), Some(maintainer), None)
        .await
        .unwrap();
    let target = monorepo
        .find_reference(&default_branch.to_string())
        .unwrap()
        .target()
        .expect("couldn't find default branch target");

    for project_peer in crate::state::list_project_peers(peer, project.clone()).await? {
        let remote = match project_peer {
            crate::project::Peer::Local { .. } => None,
            crate::project::Peer::Remote { peer_id, .. } => Some(peer_id),
        };
        let ref_pattern = librad::git::types::Reference {
            remote: remote,
            category: librad::git::types::RefsCategory::Tags,
            name: librad::refspec_pattern!("merge-request/*"),
            namespace: Some(namespace.clone()),
        };
        let refs = ref_pattern.references(&monorepo)?;
        for r in refs {
            let r = r?;
            let tag = monorepo.find_tag(r.target().unwrap())?;
            let merged = target == tag.target_id() || monorepo
                .graph_descendant_of(target, tag.target_id())
                .unwrap();
            let id = tag.name().unwrap().strip_prefix("merge-request/").unwrap();
            assert_eq!(tag.target_type(), Some(git2::ObjectType::Commit));
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
