use std::env;
use std::net::SocketAddr;

use async_trait::async_trait;

use librad::meta::entity;
use librad::meta::project;
use librad::meta::user;
use librad::net::discovery;
pub use librad::net::peer::{PeerApi, PeerConfig};
use librad::surf;
use librad::surf::vcs::git::git2;
use librad::uri::RadUrn;

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

/// Returns the list of [`project::Project`]s known for the configured [`paths::Paths`].
///
/// # Errors
///
/// The function will error if:
///   * A lock was poisioned. See [`Self::with_api`].
#[allow(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]
pub fn list_projects(api: &PeerApi) -> Result<Vec<project::Project<entity::Draft>>, error::Error> {
    let storage = api.storage();
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

/// Returns the list of [`user::User`]s known for the configured [`paths::Paths`].
///
/// # Errors
///
/// The function will error if:
///   * A lock was poisioned. See [`Self::with_api`].
#[allow(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]
pub fn list_users(api: &PeerApi) -> Result<Vec<user::User<entity::Draft>>, error::Error> {
    let storage = api.storage();
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
    api: &PeerApi,
    urn: &RadUrn,
) -> Result<project::Project<entity::Draft>, error::Error> {
    let storage = api.storage().reopen()?;
    Ok(storage.metadata(urn)?)
}

/// Get the user found at `urn`.
///
/// # Errors
///
///   * Resolving the project fails.
///   * Could not successfully acquire a lock to the API.
pub fn get_user(api: &PeerApi, urn: &RadUrn) -> Result<user::User<entity::Draft>, error::Error> {
    let storage = api.storage().reopen()?;
    Ok(storage.metadata(urn)?)
}

/// Get a repo browser for a project.
///
/// # Errors
///
/// The function will result in an error if the mutex guard was poisoned. See
/// [`std::sync::Mutex::lock`] for further details.
pub fn with_browser<F, T>(
    api: &PeerApi,
    project_urn: &RadUrn,
    callback: F,
) -> Result<T, error::Error>
where
    F: Send + FnOnce(&mut surf::vcs::git::Browser) -> Result<T, error::Error>,
{
    let project = get_project(api, project_urn)?;
    let default_branch = project.default_branch();
    let repo = api.storage().open_repo(project.urn())?;
    let mut browser = repo.browser(default_branch)?;
    callback(&mut browser)
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
pub fn init_project(
    api: &PeerApi,
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
        let key = api.key();

        // Create the project meta
        let mut meta = project::Project::<entity::Draft>::create(name.to_string(), owner.urn())?
            .to_builder()
            .set_description(description.to_string())
            .set_default_branch(default_branch.to_string())
            .add_key(key.public())
            .build()?;
        meta.sign_owned(key)?;
        let urn = meta.urn();

        let storage = api.storage().reopen()?;

        if storage.has_urn(&urn)? {
            return Err(error::Error::EntityExists(urn));
        } else {
            let _repo = storage.create_repo(&meta)?;
        }
        Ok(meta)
    };

    // Doing ? above breaks inference. Gaaaawwwwwd Rust!
    let meta = meta?;

    setup_remote(api, path, &meta.urn().id, default_branch)?;

    Ok(meta)
}

/// Create a [`user::User`] with the provided `handle`. This assumes that you are creating a
/// user that uses the secret key the `PeerApi` was configured with.
///
/// # Errors
///
/// Will error if:
///     * [`Self::with_api`] fails with a poisoned lock.
///     * The signing of the user metadata fails.
///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
pub fn init_user(api: &PeerApi, handle: &str) -> Result<user::User<entity::Draft>, error::Error> {
    let key = api.key();

    // Create the project meta
    let mut user = user::User::<entity::Draft>::create(handle.to_string(), key.public())?;
    user.sign_owned(key)?;
    let urn = user.urn();

    let storage = api.storage().reopen()?;

    if storage.has_urn(&urn)? {
        return Err(error::Error::EntityExists(urn));
    } else {
        let _repo = storage.create_repo(&user)?;
    }

    Ok(user)
}

/// Verify a user using a fake resolver that resolves the user to itself.
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
    api: &PeerApi,
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

    let monorepo = api.paths().git_dir().join("");
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
pub fn replicate_platinum(
    api: &PeerApi,
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
    let monorepo = api.paths().git_dir().join("");
    let workspace = monorepo.join("../workspace");
    let platinum_into = workspace.join(name);

    clone_platinum(&platinum_from, &platinum_into)?;

    let meta = init_project(
        api,
        owner,
        platinum_into.clone(),
        name,
        description,
        default_branch,
    )?;

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
pub fn setup_fixtures(api: &PeerApi, owner: &User) -> Result<(), error::Error> {
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
        replicate_platinum(api, owner, info.0, info.1, info.2)?;
    }

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
    use crate::error::Error;

    #[tokio::test]
    async fn test_can_create_user() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let annie = super::init_user(&peer, "annie_are_you_ok?");
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_can_create_project() -> Result<(), Error> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, "cloudhead")?;
        let user = super::verify_user(user).await?;
        let project = super::init_project(
            &peer,
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
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, "cloudhead")?;
        let user = super::verify_user(user).await?;
        let err = super::init_user(&peer, "cloudhead");

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
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, "cloudhead")?;
        let user = super::verify_user(user).await?;
        let _project = super::init_project(
            &peer,
            &user,
            &repo_path,
            "radicalise",
            "the people",
            "power",
        )?;

        let err = super::init_project(
            &peer,
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
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let user = super::init_user(&peer, "cloudhead")?;
        let user = super::verify_user(user).await?;
        super::setup_fixtures(&peer, &user)?;

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
        let config = config::default(key, tmp_dir.path())?;
        let peer = super::create_peer_api(config).await?;

        let cloudhead = super::init_user(&peer, "cloudhead")?;
        let _cloudhead = super::verify_user(cloudhead).await?;
        let kalt = super::init_user(&peer, "kalt")?;
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
