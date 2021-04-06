//! Utility to work with the peer api of librad.

use std::{
    convert::{TryFrom as _, TryInto},
    net::SocketAddr,
    path::PathBuf,
};

use either::Either;

use librad::{
    git::{
        identities::{
            self,
            local::{self, LocalIdentity},
            person, project,
        },
        include::{self, Include},
        local::{transport, url::LocalUrl},
        refs::Refs,
        replication::{self, ReplicateResult},
        tracking,
        types::{Namespace, Reference, Single},
        Urn,
    },
    git_ext::{OneLevel, RefLike},
    identities::{
        delegation::Indirect,
        payload::{self, PersonPayload},
        Person, Project, SomeIdentity,
    },
    internal::canonical::Cstring,
    keys,
    net::peer::Peer,
    paths,
    peer::PeerId,
    signer::BoxedSigner,
};
use radicle_keystore::sign::Signer as _;
use radicle_surf::vcs::{git, git::git2};

use crate::{peer::gossip, project::peer, source};

pub mod error;
pub use error::Error;

/// Check the storage to see if we have the given commit for project at `urn`.
///
/// # Errors
///
///   * Checking the storage for the commit fails.
pub async fn has_commit<Oid>(peer: &Peer<BoxedSigner>, urn: Urn, oid: Oid) -> Result<bool, Error>
where
    Oid: AsRef<git2::Oid> + std::fmt::Debug + Send + 'static,
{
    Ok(peer
        .using_storage(move |storage| storage.has_commit(&urn, oid))
        .await??)
}

/// Get the default owner for this `PeerApi`.
///
/// # Errors
///   * Opening the storage config failed
///   * Fetching the `Urn` from the config failed
///   * Loading the `LocalIdentity` failed
pub async fn default_owner(peer: &Peer<BoxedSigner>) -> Result<Option<LocalIdentity>, Error> {
    Ok(peer
        .using_storage(move |store| {
            if let Some(urn) = store.config()?.user()? {
                return local::load(store, urn).map_err(Error::from);
            }

            Ok::<_, Error>(None)
        })
        .await??)
}

/// Set the default owner for this `PeerApi`.
///
/// # Errors
///
///   * Fails to set the default `rad/self` for this `PeerApi`.
pub async fn set_default_owner<U>(peer: &Peer<BoxedSigner>, user: U) -> Result<(), Error>
where
    U: Into<Option<LocalIdentity>> + Send + Sync + 'static,
{
    Ok(peer
        .using_storage(move |storage| storage.config()?.set_user(user).map_err(Error::from))
        .await??)
}

/// Initialise a [`LocalIdentity`] and make them the default owner of this [`Peer`].
///
/// # Errors
///
///   * Fails to initialise `User`.
///   * Fails to verify `User`.
///   * Fails to set the default `rad/self` for this `PeerApi`.
#[allow(clippy::single_match_else)]
pub async fn init_owner<P>(peer: &Peer<BoxedSigner>, payload: P) -> Result<LocalIdentity, Error>
where
    P: TryInto<PersonPayload> + Send,
    Error: From<P::Error>,
{
    match peer
        .using_storage(move |store| local::default(store))
        .await??
    {
        Some(owner) => Ok(owner),
        None => {
            let pk = keys::PublicKey::from(peer.signer().public_key());
            let payload = payload.try_into()?;
            let person = peer
                .using_storage(move |store| {
                    person::create(store, payload, Some(pk).into_iter().collect())
                })
                .await??;

            let urn = person.urn();
            let owner = peer
                .using_storage(move |store| local::load(store, urn))
                .await??
                .ok_or_else(|| Error::PersonNotFound(person.urn()))?;

            {
                let owner = owner.clone();
                peer.using_storage(move |store| {
                    let mut config = store.config()?;
                    config.set_user(owner)?;

                    Ok::<_, Error>(())
                })
                .await??;
            }

            Ok(owner)
        },
    }
}

/// Given some hints as to where you might find it, get the urn of the project found at `url`.
///
/// # Errors
///   * Could not successfully acquire a lock to the API.
///   * Could not open librad storage.
///   * Failed to clone the project.
///   * Failed to set the rad/self of this project.
pub async fn clone_project<C, Addrs>(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    remote_peer: PeerId,
    addr_hints: Addrs,
    config: C,
) -> Result<ReplicateResult, Error>
where
    C: Into<Option<replication::Config>> + Send,
    Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
{
    let config = config
        .into()
        .unwrap_or_else(|| peer.protocol_config().replication);
    let owner = default_owner(peer).await?.ok_or(Error::MissingOwner)?;
    Ok(peer
        .using_storage(move |store| {
            replication::replicate(
                store,
                config,
                Some(owner),
                urn.clone(),
                remote_peer,
                addr_hints,
            )
        })
        .await??)
}

/// Get the project found at `urn`.
///
/// # Errors
///
///   * Resolving the project fails.
pub async fn get_project(peer: &Peer<BoxedSigner>, urn: Urn) -> Result<Option<Project>, Error> {
    peer.using_storage(move |store| identities::project::get(store, &urn))
        .await?
        .map_err(Error::from)
}

/// Returns the list of [`Project`]s for the local peer.
///
/// # Errors
///
///   * Retrieving the project entities from the store fails.
pub async fn list_projects(peer: &Peer<BoxedSigner>) -> Result<Vec<Project>, Error> {
    // FIXME(xla): Instead of implicitely expecting a presence of a default owner, there either
    // should be an explicit argument, or it's made impossible to call this function without an
    // owner associated with the state.
    let owner = match default_owner(peer).await? {
        None => return Err(Error::MissingOwner),
        Some(owner) => owner.into_inner().into_inner(),
    };

    peer.using_storage(move |store| {
        let projects = identities::any::list(store)?
            .filter_map(Result::ok)
            .filter_map(|id| match id {
                SomeIdentity::Project(project) => {
                    let rad_self = Reference::rad_self(Namespace::from(project.urn()), None);
                    let urn = Urn::try_from(rad_self).ok()?;
                    let project_self = person::get(store, &urn).ok()??;
                    // Filter projects that have a rad/self pointing to current default
                    // owner
                    if project_self == owner {
                        Some(project)
                    } else {
                        None
                    }
                },
                _ => None,
            })
            .collect::<Vec<_>>();

        Ok::<_, Error>(projects)
    })
    .await?
}

/// Retrieves the [`librad::git::refs::Refs`] for the state owner.
///
/// # Errors
///
/// * if opening the storage fails
pub async fn list_owner_project_refs(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
) -> Result<Option<Refs>, Error> {
    peer.using_storage(move |store| Refs::load(store, &urn, None))
        .await?
        .map_err(Error::from)
}

/// Retrieves the [`librad::git::refs::Refs`] for the given project urn.
///
/// # Errors
///
/// * if opening the storage fails
pub async fn list_peer_project_refs(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    peer_id: PeerId,
) -> Result<Option<Refs>, Error> {
    peer.using_storage(move |store| Refs::load(store, &urn, Some(peer_id)))
        .await?
        .map_err(Error::from)
}

/// Returns the list of [`Person`]s known for your peer.
///
/// # Errors
///
///   * Retrieval of the user entities from the store fails.
pub async fn list_users(peer: &Peer<BoxedSigner>) -> Result<Vec<Person>, Error> {
    peer.using_storage(move |store| {
        let projects = identities::any::list(store)?
            .filter_map(Result::ok)
            .filter_map(|id| match id {
                SomeIdentity::Person(person) => Some(person),
                _ => None,
            })
            .collect::<Vec<_>>();

        Ok::<_, Error>(projects)
    })
    .await?
}

/// Given some hints as to where you might find it, get the urn of the user found at `url`.
///
/// # Errors
///
///   * Could not successfully acquire a lock to the API.
///   * Could not open librad storage.
///   * Failed to clone the user.
pub async fn clone_user<C, Addrs>(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    remote_peer: PeerId,
    addr_hints: Addrs,
    config: C,
) -> Result<ReplicateResult, Error>
where
    C: Into<Option<replication::Config>> + Send,
    Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
{
    let config = config
        .into()
        .unwrap_or_else(|| peer.protocol_config().replication);
    peer.using_storage(move |store| {
        replication::replicate(store, config, None, urn, remote_peer, addr_hints)
    })
    .await?
    .map_err(Error::from)
}

/// Get the user found at `urn`.
///
/// # Errors
///
///   * Resolving the user fails.
///   * Could not successfully acquire a lock to the API.
pub async fn get_user(peer: &Peer<BoxedSigner>, urn: Urn) -> Result<Option<LocalIdentity>, Error> {
    peer.using_storage(move |store| match identities::person::get(store, &urn)? {
        None => Ok(None),
        Some(person) => local::load(store, person.urn()),
    })
    .await?
    .map_err(Error::from)
}

/// Fetch any updates at the given `RadUrl`, providing address hints if we have them.
///
/// # Errors
///
///   * Could not successfully acquire a lock to the API.
///   * Could not open librad storage.
///   * Failed to fetch the updates.
///   * Failed to set the rad/self of this project.
pub async fn fetch<C, Addrs>(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    remote_peer: PeerId,
    addr_hints: Addrs,
    config: C,
) -> Result<ReplicateResult, Error>
where
    C: Into<Option<replication::Config>> + Send,
    Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
{
    let config = config
        .into()
        .unwrap_or_else(|| peer.protocol_config().replication);
    Ok(peer
        .using_storage(move |store| {
            replication::replicate(store, config, None, urn, remote_peer, addr_hints)
        })
        .await??)
}

/// Initialize a [`Project`] that is owned by the `owner`.
/// This kicks off the history of the project, tracked by `librad`'s mono-repo.
///
/// # Errors
///
/// Will error if:
///     * The signing of the project metadata fails.
///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
pub async fn init_project(
    peer: &Peer<BoxedSigner>,
    owner: &LocalIdentity,
    create: crate::project::Create,
) -> Result<Project, Error> {
    let default_branch = create.default_branch.to_string();
    let description = create.description.to_string();
    let name = create
        .repo
        .project_name()
        .map_err(crate::project::create::Error::from)?;
    let owner = owner.clone();
    let payload = payload::Project {
        default_branch: Some(Cstring::from(default_branch)),
        description: Some(Cstring::from(description)),
        name: Cstring::from(name),
    };
    let delegations = Indirect::from(owner.clone().into_inner().into_inner());
    let (repository, project) = peer
        .using_storage(move |store| {
            let urn = project::urn(store, payload.clone(), delegations.clone())?;
            let url = LocalUrl::from(urn.clone());
            let repository = create
                .validate(url)
                .map_err(crate::project::create::Error::from)?;

            if store.has_urn(&urn)? {
                Err(Error::IdentityExists(urn))
            } else {
                Ok((
                    repository,
                    project::create(store, owner.clone(), payload, delegations)?,
                ))
            }
        })
        .await??;

    log::debug!(
        "Created project '{}#{}'",
        project.urn(),
        project.subject().name
    );

    let repo = repository
        .setup_repo(
            settings(peer),
            project
                .subject()
                .description
                .as_deref()
                .unwrap_or(&String::default()),
        )
        .map_err(crate::project::create::Error::from)?;
    let include_path = update_include(peer, project.urn()).await?;
    include::set_include_path(&repo, include_path)?;
    gossip::announce(peer, &project.urn(), None);

    Ok(project)
}

/// Create a [`LocalIdentity`] with the provided `handle`. This assumes that you are creating a
/// user that uses the secret key the `PeerApi` was configured with.
///
/// # Errors
///
/// Will error if:
///     * The signing of the user metadata fails.
///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
pub async fn init_user(peer: &Peer<BoxedSigner>, name: String) -> Result<LocalIdentity, Error> {
    let pk = keys::PublicKey::from(peer.signer().public_key());
    peer.using_storage(move |store| {
        let malkovich = person::create(
            store,
            payload::Person {
                name: Cstring::from(name),
            },
            Some(pk).into_iter().collect(),
        )?;

        Ok::<_, Error>(local::load(store, malkovich.urn())?)
    })
    .await??
    .ok_or(Error::IdentityCreationFailed)
}

/// Wrapper around the storage track.
///
/// # Errors
///
/// * When the storage operation fails.
pub async fn track(peer: &Peer<BoxedSigner>, urn: Urn, remote_peer: PeerId) -> Result<(), Error> {
    {
        let urn = urn.clone();
        peer.using_storage(move |store| tracking::track(store, &urn, remote_peer))
            .await??;
    }

    gossip::query(peer, &urn, Some(remote_peer));
    let path = update_include(peer, urn).await?;
    log::debug!("Updated include path @ `{}`", path.display());
    Ok(())
}

/// Wrapper around the storage untrack.
///
/// # Errors
///
/// * When the storage operation fails.
pub async fn untrack(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    remote_peer: PeerId,
) -> Result<bool, Error> {
    let res = {
        let urn = urn.clone();
        peer.using_storage(move |store| tracking::untrack(store, &urn, remote_peer))
            .await??
    };

    // Only need to update if we did untrack an existing peer
    if res {
        let path = update_include(peer, urn).await?;
        log::debug!("Updated include path @ `{}`", path.display());
    }
    Ok(res)
}

/// Get the [`crate::project::Peer`]s that are tracking this project, including their
/// [`PeerId`].
///
/// # Errors
///
/// * If we could not acquire the lock
/// * If we could not open the storage
/// * If did not have the `urn` in storage
/// * If we could not fetch the tracked peers
/// * If we could not get the `rad/self` of the peer
pub async fn tracked(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
) -> Result<Vec<crate::project::Peer<peer::Status<Person>>>, Error> {
    let project = get_project(peer, urn.clone())
        .await?
        .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;

    peer.using_storage(move |store| {
        let mut peers = vec![];

        for peer_id in tracking::tracked(store, &urn)? {
            let rad_self =
                Urn::try_from(Reference::rad_self(Namespace::from(urn.clone()), peer_id))
                    .expect("namespace is set");
            let status = if store.has_urn(&rad_self)? {
                let malkovich =
                    person::get(store, &rad_self)?.ok_or(Error::PersonNotFound(rad_self))?;

                let role = role(store, &project, Either::Right(peer_id))?;
                peer::Status::replicated(role, malkovich)
            } else {
                peer::Status::NotReplicated
            };

            peers.push(crate::project::Peer::Remote { peer_id, status });
        }

        Ok::<_, Error>(peers)
    })
    .await?
}

// TODO(xla): Account for projects not replicated but wanted.
/// Constructs the list of [`crate::project::Peer`] for the given `urn`. The basis is the list
/// of tracking peers of the project combined with the local view.
///
/// # Errors
///
/// * if the project is not present in the monorepo
/// * if the retrieval of tracking peers fails
///
/// # Panics
///
/// * if the default owner can't be fetched
#[allow(clippy::blocks_in_if_conditions)]
pub async fn list_project_peers(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
) -> Result<Vec<crate::project::Peer<peer::Status<Person>>>, Error> {
    let project = get_project(peer, urn.clone())
        .await?
        .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;

    let mut peers = vec![];

    let owner = default_owner(peer)
        .await?
        .ok_or(Error::MissingOwner)?
        .into_inner()
        .into_inner();

    let local = peer.peer_id();
    let role = peer
        .using_storage(move |store| role(store, &project, Either::Left(local)))
        .await??;
    let status = peer::Status::replicated(role, owner);
    peers.push(crate::project::Peer::Local {
        peer_id: peer.peer_id(),
        status,
    });

    let mut remotes = tracked(peer, urn).await?;

    peers.append(&mut remotes);

    Ok(peers)
}

/// Creates a working copy for the project of the given `urn`.
///
/// The `destination` is the directory where the caller wishes to place the working copy.
///
/// The `peer_id` is from which peer we wish to base our checkout from.
///
/// # Errors
///
/// * if the project can't be found
/// * if the include file creation fails
/// * if the clone of the working copy fails
pub async fn checkout<P>(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    peer_id: P,
    destination: PathBuf,
) -> Result<PathBuf, Error>
where
    P: Into<Option<PeerId>> + Send + 'static,
{
    let peer_id = peer_id.into();
    let proj = get_project(peer, urn.clone())
        .await?
        .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;
    let include_path = update_include(peer, urn.clone()).await?;
    let name = proj.subject().name.to_string();
    let default_branch: OneLevel = OneLevel::from(
        proj.subject()
            .default_branch
            .clone()
            .ok_or(Error::NoDefaultBranch {
                name: name.clone(),
                urn: urn.clone(),
            })?
            .parse::<RefLike>()?,
    );
    let checkout = crate::project::Checkout {
        urn: proj.urn(),
        name,
        default_branch,
        path: destination,
        include_path,
    };

    log::debug!("Determing Owner");
    let ownership = match peer_id {
        None => crate::project::checkout::Ownership::Local(peer.peer_id()),
        Some(remote) => {
            let handle = {
                let rad_self =
                    Urn::try_from(Reference::rad_self(Namespace::from(urn.clone()), peer_id))
                        .expect("namespace is set");
                log::debug!("Monorepo: {}", monorepo(peer).display());
                let person = peer
                    .using_storage(move |store| {
                        log::debug!("Urn -> {}", rad_self);
                        person::get(store, &rad_self)?.ok_or(Error::PersonNotFound(rad_self))
                    })
                    .await??;

                person.subject().name.to_string()
            };

            crate::project::checkout::Ownership::Remote {
                handle,
                remote,
                local: peer.peer_id(),
            }
        },
    };

    let settings = settings(peer);
    log::debug!("Cloning");
    let path = checkout.run(settings, ownership).map_err(Error::from)?;

    Ok(path)
}

/// Prepare the include file for the given `project` with the latest tracked peers.
///
/// # Errors
///
/// * if getting the list of tracked peers fails
pub async fn update_include(peer: &Peer<BoxedSigner>, urn: Urn) -> Result<PathBuf, Error> {
    let local_url = LocalUrl::from(urn.clone());
    let tracked = tracked(peer, urn).await?;
    let include = Include::from_tracked_persons(
        paths(peer).git_includes_dir().to_path_buf(),
        local_url,
        tracked
            .into_iter()
            .filter_map(|peer| {
                crate::project::Peer::replicated_remote(peer)
                    .map(|(p, u)| RefLike::try_from(u.subject().name.to_string()).map(|r| (r, p)))
            })
            .collect::<Result<Vec<_>, _>>()?,
    );
    let include_path = include.file_path();
    log::info!("creating include file @ '{:?}'", include_path);
    include.save()?;

    Ok(include_path)
}

/// Provide a repo [`git::Browser`] where the `Browser` is initialised with the provided
/// `reference`.
///
/// See [`find_default_branch`] and [`get_branch`] for obtaining a
/// [`Reference`].
///
/// # Errors
///   * If the namespace of the reference could not be converted to a [`git::Namespace`].
///   * If we could not open the backing storage.
///   * If we could not initialise the `Browser`.
///   * If the callback provided returned an error.
pub async fn with_browser<T, F>(
    peer: &Peer<BoxedSigner>,
    reference: Reference<Single>,
    callback: F,
) -> Result<T, Error>
where
    F: FnOnce(&mut git::Browser) -> Result<T, source::Error> + Send,
{
    let namespace = git::namespace::Namespace::try_from(
        reference
            .namespace
            .ok_or(Error::MissingNamespace)?
            .to_string()
            .as_str(),
    )?;

    let branch = match reference.remote {
        None => git::Branch::local(reference.name.as_str()),
        Some(peer) => git::Branch::remote(
            &format!("heads/{}", reference.name.as_str()),
            &peer.to_string(),
        ),
    };

    let monorepo = monorepo(peer);
    let repo = git::Repository::new(monorepo).map_err(source::Error::from)?;
    let mut browser =
        git::Browser::new_with_namespace(&repo, &namespace, branch).map_err(source::Error::from)?;

    callback(&mut browser).map_err(Error::from)
}

/// This method helps us get a branch for a given [`Urn`] and optional [`PeerId`].
///
/// If the `branch_name` is `None` then we get the project for the given [`Urn`] and use its
/// `default_branch`.
///
/// # Errors
///   * If the storage operations fail.
///   * If the requested reference was not found.
pub async fn get_branch<P, B>(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
    remote: P,
    branch_name: B,
) -> Result<Reference<Single>, Error>
where
    P: Into<Option<PeerId>> + Clone + Send,
    B: Into<Option<Cstring>> + Clone + Send,
{
    let name = match branch_name.into() {
        None => {
            let project = get_project(peer, urn.clone())
                .await?
                .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;
            project
                .subject()
                .default_branch
                .clone()
                .ok_or(Error::NoDefaultBranch {
                    name: project.subject().name.to_string(),
                    urn: urn.clone(),
                })?
        },
        Some(name) => name,
    }
    .parse()?;

    let remote = match remote.into() {
        Some(peer_id) if peer_id == peer.peer_id() => None,
        Some(peer_id) => Some(peer_id),
        None => None,
    };
    let reference = Reference::head(Namespace::from(urn), remote, name);
    let exists = {
        let reference = reference.clone();
        peer.using_storage(move |storage| storage.has_ref(&reference))
            .await??
    };

    if exists {
        Ok(reference)
    } else {
        Err(Error::MissingRef { reference })
    }
}

/// This method helps us get the default branch for a given [`Urn`].
///
/// It does this by:
///     * First checking if the owner of this storage has a reference to the default
/// branch.
///     * If the owner does not have this reference then it falls back to the first maintainer.
///
/// # Errors
///   * If the storage operations fail.
///   * If no default branch was found for the provided [`Urn`].
pub async fn find_default_branch(
    peer: &Peer<BoxedSigner>,
    urn: Urn,
) -> Result<Reference<Single>, Error> {
    let project = get_project(peer, urn.clone())
        .await?
        .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;

    let default_branch =
        project
            .subject()
            .default_branch
            .clone()
            .ok_or(Error::NoDefaultBranch {
                name: project.subject().name.to_string(),
                urn: urn.clone(),
            })?;

    // TODO(xla): Check for all delegations if there is default branch.
    let remote = project
        .delegations()
        .iter()
        .flat_map(|either| match either {
            Either::Left(pk) => Either::Left(std::iter::once(PeerId::from(*pk))),
            Either::Right(indirect) => {
                Either::Right(indirect.delegations().iter().map(|pk| PeerId::from(*pk)))
            },
        })
        .next()
        .expect("missing delegation");

    let (owner, remote) = tokio::join!(
        get_branch(peer, urn.clone(), None, default_branch.to_owned()),
        get_branch(peer, urn.clone(), remote, default_branch.to_owned())
    );
    match owner.or(remote) {
        Ok(reference) => Ok(reference),
        Err(Error::MissingRef { .. }) => Err(Error::NoDefaultBranch {
            name: project.subject().name.to_string(),
            urn,
        }),
        Err(err) => Err(err),
    }
}

/// Returns the [`PathBuf`] to the underlying monorepo.
#[must_use]
pub fn monorepo(peer: &Peer<BoxedSigner>) -> PathBuf {
    peer.protocol_config().paths.git_dir().to_owned()
}

/// Returns the underlying [`paths::Paths`].
#[must_use]
pub fn paths(peer: &Peer<BoxedSigner>) -> paths::Paths {
    peer.protocol_config().paths.clone()
}

/// Construct the local [`transport::Settings`] for interacting with git related I/O.
#[must_use]
pub fn settings(peer: &Peer<BoxedSigner>) -> transport::Settings {
    transport::Settings {
        paths: peer.protocol_config().paths.clone(),
        signer: peer.signer().clone(),
    }
}

/// Determine the [`peer::Role`] for a given [`Project`] and [`PeerId`].
///
/// If `peer` is `Either::Left` then we have the local `PeerId` and we can ignore it for looking
/// at `rad/signed_refs`.
///
/// If `peer` is `Either::Right` then it is a remote peer and we use it for looking at
/// `rad/signed_refs`.
fn role(
    store: &librad::git::storage::Storage,
    project: &Project,
    peer: Either<PeerId, PeerId>,
) -> Result<peer::Role, Error> {
    let role = if project
        .delegations()
        .owner(peer.clone().into_inner().as_public_key())
        .is_some()
    {
        peer::Role::Maintainer
    } else if Refs::load(store, &project.urn(), peer.right())?
        .map_or(false, |refs| !refs.heads.is_empty())
    {
        peer::Role::Contributor
    } else {
        peer::Role::Tracker
    };

    Ok(role)
}

#[allow(clippy::panic, clippy::unwrap_used)]
#[cfg(test)]
pub mod test {
    use std::{env, path::PathBuf};

    use librad::{git_ext::OneLevel, identities::payload::Person, keys::SecretKey, net, reflike};

    use crate::{config, project, signer};

    fn fakie_project(path: PathBuf) -> project::Create {
        project::Create {
            repo: project::Repo::New {
                path,
                name: "fakie-nose-kickflip-backside-180-to-handplant".to_string(),
            },
            description: "rad git tricks".to_string(),
            default_branch: OneLevel::from(reflike!("dope")),
        }
    }

    fn radicle_project(path: PathBuf) -> project::Create {
        project::Create {
            repo: project::Repo::New {
                path,
                name: "radicalise".to_string(),
            },
            description: "the people".to_string(),
            default_branch: OneLevel::from(reflike!("power")),
        }
    }

    fn fixtures(path: PathBuf) -> Vec<project::Create> {
        vec![
            project::Create {
                repo: project::Repo::New {
                    path: path.clone(),
                    name: "monokel".to_string(),
                },
                description: "A looking glass into the future".to_string(),
                default_branch: OneLevel::from(reflike!("mastor")),
            },
            project::Create {
                repo: project::Repo::New {
                    path: path.clone(),
                    name: "Monadic".to_string(),
                },
                description: "Open source organization of amazing things.".to_string(),
                default_branch: OneLevel::from(reflike!("mastor")),
            },
            project::Create {
                repo: project::Repo::New {
                    path: path.clone(),
                    name: "open source coin".to_string(),
                },
                description: "Research for the sustainability of the open source community."
                    .to_string(),
                default_branch: OneLevel::from(reflike!("mastor")),
            },
            project::Create {
                repo: project::Repo::New {
                    path,
                    name: "radicle".to_string(),
                },
                description: "Decentralized open source collaboration".to_string(),
                default_branch: OneLevel::from(reflike!("mastor")),
            },
        ]
    }

    #[tokio::test]
    async fn can_create_user() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config);

        let annie = super::init_user(&peer, "annie_are_you_ok?".to_string()).await;
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        env::set_var("RAD_HOME", tmp_dir.path());
        let repo_path = tmp_dir.path().join("radicle");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config);

        let user = super::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;
        let project = super::init_project(&peer, &user, radicle_project(repo_path.clone())).await;

        assert!(project.is_ok());
        assert!(repo_path.join("radicalise").exists());

        Ok(())
    }

    #[tokio::test]
    async fn can_create_project_for_existing_repo() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let repo_path = repo_path.join("radicalise");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config);

        let user = super::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;
        let project = super::init_project(&peer, &user, radicle_project(repo_path.clone())).await;

        assert!(project.is_ok());
        assert!(repo_path.exists());

        Ok(())
    }

    #[tokio::test]
    async fn list_projects() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");

        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config);

        let user = super::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;

        for fixture in fixtures(repo_path.clone()) {
            super::init_project(&peer, &user, fixture).await?;
        }

        let kalt = super::init_user(&peer, "kalt".to_string()).await?;
        let fakie = super::init_project(&peer, &kalt, fakie_project(repo_path)).await?;

        let projects = super::list_projects(&peer).await?;
        let mut project_names = projects
            .into_iter()
            .map(|project| project.subject().name.to_string())
            .collect::<Vec<_>>();
        project_names.sort();

        assert_eq!(
            project_names,
            vec!["Monadic", "monokel", "open source coin", "radicle"]
        );

        assert!(!project_names.contains(&fakie.subject().name.to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn list_users() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = signer::BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config);

        let _cloudhead = super::init_user(&peer, "cloudhead".to_string()).await?;
        let _kalt = super::init_user(&peer, "kalt".to_string()).await?;

        let users = super::list_users(&peer).await?;
        let mut user_handles = users
            .into_iter()
            .map(|user| user.subject().name.to_string())
            .collect::<Vec<_>>();
        user_handles.sort();

        assert_eq!(user_handles, vec!["cloudhead", "kalt"],);

        Ok(())
    }
}
