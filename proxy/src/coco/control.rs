use std::convert::TryFrom;
use std::env;

use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use librad::net::peer::PeerApi;
use radicle_surf::vcs::git::git2;

use crate::coco::config;
use crate::coco::peer::{init_project, init_user, User};
use crate::error;

/// Deletes the local git repsoitory coco uses to keep its state.
///
/// # Errors
///
/// Will error in case the call to the [`std::fs::remove_dir_all`] fails.
pub fn nuke_monorepo() -> Result<(), std::io::Error> {
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
#[allow(clippy::needless_pass_by_value)] // We don't want to keep `SecretKey` in memory.
pub fn setup_fixtures(
    peer: &PeerApi,
    key: keys::SecretKey,
    owner: &User,
) -> Result<(), error::Error> {
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
        // let path = format!("{}/{}/{}", root, "repos", info.0);
        // std::fs::create_dir_all(path.clone())?;
        replicate_platinum(peer, &key, owner, info.0, info.1, info.2)?;
    }

    Ok(())
}

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the git interaction fail, or the initialisation of
/// the coco project.
pub fn replicate_platinum(
    peer: &PeerApi,
    key: &keys::SecretKey,
    owner: &User,
    name: &str,
    description: &str,
    default_branch: &str,
) -> Result<project::Project<entity::Draft>, error::Error> {
    // Craft the absolute path to git-platinum fixtures.
    let mut platinum_path = env::current_dir()?;
    platinum_path.push("../fixtures/git-platinum");
    let mut platinum_from = String::from("file://");
    platinum_from.push_str(platinum_path.to_str().expect("unable get path"));

    // Construct path for fixtures to clone into.
    let monorepo = peer.paths().git_dir().join("");
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_from, &platinum_into)?;

    let meta = init_project(
        peer,
        key.clone(),
        owner,
        platinum_into.clone(),
        name,
        description,
        default_branch,
    )?;

    // Push branches and tags.
    {
        let repo = git2::Repository::open(platinum_into)?;
        let mut rad_remote = repo.find_remote("rad")?;
        let namespace_prefix = format!("refs/namespaces/{}/refs", meta.urn().id);

        // Push all tags to rad remote.
        let tags = repo
            .tag_names(None)?
            .into_iter()
            .flatten()
            .map(|t| format!("+refs/tags/{}:{}/tags/{}", t, namespace_prefix, t))
            .collect::<Vec<_>>();
        rad_remote.push(&tags, None)?;

        // Push branches.
        rad_remote.push(
            &[
                &format!("refs/heads/dev:{}/heads/dev", namespace_prefix),
                &format!("refs/heads/master:{}/heads/master", namespace_prefix),
            ],
            None,
        )?;
    }

    // Init as rad project.
    Ok(meta)
}

/// Create and track a fake peer.
#[must_use]
pub fn track_fake_peer(
    peer: &PeerApi,
    key: keys::SecretKey,
    project: &project::Project<entity::Draft>,
    fake_user_handle: &str,
) -> (
    librad::peer::PeerId,
    librad::meta::entity::Entity<librad::meta::user::UserInfo, librad::meta::entity::Draft>,
) {
    // TODO(finto): We're faking a lot of the networking interaction here.
    // Create git references of the form and track the peer.
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/refs/heads
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/id
    //   refs/namespaces/<platinum_project.id>/remotes/<fake_peer_id>/rad/self <- points
    //   to fake_user
    let urn = project.urn();
    let fake_user =
        init_user(peer, key, fake_user_handle).unwrap_or_else(|_| panic!("User account creation for fake peer: {} failed, make sure your mocked user accounts don't clash!", fake_user_handle));
    let remote = librad::peer::PeerId::from(keys::SecretKey::new());
    let monorepo = git2::Repository::open(peer.paths().git_dir()).expect("failed to open monorepo");
    let prefix = format!("refs/namespaces/{}/refs/remotes/{}", urn.id, remote);

    // Grab the Oid of master for the given project.
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/heads/master", urn.id))
        .expect("failed to get master")
        .target()
        .expect("missing target");

    // TODO: try pass branches in
    // Creates a new reference to the 'target' Oid above.
    let _heads = monorepo
        .reference(
            &format!("{}/heads/master", prefix),
            target,
            false,
            "remote heads",
        )
        .expect("failed to create heads");

    // Copy the rad/id under the remote
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/rad/id", urn.id))
        .expect("failed to get rad/id")
        .target()
        .expect("missing target");
    let _rad_id = monorepo
        .reference(&format!("{}/rad/id", prefix), target, false, "rad/id")
        .expect("failed to create rad/id");

    // Create symlink to the User Identity for this project
    let _rad_self = monorepo
        .reference_symbolic(
            &format!("{}/rad/self", prefix),
            &format!("refs/namespaces/{}/refs/rad/id", fake_user.urn().id),
            false,
            "rad/self",
        )
        .expect("failed to create rad/self");

    // Create the copy of the rad/refs under the remote
    let target = monorepo
        .find_reference(&format!("refs/namespaces/{}/refs/rad/refs", urn.id))
        .expect("failed to get rad/refs")
        .target()
        .expect("missing target");
    let _rad_id = monorepo
        .reference(&format!("{}/rad/refs", prefix), target, false, "rad/refs")
        .expect("failed to create rad/refs");

    peer.storage()
        .track(&urn, &remote)
        .expect("failed to track peer");

    (remote, fake_user)
}

/// This function exists as a standalone because the logic does not play well with async in
/// `replicate_platinum`.
fn clone_platinum(
    platinum_from: &str,
    platinum_into: &std::path::PathBuf,
) -> Result<(), error::Error> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(platinum_from, platinum_into.as_path())
        .expect("unable to clone fixtures repo");

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
