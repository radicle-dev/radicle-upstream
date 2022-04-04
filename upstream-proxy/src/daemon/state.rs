// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

//! Utility to work with the peer api of librad.

use std::{
    convert::{TryFrom as _, TryInto},
    net::SocketAddr,
    path::PathBuf,
};

use either::Either;
use tokio::task::spawn_blocking;

use librad::{
    canonical::Cstring,
    crypto::{BoxedSigner, SomeSigner},
    git::{
        identities::{
            self,
            local::{self, LocalIdentity},
            person, project,
        },
        include::{self, Include},
        local::{transport, url::LocalUrl},
        refs::Refs,
        storage::ReadOnlyStorage as _,
        tracking,
        types::{Namespace, Reference, Single},
        Urn,
    },
    git_ext::{OneLevel, RefLike},
    identities::{
        delegation::{Direct, Indirect},
        payload::{self, PersonPayload},
        Person, Project, SomeIdentity,
    },
    net::{peer::Peer, replication},
    paths, PeerId, PublicKey, Signer,
};

use crate::daemon::{
    peer::gossip,
    project::{create::Signature, peer},
};

pub mod error;
pub use error::Error;

/// Get the default owner for this `PeerApi`.
///
/// # Errors
///   * Opening the storage config failed
///   * Fetching the `Urn` from the config failed
///   * Loading the `LocalIdentity` failed
pub async fn default_owner<S>(peer: &Peer<S>) -> Result<Option<LocalIdentity>, Error>
where
    S: Clone + Signer,
{
    Ok(peer.using_storage(local::default).await??)
}

/// Set the default owner for this `PeerApi`.
///
/// # Errors
///
///   * Fails to set the default `rad/self` for this `PeerApi`.
pub async fn set_default_owner<S, U>(peer: &Peer<S>, user: U) -> Result<(), Error>
where
    S: Clone + Signer,
    U: Into<Option<LocalIdentity>> + Send + Sync + 'static,
{
    peer.using_storage(move |storage| storage.config()?.set_user(user).map_err(Error::from))
        .await?
}

/// Initialise a [`LocalIdentity`] and make them the default owner of this
/// [`Peer`].
///
/// # Errors
///
///   * Fails to initialise `User`.
///   * Fails to verify `User`.
///   * Fails to set the default `rad/self` for this `PeerApi`.
#[allow(clippy::single_match_else)]
pub async fn init_owner<S, P>(peer: &Peer<S>, payload: P) -> Result<LocalIdentity, Error>
where
    S: Clone + Signer,
    P: TryInto<PersonPayload> + Send,
    Error: From<P::Error>,
{
    if let Some(owner) = default_owner(peer).await? {
        return Ok(owner);
    }

    let payload = payload.try_into()?;
    let pk = PublicKey::from(peer.signer().public_key());
    let delegations = Direct::new(pk);
    let person = peer
        .using_storage(move |store| person::create(store, payload, delegations))
        .await??;

    let urn = person.urn();
    let owner = peer
        .using_storage(move |store| local::load(store, urn))
        .await??
        .ok_or_else(|| Error::PersonNotFound(person.urn()))?;

    set_default_owner(peer, owner.clone()).await?;

    Ok(owner)
}

/// Sets a new person payload for the default owner of this [`Peer`].
///
/// # Errors
///
///   * Fails to load the default owner
///   * Fails to verify `User`.
///   * Fails to set the default `rad/self` for this `PeerApi`.
#[allow(clippy::single_match_else)]
pub async fn update_owner_payload<S, P>(peer: &Peer<S>, payload: P) -> Result<(), Error>
where
    S: Clone + Signer,
    P: TryInto<PersonPayload> + Send,
    Error: From<P::Error>,
{
    let urn = default_owner(peer).await?.ok_or(Error::MissingOwner)?.urn();
    let payload = payload.try_into()?;
    peer.using_storage(move |store| person::update(store, &urn, None, payload, None))
        .await??;
    Ok(())
}

/// Given some hints as to where you might find it, get the urn of the project
/// found at `url`.
///
/// # Errors
///   * Could not successfully acquire a lock to the API.
///   * Could not open librad storage.
///   * Failed to clone the project.
///   * Failed to set the rad/self of this project.
pub async fn clone_project<S, Addrs>(
    peer: &Peer<S>,
    urn: Urn,
    remote_peer: PeerId,
    addr_hints: Addrs,
) -> Result<replication::Success, Error>
where
    S: Clone + Signer,
    Addrs: IntoIterator<Item = SocketAddr> + Send + 'static,
{
    let owner = default_owner(peer).await?.ok_or(Error::MissingOwner)?;
    let addr_hints = addr_hints.into_iter().collect::<Vec<_>>();
    Ok(peer
        .replicate((remote_peer, addr_hints), urn, Some(owner))
        .await?)
}

/// Get the project found at `urn`.
///
/// # Errors
///
///   * Resolving the project fails.
pub async fn get_project<S>(peer: &Peer<S>, urn: Urn) -> Result<Option<Project>, Error>
where
    S: Clone + Signer,
{
    peer.using_storage(move |store| identities::project::get(store, &urn))
        .await?
        .map_err(Error::from)
}

/// Returns the list of [`Project`]s for the local peer.
///
/// # Errors
///
///   * Retrieving the project entities from the store fails.
pub async fn list_projects<S>(peer: &Peer<S>) -> Result<Vec<Project>, Error>
where
    S: Clone + Signer,
{
    // FIXME(xla): Instead of implicitely expecting a presence of a default owner,
    // there either should be an explicit argument, or it's made impossible to
    // call this function without an owner associated with the state.
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
pub async fn load_refs<S>(peer: &Peer<S>, urn: Urn) -> Result<Option<Refs>, Error>
where
    S: Clone + Signer,
{
    peer.using_storage(move |store| Refs::load(store, &urn, None))
        .await?
        .map_err(Error::from)
}

/// Initialize a [`Project`] that is owned by the `owner`.
/// This kicks off the history of the project, tracked by `librad`'s mono-repo.
///
/// # Errors
///
/// Will error if:
///     * The signing of the project metadata fails.
///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
pub async fn init_project<S>(
    peer: &Peer<S>,
    owner: &LocalIdentity,
    create: crate::daemon::project::Create,
) -> Result<Project, Error>
where
    S: Clone + Signer,
{
    let default_branch = create.default_branch.to_string();
    let description = create.description.to_string();
    let name = create
        .repo
        .project_name()
        .map_err(crate::daemon::project::create::Error::from)?;
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
            let config = store.config()?;
            let signature = Signature {
                name: config.user_name()?,
                email: config.user_email()?,
            };
            let repository = create
                .validate(url, signature)
                .map_err(crate::daemon::project::create::Error::from)?;

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

    tracing::debug!(
        urn = ?project.urn(),
        name = ?project.subject().name,
        "created project",
    );

    let repo = spawn_blocking({
        let peer = peer.clone();
        let desc = project
            .subject()
            .description
            .as_deref()
            .cloned()
            .unwrap_or_default();
        move || {
            repository
                .setup_repo(settings(&peer), &desc)
                .map_err(crate::daemon::project::create::Error::from)
        }
    })
    .await??;
    let include_path = update_include(peer, project.urn()).await?;
    spawn_blocking(move || include::set_include_path(&repo, include_path)).await??;
    gossip::announce(peer, &project.urn(), None);

    Ok(project)
}

#[cfg(test)]
/// Create a [`LocalIdentity`] with the provided `handle`. This assumes that you
/// are creating a user that uses the secret key the `PeerApi` was configured
/// with.
///
/// # Errors
///
/// Will error if:
///     * The signing of the user metadata fails.
///     * The interaction with `librad` [`librad::git::storage::Storage`] fails.
pub async fn init_user<S>(peer: &Peer<S>, name: String) -> Result<LocalIdentity, Error>
where
    S: Clone + Signer,
{
    let pk = PublicKey::from(peer.signer().public_key());
    peer.using_storage(move |store| {
        let malkovich = person::create(
            store,
            payload::Person {
                name: Cstring::from(name),
            },
            Direct::new(pk),
        )?;

        Ok::<_, Error>(local::load(store, malkovich.urn())?)
    })
    .await??
    .ok_or(Error::IdentityCreationFailed)
}

#[cfg(test)]
/// Returns the list of [`Person`]s known for your peer.
///
/// # Errors
///
///   * Retrieval of the user entities from the store fails.
pub async fn list_users<S>(peer: &Peer<S>) -> Result<Vec<Person>, Error>
where
    S: Clone + Signer,
{
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

/// Wrapper around the storage track.
///
/// # Errors
///
/// * When the storage operation fails.
pub async fn track<S>(peer: &Peer<S>, urn: Urn, remote_peer: PeerId) -> Result<(), Error>
where
    S: Clone + Signer,
{
    {
        match peer
            .using_storage({
                let urn = urn.clone();
                move |store| {
                    tracking::track(
                        store,
                        &urn,
                        Some(remote_peer),
                        tracking::Config::default(),
                        tracking::policy::Track::MustNotExist,
                    )
                }
            })
            .await??
        {
            Ok(r) => {
                tracing::trace!(reference = %r.name, peer = %remote_peer, "successfully tracked peer");
            },
            Err(err) => tracing::trace!(err = %err, "tracking policy violated"),
        }
    }

    gossip::query(peer, &urn, Some(remote_peer));
    update_include(peer, urn).await?;
    Ok(())
}

/// Wrapper around the storage untrack.
///
/// # Errors
///
/// * When the storage operation fails.
pub async fn untrack<S>(peer: &Peer<S>, urn: Urn, remote_peer: PeerId) -> Result<bool, Error>
where
    S: Clone + Signer,
{
    let res = {
        let urn = urn.clone();
        peer.using_storage(move |store| {
            tracking::untrack(
                store,
                &urn,
                remote_peer,
                tracking::UntrackArgs::prune(tracking::policy::Untrack::Any),
            )
        })
        .await??
        .is_ok()
    };

    // Only need to update if we did untrack an existing peer
    if res {
        update_include(peer, urn).await?;
    }
    Ok(res)
}

/// Get the [`crate::daemon::project::Peer`]s that are tracking this project, including
/// their [`PeerId`].
///
/// # Errors
///
/// * If we could not acquire the lock
/// * If we could not open the storage
/// * If did not have the `urn` in storage
/// * If we could not fetch the tracked peers
/// * If we could not get the `rad/self` of the peer
pub async fn tracked<S>(
    peer: &Peer<S>,
    urn: Urn,
) -> Result<Vec<crate::daemon::project::Peer<peer::Status<Person>>>, Error>
where
    S: Clone + Signer,
{
    let project = get_project(peer, urn.clone())
        .await?
        .ok_or_else(|| Error::ProjectNotFound(urn.clone()))?;

    peer.using_storage(move |store| {
        let mut peers = vec![];

        for peer_id in tracking::tracked_peers(store, Some(&urn))? {
            let peer_id = peer_id?;
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

            peers.push(crate::daemon::project::Peer::Remote { peer_id, status });
        }

        Ok::<_, Error>(peers)
    })
    .await?
}

// TODO(xla): Account for projects not replicated but wanted.
/// Constructs the list of [`crate::daemon::project::Peer`] for the given `urn`. The
/// basis is the list of tracking peers of the project combined with the local
/// view.
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
pub async fn list_project_peers<S>(
    peer: &Peer<S>,
    urn: Urn,
) -> Result<Vec<crate::daemon::project::Peer<peer::Status<Person>>>, Error>
where
    S: Clone + Signer,
{
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
    peers.push(crate::daemon::project::Peer::Local {
        peer_id: peer.peer_id(),
        status,
    });

    let mut remotes = tracked(peer, urn).await?;

    peers.append(&mut remotes);

    Ok(peers)
}

/// Creates a working copy for the project of the given `urn`.
///
/// The `destination` is the directory where the caller wishes to place the
/// working copy.
///
/// The `peer_id` is from which peer we wish to base our checkout from.
///
/// # Errors
///
/// * if the project can't be found
/// * if the include file creation fails
/// * if the clone of the working copy fails
pub async fn checkout<S, P>(
    peer: &Peer<S>,
    urn: Urn,
    peer_id: P,
    destination: PathBuf,
) -> Result<PathBuf, Error>
where
    S: Clone + Signer,
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
    let checkout = crate::daemon::project::Checkout {
        urn: proj.urn(),
        name,
        default_branch,
        path: destination,
        include_path,
    };

    let ownership = match peer_id {
        None => crate::daemon::project::checkout::Ownership::Local(peer.peer_id()),
        Some(remote) => {
            let handle = {
                let rad_self =
                    Urn::try_from(Reference::rad_self(Namespace::from(urn.clone()), peer_id))
                        .expect("namespace is set");
                let person = peer
                    .using_storage(move |store| {
                        tracing::debug!(?rad_self, "cloning from peer");
                        person::get(store, &rad_self)?.ok_or(Error::PersonNotFound(rad_self))
                    })
                    .await??;

                person.subject().name.to_string()
            };

            crate::daemon::project::checkout::Ownership::Remote {
                handle,
                remote,
                local: peer.peer_id(),
            }
        },
    };

    let settings = settings(peer);
    let path = spawn_blocking(move || checkout.run(settings, ownership)).await??;

    Ok(path)
}

/// Prepare the include file for the given `project` with the latest tracked
/// peers.
///
/// # Errors
///
/// * if getting the list of tracked peers fails
pub async fn update_include<S>(peer: &Peer<S>, urn: Urn) -> Result<PathBuf, Error>
where
    S: Clone + Signer,
{
    let local_url = LocalUrl::from(urn.clone());
    let tracked = tracked(peer, urn).await?;
    let include = spawn_blocking({
        let path = paths(peer).git_includes_dir().to_path_buf();
        move || {
            let inc = Include::from_tracked_persons(
                path,
                local_url,
                tracked
                    .into_iter()
                    .filter_map(|peer| {
                        crate::daemon::project::Peer::replicated_remote(peer).map(|(p, u)| {
                            RefLike::try_from(u.subject().name.to_string()).map(|r| (r, p))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
            Ok::<_, Error>(inc)
        }
    })
    .await??;
    let include_path = include.file_path();
    tracing::debug!(path = ?include_path, "updaing include");
    include.save()?;

    Ok(include_path)
}

/// This method helps us get a branch for a given [`Urn`] and optional
/// [`PeerId`].
///
/// If the `branch_name` is `None` then we get the project for the given [`Urn`]
/// and use its `default_branch`.
///
/// # Errors
///   * If the storage operations fail.
///   * If the requested reference was not found.
pub async fn get_branch<S, P, B>(
    peer: &Peer<S>,
    urn: Urn,
    remote: P,
    branch_name: B,
) -> Result<Reference<Single>, Error>
where
    S: Clone + Signer,
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
///     * If the owner does not have this reference then it falls back to the first delegate.
///
/// # Errors
///   * If the storage operations fail.
///   * If no default branch was found for the provided [`Urn`].
pub async fn find_default_branch<S>(peer: &Peer<S>, urn: Urn) -> Result<Reference<Single>, Error>
where
    S: Clone + Signer,
{
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
        get_branch(peer, urn.clone(), None, default_branch.clone()),
        get_branch(peer, urn.clone(), remote, default_branch.clone())
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
pub fn monorepo<S>(peer: &Peer<S>) -> PathBuf
where
    S: Clone + Signer,
{
    peer.protocol_config().paths.git_dir().to_owned()
}

/// Returns the underlying [`paths::Paths`].
#[must_use]
pub fn paths<S>(peer: &Peer<S>) -> paths::Paths
where
    S: Clone + Signer,
{
    peer.protocol_config().paths.clone()
}

/// Construct the local [`transport::Settings`] for interacting with git related
/// I/O.
#[must_use]
pub fn settings<S>(peer: &Peer<S>) -> transport::Settings
where
    S: Clone + Signer,
{
    transport::Settings {
        paths: peer.protocol_config().paths.clone(),
        signer: BoxedSigner::from(SomeSigner {
            signer: peer.signer().clone(),
        }),
    }
}

/// Returns the list of [`SomeIdentity`]s for the local peer.
///
/// # Errors
///
///   * Retrieving the project entities from the store fails.
pub async fn list_identities<S>(peer: &Peer<S>) -> Result<Vec<SomeIdentity>, Error>
where
    S: Clone + Signer,
{
    peer.using_storage(move |store| {
        let identities = identities::any::list(store)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Ok(identities)
    })
    .await?
}

/// Determine the [`peer::Role`] for a given [`Project`] and [`PeerId`].
///
/// If `peer` is `Either::Left` then we have the local `PeerId` and we can
/// ignore it for looking at `rad/signed_refs`.
///
/// If `peer` is `Either::Right` then it is a remote peer and we use it for
/// looking at `rad/signed_refs`.
fn role(
    store: &librad::git::storage::Storage,
    project: &Project,
    peer: Either<PeerId, PeerId>,
) -> Result<peer::Role, Error> {
    let role = if project
        .delegations()
        .owner(peer.into_inner().as_public_key())
        .is_some()
    {
        peer::Role::Delegate
    } else if Refs::load(store, &project.urn(), peer.right())?
        .map_or(false, |refs| refs.heads().next().is_some())
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
    use crate::daemon::{config, identities::payload::HasNamespace, project};
    use lazy_static::lazy_static;
    use librad::{
        crypto::BoxedSigner, git_ext::OneLevel, identities::payload::Person, net, reflike,
        SecretKey,
    };
    use serde::{Deserialize, Serialize};
    use std::{env, path::PathBuf};
    use url::Url;

    #[derive(Deserialize, Serialize)]
    struct TestExt(String);

    lazy_static! {
        static ref NAMESPACE: Url = "https://radicle.xyz/test"
            .parse()
            .expect("Static URL malformed");
    }

    impl HasNamespace for TestExt {
        fn namespace() -> &'static Url {
            &NAMESPACE
        }
    }

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

    #[tokio::test(flavor = "multi_thread")]
    async fn can_create_user() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

        let annie = super::init_user(&peer, "annie_are_you_ok?".to_string()).await;
        assert!(annie.is_ok());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn can_init_owner() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;
        let payload = super::PersonPayload::new(Person {
            name: "cloudhead".into(),
        })
        .with_ext(TestExt("test".to_string()))?;

        super::init_owner(&peer, payload).await?;

        peer.using_storage(|storage| {
            assert_eq!(
                storage.config()?.user_name()?,
                "cloudhead",
                "Invalid config user name"
            );
            Ok::<_, super::Error>(())
        })
        .await??;
        let owner = super::default_owner(&peer).await?.expect("No owner set");
        assert_eq!(*owner.subject().name, "cloudhead", "Invalid owner name");
        let ext: TestExt = owner.payload().get_ext()?.expect("No owner extension");
        assert_eq!(ext.0, "test", "Invalid owner extension");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn can_update_owner_payload() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;
        let payload = super::PersonPayload::new(Person {
            name: "cloudhead".into(),
        })
        .with_ext(TestExt("test".to_string()))?;
        super::init_owner(&peer, payload).await?;
        let new_payload = super::PersonPayload::new(Person {
            name: "cloudhead_next".into(),
        })
        .with_ext(TestExt("test_next".to_string()))?;

        super::update_owner_payload(&peer, new_payload).await?;

        let owner = super::default_owner(&peer).await?.expect("No owner set");
        assert_eq!(
            *owner.subject().name,
            "cloudhead_next",
            "Invalid owner name"
        );
        let ext: TestExt = owner.payload().get_ext()?.expect("No owner extension");
        assert_eq!(ext.0, "test_next", "Invalid owner extension");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn can_create_project() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        env::set_var("RAD_HOME", tmp_dir.path());
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

        let user = super::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;
        let project =
            super::init_project(&peer, &user, radicle_project(tmp_dir.path().to_path_buf())).await;

        assert!(project.is_ok());
        assert!(tmp_dir.path().join("radicalise").exists());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn can_create_project_for_existing_repo() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let repo_path = tmp_dir.path().join("radicle");
        let repo_path = repo_path.join("radicalise");
        std::fs::create_dir_all(repo_path.clone()).expect("failed to create directory path");
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

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

    #[tokio::test(flavor = "multi_thread")]
    async fn list_projects() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");

        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

        let user = super::init_owner(
            &peer,
            Person {
                name: "cloudhead".into(),
            },
        )
        .await?;

        for fixture in fixtures(tmp_dir.path().to_path_buf()) {
            super::init_project(&peer, &user, fixture).await?;
        }

        let kalt = super::init_user(&peer, "kalt".to_string()).await?;
        let fakie =
            super::init_project(&peer, &kalt, fakie_project(tmp_dir.path().to_path_buf())).await?;

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

    #[tokio::test(flavor = "multi_thread")]
    async fn list_users() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::tempdir().expect("failed to create temdir");
        let key = SecretKey::new();
        let signer = BoxedSigner::from(key.clone());
        let config = config::default(signer.clone(), tmp_dir.path())?;
        let peer = net::peer::Peer::new(config)?;

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
