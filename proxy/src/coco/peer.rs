//! Utility to work with the peer api of librad.

use std::convert::TryFrom;
use std::net::SocketAddr;
use std::path::{self, PathBuf};
use std::sync::{Arc, Mutex};

use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use librad::meta::user;
use librad::net::discovery;
pub use librad::net::peer::{PeerApi, PeerConfig};
use librad::paths;
use librad::peer::PeerId;
use librad::signer;
use librad::uri::RadUrn;
use radicle_surf::vcs::git;

use crate::coco;
use crate::error;

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// Blanket trait to use as our generic [`signer::Signer`].
pub trait Signer: keys::AsPKCS8 + signer::Signer + Clone {}

impl<T: keys::AsPKCS8 + signer::Signer + Clone> Signer for T {}

/// High-level interface to the coco monorepo and gossip layer.
pub struct Api<S>
where
    S: Signer,
    S::Error: keys::SignError,
{
    /// Thread-safe wrapper around [`PeerApi`].
    peer_api: Arc<Mutex<PeerApi<S>>>,
}

impl<S> Api<S>
where
    S: Signer,
    S::Error: keys::SignError,
{
    /// Create a new `PeerApi` given a `PeerConfig`.
    ///
    /// # Errors
    ///
    /// If turning the config into a `Peer` fails
    /// If trying to accept on the socket fails
    pub async fn new<I>(
        config: PeerConfig<discovery::Static<I, SocketAddr>, S>,
    ) -> Result<Self, error::Error>
    where
        I: Iterator<Item = (PeerId, SocketAddr)> + Send + 'static,
    {
        let peer = config.try_into_peer().await?;
        // TODO(finto): discarding the run loop below. Should be used to subsrcibe to events and
        // publish events.
        let (api, _futures) = peer.accept()?;

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
    pub fn reopen(&self) -> Result<(), error::Error> {
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
    pub fn set_default_owner(&self, user: User) -> Result<(), error::Error> {
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
    pub fn init_owner(&self, signer: S, handle: &str) -> Result<User, error::Error> {
        let user = self.init_user(signer, handle)?;
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
    pub fn list_projects(&self) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
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
    pub fn list_users(&self) -> Result<Vec<user::User<entity::Draft>>, error::Error> {
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
    ) -> Result<project::Project<entity::Draft>, error::Error>
    where
        P: Into<Option<PeerId>>,
    {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;

        Ok(storage.metadata_of(urn, peer)?)
    }

    /// Get the user found at `urn`.
    ///
    /// # Errors
    ///
    ///   * Resolving the user fails.
    ///   * Could not successfully acquire a lock to the API.
    pub fn get_user(&self, urn: &RadUrn) -> Result<user::User<entity::Draft>, error::Error> {
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
    pub fn with_browser<F, T>(&self, urn: &RadUrn, callback: F) -> Result<T, error::Error>
    where
        F: Send + FnOnce(&mut git::Browser) -> Result<T, error::Error>,
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
        signer: S,
        owner: &User,
        project: &coco::project::Create<P>,
    ) -> Result<project::Project<entity::Draft>, error::Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;

        let mut meta = project.build(owner, signer.clone())?;
        meta.sign_owned(&signer)?;

        let urn = meta.urn();
        if storage.has_urn(&urn)? {
            return Err(error::Error::EntityExists(urn));
        } else {
            let repo = storage.create_repo(&meta)?;
            repo.set_rad_self(librad::git::storage::RadSelfSpec::Urn(owner.urn()))?;
        }

        let _ = project.setup_repo(api.paths().git_dir(), &urn)?;

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
        signer: S,
        handle: &str,
    ) -> Result<user::User<entity::Draft>, error::Error> {
        // Create the project meta
        let mut user =
            user::User::<entity::Draft>::create(handle.to_string(), signer.public_key().into())?;
        user.sign_owned(&signer)?;
        let urn = user.urn();

        // Initialising user in the storage.
        {
            let api = self.peer_api.lock().expect("unable to acquire lock");
            let storage = api.storage().reopen()?;

            if storage.has_urn(&urn)? {
                return Err(error::Error::EntityExists(urn));
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
    pub fn track(&self, urn: &RadUrn, remote: &PeerId) -> Result<(), error::Error> {
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
    pub fn tracked(
        &self,
        urn: &RadUrn,
    ) -> Result<Vec<(PeerId, user::User<entity::Draft>)>, error::Error> {
        let api = self.peer_api.lock().expect("unable to acquire lock");
        let storage = api.storage().reopen()?;
        let repo = storage.open_repo(urn.clone())?;
        repo.tracked()?
            .map(move |peer_id| {
                repo.get_rad_self_of(peer_id.clone())
                    .map(|user| (peer_id.clone(), user))
                    .map_err(error::Error::from)
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
pub fn verify_user(user: user::User<entity::Draft>) -> Result<User, error::Error> {
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
    use std::path::PathBuf;

    use librad::keys::SecretKey;

    use crate::coco::config;
    use crate::coco::control;
    use crate::coco::project;
    use crate::error::Error;

    use super::Api;

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
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let annie = api.init_user(key, "annie_are_you_ok?");
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let api = Api::new(config).await?;

        let user = api.init_owner(key.clone(), "cloudhead")?;
        let project = api.init_project(key, &user, &radicle_project(repo_path.clone()));

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

        let user = api.init_owner(key.clone(), "cloudhead")?;
        let project = api.init_project(key.clone(), &user, &radicle_project(repo_path.clone()));

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

        let user = api.init_owner(key.clone(), "cloudhead")?;
        let err = api.init_user(key, "cloudhead");

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

        let user = api.init_owner(key.clone(), "cloudhead")?;
        let project_creation = radicle_project(repo_path.clone());
        let project = api.init_project(key.clone(), &user, &project_creation)?;

        let err = api.init_project(key, &user, &project_creation.into_existing());

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

        let user = api.init_owner(key.clone(), "cloudhead")?;

        control::setup_fixtures(&api, key.clone(), &user)?;

        let kalt = api.init_user(key.clone(), "kalt")?;
        let kalt = super::verify_user(kalt)?;
        let fakie = api.init_project(key, &kalt, &fakie_project(repo_path))?;

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

        let cloudhead = api.init_user(key.clone(), "cloudhead")?;
        let _cloudhead = super::verify_user(cloudhead)?;
        let kalt = api.init_user(key, "kalt")?;
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
}
