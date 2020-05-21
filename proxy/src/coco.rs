//! Abstractions and utilities for git interactions through the API.

use std::env;
use std::str::FromStr;

use librad::git;
use librad::keys;
use librad::uri::{self, RadUrn};
use librad::meta::entity;
use librad::meta::user::User;
use librad::meta::project;
use librad::paths::Paths;
use librad::surf;
use librad::surf::git::git2;
use librad::git::storage::Storage;
use radicle_keystore::{Keystore, SecretKeyExt};

use crate::error;

mod types;
pub use types::*; // TODO: make explicit

// TODO(finto): should bundle these up so we can pass them all in at once
pub async fn get_browser<'a, K, R>(
    paths: &Paths,
    key_store: &K,
    project_resolver: &R,
    project_urn: String,
) -> Result<(surf::git::Browser<'a>, project::Project), error::Error>
where
    K: Keystore<
        PublicKey = keys::PublicKey,
        SecretKey = keys::SecretKey,
        Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
        Error = error::Error
    >,
    R: entity::Resolver<project::Project>,
{
    let project_urn = RadUrn::from_str(&project_urn)?;
    let keypair = key_store.get_key()?;
    let project = project_resolver.resolve(&project_urn).await?;

    let storage = Storage::open(paths, keypair.secret_key)?;
    let mut repo = git::repo::Repo::open(storage, project_urn)?;
    let bro = repo.locked().browser()?;

    Ok((bro, project))
}

/// Returns the [`Blob`] for a file at `revision` under `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or a surf interaction fails.
pub fn blob<'a>(
    browser: &surf::git::Browser<'a>,
    revision: String,
    maybe_path: Option<String>,
) -> Result<Blob, error::Error> {
    let path = maybe_path.clone().unwrap_or_default();

    // Best effort to guess the revision.
    browser.revspec(&revision)?;

    let root = browser.get_directory()?;

    let mut p = surf::file_system::Path::from_str(&path)?;

    let file = root
        .find_file(&p)
        .ok_or_else(|| error::Error::PathNotFound)?;

    let mut commit_path = surf::file_system::Path::root();
    commit_path.append(&mut p);

    let last_commit = browser.last_commit(&commit_path)?.map(|c| Commit::from(&c));
    let (_rest, last) = p.split_last();
    let content = match std::str::from_utf8(&file.contents) {
        Ok(content) => BlobContent::Ascii(content.to_string()),
        Err(_) => BlobContent::Binary,
    };

    Ok(Blob {
        content,
        info: Info {
            name: last.to_string(),
            object_type: ObjectType::Blob,
            last_commit,
        },
        path: maybe_path.unwrap_or(last.to_string()),
    })
}

/// Given a project id to a repo returns the list of branches.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn branches<'a>(browser: &surf::git::Browser<'a>) -> Result<Vec<Branch>, error::Error> {
    let mut branches = browser
        .list_branches(None)?
        .into_iter()
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Given a path to a repo returns the list of branches.
///
/// # Errors
///
/// Will return [`error::Error`] if the repository doesn't exist.
pub fn local_branches(repo_path: &str) -> Result<Vec<Branch>, error::Error> {
    let repo = surf::git::Repository::new(repo_path)?;
    let browser = surf::git::Browser::new(&repo)?;
    let mut branches = browser
        .list_branches(None)?
        .into_iter()
        .map(|b| Branch(b.name.name().to_string()))
        .collect::<Vec<Branch>>();

    branches.sort();

    Ok(branches)
}

/// Retrieves the [`Commit`] for the given `sha1`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commit<'a>(browser: &surf::git::Browser<'a>, sha1: &str) -> Result<Commit, error::Error> {
    browser.commit(surf::git::Oid::from_str(sha1)?)?;

    let history = browser.get();
    let commit = history.first();

    Ok(Commit::from(commit))
}

/// Retrieves the [`Commit`] history for the given `branch`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn commits<'a>(browser: &surf::git::Browser<'a>, branch: &str) -> Result<Vec<Commit>, error::Error> {
    browser.branch(surf::git::BranchName::new(branch))?;

    let commits = browser.get().iter().map(Commit::from).collect();

    Ok(commits)
}

/// Retrieves the list of [`Tag`] for the given project `id`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or the surf interaction fails.
pub fn tags<'a>(browser: &surf::git::Browser<'a>) -> Result<Vec<Tag>, error::Error> {
    let tag_names = browser.list_tags()?;

    let mut tags: Vec<Tag> = tag_names
        .into_iter()
        .map(|tag_name| Tag(tag_name.name().to_string()))
        .collect();

    tags.sort();

    Ok(tags)
}

/// Retrieve the [`Tree`] for the given `revision` and directory `prefix`.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the surf interactions fail.
/// TODO(fintohaps): default branch fall back from Browser
pub fn tree<'a>(
    browser: &surf::git::Browser<'a>,
    revision: String,
    maybe_prefix: Option<String>,
) -> Result<Tree, error::Error> {
    let prefix = maybe_prefix.unwrap_or_default();

    browser.revspec(&revision)?;

    let path = if prefix == "/" || prefix == "" {
        surf::file_system::Path::root()
    } else {
        surf::file_system::Path::from_str(&prefix)?
    };

    let root_dir = browser.get_directory()?;
    let prefix_dir = if path.is_root() {
        root_dir
    } else {
        root_dir
            .find_directory(&path)
            .ok_or_else(|| error::Error::PathNotFound)?
    };
    let mut prefix_contents = prefix_dir.list_directory();
    prefix_contents.sort();

    let entries_results: Result<Vec<TreeEntry>, error::Error> = prefix_contents
        .iter()
        .map(|(label, system_type)| {
            let mut entry_path = if path.is_root() {
                surf::file_system::Path::try_from(vec![label.clone()])
            } else {
                let mut p = path.clone();
                p.push(label.clone());
                p
            };
            let mut commit_path = surf::file_system::Path::root();
            commit_path.append(&mut entry_path);

            let info = Info {
                name: label.to_string(),
                object_type: match system_type {
                    surf::file_system::SystemType::Directory => ObjectType::Tree,
                    surf::file_system::SystemType::File => ObjectType::Blob,
                },
                last_commit: None,
            };

            Ok(TreeEntry {
                info,
                path: entry_path.to_string(),
            })
        })
        .collect();

    let mut entries = entries_results?;

    // We want to ensure that in the response Tree entries come first. `Ord` being derived on
    // the enum ensures Variant declaration order.
    //
    // https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
    entries.sort_by(|a, b| a.info.object_type.cmp(&b.info.object_type));

    let last_commit = if path.is_root() {
        Some(Commit::from(browser.get().first()))
    } else {
        None
    };
    let name = if path.is_root() {
        "".into()
    } else {
        let (_first, last) = path.split_last();
        last.to_string()
    };
    let info = Info {
        name,
        object_type: ObjectType::Tree,
        last_commit,
    };

    Ok(Tree {
        path: prefix,
        entries,
        info,
    })
}

/// Retrieves project metadata.
///
/// # Errors
///
/// Will return [`error::Error`] if the project for the given `id` doesn't exist.
pub async fn get_project_meta(paths: &Paths, urn: &RadUrn, project: impl entity::Resolver<project::Project>) -> Result<project::Project, error::Error> {
    Ok(project.resolve(&urn).await?)
}

/// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
#[must_use]
pub fn list_projects(paths: &Paths) -> Vec<(RadUrn, project::Project)> {
    todo!() // TODO: not implemented by link yet
}

/// Initialize a [`librad::project::Project`] in the location of the given `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the git2 repository is not present for the `path` or any of the
/// librad interactions fail.
pub async fn init_project(
    librad_paths: &Paths,
    path: &str,
    owner: RadUrn,
    user: impl entity::Resolver<User>,
    name: String,
    description: &str,
    default_branch: &str,
) -> Result<(RadUrn, git::repo::Repo), error::Error> {
    // Fetch the owner and build the repo path
    let user = user.resolve(&owner).await?; // TODO: verify
    let path = uri::Path::from_str(path)?;
    let urn = RadUrn::new(user.root_hash().clone(), uri::Protocol::Git, path);

    // Create the project meta
    let mut meta = project::Project::new(name, urn)?.to_builder();
    meta.set_description(description.to_string());
    meta.set_default_branch(default_branch.to_string());
    let meta = meta.build()?;

    // Set up storage
    let key = keys::SecretKey::new();
    let storage = Storage::init(librad_paths, key)?;

    Ok((urn, git::repo::Repo::create(storage, &meta)?))
}

/// Initialize a [`radicle_surf::git::git2::Repository`] at the given path.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the git interactions break.
pub fn init_repo(path: String) -> Result<(), error::Error> {
    let repo = git2::Repository::init(path)?;

    // First use the config to initialize a commit signature for the user.
    let sig = repo.signature()?;
    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // For our purposes, we'll leave the index empty for now.
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

/// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
/// branch.
///
/// # Errors
///
/// Will return [`error::Error`] if any of the git interaction fail, or the initialisation of the
/// coco project.
pub fn replicate_platinum(
    tmp_dir: &tempfile::TempDir,
    librad_paths: &Paths,
    name: &str,
    description: &str,
    default_branch: &str,
) -> Result<(RadUrn, project::Project), error::Error> {
    // Craft the absolute path to git-platinum fixtures.
    let mut platinum_path = env::current_dir()?;
    platinum_path.push("../fixtures/git-platinum");
    let mut platinum_from = String::from("file://");
    platinum_from.push_str(platinum_path.to_str().expect("unable get path"));

    // Construct path for fixtures to clone into.
    let platinum_into = tmp_dir.path().join("git-platinum");

    // Clone a copy into temp directory.
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let platinum_repo = git2::build::RepoBuilder::new()
        .branch("master")
        .clone_local(git2::build::CloneLocal::Auto)
        .fetch_options(fetch_options)
        .clone(&platinum_from, platinum_into.as_path())
        .expect("unable to clone fixtures repo");

    let platinum_surf_repo = surf::git::Repository::new(
        platinum_into
            .to_str()
            .expect("unable to convert into string"),
    )?;
    let platinum_browser = surf::git::Browser::new(&platinum_surf_repo)?;

    let tags = platinum_browser
        .list_tags()
        .expect("unable to get list of tags")
        .iter()
        .map(|t| format!("+refs/tags/{}", t.name()))
        .collect::<Vec<String>>();

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

    // Init as rad project.
    let (id, repo) = init_project(
        librad_paths,
        platinum_into.to_str().expect("unable to get path"),
        name,
        description,
        default_branch,
    )?;
    let mut rad_remote = platinum_repo.find_remote("rad")?;

    // Push all tags to rad remote.
    rad_remote.push(&tags.iter().map(String::as_str).collect::<Vec<_>>(), None)?;
    // Push dev branch.
    rad_remote.push(&["+refs/heads/dev"], None)?;

    Ok((id, repo))
}

/// Creates a small set of projects in [`Paths`].
///
/// # Errors
///
/// Will error if filesystem access is not granted or broken for the configured
/// [`librad::paths::Paths`].
pub fn setup_fixtures(librad_paths: &Paths, root: &str) -> Result<(), error::Error> {
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
        let path = format!("{}/{}/{}", root, "repos", info.0);
        std::fs::create_dir_all(path.clone())?;

        init_repo(path.clone())?;
        init_project(librad_paths, &path, info.0, info.1, info.2)?;
    }

    Ok(())
}
