//! Utility for fixture data in the monorepo.

use std::{convert::TryFrom, env, io, path};

use librad::{
    keys,
    meta::{entity, project as librad_project},
    peer::PeerId,
};
use radicle_surf::vcs::git::git2;

use crate::{
    config, project, signer,
    state::{Error, State},
    user::User,
};

/// Generate a fresh `PeerId` for use in tests.
#[must_use]
pub fn generate_peer_id() -> PeerId {
    PeerId::from(keys::SecretKey::new())
}

/// Deletes the local git repsoitory coco uses for its state.
///
/// # Errors
///
/// * if the call to [`std::fs::remove_dir_all`] fails.
pub fn reset_monorepo() -> Result<(), std::io::Error> {
    let paths =
        librad::paths::Paths::try_from(config::Paths::default()).expect("unable to create paths");
    std::fs::remove_dir_all(paths.git_dir())
}

/// Creates a small set of projects in your peer.
///
/// # Errors
///
/// Will error if filesystem access is not granted or broken for the configured
/// [`librad::paths::Paths`].
pub async fn setup_fixtures(
    api: &State,
    signer: &signer::BoxedSigner,
    owner: &User,
) -> Result<(), Error> {
    let infos = vec![
        ("monokel", "A looking glass into the future", "master"),
        (
            "Monadic",
            "Open source organization of amazing things.",
            "master",
        ),
        (
            "open source coin",
            "Research for the sustainability of the open source community.",
            "master",
        ),
        (
            "radicle",
            "Decentralized open source collaboration",
            "master",
        ),
    ];

    for info in infos {
        replicate_platinum(api, signer, owner, info.0, info.1, info.2).await?;
    }

    Ok(())
}

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`Error`] if any of the git interaction fail, or the initialisation of
/// the coco project.
pub async fn replicate_platinum(
    api: &State,
    signer: &signer::BoxedSigner,
    owner: &User,
    name: &str,
    description: &str,
    default_branch: &str,
) -> Result<librad_project::Project<entity::Draft>, Error> {
    // Construct path for fixtures to clone into.
    let monorepo = api.monorepo();
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_into)?;

    let project_creation = project::Create {
        description: description.to_string(),
        default_branch: default_branch.to_string(),
        repo: project::Repo::Existing {
            path: platinum_into.clone(),
        },
    };

    let meta = api.init_project(signer, owner, project_creation).await?;

    // Push branches and tags.
    {
        let repo = git2::Repository::open(platinum_into)?;
        let mut rad_remote = repo.find_remote("rad")?;

        // Push all tags to rad remote.
        let tags = repo
            .tag_names(None)?
            .into_iter()
            .flatten()
            .map(|t| format!("+refs/tags/{}", t))
            .collect::<Vec<_>>();
        rad_remote.push(&tags, None)?;

        // Push branches.
        rad_remote.push(&["refs/heads/dev", "refs/heads/master"], None)?;
    }

    // Init as rad project.
    Ok(meta)
}

/// Craft the absolute path to git-platinum fixtures.
///
/// # Errors
///
///   * Failed to get current directory
pub fn platinum_directory() -> io::Result<path::PathBuf> {
    let mut platinum_path = env::current_dir()?;

    if platinum_path.as_path().ends_with("proxy") {
        platinum_path.push("..");
    } else {
        platinum_path.push("../..");
    }

    platinum_path.push("fixtures/git-platinum");
    Ok(path::Path::new("file://").join(platinum_path))
}

/// Create and track a fake peer.
pub async fn track_fake_peer(
    state: &State,
    signer: &signer::BoxedSigner,
    project: &librad_project::Project<entity::Draft>,
    fake_user_handle: &str,
) -> (
    PeerId,
    librad::meta::entity::Entity<librad::meta::user::UserInfo, librad::meta::entity::Draft>,
) {
    // TODO(finto): We're faking a lot of the networking interaction here.
    // Create git references of the form and track the peer.
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/signed_refs/heads
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/id
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/self <- points
    //   to fake_user
    let urn = project.urn();
    let fake_user =
        state.init_user(signer, fake_user_handle).await.unwrap_or_else(|_| panic!("User account creation for fake peer: {} failed, make sure your mocked user accounts don't clash!", fake_user_handle));
    let remote = generate_peer_id();
    let monorepo = git2::Repository::open(state.monorepo()).expect("failed to open monorepo");
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

    state
        .track(urn, remote.clone())
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
    let platinum_from = platinum_directory().expect("failed to get platinum directory");
    let platinum_from = platinum_from
        .to_str()
        .expect("failed to get platinum directory");
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(platinum_from, platinum_into.as_ref())?;

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
