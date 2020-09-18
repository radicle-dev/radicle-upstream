//! Utility to work with the peer api of librad.

use std::{
    convert::TryFrom,
    future::Future,
    net::{IpAddr, SocketAddr},
    path::{self, PathBuf},
    sync::Arc,
    time::Duration,
};

use tokio::sync::Mutex;

use librad::{
    git::{
        local::{transport, url::LocalUrl},
        refs::Refs,
        storage,
        repo,
    },
    keys,
    meta::{entity, project as librad_project, user},
    net::{gossip::PeerInfo, peer::PeerApi},
    paths,
    peer::PeerId,
    uri::{RadUrl, RadUrn},
};
use radicle_keystore::sign::Signer as _;
use radicle_surf::vcs::{git, git::git2};

use crate::{
    error::Error,
    project,
    seed::Seed,
    signer,
    user::{verify as verify_user, User},
};

/// Thread-safe wrapper for [`State`].
pub type Lock = Arc<Mutex<State>>;

impl From<State> for Lock {
    fn from(state: State) -> Self {
        Arc::new(Mutex::new(state))
    }
}

/// High-level interface to the coco monorepo and gossip layer.
pub struct State {
    /// Internal handle on [`PeerApi`].
    pub(crate) api: PeerApi<keys::SecretKey>,
}

impl State {
    /// Create a new [`State`] given a [`PeerApi`].
    #[must_use]
    pub fn new(api: PeerApi<keys::SecretKey>, signer: signer::BoxedSigner) -> Self {
        let paths = api.paths();

        // Register the transport so to use git2 to execute actions such as checkouts, fetch, and
        // push. The transport will then handle the interaction with the monorepo.
        transport::register(transport::Settings {
            paths: paths.clone(),
            signer,
        });

        Self { api }
    }

    /// Returns the [`PathBuf`] to the underlying monorepo.
    #[must_use]
    pub fn monorepo(&self) -> PathBuf {
        self.api.paths().git_dir().join("")
    }

    /// Returns the underlying [`paths::Paths`].
    #[must_use]
    pub fn paths(&self) -> paths::Paths {
        self.api.paths().clone()
    }

    /// Check the storage to see if we have the given commit for project at `urn`.
    ///
    /// # Errors
    ///
    ///   * Checking the storage for the commit fails.
    ///
    /// # Panics
    ///
    ///   * Unable to acquire the lock.
    pub async fn has_commit(&self, urn: RadUrn, oid: impl Into<git2::Oid>) -> Result<bool, Error> {
        let oid = oid.into();
        Ok(self.api.with_storage(move |storage| storage.has_commit(&urn, oid)).await??)
    }

    /// The local machine's [`PeerId`].
    #[must_use]
    pub fn peer_id(&self) -> PeerId {
        self.api.peer_id().clone()
    }

    /// The [`SocketAddr`] this [`PeerApi`] is listening on.
    #[must_use]
    pub fn listen_addr(&self) -> SocketAddr {
        self.api.listen_addr()
    }

    /// Get the default owner for this `PeerApi`.
    #[must_use]
    pub async fn default_owner(&self) -> Option<user::User<entity::Draft>> {
        self.api.with_storage(move |storage|
            storage
            .default_rad_self()
            .map_err(|err| log::warn!("an error occurred while trying to get 'rad/self': {}", err))
            .ok()
        ).await.ok().flatten()
    }

    /// Set the default owner for this `PeerApi`.
    ///
    /// # Errors
    ///
    ///   * Fails to set the default `rad/self` for this `PeerApi`.
    pub async fn set_default_owner(&self, user: User) -> Result<(), Error> {
        self.api.with_storage(move |storage|
            storage.set_default_rad_self(user)
            .map_err(Error::from)
        ).await?
    }

    /// Initialise a [`User`] and make them the default owner of this [`PeerApi`].
    ///
    /// # Errors
    ///
    ///   * Fails to initialise `User`.
    ///   * Fails to verify `User`.
    ///   * Fails to set the default `rad/self` for this `PeerApi`.
    pub async fn init_owner(&self, signer: &signer::BoxedSigner, handle: &str) -> Result<User, Error> {
        let user = self.init_user(signer, handle).await?;
        let user = verify_user(user)?;

        self.set_default_owner(user.clone()).await?;

        Ok(user)
    }

    /// Given some hints as to where you might find it, get the urn of the project found at `url`.
    ///
    /// # Errors
    ///   * Could not successfully acquire a lock to the API.
    ///   * Could not open librad storage.
    ///   * Failed to clone the project.
    ///   * Failed to set the rad/self of this project.
    pub async fn clone_project<Addrs>(&self, url: RadUrl, addr_hints: Addrs) -> Result<RadUrn, Error>
    where
        Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
    {
        let urn = self.api.with_storage(move |storage| {
            let repo = storage.clone_repo::<librad_project::ProjectInfo, _>(url, addr_hints)?;
            repo.set_rad_self(storage::RadSelfSpec::Default)?;
            Ok::<_, repo::Error>(repo.urn)
        }).await??;

        Ok(urn)
    }

    /// Get the project found at `urn`.
    ///
    /// # Errors
    ///
    ///   * Resolving the project fails.
    pub async fn get_project<P>(
        &self,
        urn: RadUrn,
        peer: P,
    ) -> Result<librad_project::Project<entity::Draft>, Error>
    where
        P: Into<Option<PeerId>> + Send + 'static,
    {
        Ok(self.api.with_storage(move |storage| storage.metadata_of(&urn, peer)).await??)
    }

    /// Returns the list of [`librad_project::Project`]s for the local peer.
    ///
    /// # Errors
    ///
    ///   * Retrieving the project entities from the store fails.
    #[allow(
        clippy::match_wildcard_for_single_variants,
        clippy::wildcard_enum_match_arm
    )]
    pub async fn list_projects(&self) -> Result<Vec<librad_project::Project<entity::Draft>>, Error> {
        let project_meta = self.api.with_storage(move |storage| {
            let owner = storage.default_rad_self()?;

            let meta = storage.all_metadata()?
            .flat_map(|entity| {
                let entity = entity.ok()?;
                let rad_self = storage.get_rad_self(&entity.urn()).ok()?;

                // We only list projects that are owned by the peer
                if rad_self.urn() != owner.urn() {
                    return None;
                }

                entity.try_map(|info| match info {
                    entity::data::EntityInfo::Project(info) => Some(info),
                    _ => None,
                })
            })
            .collect::<Vec<_>>();

            Ok::<_, storage::Error>(meta)
        }).await??;

        Ok(project_meta)
    }

    /// Retrieves the [`librad::git::refs::Refs`] for the state owner.
    ///
    /// # Errors
    ///
    /// * if opening the storage fails
    pub async fn list_owner_project_refs(&self, urn: RadUrn) -> Result<Refs, Error> {
        Ok(self.api.with_storage(move |storage| storage.rad_signed_refs(&urn)).await??)
    }

    /// Retrieves the [`librad::git::refs::Refs`] for the given project urn.
    ///
    /// # Errors
    ///
    /// * if opening the storage fails
    pub async fn list_peer_project_refs(&self, urn: RadUrn, peer_id: PeerId) -> Result<Refs, Error> {
        Ok(self.api.with_storage(move |storage| storage .rad_signed_refs_of(&urn, peer_id)).await??)
    }

    /// Returns the list of [`user::User`]s known for your peer.
    ///
    /// # Errors
    ///
    ///   * Retrieval of the user entities from the store fails.
    #[allow(
        clippy::match_wildcard_for_single_variants,
        clippy::wildcard_enum_match_arm
    )]
    pub async fn list_users(&self) -> Result<Vec<user::User<entity::Draft>>, Error> {
        let entities = self.api.with_storage(move |storage| {
            let mut entities = vec![];
            for entity in storage.all_metadata()? {
                let entity = entity?;

                if let Some(e) = entity.try_map(|info| match info {
                    entity::data::EntityInfo::User(info) => Some(info),
                    _ => None,
                }) {
                    entities.push(e);
                }
            }

            Ok::<_, storage::Error>(entities)
        }).await??;

        Ok(entities)
    }

    /// Given some hints as to where you might find it, get the urn of the user found at `url`.
    ///
    /// # Errors
    ///
    ///   * Could not successfully acquire a lock to the API.
    ///   * Could not open librad storage.
    ///   * Failed to clone the user.
    pub async fn clone_user<Addrs>(&self, url: RadUrl, addr_hints: Addrs) -> Result<RadUrn, Error>
    where
        Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
    {
        Ok(self.api.with_storage(move |storage| storage.clone_repo::<user::UserInfo, _>(url, addr_hints).map(|repo| repo.urn)).await??)
    }

    /// Get the user found at `urn`.
    ///
    /// # Errors
    ///
    ///   * Resolving the user fails.
    ///   * Could not successfully acquire a lock to the API.
    pub async fn get_user(&self, urn: RadUrn) -> Result<user::User<entity::Draft>, Error> {
        Ok(self.api.with_storage(move |storage| storage.metadata(&urn)).await??)
    }

    /// Fetch any updates at the given `RadUrl`, providing address hints if we have them.
    ///
    /// # Errors
    ///
    ///   * Could not successfully acquire a lock to the API.
    ///   * Could not open librad storage.
    ///   * Failed to fetch the updates.
    ///   * Failed to set the rad/self of this project.
    pub async fn fetch<Addrs>(&self, url: RadUrl, addr_hints: Addrs) -> Result<(), Error>
    where
        Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
    {
        Ok(self.api.with_storage(move |storage| storage.fetch_repo(url, addr_hints)).await??)
    }

    /// Provide a a repo [`git::Browser`] for the project of `urn`.
    ///
    /// # Errors
    ///
    /// * If no project for the `urn` was found.
    /// * If the [`git::Browser`] fails.
    /// * If the passed `callback` errors.
    pub async fn with_browser<F, T>(&self, urn: RadUrn, callback: F) -> Result<T, Error>
    where
        F: Send + FnOnce(&mut git::Browser) -> Result<T, Error>,
    {
        let monorepo = self.monorepo();
        let project = self.get_project(urn, None).await?;
        let default_branch = git::Branch::local(project.default_branch());
        let repo = git::Repository::new(monorepo)?;
        let namespace = git::Namespace::try_from(project.urn().id.to_string().as_str())?;
        let mut browser = git::Browser::new_with_namespace(&repo, &namespace, default_branch)?;

        callback(&mut browser)
    }

    /// Initialize a [`librad_project::Project`] that is owned by the `owner`.
    /// This kicks off the history of the project, tracked by `librad`'s mono-repo.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///     * The signing of the project metadata fails.
    ///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
    pub async fn init_project<P: AsRef<path::Path> + Send + Sync + 'static>(
        &self,
        signer: &signer::BoxedSigner,
        owner: &User,
        project: project::Create<P>,
    ) -> Result<librad_project::Project<entity::Draft>, Error> {
        let mut meta = project.build(owner, signer.public_key().into())?;
        meta.sign_owned(signer)?;

        let owner_urn = owner.urn();
        let project_urn = meta.urn();
        let local_peer_id = self.api.peer_id().clone();

        let meta = self.api.with_storage(move |storage| {
            if storage.has_urn(&project_urn)? {
                return Err(Error::EntityExists(project_urn));
            }

            let repo = storage.create_repo(&meta)?;
            repo.set_rad_self(librad::git::storage::RadSelfSpec::Urn(owner_urn))?;
            log::debug!("Created project with Urn '{}'", project_urn);

            let repo = project.setup_repo(LocalUrl::from_urn(project_urn, local_peer_id))?;
            log::debug!("Setup repository at path '{}'", repo.path().display());

            Ok(meta)
        }).await??;

        Ok(meta)
    }

    /// Create a [`user::User`] with the provided `handle`. This assumes that you are creating a
    /// user that uses the secret key the `PeerApi` was configured with.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///     * The signing of the user metadata fails.
    ///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
    pub async fn init_user(
        &self,
        signer: &signer::BoxedSigner,
        handle: &str,
    ) -> Result<user::User<entity::Draft>, Error> {
        let mut user =
            user::User::<entity::Draft>::create(handle.to_string(), signer.public_key().into())?;
        user.sign_owned(signer)?;
        let urn = user.urn();

        Ok(self.api.with_storage(move |storage| {
            if storage.has_urn(&urn)? {
                return Err(Error::EntityExists(urn));
            } else {
                let _ = storage.create_repo(&user)?;
                Ok(user)
            }
        }).await??)
    }

    /// Query the network for providers of the given [`RadUrn`] within a given `timeout`.
    pub fn providers(
        &self,
        urn: RadUrn,
        timeout: Duration,
    ) -> impl Future<Output = impl futures::Stream<Item = PeerInfo<IpAddr>>> {
        self.api.providers(urn, timeout)
    }

    /// Wrapper around the storage track.
    ///
    /// # Errors
    ///
    /// * When the storage operation fails.
    pub async fn track(&self, urn: RadUrn, remote: PeerId) -> Result<(), Error> {
        Ok(self.api.with_storage(move |storage| storage.track(&urn, &remote)).await??)
    }

    /// Get the [`user::User`]s that are tracking this project, including their [`PeerId`].
    ///
    /// # Errors
    ///
    /// * If we could not acquire the lock
    /// * If we could not open the storage
    /// * If did not have the `urn` in storage
    /// * If we could not fetch the tracked peers
    /// * If we could not get the `rad/self` of the peer
    pub async fn tracked(&self, urn: RadUrn) -> Result<Vec<(PeerId, user::User<entity::Draft>)>, Error> {
        Ok(self.api.with_storage(move |storage| {
            let repo = storage.open_repo(urn)?;
            repo.tracked()?
                .map(move |peer_id| {
                    repo.get_rad_self_of(peer_id.clone())
                        .map(|user| (peer_id.clone(), user))
                })
                .collect::<Result<Vec<_>, _>>()
        }).await??)
    }
}

impl From<&State> for Seed {
    fn from(state: &State) -> Self {
        Self {
            peer_id: state.peer_id(),
            addr: state.listen_addr(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod test {
    use std::{env, path::PathBuf, process::Command};

    use librad::keys::SecretKey;

    use crate::{config, control, project, signer};

    use super::{Error, State};

    fn fakie_project(path: PathBuf) -> project::Create<PathBuf> {
        project::Create {
            repo: project::Repo::New {
                path,
                name: "fakie-nose-kickflip-backside-180-to-handplant".to_string(),
            },
            description: "rad git tricks".to_string(),
            default_branch: "dope".to_string(),
        }
    }

    fn radicle_project(path: PathBuf) -> project::Create<PathBuf> {
        project::Create {
            repo: project::Repo::New {
                path,
                name: "radicalise".to_string(),
            },
            description: "the people".to_string(),
            default_branch: "power".to_string(),
        }
    }

    #[tokio::test]
    async fn can_create_user() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let annie = state.init_user(&signer, "annie_are_you_ok?");
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        env::set_var("RAD_HOME", tmp_dir.path());
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let user = state.init_owner(&signer, "cloudhead")?;
        let project = state.init_project(&signer, &user, &radicle_project(repo_path.clone()));

        assert!(project.is_ok());
        assert!(repo_path.join("radicalise").exists());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project_directory_exists() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let repo_path = repo_path.join("radicalise");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let user = state.init_owner(&signer, "cloudhead")?;
        let project = state.init_project(&signer, &user, &radicle_project(repo_path.clone()));

        assert!(project.is_ok());
        assert!(repo_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn cannot_create_user_twice() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let user = state.init_owner(&signer, "cloudhead")?;
        let err = state.init_user(&signer, "cloudhead");

        if let Err(Error::EntityExists(urn)) = err {
            assert_eq!(urn, user.urn())
        } else {
            panic!(
                "unexpected error when creating the user a second time: {:?}",
                err
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn cannot_create_project_twice() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let user = state.init_owner(&signer, "cloudhead")?;
        let project_creation = radicle_project(repo_path.clone());
        let project = state.init_project(&signer, &user, &project_creation)?;

        let err = state.init_project(&signer, &user, &project_creation.into_existing());

        if let Err(Error::EntityExists(urn)) = err {
            assert_eq!(urn, project.urn())
        } else {
            panic!(
                "unexpected error when creating the project a second time: {:?}",
                err
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn list_projects() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");

        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let user = state.init_owner(&signer, "cloudhead")?;

        control::setup_fixtures(&state, &signer, &user).expect("unable to setup fixtures");

        let kalt = state.init_user(&signer, "kalt")?;
        let kalt = super::verify_user(kalt)?;
        let fakie = state.init_project(&signer, &kalt, &fakie_project(repo_path))?;

        let projects = state.list_projects()?;
        let mut project_names = projects
            .into_iter()
            .map(|project| project.name().to_string())
            .collect::<Vec<_>>();
        project_names.sort();

        assert_eq!(
            project_names,
            vec!["Monadic", "monokel", "open source coin", "radicle"]
        );

        assert!(!project_names.contains(&fakie.name().to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn list_users() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let cloudhead = state.init_user(&signer, "cloudhead")?;
        let _cloudhead = super::verify_user(cloudhead)?;
        let kalt = state.init_user(&signer, "kalt")?;
        let _kalt = super::verify_user(kalt)?;

        let users = state.list_users()?;
        let mut user_handles = users
            .into_iter()
            .map(|user| user.name().to_string())
            .collect::<Vec<_>>();
        user_handles.sort();

        assert_eq!(user_handles, vec!["cloudhead", "kalt"],);

        Ok(())
    }

    #[tokio::test]
    async fn create_with_existing_remote_with_reset() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let kalt = state.init_owner(&signer, "kalt")?;

        let fakie = state.init_project(&signer, &kalt, &fakie_project(repo_path.clone()))?;

        assert!(repo_path.join(fakie.name()).exists());

        // Simulate resetting the monorepo
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        // Create fakie project from the existing directory above.
        let kalt = state.init_owner(&signer, "kalt")?;
        let fakie =
            state.init_project(&signer, &kalt, &fakie_project(repo_path).into_existing())?;

        // Attempt to initialise a browser to ensure we can look at branches in the project
        let _stats = state.with_browser(&fakie.urn(), |browser| Ok(browser.get_stats()?))?;

        Ok(())
    }

    #[tokio::test]
    async fn create_with_existing_remote() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(key, tmp_dir.path())?;
        let (api, _run_loop) = config.try_into_peer().await?.accept()?;
        let state = State::new(api, signer.clone());

        let kalt = state.init_owner(&signer, "kalt")?;
        let fakie = state.init_project(&signer, &kalt, &fakie_project(repo_path.clone()))?;
        let fake_fakie = repo_path.join("fake-fakie");
        let copy = Command::new("cp")
            .arg("-rf")
            .arg(repo_path.join(fakie.name()))
            .arg(fake_fakie.clone())
            .status()
            .expect("failed to copy directory");

        assert!(copy.success());

        let fake_fakie = project::Create {
            repo: project::Repo::Existing { path: fake_fakie },
            description: "".to_string(),
            default_branch: fakie.default_branch().to_owned(),
        };
        let _fake_fakie = state.init_project(&signer, &kalt, &fake_fakie)?;

        Ok(())
    }
}
