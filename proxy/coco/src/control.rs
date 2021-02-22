//! Utility for fixture data in the monorepo.

use std::{env, io, path, str::FromStr};

use nonempty::NonEmpty;

use librad::{
    git::{
        identities::local::LocalIdentity,
        local::{transport, url::LocalUrl},
        types::{
            remote::{LocalPushspec, Remote},
            Force, Pushspec, Refspec,
        },
    },
    identities::Project,
    keys,
    net::peer::Peer,
    peer::PeerId,
    reflike, refspec_pattern,
    signer::BoxedSigner,
};
use radicle_git_ext::OneLevel;
use radicle_surf::vcs::git::git2;

use crate::{
    project,
    state::{self, Error},
};

/// Generate a fresh `PeerId` for use in tests.
#[must_use]
pub fn generate_peer_id() -> PeerId {
    PeerId::from(keys::SecretKey::new())
}

/// Creates a small set of projects in your peer.
///
/// # Errors
///
/// Will error if filesystem access is not granted or broken for the configured
/// [`librad::paths::Paths`].
pub async fn setup_fixtures(
    peer: &Peer<BoxedSigner>,
    owner: &LocalIdentity,
) -> Result<Vec<Project>, Error> {
    let infos = vec![
        (
            "monokel",
            "A looking glass into the future",
            default_branch(),
        ),
        (
            "Monadic",
            "Open source organization of amazing things.",
            default_branch(),
        ),
        (
            "open source coin",
            "Research for the sustainability of the open source community.",
            default_branch(),
        ),
        (
            "radicle",
            "Decentralized open source collaboration",
            default_branch(),
        ),
    ];

    let mut projects = Vec::with_capacity(infos.len());
    for info in infos {
        projects.push(replicate_platinum(peer, owner, info.0, info.1, info.2).await?);
    }
    Ok(projects)
}

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`Error`] if any of the git interaction fail, or the initialisation of
/// the coco project.
pub async fn replicate_platinum(
    peer: &Peer<BoxedSigner>,
    owner: &LocalIdentity,
    name: &str,
    description: &str,
    default_branch: OneLevel,
) -> Result<Project, Error> {
    // Construct path for fixtures to clone into.
    let monorepo = state::monorepo(peer);
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_into)?;

    let project_creation = project::Create {
        description: description.to_string(),
        default_branch,
        repo: project::Repo::Existing {
            path: platinum_into.clone(),
        },
    };

    let meta = state::init_project(peer, owner, project_creation).await?;

    // Push branches and tags.
    {
        let repo = git2::Repository::open(platinum_into)?;
        let mut rad = Remote::rad_remote(
            LocalUrl::from(meta.urn()),
            Refspec {
                src: refspec_pattern!("refs/tags/*"),
                dst: refspec_pattern!("refs/tags/*"),
                force: Force::False,
            },
        );
        let storage = state::settings(peer);
        // Push all tags to rad remote.
        push_tags(&mut rad, storage, &repo)?
    }

    // Init as rad project.
    Ok(meta)
}

/// Push any tags that are in the `repo` to the monorepo storage.
///
/// # Errors
///   * If we could not retrive the tag names from the repository.
pub fn push_tags(
    remote: &mut Remote<LocalUrl>,
    storage: transport::Settings,
    repo: &git2::Repository,
) -> Result<(), Error> {
    let tags = repo
        .tag_names(None)?
        .into_iter()
        .flatten()
        .flat_map(|tag| Pushspec::from_str(&format!("+refs/tags/{}:refs/tags/{}", tag, tag)).ok())
        .collect::<Vec<_>>();
    let tags = NonEmpty::from_vec(tags);

    match tags {
        None => {
            log::debug!("No tags to push to remote");
            Ok(())
        },
        Some(tags) => {
            let _ = remote.push(storage, repo, LocalPushspec::Specs(tags));
            Ok(())
        },
    }
}

/// Return the canonicalized path to the `git-platinum` fixtures repo.
///
/// # Errors
///
///   * The path could not be canonicalized. This happens if the path does not exist.
fn platinum_directory() -> io::Result<path::PathBuf> {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    path::Path::new(cargo_manifest_dir)
        .join("../../fixtures/git-platinum")
        .canonicalize()
}

/// TODO(finto): Burn this. It's just a big foot gun and it breaks whenever we try to access
/// `signed_refs`.
///
/// Create and track a fake peer.
pub async fn track_fake_peer(
    peer: &Peer<BoxedSigner>,
    project: &Project,
    fake_user_handle: &str,
) -> (PeerId, LocalIdentity) {
    // TODO(finto): We're faking a lot of the networking interaction here.
    // Create git references of the form and track the peer.
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/signed_refs/heads
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/id
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/self <- points
    //   to fake_user
    let urn = project.urn();
    let fake_user =
        state::init_user(peer, fake_user_handle.to_string()).await.unwrap_or_else(|_| panic!("User account creation for fake peer: {} failed, make sure your mocked user accounts don't clash!", fake_user_handle));
    let remote = generate_peer_id();
    let monorepo = git2::Repository::open(state::monorepo(peer)).expect("failed to open monorepo");
    let prefix = format!("refs/namespaces/{}/refs/remotes/{}", urn.id, remote);

    // Grab the Oid of master for the given project.
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/heads/master", urn.id))
        .expect("failed to get master")
        .target()
        .expect("missing target");

    // TODO: try pass branches in
    // Creates a new reference to the 'target' Oid above.
    {
        let _heads = monorepo
            .reference(
                &format!("{}/heads/master", prefix),
                target,
                false,
                "remote heads",
            )
            .expect("failed to create heads");
    }

    // Copy the rad/id under the remote
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/rad/id", urn.id))
        .expect("failed to get rad/id")
        .target()
        .expect("missing target");
    {
        let _rad_id = monorepo
            .reference(&format!("{}/rad/id", prefix), target, false, "rad/id")
            .expect("failed to create rad/id");
    }

    // Create symlink to the User Identity for this project
    {
        let _rad_self = monorepo
            .reference_symbolic(
                &format!("{}/rad/self", prefix),
                &format!("refs/namespaces/{}/refs/rad/id", fake_user.urn().id),
                false,
                "rad/self",
            )
            .expect("failed to create rad/self");
    }

    // Create the copy of the rad/refs under the remote
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/rad/signed_refs", urn.id))
        .expect("failed to get rad/signed_refs")
        .target()
        .expect("missing target");
    {
        let _rad_id = monorepo
            .reference(
                &format!("{}/rad/signed_refs", prefix),
                target,
                false,
                "rad/signed_refs",
            )
            .expect("failed to create rad/refs");
    }

    state::track(peer, urn, remote)
        .await
        .expect("failed to track peer");

    (remote, fake_user)
}

/// This function exists as a standalone because the logic does not play well with async in
/// `replicate_platinum`.
///
/// # Errors
///
///   * Cloning the repository failed
///   * We could not fetch branches
///
/// # Panics
///
///   * The platinum directory path was malformed
///   * Getting the branches fails
pub fn clone_platinum(platinum_into: impl AsRef<path::Path>) -> Result<(), Error> {
    let platinum_dir = platinum_directory().expect("failed to get platinum directory");
    let platinum_from = format!(
        "file://{}",
        platinum_dir
            .to_str()
            .expect("failed to get platinum directory")
    );
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(&platinum_from, platinum_into.as_ref())?;

    {
        let branches = platinum_repo.branches(Some(git2::BranchType::Remote))?;

        for branch in branches {
            let (branch, _branch_type) = branch?;
            let name = &branch
                .name()
                .expect("unable to get branch name")
                .expect("branch not present")
                .get(7..)
                .expect("unable to extract branch name");
            let oid = branch.get().target().expect("can't find OID");
            let commit = platinum_repo.find_commit(oid)?;

            if *name != "master" {
                platinum_repo.branch(name, &commit, false)?;
            }
        }
    }

    Ok(())
}

/// **Testing Only**
///
/// Default reference name for testing purposes.
#[must_use]
pub fn default_branch() -> OneLevel {
    OneLevel::from(reflike!("master"))
}
