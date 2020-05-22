//! Abstractions and utilities for git interactions through the API.

use async_trait::async_trait;
use std::env;
use std::str::FromStr;

use librad::git;
use librad::git::storage::Storage;
use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use librad::meta::user;
use librad::paths::Paths;
use librad::surf;
use librad::surf::git::git2;
use librad::uri::{self, RadUrn};
use radicle_keystore::{Keypair, Keystore, SecretKeyExt};

use crate::error;

mod source;
pub use source::{
    blob, branches, commit, commits, local_branches, tags, tree, Blob, BlobContent, Branch, Commit,
    Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

/// The set of capabilities necessary for interacting with `radicle-link`.
#[async_trait]
pub trait Client:
    Keystore<
        PublicKey = keys::PublicKey,
        SecretKey = keys::SecretKey,
        Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
        Error = error::Error,
    > + entity::Resolver<project::Project>
    + entity::Resolver<user::User>
    + Send
    + Sync
{
    /// Get the [`git::repo::Repo`] for the given `project_urn`.
    async fn get_repo(
        &self,
        project_urn: String,
    ) -> Result<(git::repo::Repo, project::Project), error::Error>;

    /// Initialize a [`librad::project::Project`] in the location of the given `path`.
    ///
    /// # Errors
    ///
    /// Will return [`error::Error`] if the git2 repository is not present for the `path` or any of
    /// the librad interactions fail.
    fn init_project(
        &self,
        owner: &user::User,
        path: &str,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<(RadUrn, project::Project), error::Error>;

    /// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
    #[must_use]
    fn list_projects(&self) -> Vec<(RadUrn, project::Project)>;

    /// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
    /// branch.
    ///
    /// # Errors
    ///
    /// Will return [`error::Error`] if any of the git interaction fail, or the initialisation of
    /// the coco project.
    fn replicate_platinum(
        &self,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<(RadUrn, project::Project), error::Error>;
}

/// The set of data and capabilities that are needed for interacting with `radicle-link`.
/// It implements [`Client`], which is a collection of these trait capabilities. `Client` should be
/// used by functions up the stack, while `Coco` should be passed down from the top.
pub struct Coco<
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
    P: entity::Resolver<project::Project>,
    U: entity::Resolver<user::User>,
> {
    /// The `librad` paths.
    paths: Paths,
    /// Storage for where to retrieve your keys from.
    keystore: K,
    /// The project resolver.
    project_resolver: P,
    /// The user resolver.
    user_resolver: U,
}

impl Coco<NoopKeystore, NoopProjectResolver, NoopUserResolver> {
    /// Constructs a [`Coco`] with state being backed by a temporary directory.
    pub fn tmp(tmp_path: &std::path::Path) -> Result<Self, error::Error> {
        let paths = Paths::from_root(tmp_path)?;

        Ok(Self {
            paths,
            keystore: NoopKeystore {},
            project_resolver: NoopProjectResolver {},
            user_resolver: NoopUserResolver {},
        })
    }
}

impl<K, P, U> Keystore for Coco<K, P, U>
where
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
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

    fn get_key(&self) -> Result<Keypair<Self::PublicKey, Self::SecretKey>, Self::Error> {
        self.keystore.get_key()
    }

    fn show_key(&self) -> Result<(Self::PublicKey, Self::Metadata), Self::Error> {
        self.keystore.show_key()
    }
}

#[async_trait]
impl<K, P, U> entity::Resolver<project::Project> for Coco<K, P, U>
where
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    /// Resolve the given URN and deserialize the target `Entity`
    async fn resolve(&self, uri: &RadUrn) -> Result<project::Project, entity::Error> {
        self.project_resolver.resolve(uri).await
    }

    async fn resolve_revision(
        &self,
        uri: &RadUrn,
        revision: u64,
    ) -> Result<project::Project, entity::Error> {
        self.project_resolver.resolve_revision(uri, revision).await
    }
}

#[async_trait]
impl<K, P, U> entity::Resolver<user::User> for Coco<K, P, U>
where
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    /// Resolve the given URN and deserialize the target `Entity`
    async fn resolve(&self, uri: &RadUrn) -> Result<user::User, entity::Error> {
        self.user_resolver.resolve(uri).await
    }

    async fn resolve_revision(
        &self,
        uri: &RadUrn,
        revision: u64,
    ) -> Result<user::User, entity::Error> {
        self.user_resolver.resolve_revision(uri, revision).await
    }
}

#[async_trait]
impl<K, P, U> Client for Coco<K, P, U>
where
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    async fn get_repo(
        &self,
        project_urn: String,
    ) -> Result<(git::repo::Repo, project::Project), error::Error> {
        let project_urn = RadUrn::from_str(&project_urn)?;
        let keypair = self.get_key()?;
        let project: project::Project = self.project_resolver.resolve(&project_urn).await?;

        let storage = Storage::open(&self.paths, keypair.secret_key)?;
        let repo = git::repo::Repo::open(storage, project_urn)?;

        Ok((repo, project))
    }

    fn list_projects(&self) -> Vec<(RadUrn, project::Project)> {
        todo!() // TODO(fintohaps): not implemented by link yet
    }

    fn init_project(
        &self,
        owner: &user::User,
        path: &str,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<(RadUrn, project::Project), error::Error> {
        // Set up storage
        let key = self.get_key()?.secret_key;
        let storage = Storage::init(&self.paths, key)?;

        // Fetch the owner and build the repo path
        let path = uri::Path::from_str(path)?;
        let urn = RadUrn::new(owner.root_hash().clone(), uri::Protocol::Git, path);

        // Create the project meta
        let meta = project::Project::new(name.to_string(), urn.clone())?
            .to_builder()
            .set_description(description.to_string())
            .set_default_branch(default_branch.to_string());
        let meta = meta.build()?;

        let _repo = git::repo::Repo::create(storage, &meta)?;

        Ok((urn, meta))
    }

    fn replicate_platinum(
        &self,
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
        let platinum_into = self.paths.git_dir().join("../git-platinum");

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
        let (id, repo) = self.init_project(
            // TODO(xla): Construct or expect a proper user.
            &fake_owner(),
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
}

impl<K, P, U> Coco<K, P, U>
where
    K: Keystore<
            PublicKey = keys::PublicKey,
            SecretKey = keys::SecretKey,
            Metadata = <keys::SecretKey as SecretKeyExt>::Metadata,
            Error = error::Error,
        > + Send
        + Sync,
    P: entity::Resolver<project::Project> + Send + Sync,
    U: entity::Resolver<user::User> + Send + Sync,
{
    /// Creates a small set of projects in [`Paths`].
    ///
    /// # Errors
    ///
    /// Will error if filesystem access is not granted or broken for the configured
    /// [`librad::paths::Paths`].
    pub fn setup_fixtures(&self, root: &str) -> Result<(), error::Error> {
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

        // TODO(xla): Do we need a proper owner here?
        let owner = fake_owner();

        for info in infos {
            let path = format!("{}/{}/{}", root, "repos", info.0);
            std::fs::create_dir_all(path.clone())?;

            source::init_repo(path.clone())?;
            self.init_project(&owner, &path, info.0, info.1, info.2)?;
        }

        Ok(())
    }
}

/// Intermediate type until we have a proper implementation.
pub struct NoopKeystore {}

impl Keystore for NoopKeystore {
    type PublicKey = keys::PublicKey;
    type SecretKey = keys::SecretKey;
    type Metadata = <keys::SecretKey as SecretKeyExt>::Metadata;
    type Error = error::Error;

    fn get_key(&self) -> Result<Keypair<Self::PublicKey, Self::SecretKey>, Self::Error> {
        todo!()
    }

    fn put_key(&mut self, _key: Self::SecretKey) -> Result<(), Self::Error> {
        todo!()
    }

    fn show_key(&self) -> Result<(Self::PublicKey, Self::Metadata), Self::Error> {
        todo!()
    }
}

/// Intermediate type until we have a proper implementation.
pub struct NoopProjectResolver {}

#[async_trait]
impl entity::Resolver<project::Project> for NoopProjectResolver {
    async fn resolve(&self, _uri: &RadUrn) -> Result<project::Project, entity::Error> {
        todo!()
    }

    async fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<project::Project, entity::Error> {
        todo!()
    }
}

/// Intermediate type until we have a proper implementation.
pub struct NoopUserResolver {}

#[async_trait]
impl entity::Resolver<user::User> for NoopUserResolver {
    async fn resolve(&self, _uri: &RadUrn) -> Result<user::User, entity::Error> {
        todo!()
    }

    async fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<user::User, entity::Error> {
        todo!()
    }
}

/// Constructs a fake user to be used as an owner of projects until we have more permanent key and
/// user management.
pub fn fake_owner() -> user::User {
    let p = keys::SecretKey::new().public();
    user::User::new("cloudhead".into(), p).expect("unable to create user")
}
