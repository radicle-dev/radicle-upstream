//! Abstractions and utilities for git interactions through the API.

use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path;

use async_trait::async_trait;

use librad::git::storage;
pub use librad::keys;
use librad::meta::entity::{self, Resolver as _};
use librad::meta::project;
use librad::meta::user;
pub use librad::net;
pub use librad::paths;
pub use librad::peer;
use librad::surf::vcs::git as surf;
use librad::surf::vcs::git::git2;
use librad::uri::RadUrn;

use crate::error;

mod source;
pub use source::{
    blob, branches, commit, commits, init_repo, local_branches, tags, tree, Blob, BlobContent,
    Branch, Commit, Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

/// `UserPeer` carries the user that is logged-in as well as the [`peer::PeerApi`] so we can
/// interact with the protocol.
pub struct UserPeer {
    /// Me, myself, and I.
    pub me: user::User<entity::Draft>, // TODO(finto): this should be verified. Unpublic
    /// The protocol API for shelling out commands.
    pub api: net::peer::PeerApi,
    /// The paths used to configure this Peer.
    pub paths: paths::Paths, // TODO(finto): Unpublify
    /// Mocking a way to look up and store projects
    projects: HashMap<RadUrn, project::Project<entity::Draft>>,
}

#[async_trait]
impl entity::Resolver<project::Project<entity::Draft>> for UserPeer {
    async fn resolve(
        &self,
        uri: &RadUrn,
    ) -> Result<project::Project<entity::Draft>, entity::Error> {
        Ok(self.projects.get(uri).expect("project was missing").clone())
    }

    async fn resolve_revision(
        &self,
        uri: &RadUrn,
        _revision: u64,
    ) -> Result<project::Project<entity::Draft>, entity::Error> {
        Ok(self.projects.get(uri).expect("project was missing").clone())
    }
}

// TODO(finto): Peer is not Sync, so we need to figure out how we share it.
unsafe impl Sync for UserPeer {}

impl UserPeer {
    /// We create a default `UserPeer` using the `tmp_dir_path` we provide.
    pub async fn tmp(tmp_dir_path: impl AsRef<path::Path>) -> Result<Self, error::Error> {
        let key = keys::SecretKey::new();
        let me = fake_owner(key.clone());
        let paths = paths::Paths::from_root(tmp_dir_path)?;
        let gossip_params = Default::default();
        let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
        // TODO(finto): could we initialise with known seeds from a cache?
        let seeds: Vec<(peer::PeerId, SocketAddr)> = vec![];
        let disco = net::discovery::Static::new(seeds);

        // Initialise the storage
        let storage = storage::Storage::init(&paths, key.clone())?;
        let _ = storage.create_repo(&me)?;

        let peer_config = net::peer::PeerConfig {
            key,
            paths: paths.clone(),
            listen_addr,
            gossip_params,
            disco,
        };

        let peer = peer_config.try_into_peer().await?;
        // TODO(finto): discarding the run loop below. Should be used to subsrcibe to events and
        // publish events.
        let (api, _futures) = peer.accept()?;
        Ok(Self {
            me,
            api,
            paths, // TODO(finto): See https://github.com/radicle-dev/radicle-link/issues/157
            projects: HashMap::new(),
        })
    }

    /// Fetch a browser for the `project_urn` we supplied to this function.
    ///
    /// TODO(finto): The call to `browser` is not actually selecting the correct browser yet.
    pub fn project_repo(&self, _project_urn: String) -> Result<surf::Repository, error::Error> {
        // TODO(finto): fetch project meta and build browser
        let project_name = "git-platinum";
        let path = self.paths.git_dir().join(project_name);
        // TODO(finto): https://github.com/radicle-dev/radicle-surf/issues/126
        let repo = surf::Repository::new(path.to_str().unwrap())?;

        Ok(repo)
    }

    /// Returns the list of [`librad::project::Project`] known for the configured [`Paths`].
    #[must_use]
    pub fn list_projects(
        &self,
    ) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
        Ok(self.projects.values().cloned().collect())
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
    pub async fn init_project<'a>(
        &mut self,
        name: &str,
        description: &str,
        default_branch: &str,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        let key = self.api.key();

        // Create the project meta
        let mut meta = project::Project::<entity::Draft>::create(name.to_string(), self.me.urn())?
            .to_builder()
            .set_description(description.to_string())
            .set_default_branch(default_branch.to_string())
            .add_key(key.public())
            .build()?;
        meta.sign_owned(&key)?;

        let storage = self.api.storage().reopen()?;
        let _repo = storage.create_repo(&meta)?;

        // TODO(finto): mocking
        self.projects.insert(meta.urn().clone(), meta.clone());

        Ok(meta)
    }

    // This function exists as a standalone because the logic does not play well with async in
    // `replicate_platinum`.
    fn clone_platinum(
        platinum_from: String,
        platinum_into: std::path::PathBuf,
    ) -> Result<(), error::Error> {
        // Clone a copy into temp directory.
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.download_tags(git2::AutotagOption::All);

        let platinum_repo = git2::build::RepoBuilder::new()
            .branch("master")
            .clone_local(git2::build::CloneLocal::Auto)
            .fetch_options(fetch_options)
            .clone(&platinum_from, platinum_into.as_path())
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
        let platinum_into = self.paths.git_dir().join("git-platinum");

        Self::clone_platinum(platinum_from, platinum_into)?;

        // Init as rad project.
        Ok(self.init_project(name, description, default_branch).await?)
    }

    /// Creates a small set of projects in [`Paths`].
    ///
    /// # Errors
    ///
    /// Will error if filesystem access is not granted or broken for the configured
    /// [`librad::paths::Paths`].
    pub async fn setup_fixtures(&mut self, root: &str) -> Result<(), error::Error> {
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
            self.init_project(info.0, info.1, info.2).await?;
        }

        Ok(())
    }
}

/// Constructs a fake user to be used as an owner of projects until we have more permanent key and
/// user management.
pub fn fake_owner(key: keys::SecretKey) -> user::User<entity::Draft> {
    let mut user = user::User::<entity::Draft>::create("cloudhead".into(), key.public().clone())
        .expect("unable to create user");
    user.sign_owned(&key).expect("unable to sign user");
    user
}
