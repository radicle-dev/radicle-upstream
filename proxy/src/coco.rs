//! Abstractions and utilities for git interactions through the API.

use async_trait::async_trait;
use std::env;
use std::str::FromStr;

use librad::git;
use librad::keys;
use librad::uri::{self, RadUrn};
use librad::meta::entity;
use librad::meta::user;
use librad::meta::project;
use librad::paths::Paths;
use librad::surf;
use librad::surf::git::git2;
use librad::git::storage::Storage;
use radicle_keystore::{Keystore, SecretKeyExt, Keypair};

use crate::error;

mod types;
pub use types::*; // TODO: make explicit

/// The set of capabilities necessary for interacting with `radicle-link`.
pub trait Client: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + entity::Resolver<project::Project> + entity::Resolver<user::User> + HasPaths + Me + Send + Sync {}

/// Fetching and setting [`Paths`] of some data structure.
pub trait HasPaths {
    /// Get the [`Paths`].
    fn get_paths(&self) -> &Paths;
    /// Set the [`Paths`].
    fn set_paths(&mut self, paths: Paths);
}

/// Get a reference to the [`user::User`] that is logged in.
pub trait Me {
    /// Who am I?
    fn me(&self) -> &user::User;
}

/// The set of data and capabilities that are needed for interacting with `radicle-link`.
/// It implements [`Client`], which is a collection of these trait capabilities. `Client` should be
/// used by functions lower down the stack, while `Coco` should be passed down from the top.
pub struct Coco<
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project>,
    U: entity::Resolver<user::User>,
> {
    /// The `librad` paths.
    pub paths: Paths,
    /// Storage for where to retrieve your keys from.
    pub keystore: K,
    /// The project resolver.
    pub project: P,
    /// The user resolver.
    pub user: U,
    /// The user that is logged in for Upstream.
    pub me: user::User,
}

impl<K, P, U> Me for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    fn me(&self) -> &user::User {
        &self.me
    }
}

impl<K, P, U> HasPaths for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    fn get_paths(&self) -> &Paths {
        &self.paths
    }

    fn set_paths(&mut self, paths: Paths) {
        self.paths = paths;
    }
}

impl<K, P, U> Keystore for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    type PublicKey = K::PublicKey;
    type SecretKey = K::SecretKey;
    type Metadata = K::Metadata;
    type Error = K::Error;

    fn put_key(&mut self, key: Self::SecretKey) -> Result<(), Self::Error> {
        self.keystore.put_key(key)
    }

    fn get_key(
        &self
    ) -> Result<Keypair<Self::PublicKey, Self::SecretKey>, Self::Error> {
        self.keystore.get_key()
    }

    fn show_key(&self) -> Result<(Self::PublicKey, Self::Metadata), Self::Error> {
        self.keystore.show_key()
    }
}

#[async_trait]
impl<K, P, U> entity::Resolver<project::Project> for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    /// Resolve the given URN and deserialize the target `Entity`
    async fn resolve(&self, uri: &RadUrn) -> Result<project::Project, entity::Error> {
        self.project.resolve(uri).await
    }

    async fn resolve_revision(&self, uri: &RadUrn, revision: u64) -> Result<project::Project, entity::Error> {
        self.project.resolve_revision(uri, revision).await
    }
}

#[async_trait]
impl<K, P, U> entity::Resolver<user::User> for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    /// Resolve the given URN and deserialize the target `Entity`
    async fn resolve(&self, uri: &RadUrn) -> Result<user::User, entity::Error> {
        self.user.resolve(uri).await
    }

    async fn resolve_revision(&self, uri: &RadUrn, revision: u64) -> Result<user::User, entity::Error> {
        self.user.resolve_revision(uri, revision).await
    }
}

impl<K, P, U> Client for Coco<K, P, U>
where
    K: Keystore<PublicKey = keys::PublicKey, SecretKey = keys::SecretKey, Metadata = <keys::SecretKey as SecretKeyExt>::Metadata, Error = error::Error> + Send + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{}

/// Get the [`git::repo::Repo`] for the given `project_urn`.
pub async fn get_repo<'a, C>(
    coco: &C,
    project_urn: String,
) -> Result<(git::repo::Repo, project::Project), error::Error>
where
    C: Client,
{
    let paths = coco.get_paths();
    let project_urn = RadUrn::from_str(&project_urn)?;
    let keypair = coco.get_key()?;
    let project: project::Project = coco.resolve(&project_urn).await?;

    let storage = Storage::open(paths, keypair.secret_key)?;
    let repo = git::repo::Repo::open(storage, project_urn)?;

    Ok((repo, project))
}

/// Returns the [`Blob`] for a file at `revision` under `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the project doesn't exist or a surf interaction fails.
pub fn blob<'a>(
    browser: &mut surf::git::Browser<'a>,
    revision: String,
    maybe_path: Option<String>,
) -> Result<Blob, error::Error> {
    let path = maybe_path.clone().unwrap_or_default();

    // Best effort to guess the revision.
    browser.revspec(&revision)?;

    let root = browser.get_directory()?;

    let p = surf::file_system::Path::from_str(&path)?;

    let file = root
        .find_file(p.clone())
        .ok_or_else(|| error::Error::PathNotFound)?;

    let mut commit_path = surf::file_system::Path::root();
    commit_path.append(p.clone());

    let last_commit = browser.last_commit(commit_path)?.map(|c| Commit::from(&c));
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
pub fn commit<'a>(browser: &mut surf::git::Browser<'a>, sha1: &str) -> Result<Commit, error::Error> {
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
pub fn commits<'a>(browser: &mut surf::git::Browser<'a>, branch: &str) -> Result<Vec<Commit>, error::Error> {
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
    browser: &mut surf::git::Browser<'a>,
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
            .find_directory(path.clone())
            .ok_or_else(|| error::Error::PathNotFound)?
    };
    let mut prefix_contents = prefix_dir.list_directory();
    prefix_contents.sort();

    let entries_results: Result<Vec<TreeEntry>, error::Error> = prefix_contents
        .iter()
        .map(|(label, system_type)| {
            let entry_path = if path.is_root() {
                surf::file_system::Path::new(label.clone())
            } else {
                let mut p = path.clone();
                p.push(label.clone());
                p
            };
            let mut commit_path = surf::file_system::Path::root();
            commit_path.append(entry_path.clone());

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

/// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
#[must_use]
pub fn list_projects(_paths: &Paths) -> Vec<(RadUrn, project::Project)> {
    todo!() // TODO(fintohaps): not implemented by link yet
}

/// Initialize a [`librad::project::Project`] in the location of the given `path`.
///
/// # Errors
///
/// Will return [`error::Error`] if the git2 repository is not present for the `path` or any of the
/// librad interactions fail.
pub fn init_project<C>(
    coco: &mut C,
    path: &str,
    name: &str,
    description: &str,
    default_branch: &str,
) -> Result<(RadUrn, project::Project), error::Error>
where
    C: Client,
{
    let paths = coco.get_paths();
    let me = coco.me();

    // Set up storage
    let key = coco.get_key()?.secret_key;
    let storage = Storage::init(paths, key)?;

    // Fetch the owner and build the repo path
    let path = uri::Path::from_str(path)?;
    let urn = RadUrn::new(me.root_hash().clone(), uri::Protocol::Git, path);

    // Create the project meta
    let meta = project::Project::new(name.to_string(), urn.clone())?
        .to_builder()
        .set_description(description.to_string())
        .set_default_branch(default_branch.to_string());
    let meta = meta.build()?;

    let _repo = git::repo::Repo::create(storage, &meta)?;

    Ok((urn, meta))
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
pub fn replicate_platinum<C>(
    coco: &mut C,
    tmp_dir: &tempfile::TempDir,
    name: &str,
    description: &str,
    default_branch: &str,
) -> Result<(RadUrn, project::Project), error::Error>
where
    C: Client,
{
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
        coco,
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
pub async fn setup_fixtures<C>(coco: &mut C, root: &str) -> Result<(), error::Error>
where
    C: Client,
{
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
        init_project(coco, &path, info.0, info.1, info.2)?;
    }

    Ok(())
}
