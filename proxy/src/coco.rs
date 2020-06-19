//! Abstractions and utilities for git interactions through the API.

use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use librad::keys;
use librad::meta::entity::{self, Resolver as _};
use librad::meta::project;
use librad::meta::user;
use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;
use librad::surf;
use librad::surf::vcs::git::git2;
use librad::uri::RadUrn;

use crate::error;

/// Module that captures all types and functions for source code.
mod source;
pub use source::{
    blob, branches, commit, commits, local_branches, local_state, tags, tree, Blob, BlobContent,
    Branch, Commit, Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// `Peer` carries the user that is logged-in as well as the [`net::peer::PeerApi`] so we can
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
    ///     * Initialising [`librad::git::storage::Storage`] fails.
    ///     * Initialising [`net::peer::Peer`] fails.
    pub async fn new<I>(
        config: net::peer::PeerConfig<discovery::Static<I, SocketAddr>>,
    ) -> Result<Self, error::Error>
    where
        I: Iterator<Item = (peer::PeerId, SocketAddr)> + Send + 'static,
    {
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

    /// Returns the list of [`project::Project`] known for the configured [`paths::Paths`].
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

    /// Get a repo browser for a project.
    ///
    /// # Errors
    ///
    /// The function will result in an error if the mutex guard was poisoned. See
    /// [`std::sync::Mutex::lock`] for further details.
    pub async fn with_browser<F, T>(
        &self,
        project_urn: &str,
        callback: F,
    ) -> Result<T, error::Error>
    where
        F: Send + FnOnce(&mut surf::vcs::git::Browser) -> Result<T, error::Error>,
    {
        let project = self.get_project(project_urn).await?;
        let default_branch = project.default_branch();
        let api = self.api.lock().map_err(|_| error::Error::LibradLock)?;
        let repo = api.storage().open_repo(project.urn())?;
        let mut browser = repo.browser(default_branch)?;
        callback(&mut browser)
    }

    /// Get all peer IDs and their branches.
    pub async fn remotes(
        &self,
        project_urn: &str,
    ) -> Result<Vec<(String, Vec<Branch>)>, error::Error> {
        let project = self.get_project(&project_urn).await?;
        let api = self.api.lock().unwrap();
        let peer_id = api.peer_id();
        let storage = api.storage();
        let repo = storage.open_repo(project.urn()).unwrap();
        let refs = repo.rad_refs().unwrap();

        let result = refs
            .remotes
            .flatten()
            .map(|remote| {
                let refs = if remote == &peer_id {
                    let browser = repo.browser(project.default_branch()).unwrap();
                    local_branches(&browser).unwrap()
                } else {
                    let refs = storage.rad_refs_of(&project.urn(), remote.clone()).unwrap();
                    refs.heads.keys().cloned().map(Branch).collect()
                };

                (remote.default_encoding(), refs)
            })
            .collect();

        Ok(result)
    }

    /// Initialize a [`project::Project`] that is owned by the `owner`.
    /// This kicks off the history of the project, tracked by `librad`'s mono-repo.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///     * [`Self::with_api`] fails with a poisoned lock.
    ///     * The signing of the project metadata fails.
    ///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
    pub async fn init_project(
        &mut self,
        owner: &User,
        path: impl AsRef<std::path::Path> + Send,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        // Test if the repo has setup rad remote.
        if let Ok(repo) = git2::Repository::open(&path) {
            if repo.find_remote("rad").is_ok() {
                return Err(error::Error::RadRemoteExists(format!(
                    "{}",
                    path.as_ref().display(),
                )));
            }
        }

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

        self.setup_remote(path, &meta.urn().id, default_branch)?;

        // TODO(finto): mocking
        let mut projects = self.projects.lock().map_err(|_| error::Error::LibradLock)?;
        projects.insert(meta.urn(), meta.clone());

        Ok(meta)
    }

    /// Equips a repository with a rad remote for the given id. If the directory at the given path
    /// is not managed by git yet we initialise it first.
    fn setup_remote(
        &self,
        path: impl AsRef<std::path::Path>,
        id: &librad::hash::Hash,
        default_branch: &str,
    ) -> Result<(), error::Error> {
        // Check if directory at path is a git repo.
        if git2::Repository::open(&path).is_err() {
            let repo = git2::Repository::init(&path)?;
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
            repo.commit(
                Some(&format!("refs/heads/{}", default_branch)),
                &sig,
                &sig,
                "Initial commit",
                &tree,
                &[],
            )?;
        }

        let repo = git2::Repository::open(path)?;

        if let Err(err) = repo.resolve_reference_from_short_name(default_branch) {
            log::error!("error while trying to find default branch: {:?}", err);
            return Err(error::Error::DefaultBranchMissing(
                id.to_string(),
                default_branch.to_string(),
            ));
        }

        let monorepo = self.with_api(|api| api.paths().git_dir().join(""))?;
        let namespace_prefix = format!("refs/namespaces/{}/refs", id);
        let mut remote = repo.remote_with_fetch(
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
        remote.push(
            &[&format!(
                "refs/heads/{}:{}/heads/{}",
                default_branch, namespace_prefix, default_branch
            )],
            None,
        )?;

        Ok(())
    }

    // TODO(xla): Move into coco::source.
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

    // TODO(xla): Move into control module.
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

        Self::clone_platinum(&platinum_from, &platinum_into)?;

        let meta = self
            .init_project(
                owner,
                platinum_into.clone(),
                name,
                description,
                default_branch,
            )
            .await?;

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

    // TODO(xla): Move into control module.
    /// Creates a small set of projects in [`paths::Paths`].
    ///
    /// # Errors
    ///
    /// Will error if filesystem access is not granted or broken for the configured
    /// [`librad::paths::Paths`].
    pub async fn setup_fixtures(&mut self, owner: &User) -> Result<(), error::Error> {
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
            self.replicate_platinum(owner, info.0, info.1, info.2)
                .await?;
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

// TODO(xla): Transform into Peer::create_user.
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
