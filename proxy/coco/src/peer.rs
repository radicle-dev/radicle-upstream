//! Utility to work with the peer api of librad.

use std::convert::TryFrom;
use std::net::SocketAddr;
use std::path::{self, PathBuf};
use std::sync::{Arc, Mutex};

use futures::stream::StreamExt;

use librad::git::local::{transport, url::LocalUrl};
use librad::git::{repo, storage};
use librad::keys;
use librad::meta::entity;
use librad::meta::project as librad_project;
use librad::meta::user;
use librad::net::peer::{PeerApi, PeerConfig};
use librad::net::{self, discovery};
use librad::paths;
use librad::peer::PeerId;
use librad::signer::SomeSigner;
use librad::uri::{RadUrl, RadUrn};
use radicle_surf::vcs::git;

use crate::config;
use crate::project;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Accept(#[from] net::peer::AcceptError),

    #[error(transparent)]
    Bootstrap(#[from] net::peer::BootstrapError),

    #[error(transparent)]
    Config(#[from] config::Error),

    /// Returned when an attempt to create an identity was made and there is one present.
    #[error("the identity '{0}' already exits")]
    EntityExists(RadUrn),

    #[error(transparent)]
    Meta(#[from] entity::Error),

    #[error(transparent)]
    ProjectCreate(#[from] project::create::Error),

    #[error(transparent)]
    Repo(#[from] repo::Error),

    #[error(transparent)]
    Storage(#[from] storage::Error),

    #[error(transparent)]
    SurfGit(#[from] git::error::Error),

    #[error(transparent)]
    Verification(#[from] entity::HistoryVerificationError),
}

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// High-level interface to the coco monorepo and gossip layer.
#[derive(Clone)]
pub struct Api {
    /// Thread-safe wrapper around [`PeerApi`].
    peer_api: Arc<Mutex<PeerApi<keys::SecretKey>>>,
}

impl Api {
    /// Create a new `PeerApi` given a `PeerConfig`.
    ///
    /// # Errors
    ///
    /// If turning the config into a `Peer` fails
    /// If trying to accept on the socket fails
    pub async fn new<I>(
        config: PeerConfig<discovery::Static<I, SocketAddr>, keys::SecretKey>,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = (PeerId, SocketAddr)> + Send + 'static,
    {
        let paths = config.paths.clone();
        let signer = config.signer.clone();

        let peer = config.try_into_peer().await?;
        // TODO(finto): discarding the run loop below. Should be used to subsrcibe to events and
        // publish events.
        let (api, run_loop) = peer.accept()?;

        let protocol = api.protocol();
        let protocol_subscriber = protocol.subscribe().await;
        let protocol_notifications = protocol_subscriber.for_each(|notification| {
            log::info!("protocol.notification = {:?}", notification);

            futures::future::ready(())
        });
        tokio::spawn(protocol_notifications);

        let subscriber = api.subscribe();
        let api_notifications = subscriber.await.for_each(|notification| {
            log::info!("peer.event = {:?}", notification);

            futures::future::ready(())
        });
        tokio::spawn(api_notifications);

        tokio::spawn(async move {
            run_loop.await;
        });

        // Register the rad:// transport protocol
        transport::register(transport::Settings {
            paths,
            signer: SomeSigner { signer }.into(),
        });

        Ok(Self {
            peer_api: Arc::new(Mutex::new(api)),
        })
    }

    /// Returns the [`PathBuf`] to the underlying monorepo.
    #[must_use]
    pub fn monorepo(&self) -> PathBuf {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        api.paths().git_dir().join("")
    }

    /// Returns the underlying [`paths::Paths`].
    #[must_use]
    pub fn paths(&self) -> paths::Paths {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        api.paths().clone()
    }

    /// Convenience method to trigger a reopen of the storage.
    ///
    /// # Errors
    ///
    /// When the underlying lock acquisition fails or opening the storage.
    pub fn reopen(&self) -> Result<(), Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        api.storage().reopen()?;

        Ok(())
    }

    /// Our current peers [`PeerId`].
    #[must_use]
    pub fn peer_id(&self) -> PeerId {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        api.peer_id().clone()
    }

    /// The address this peer is listening on.
    #[must_use]
    pub fn listen_addr(&self) -> SocketAddr {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        api.listen_addr()
    }

    /// Get the default owner for this `PeerApi`.
    #[must_use]
    pub fn default_owner(&self) -> Option<user::User<entity::Draft>> {
        let api = self.peer_api.lock().expect("unable to acquire lock");

        match api.storage().default_rad_self() {
            Ok(user) => Some(user),
            Err(err) => {
                log::warn!("an error occurred while trying to get 'rad/self': {}", err);
                None
            }
        }
    }

    /// Set the default owner for this `PeerApi`.
    ///
    /// # Errors
    ///
    ///   * Fails to set the default `rad/self` for this `PeerApi`.
    pub fn set_default_owner(&self, user: User) -> Result<(), Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        Ok(api.storage().set_default_rad_self(user)?)
    }

    /// Initialise a [`User`] and make them the default owner of this `PeerApi`.
    ///
    /// # Errors
    ///
    ///   * Fails to initialise `User`.
    ///   * Fails to verify `User`.
    ///   * Fails to set the default `rad/self` for this `PeerApi`.
    pub fn init_owner(&self, key: &keys::SecretKey, handle: &str) -> Result<User, Error> {
        let user = self.init_user(key, handle)?;
        let user = verify_user(user)?;

        self.set_default_owner(user.clone())?;

        Ok(user)
    }

    /// Returns the list of [`project::Project`]s for your peer.
    ///
    /// # Errors
    ///
    ///   * Retrieving the project entities from the store fails.
    #[allow(
        clippy::match_wildcard_for_single_variants,
        clippy::wildcard_enum_match_arm
    )]
    pub fn list_projects(&self) -> Result<Vec<librad_project::Project<entity::Draft>>, Error> {
        let project_meta = {
            let api = self.peer_api.lock().expect("unable to acquire lock");
            let storage = api.storage().reopen()?;
            let owner = storage.default_rad_self()?;

            let meta = storage.all_metadata()?;
            meta.flat_map(|entity| {
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
            .collect::<Vec<_>>()
        };

        Ok(project_meta)
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
    pub fn list_users(&self) -> Result<Vec<user::User<entity::Draft>>, Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage();

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

        Ok(entities)
    }

    /// Get the project found at `urn`.
    ///
    /// # Errors
    ///
    ///   * Resolving the project fails.
    pub fn get_project<P>(
        &self,
        urn: &RadUrn,
        peer: P,
    ) -> Result<librad_project::Project<entity::Draft>, Error>
    where
        P: Into<Option<PeerId>>,
    {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;

        Ok(storage.metadata_of(urn, peer)?)
    }

    /// Given some hints as to where you might find it, get the urn of the project found at `url`.
    ///
    /// **N.B.** This needs to be run with `tokio::spawn_blocking`.
    ///
    /// # Errors
    ///   * Could not successfully acquire a lock to the API.
    ///   * Could not open librad storage.
    ///   * Failed to clone the project.
    ///   * Failed to set the rad/self of this project.
    pub fn clone_project<Addrs>(&self, url: RadUrl, addr_hints: Addrs) -> Result<RadUrn, Error>
    where
        Addrs: IntoIterator<Item = SocketAddr>,
    {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;
        let repo = storage.clone_repo::<librad_project::ProjectInfo, _>(url, addr_hints)?;
        repo.set_rad_self(storage::RadSelfSpec::Default)?;
        Ok(repo.urn)
    }

    /// Given some hints as to where you might find it, get the urn of the user found at `url`.
    ///
    /// **N.B.** This needs to be run with `tokio::spawn_blocking`.
    ///
    /// # Errors
    ///
    ///   * Could not successfully acquire a lock to the API.
    ///   * Could not open librad storage.
    ///   * Failed to clone the user.
    pub fn clone_user<Addrs>(&self, url: RadUrl, addr_hints: Addrs) -> Result<RadUrn, Error>
    where
        Addrs: IntoIterator<Item = SocketAddr>,
    {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;
        let repo = storage.clone_repo::<user::UserInfo, _>(url, addr_hints)?;
        Ok(repo.urn)
    }

    /// Get the user found at `urn`.
    ///
    /// # Errors
    ///
    ///   * Resolving the user fails.
    ///   * Could not successfully acquire a lock to the API.
    pub fn get_user(&self, urn: &RadUrn) -> Result<user::User<entity::Draft>, Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;

        Ok(storage.metadata(urn)?)
    }

    /// Get a repo browser for a project.
    ///
    /// # Errors
    ///
    /// The function will result in an error if the mutex guard was poisoned. See
    /// [`std::sync::Mutex::lock`] for further details.
    pub fn with_browser<F, T>(&self, urn: &RadUrn, callback: F) -> Result<T, Error>
    where
        F: Send + FnOnce(&mut git::Browser) -> Result<T, Error>,
    {
        let git_dir = self.monorepo();

        let project = self.get_project(urn, None)?;
        let default_branch = git::Branch::local(project.default_branch());
        let repo = git::Repository::new(git_dir)?;
        let namespace = git::Namespace::try_from(project.urn().id.to_string().as_str())?;
        let mut browser = git::Browser::new_with_namespace(&repo, &namespace, default_branch)?;

        callback(&mut browser)
    }

    /// Initialize a [`project::Project`] that is owned by the `owner`.
    /// This kicks off the history of the project, tracked by `librad`'s mono-repo.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///     * The signing of the project metadata fails.
    ///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
    pub fn init_project<P: AsRef<path::Path> + Send>(
        &self,
        key: &keys::SecretKey,
        owner: &User,
        project: &project::Create<P>,
    ) -> Result<librad_project::Project<entity::Draft>, Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;

        let mut meta = project.build(owner, key.public())?;
        meta.sign_owned(key)?;

        let urn = meta.urn();
        if storage.has_urn(&urn)? {
            return Err(Error::EntityExists(urn));
        }

        let repo = storage.create_repo(&meta)?;
        repo.set_rad_self(librad::git::storage::RadSelfSpec::Urn(owner.urn()))?;
        log::debug!("Created project with Urn '{}'", urn);

        let repo = project.setup_repo(LocalUrl::from_urn(urn, api.peer_id().clone()))?;
        log::debug!("Setup repository at path '{}'", repo.path().display());

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
    pub fn init_user(
        &self,
        key: &keys::SecretKey,
        handle: &str,
    ) -> Result<user::User<entity::Draft>, Error> {
        // Create the project meta
        let mut user = user::User::<entity::Draft>::create(handle.to_string(), key.public())?;
        user.sign_owned(key)?;
        let urn = user.urn();

        // Initialising user in the storage.
        {
            let api = self.peer_api.lock().expect("unable to acquire lock");
            let storage = api.storage().reopen()?;

            if storage.has_urn(&urn)? {
                return Err(Error::EntityExists(urn));
            } else {
                let _repo = storage.create_repo(&user)?;
            }
        }

        Ok(user)
    }

    /// Wrapper around the storage track.
    ///
    /// # Errors
    ///
    /// * When the storage operation fails.
    pub fn track(&self, urn: &RadUrn, remote: &PeerId) -> Result<(), Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        Ok(api.storage().track(urn, remote)?)
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
    pub fn tracked(&self, urn: &RadUrn) -> Result<Vec<(PeerId, user::User<entity::Draft>)>, Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;
        let repo = storage.open_repo(urn.clone())?;
        repo.tracked()?
            .map(move |peer_id| {
                repo.get_rad_self_of(peer_id.clone())
                    .map(|user| (peer_id.clone(), user))
                    .map_err(Error::from)
            })
            .collect()
    }
}

/// Verify a user using a fake resolver that resolves the user to itself.
///
/// TODO(finto): Should not live here permanently, because resolvers should solve this verification.
///
/// # Errors
///
/// If any of the verification steps fail
pub fn verify_user(user: user::User<entity::Draft>) -> Result<User, Error> {
    let fake_resolver = FakeUserResolver(user.clone());
    let verified_user = user.check_history_status(&fake_resolver, &fake_resolver)?;
    Ok(verified_user)
}

/// Acting as a fake resolver where a User resolves to itself.
/// This allows us to check the history status of a single User.
/// TODO(finto): Remove this once Resolvers are complete.
struct FakeUserResolver(user::User<entity::Draft>);

impl entity::Resolver<user::User<entity::Draft>> for FakeUserResolver {
    fn resolve(&self, _uri: &RadUrn) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }

    fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod test {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    use librad::keys::SecretKey;

    use crate::config;
    use crate::control;
    use crate::project;

    use super::{Api, Error};

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

    fn shia_le_pathbuf(path: PathBuf) -> project::Create<PathBuf> {
        project::Create {
            repo: project::Repo::New {
                path,
                name: "just".to_string(),
            },
            description: "do".to_string(),
            default_branch: "it".to_string(),
        }
    }

    #[tokio::test]
    async fn can_create_user() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let annie = api.init_user(&key, "annie_are_you_ok?");
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        env::set_var("RAD_HOME", tmp_dir.path());
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(&key, "cloudhead")?;
        let project = api.init_project(&key, &user, &radicle_project(repo_path.clone()));

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
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(&key, "cloudhead")?;
        let project = api.init_project(&key, &user, &radicle_project(repo_path.clone()));

        assert!(project.is_ok());
        assert!(repo_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn cannot_create_user_twice() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(&key, "cloudhead")?;
        let err = api.init_user(&key, "cloudhead");

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
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(&key, "cloudhead")?;
        let project_creation = radicle_project(repo_path.clone());
        let project = api.init_project(&key, &user, &project_creation)?;

        let err = api.init_project(&key, &user, &project_creation.into_existing());

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
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(&key, "cloudhead")?;

        control::setup_fixtures(&api, &key, &user).expect("unable to setup fixtures");

        let kalt = api.init_user(&key, "kalt")?;
        let kalt = super::verify_user(kalt)?;
        let fakie = api.init_project(&key, &kalt, &fakie_project(repo_path))?;

        let projects = api.list_projects()?;
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
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let cloudhead = api.init_user(&key, "cloudhead")?;
        let _cloudhead = super::verify_user(cloudhead)?;
        let kalt = api.init_user(&key, "kalt")?;
        let _kalt = super::verify_user(kalt)?;

        let users = api.list_users()?;
        let mut user_handles = users
            .into_iter()
            .map(|user| user.name().to_string())
            .collect::<Vec<_>>();
        user_handles.sort();

        assert_eq!(user_handles, vec!["cloudhead", "kalt"],);

        Ok(())
    }

    #[tokio::test]
    async fn can_clone_project() -> Result<(), Error> {
        let alice_key = SecretKey::new();

        let alice_tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let alice_repo_path = alice_tmp_dir.path().join("radicle");
        let config = config::default(alice_key.clone(), alice_tmp_dir.path())?;
        let alice_peer = Api::new(config).await?;

        let alice = alice_peer.init_owner(&alice_key, "alice")?;
        let project =
            alice_peer.init_project(&alice_key, &alice, &shia_le_pathbuf(alice_repo_path))?;

        let bob_key = SecretKey::new();

        let bob_tmp_dir = tempfile::tempdir().expect("failed to create tempdir");

        let bob_config = config::default(bob_key.clone(), bob_tmp_dir.path())?;
        let bob_peer = Api::new(bob_config).await?;
        let _bob = bob_peer.init_owner(&bob_key, "bob")?;

        let bobby = bob_peer.clone();
        let project_urn = tokio::task::spawn_blocking(move || {
            bobby.clone_project(
                project.urn().into_rad_url(alice_peer.peer_id()),
                vec![alice_peer.listen_addr()].into_iter(),
            )
        })
        .await
        .expect("failed to join thread")?;

        assert_eq!(
            bob_peer
                .list_projects()?
                .into_iter()
                .map(|project| project.urn())
                .collect::<Vec<_>>(),
            vec![project_urn]
        );

        Ok(())
    }

    #[tokio::test]
    async fn can_clone_user() -> Result<(), Error> {
        let alice_key = SecretKey::new();
        let bob_key = SecretKey::new();

        let alice_tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let config = config::default(alice_key.clone(), alice_tmp_dir.path())?;
        let alice_peer = Api::new(config).await?;

        let bob_tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let config = config::default(bob_key.clone(), bob_tmp_dir.path())?;
        let bob_peer = Api::new(config).await?;

        let alice = alice_peer.init_user(&alice_key, "alice")?;
        let bobby = bob_peer.clone();
        let user_urn = tokio::task::spawn_blocking(move || {
            bobby.clone_user(
                alice.urn().into_rad_url(alice_peer.peer_id()),
                vec![alice_peer.listen_addr()].into_iter(),
            )
        })
        .await
        .expect("failed to join thread")?;

        assert_eq!(
            bob_peer
                .list_users()?
                .into_iter()
                .map(|user| user.urn())
                .collect::<Vec<_>>(),
            vec![user_urn]
        );

        Ok(())
    }

    #[tokio::test]
    async fn create_with_existing_remote_with_reset() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let kalt = api.init_owner(&key, "kalt")?;

        let fakie = api.init_project(&key, &kalt, &fakie_project(repo_path.clone()))?;

        assert!(repo_path.join(fakie.name()).exists());

        // Simulate resetting the monorepo
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        // Create fakie project from the existing directory above.
        let kalt = api.init_owner(&key, "kalt")?;
        let fakie = api.init_project(&key, &kalt, &fakie_project(repo_path).into_existing())?;

        // Attempt to initialise a browser to ensure we can look at branches in the project
        let _stats = api.with_browser(&fakie.urn(), |browser| Ok(browser.get_stats()?))?;

        Ok(())
    }

    #[tokio::test]
    async fn create_with_existing_remote() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create tempdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let kalt = api.init_owner(&key, "kalt")?;

        let fakie = api.init_project(&key, &kalt, &fakie_project(repo_path.clone()))?;

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
        let _fake_fakie = api.init_project(&key, &kalt, &fake_fakie)?;

        Ok(())
    }
}
