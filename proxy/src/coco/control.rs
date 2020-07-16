//! Utility for fixture data in the monorepo.

use std::convert::TryFrom;
use std::env;

use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use radicle_surf::vcs::git::git2;

use crate::coco::config;
use crate::coco::peer::{Api, User};
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
pub fn setup_fixtures(api: &Api, key: keys::SecretKey, owner: &User) -> Result<(), error::Error> {
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
        replicate_platinum(api, key.clone(), owner, info.0, info.1, info.2)?;
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
    api: &Api,
    key: keys::SecretKey,
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
    let monorepo = api.monorepo();
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_from, &platinum_into)?;

    let meta = api.init_project(
        key,
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
