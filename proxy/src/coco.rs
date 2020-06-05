//! Abstractions and utilities for git interactions through the API.

use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use librad::git::storage;
use librad::keys;
use librad::meta::entity::{self, Resolver as _};
use librad::meta::project;
use librad::meta::user;
use librad::net;
use librad::net::discovery;
use librad::paths;
use librad::peer;
use librad::surf::vcs::git as surf;
use librad::surf::vcs::git::git2;
use librad::uri::RadUrn;

use crate::error;

/// Module that captures all types and functions for source code.
mod source;
pub use source::{
    blob, branches, commit, commits, init_repo, local_branches, tags, tree, Blob, BlobContent,
    Branch, Commit, Info, ObjectType, Person, Tag, Tree, TreeEntry,
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
        let projects = self.projects.lock().unwrap();
        Ok(projects.get(uri).expect("project was missing").clone())
    }

    async fn resolve_revision(
        &self,
        uri: &RadUrn,
        _revision: u64,
    ) -> Result<project::Project<entity::Draft>, entity::Error> {
        let projects = self.projects.lock().unwrap();
        Ok(projects.get(uri).expect("project was missing").clone())
    }
}

impl Peer {
    /// We create a default `Peer` using the `tmp_dir_path` we provide.
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
    pub fn with_api<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&net::peer::PeerApi) -> T,
    {
        let api = self.api.lock().unwrap();
        f(&api)
    }

    /// Fetch a browser for the `project_urn` we supplied to this function.
    ///
    /// TODO(finto): The call to `browser` is not actually selecting the correct browser yet.
    pub fn project_repo(&self, _project_urn: &str) -> Result<surf::Repository, error::Error> {
        // TODO(finto): fetch project meta and build browser
        let project_name = "git-platinum";
        let path = self.with_api(|api| api.paths().git_dir().join(project_name));
        // TODO(finto): https://github.com/radicle-dev/radicle-surf/issues/126
        let repo = surf::Repository::new(path.to_str().unwrap())?;

        Ok(repo)
    }

    /// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
    pub fn list_projects(&self) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
        let projects = self.projects.lock().unwrap();
        Ok(projects.values().cloned().collect())
    }

    /// Get the project found at `project_urn`.
    pub async fn get_project(
        &self,
        project_urn: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        // TODO(finto): we need the storage to be a resolver
        let urn = project_urn.parse()?;
        let project = self.resolve(&urn).await?;
        Ok(project)
    }

    /// Initialize a [`librad::project::Project`] in the location of the given `path`.
    ///
    /// # Errors
    ///
    /// Will return [`error::Error`] if the git2 repository is not present for the `path` or any of
    /// the librad interactions fail.
    pub async fn init_project(
        &mut self,
        owner: &User, // TODO(finto): verify and testify
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        let meta: Result<project::Project<entity::Draft>, error::Error> = self.with_api(|api| {
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
        });

        // Doing ? above breaks inference. Gaaaawwwwwd Rust!
        let meta = meta?;

        // TODO(finto): mocking
        let mut projects = self.projects.lock().unwrap();
        projects.insert(meta.urn(), meta.clone());

        Ok(meta)
    }

    /// This function exists as a standalone because the logic does not play well with async in
    /// `replicate_platinum`.
    fn clone_platinum(
        platinum_from: &str,
        platinum_into: &std::path::PathBuf,
    ) -> Result<(), error::Error> {
        // Clone a copy into temp directory.
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
        let platinum_into = self.with_api(|api| api.paths().git_dir().join("git-platinum"));

        Self::clone_platinum(&platinum_from, &platinum_into)?;

        // Init as rad project.
        Ok(self
            .init_project(owner, name, description, default_branch)
            .await?)
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

            init_repo(path.clone())?;
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
    let key = peer.with_api(|api| api.key().clone());
    let mut user = user::User::<entity::Draft>::create("cloudhead".into(), key.public().clone())
        .expect("unable to create user");
    user.sign_owned(&key).expect("unable to sign user");
    let fake_resolver = FakeUserResolver(user.clone());
    user.check_history_status(&fake_resolver, &fake_resolver)
        .await
        .expect("failed to verify user")
}

/// Provide the default config.
///
/// Address: 127.0.0.1:0
/// No seeds.
/// Default gossip parameters.
pub fn default_config(
    key: keys::SecretKey,
    path: impl AsRef<std::path::Path>,
) -> Result<
    net::peer::PeerConfig<
        discovery::Static<std::vec::IntoIter<(peer::PeerId, SocketAddr)>, SocketAddr>,
    >,
    error::Error,
> {
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
