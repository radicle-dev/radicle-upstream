use std::net::SocketAddr;

use async_trait::async_trait;

use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use librad::meta::user;
use librad::net::discovery;
pub use librad::net::peer::{PeerApi, PeerConfig};
use librad::uri::RadUrn;
use radicle_surf::vcs::git::{self, git2};

use crate::error;

/// Export a verified [`user::User`] type.
pub type User = user::User<entity::Verified>;

/// Create a new `PeerApi` given a `PeerConfig`.
///
/// # Errors
///
/// If turning the config into a `Peer` fails
/// If trying to accept on the socket fails
pub async fn create_peer_api<I>(
    config: PeerConfig<discovery::Static<I, SocketAddr>>,
) -> Result<PeerApi, error::Error>
where
    I: Iterator<Item = (librad::peer::PeerId, SocketAddr)> + Send + 'static,
{
    let peer = config.try_into_peer().await?;
    // TODO(finto): discarding the run loop below. Should be used to subsrcibe to events and
    // publish events.
    let (api, _futures) = peer.accept()?;
    Ok(api)
}

/// Returns the list of [`project::Project`]s for your peer.
///
/// # Errors
///
/// The function will error if:
///   * The retrieving the project entities from the store fails.
#[allow(
    clippy::match_wildcard_for_single_variants,
    clippy::wildcard_enum_match_arm
)]
pub fn list_projects(peer: &PeerApi) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
    let storage = peer.storage();
    Ok(storage
        .all_metadata()?
        .flat_map(|entity| {
            entity.ok()?.try_map(|info| match info {
                entity::data::EntityInfo::Project(info) => Some(info),
                _ => None,
            })
        })
        .collect())
}

/// Returns the list of [`user::User`]s known for your peer.
///
/// # Errors
///
/// The function will error if:
///   * The retrieving the project entities from the store fails.
#[allow(
    clippy::match_wildcard_for_single_variants,
    clippy::wildcard_enum_match_arm
)]
pub fn list_users(peer: &PeerApi) -> Result<Vec<user::User<entity::Draft>>, error::Error> {
    let storage = peer.storage();
    Ok(storage
        .all_metadata()?
        .flat_map(|entity| {
            entity.ok()?.try_map(|info| match info {
                entity::data::EntityInfo::User(info) => Some(info),
                _ => None,
            })
        })
        .collect())
}

/// Get the project found at `project_urn`.
///
/// # Errors
///
/// `get_project` fails if:
///     * Parsing the `project_urn` fails.
///     * Resolving the project fails.
pub fn get_project(
    peer: &PeerApi,
    urn: &RadUrn,
) -> Result<project::Project<entity::Draft>, error::Error> {
    let storage = peer.storage().reopen()?;
    Ok(storage.metadata(urn)?)
}

/// Get the user found at `urn`.
///
/// # Errors
///
///   * Resolving the project fails.
///   * Could not successfully acquire a lock to the API.
pub fn get_user(peer: &PeerApi, urn: &RadUrn) -> Result<user::User<entity::Draft>, error::Error> {
    let storage = peer.storage().reopen()?;
    Ok(storage.metadata(urn)?)
}

/// Get a repo browser for a project.
///
/// # Errors
///
/// The function will result in an error if the mutex guard was poisoned. See
/// [`std::sync::Mutex::lock`] for further details.
pub fn with_browser<F, T>(
    peer: &PeerApi,
    project_urn: &RadUrn,
    callback: F,
) -> Result<T, error::Error>
where
    F: Send + FnOnce(&mut git::Browser) -> Result<T, error::Error>,
{
    let project = get_project(peer, project_urn)?;
    let default_branch = project.default_branch();
    let git_dir = peer.paths().git_dir();
    let repo = git::Repository::new(git_dir)?;
    let namespace = git::Namespace::from(project.urn().id.to_string().as_str());
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
#[allow(clippy::needless_pass_by_value)] // We don't want to keep `SecretKey` in memory.
pub fn init_project(
    peer: &PeerApi,
    key: keys::SecretKey,
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

    let meta: Result<project::Project<entity::Draft>, error::Error> = {
        // Create the project meta
        let mut meta = project::Project::<entity::Draft>::create(name.to_string(), owner.urn())?
            .to_builder()
            .set_description(description.to_string())
            .set_default_branch(default_branch.to_string())
            .add_key(key.public())
            .add_certifier(owner.urn())
            .build()?;
        meta.sign_owned(&key)?;
        let urn = meta.urn();

        let storage = peer.storage().reopen()?;

        if storage.has_urn(&urn)? {
            return Err(error::Error::EntityExists(urn));
        } else {
            let _repo = storage.create_repo(&meta)?;
        }
        Ok(meta)
    };

    // Doing ? above breaks inference. Gaaaawwwwwd Rust!
    let meta = meta?;

    setup_remote(peer, path, &meta.urn().id, default_branch)?;

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
#[allow(clippy::needless_pass_by_value)] // We don't want to keep `SecretKey` in memory.
pub fn init_user(
    peer: &PeerApi,
    key: keys::SecretKey,
    handle: &str,
) -> Result<user::User<entity::Draft>, error::Error> {
    // Create the project meta
    let mut user = user::User::<entity::Draft>::create(handle.to_string(), key.public())?;
    user.sign_owned(&key)?;
    let urn = user.urn();

    let storage = peer.storage().reopen()?;

    if storage.has_urn(&urn)? {
        return Err(error::Error::EntityExists(urn));
    } else {
        let _repo = storage.create_repo(&user)?;
    }

    Ok(user)
}

/// Verify a user using a fake resolver that resolves the user to itself.
///
/// TODO(finto): Should not live here permanently, because resolvers should solve this verification.
///
/// # Errors
///
/// If any of the verification steps fail
pub async fn verify_user(user: user::User<entity::Draft>) -> Result<User, error::Error> {
    let fake_resolver = FakeUserResolver(user.clone());
    let verified_user = user
        .check_history_status(&fake_resolver, &fake_resolver)
        .await?;
    Ok(verified_user)
}

/// Equips a repository with a rad remote for the given id. If the directory at the given path
/// is not managed by git yet we initialise it first.
fn setup_remote(
    peer: &PeerApi,
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

    let monorepo = peer.paths().git_dir().join("");
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

#[cfg(test)]
#[allow(clippy::panic)]
mod test {
    use librad::keys::SecretKey;

    use crate::coco::config;
    use crate::coco::control;
    use crate::error::Error;

    #[tokio::test]
    async fn test_can_create_user() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let annie = super::init_user(&peer, key, "annie_are_you_ok?");
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_can_create_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, key.clone(), "cloudhead")?;
        let user = super::verify_user(user).await?;
        let project = super::init_project(
            &peer,
            key,
            &user,
            &repo_path,
            "radicalise",
            "the people",
            "power",
        );

        assert!(project.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_cannot_create_user_twice() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, key.clone(), "cloudhead")?;
        let user = super::verify_user(user).await?;
        let err = super::init_user(&peer, key, "cloudhead");

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
    async fn test_cannot_create_project_twice() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, key.clone(), "cloudhead")?;
        let user = super::verify_user(user).await?;
        let _project = super::init_project(
            &peer,
            key.clone(),
            &user,
            &repo_path,
            "radicalise",
            "the people",
            "power",
        )?;

        let err = super::init_project(
            &peer,
            key,
            &user,
            &repo_path,
            "radicalise",
            "the people",
            "power",
        );

        if let Err(Error::RadRemoteExists(path)) = err {
            assert_eq!(path, format!("{}", repo_path.display()))
        } else {
            panic!(
                "unexpected error when creating the project a second time: {:?}",
                err
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_list_projects() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, key.clone(), "cloudhead")?;
        let user = super::verify_user(user).await?;
        control::setup_fixtures(&peer, key, &user)?;

        let projects = super::list_projects(&peer)?;
        let mut project_names = projects
            .into_iter()
            .map(|project| project.name().to_string())
            .collect::<Vec<_>>();
        project_names.sort();

        assert_eq!(
            project_names,
            vec!["Monadic", "monokel", "open source coin", "radicle"]
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_list_users() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key.clone(), tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let cloudhead = super::init_user(&peer, key.clone(), "cloudhead")?;
        let _cloudhead = super::verify_user(cloudhead).await?;
        let kalt = super::init_user(&peer, key, "kalt")?;
        let _kalt = super::verify_user(kalt).await?;

        let users = super::list_users(&peer)?;
        let mut user_handles = users
            .into_iter()
            .map(|user| user.name().to_string())
            .collect::<Vec<_>>();
        user_handles.sort();

        assert_eq!(user_handles, vec!["cloudhead", "kalt"],);

        Ok(())
    }
}
