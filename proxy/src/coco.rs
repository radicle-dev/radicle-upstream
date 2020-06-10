//! Abstractions and utilities for git interactions through the API.

use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

// use librad::git::repo;
use librad::git::storage;
use librad::keys;
use librad::meta::entity::{self, Resolver as _};
use librad::meta::project;
use librad::meta::user;
use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;
// use librad::surf::vcs::git as surf;
use librad::surf::vcs::git::git2;
use librad::uri::RadUrn;

use crate::error;

/// Module that captures all types and functions for source code.
mod source;
pub use source::{
    blob, branches, commit, commits, local_branches, tags, tree, Blob, BlobContent, Branch, Commit,
    Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// `Peer` carries the user that is logged-in as well as the [`peer::PeerApi`] so we can
/// interact with the protocol.
#[derive(Clone)]
pub struct Peer {
    /// The protocol API for shelling out commands.
    pub api: Arc<Mutex<net::peer::PeerApi>>,
    /// Mocking a way to look up and store projects
    pub(crate) projects: Arc<Mutex<HashMap<RadUrn, project::Project<entity::Draft>>>>,
}

#[async_trait]
impl entity::Resolver<project::Project<entity::Draft>> for Peer {
    async fn resolve(
        &self,
        uri: &RadUrn,
    ) -> Result<project::Project<entity::Draft>, entity::Error> {
        let projects = self.projects.lock().expect("failed to acquire lock");
        Ok(projects.get(uri).expect("project was missing").clone())
    }

    async fn resolve_revision(
        &self,
        uri: &RadUrn,
        _revision: u64,
    ) -> Result<project::Project<entity::Draft>, entity::Error> {
        let projects = self.projects.lock().expect("failed to acquire lock");
        Ok(projects.get(uri).expect("project was missing").clone())
    }
}

impl Peer {
    /// We create a default `Peer` using the `config` we provide.
    ///
    /// # Errors
    ///
    /// `new` fails when:
    ///     * Initialising [`storage::Storage`] fails.
    ///     * Initialising [`net::peer::Peer`] fails.
    pub async fn new<I>(
        config: net::peer::PeerConfig<discovery::Static<I, SocketAddr>>,
    ) -> Result<Self, error::Error>
    where
        I: Iterator<Item = (peer::PeerId, SocketAddr)> + Send + 'static,
    {
        // Initialise the storage
        let _ = storage::Storage::init(&config.paths, config.key.clone())?;

        let peer = config.try_into_peer().await?;
        // TODO(finto): discarding the run loop below. Should be used to subsrcibe to events and
        // publish events.
        let (api, _futures) = peer.accept()?;
        Ok(Self {
            api: Arc::new(Mutex::new(api)),
            projects: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Acquire a lock to the [`net::peer::PeerApi`] and apply a function over it.
    ///
    /// # Errors
    ///
    /// The function will result in an error if the mutex guard was poisoned. See
    /// [`std::sync::Mutex::lock`] for further details.
    pub fn with_api<F, T>(&self, f: F) -> Result<T, error::Error>
    where
        F: FnOnce(&net::peer::PeerApi) -> T,
    {
        let api = self.api.lock().map_err(|_| error::Error::LibradLock)?;
        Ok(f(&api))
    }

    /*
    /// Fetch a repository for the `project_urn` we supplied to this function.
    ///
    /// TODO(finto): The call to `browser` is not actually selecting the correct browser yet.
    ///
    /// # Errors
    ///
    /// The function will error if:
    ///   * A lock was poisioned. See [`Self::with_api`].
    ///   * The repository could not be created. See [`surf::Repository::new`].
    pub fn project_repo(&'_ self, project_urn: &str) -> Result<repo::Repo<'_>, error::Error> {
        let project_urn = project_urn.parse()?;
        // TODO(finto): fetch project meta and build browser
        let project_name = "git-platinum";
        let path = self.with_api(|api| api.paths().git_dir().join(project_name))?;
        // TODO(finto): https://github.com/radicle-dev/radicle-surf/issues/126
        let _repo = surf::Repository::new(path.to_str().expect("failed to get path"))?;

        let api = self.api.lock().map_err(|_| error::Error::LibradLock)?;
        let repo = api.storage().open_repo(project_urn)?;

        Ok(repo)
    }
    */

    /// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
    ///
    /// # Errors
    ///
    /// The function will error if:
    ///   * A lock was poisioned. See [`Self::with_api`].
    pub fn list_projects(&self) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
        let projects = self.projects.lock().map_err(|_| error::Error::LibradLock)?;
        Ok(projects.values().cloned().collect())
    }

    /// Get the project found at `project_urn`.
    ///
    /// # Errors
    ///
    /// `get_project` fails if:
    ///     * Parsing the `project_urn` fails.
    ///     * Resolving the project fails.
    pub async fn get_project(
        &self,
        project_urn: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        // TODO(finto): we need the storage to be a resolver
        let urn = project_urn.parse()?;
        let project = self.resolve(&urn).await?;
        Ok(project)
    }

    /// Initialize a [`librad::project::Project`] that is owned by the `owner`.
    /// This kicks off the history of the project, tracked by `librad`'s mono-repo.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///     * [`Self::with_api`] fails with a poisoned lock.
    ///     * The signing of the project metadata fails.
    ///     * The interaction with `librad` [`Storage`] fails.
    pub async fn init_project(
        &mut self,
        owner: &User,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        let meta: Result<project::Project<entity::Draft>, error::Error> = self
            .with_api(|api| {
                let key = api.key();

                // Create the project meta
                let mut meta =
                    project::Project::<entity::Draft>::create(name.to_string(), owner.urn())?
                        .to_builder()
                        .set_description(description.to_string())
                        .set_default_branch(default_branch.to_string())
                        .add_key(key.public())
                        .build()?;
                meta.sign_owned(key)?;

                let storage = api.storage().reopen()?;
                let _repo = storage.create_repo(&meta)?;
                Ok(meta)
            })
            .flatten();

        // Doing ? above breaks inference. Gaaaawwwwwd Rust!
        let meta = meta?;

        // TODO(finto): mocking
        let mut projects = self.projects.lock().map_err(|_| error::Error::LibradLock)?;
        projects.insert(meta.urn(), meta.clone());

        Ok(meta)
    }

    /// This function exists as a standalone because the logic does not play well with async in
    /// `replicate_platinum`.
    fn clone_platinum(
        platinum_from: &str,
        platinum_into: &std::path::PathBuf,
    ) -> Result<git2::Repository, error::Error> {
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

        Ok(platinum_repo)
    }

    /// Create a copy of the git-platinum repo, init with coco and push tags and the additional dev
    /// branch.
    ///
    /// # Errors
    ///
    /// Will return [`error::Error`] if any of the git interaction fail, or the initialisation of
    /// the coco project.
    pub async fn replicate_platinum(
        &mut self,
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
        let monorepo = self.with_api(|api| api.paths().git_dir().join(""))?;
        let workspace = monorepo.join("../workspace");
        let platinum_into = workspace.join(name);

        let repo = Self::clone_platinum(&platinum_from, &platinum_into)?;
        let meta = self
            .init_project(owner, name, description, default_branch)
            .await?;
        let namespace_prefix = format!("refs/namespaces/{}/refs", meta.urn().id);

        let mut rad_remote = repo.remote_with_fetch(
            "rad",
            &format!(
                "file://{}",
                monorepo.to_str().expect("unable to get str for monorepo")
            ),
            &format!("+{}/heads/*:refs/heads/*", namespace_prefix),
        )?;

        repo.remote_add_push(
            "rad",
            &format!("+refs/heads/*:{}/heads/*", namespace_prefix),
        )?;

        let tags = repo
            .tag_names(None)?
            .into_iter()
            .flatten()
            .map(|t| format!("+refs/tags/{}:{}/tags/{}", t, namespace_prefix, t))
            .collect::<Vec<_>>();

        // Push all tags to rad remote.
        rad_remote.push(&tags, None)?;
        // Push branches.
        rad_remote.push(
            &[
                &format!("refs/heads/master:{}/heads/dev", namespace_prefix),
                &format!("refs/heads/master:{}/heads/master", namespace_prefix),
            ],
            None,
        )?;

        // Init as rad project.
        Ok(meta)
    }

    /// Creates a small set of projects in [`Paths`].
    ///
    /// # Errors
    ///
    /// Will error if filesystem access is not granted or broken for the configured
    /// [`librad::paths::Paths`].
    pub async fn setup_fixtures(&mut self, owner: &User, root: &str) -> Result<(), error::Error> {
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

            self.init_project(owner, info.0, info.1, info.2).await?;
        }

        Ok(())
    }
}

/// Acting as a fake resolver where a User resolves to itself.
/// This allows us to check the history status of a single User.
/// TODO(finto): Remove this once Resolvers are complete.
struct FakeUserResolver(user::User<entity::Draft>);

#[async_trait]
impl entity::Resolver<user::User<entity::Draft>> for FakeUserResolver {
    async fn resolve(&self, _uri: &RadUrn) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }

    async fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }
}

/// Constructs a fake user to be used as an owner of projects until we have more permanent key and
/// user management.
pub async fn fake_owner(peer: &Peer) -> User {
    let key = peer
        .with_api(|api| api.key().clone())
        .expect("failed to get key");
    let mut user = user::User::<entity::Draft>::create("cloudhead".into(), key.public())
        .expect("unable to create user");
    user.sign_owned(&key).expect("unable to sign user");
    let fake_resolver = FakeUserResolver(user.clone());
    user.check_history_status(&fake_resolver, &fake_resolver)
        .await
        .expect("failed to verify user")
}

/// Basic [`net::peer::PeerConfig`] type for a vector of [`peer::PeerId`]s.
type PeerConfig = net::peer::PeerConfig<
    discovery::Static<std::vec::IntoIter<(peer::PeerId, SocketAddr)>, SocketAddr>,
>;

/// Provide the default config.
///
/// Address: 127.0.0.1:0
/// No seeds.
/// Default gossip parameters.
///
/// # Errors
///
/// Results in an error if the [`paths::Paths`] could not be created.
pub fn default_config(
    key: keys::SecretKey,
    path: impl AsRef<std::path::Path>,
) -> Result<PeerConfig, error::Error> {
    let gossip_params = net::gossip::MembershipParams::default();
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    // TODO(finto): could we initialise with known seeds from a cache?
    let seeds: Vec<(peer::PeerId, SocketAddr)> = vec![];
    let disco = discovery::Static::new(seeds);
    let paths = paths::Paths::from_root(path)?;
    Ok(net::peer::PeerConfig {
        key,
        paths,
        listen_addr,
        gossip_params,
        disco,
    })
}
