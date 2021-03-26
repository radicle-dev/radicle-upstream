use std::{
    convert::TryFrom,
    ffi, io,
    path::{self, PathBuf},
};

use librad::{
    git::{
        include,
        local::{transport::CanOpenStorage, url::LocalUrl},
        types::{
            remote::{LocalFetchspec, LocalPushspec, Remote},
            Flat, Force, GenericRef, Reference, Refspec,
        },
        Urn,
    },
    git_ext::{OneLevel, Qualified, RefLike},
    peer::PeerId,
    reflike, refspec_pattern,
};
use radicle_surf::vcs::git::git2;

/// When checking out a working copy, we can run into several I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The path already existed when trying to checkout the project.
    #[error("the path provided '{0}' already exists")]
    AlreadExists(PathBuf),

    /// Git error when checking out the project.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An error occurred building include files.
    #[error(transparent)]
    Include(#[from] include::Error),

    /// An error occurred validating the project path
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error occurred when attempting to strip a prefix from a reference.
    #[error(transparent)]
    Prefix(#[from] radicle_git_ext::name::StripPrefixError),

    /// An error occurred in the local transport.
    #[error(transparent)]
    Transport(#[from] librad::git::local::transport::Error),
}

/// The data necessary for checking out a project.
pub struct Checkout<P>
where
    P: AsRef<path::Path>,
{
    /// The URN identifier for the project we are checking out.
    pub urn: Urn,
    /// The name of the project.
    pub name: String,
    /// The default branch of the project.
    pub default_branch: OneLevel,
    /// The path on the filesystem where we're going to checkout to.
    pub path: P,
    /// Absolute path of the include file that will be set in the working copy config.
    pub include_path: PathBuf,
}

/// We want to know whether we're checking out from one of our own copies, or if we're checking out
/// based off of a remote's branch.
pub enum Ownership {
    /// We're checking out our own copy of the project.
    Local(PeerId),
    /// We're checking out a remote's version of the project.
    Remote {
        /// The handle of the remote peer gives themselves via their user profile. For example,
        /// `90s-kid` -- the name of the remote will then be `90s-kid@<urn.id>`.
        handle: String,
        /// The `PeerId` of the remote.
        remote: PeerId,
        /// Our own `PeerId`.
        local: PeerId,
    },
}

/// Clone a git repository to the `path` location, based off of the `remote` provided.
///
/// # Errors
///   * if initialisation of the repository fails
///   * if branch or remote manipulation fails
pub fn clone<F>(
    path: &path::Path,
    storage: F,
    mut remote: Remote<LocalUrl>,
) -> Result<(git2::Repository, Remote<LocalUrl>), Error>
where
    F: CanOpenStorage + 'static,
{
    let repo = git2::Repository::init(path)?;
    remote.save(&repo)?;
    for (reference, oid) in remote.fetch(storage, &repo, LocalFetchspec::Configured)? {
        let msg = format!("Fetched `{}->{}`", reference, oid);
        log::debug!("{}", msg);

        // FIXME(finto): we should ignore refs that don't start with heads to avoid unintended
        // side-effects.
        let branch: RefLike = OneLevel::from(reference).into();
        let branch = branch.strip_prefix(remote.name.clone())?;
        let branch = branch.strip_prefix(reflike!("heads")).unwrap_or(branch);
        let _remote_branch = repo.reference(
            reflike!("refs/remotes")
                .join(remote.name.clone())
                .join(branch.clone())
                .as_str(),
            oid,
            true,
            &msg,
        )?;
        let _local_branch = repo.reference(Qualified::from(branch).as_str(), oid, true, &msg);
    }

    Ok((repo, remote))
}

impl Ownership {
    /// Clone a project based off of the `Ownership` value. See [`Checkout::run`] for more details.
    ///
    /// # Errors
    ///   * If the cloning of the working copy fails.
    ///   * In the case of a remote clone, if the pushing of the default branch fails.
    pub fn clone<F>(
        self,
        open_storage: F,
        urn: Urn,
        default_branch: &OneLevel,
        path: &path::Path,
    ) -> Result<(git2::Repository, Remote<LocalUrl>), Error>
    where
        F: CanOpenStorage + Clone + 'static,
    {
        match self {
            Self::Local(_peer_id) => {
                let url = LocalUrl::from(urn);
                Self::local(open_storage, url, path).map_err(Error::from)
            },
            Self::Remote { handle, remote, .. } => {
                let url = LocalUrl::from(urn);
                Self::remote(open_storage, &handle, remote, url, default_branch, path)
            },
        }
    }

    /// See [`Checkout::run`].
    fn local<F>(
        open_storage: F,
        url: LocalUrl,
        path: &path::Path,
    ) -> Result<(git2::Repository, Remote<LocalUrl>), Error>
    where
        F: CanOpenStorage + 'static,
    {
        let rad = Remote::rad_remote(
            url,
            Refspec {
                src: refspec_pattern!("refs/heads/*"),
                dst: refspec_pattern!("refs/remotes/rad/*"),
                force: Force::True,
            },
        );
        clone(path, open_storage, rad)
    }

    /// See [`Checkout::run`].
    fn remote<F>(
        open_storage: F,
        handle: &str,
        peer: PeerId,
        url: LocalUrl,
        default_branch: &OneLevel,
        path: &path::Path,
    ) -> Result<(git2::Repository, Remote<LocalUrl>), Error>
    where
        F: CanOpenStorage + Clone + 'static,
    {
        let name =
            RefLike::try_from(format!("{}@{}", handle, peer)).expect("failed to parse remote name");

        let remote = Remote::new(url.clone(), name.clone()).with_fetchspecs(vec![Refspec {
            src: Reference::heads(Flat, peer),
            dst: GenericRef::heads(Flat, name),
            force: Force::True,
        }]);

        let (repo, _) = clone(path, open_storage.clone(), remote)?;

        // Create a rad remote and push the default branch so we can set it as the
        // upstream.
        let rad = {
            // Create a fetchspec `refs/heads/*:refs/remotes/rad/*`
            let fetchspec = Refspec {
                src: GenericRef::<_, RefLike, _>::heads(Flat, None),
                dst: refspec_pattern!("refs/remotes/rad/*"),
                force: Force::True,
            };
            let mut rad = Remote::rad_remote(url, fetchspec);
            rad.save(&repo)?;
            let _ = rad.push(
                open_storage,
                &repo,
                LocalPushspec::Matching {
                    pattern: Qualified::from(default_branch.clone()).into(),
                    force: Force::False,
                },
            )?;
            rad
        };

        Ok((repo, rad))
    }
}

impl<P> Checkout<P>
where
    P: AsRef<path::Path>,
{
    /// Based off of the `Ownership`, clone the project using the provided inputs.
    ///
    /// ## Local Clone
    ///
    /// If the `Ownership` is `Local` this means that we are cloning based off the user's own
    /// project and so the `url` used to clone will be built from the user's `PeerId`. The only
    /// remote that will be created is `rad` remote, pointing to the `url` built from the
    /// provided `urn` and the user's `PeerId`.
    ///
    /// ## Remote Clone
    ///
    /// If the `Ownership` is `Remote` this means that we are cloning based off of a peer's
    /// project.
    /// Due to this we need to point the remote to the specific remote in our project's hierarchy.
    /// What this means is that we need to set up a fetch refspec in the form of
    /// `refs/remotes/<peer_id>/heads/*` where the name of the remote is given by
    /// `<user_handle>@<peer_id>` -- this keeps in line with `librad::git::include`. To finalise
    /// the setup of the clone, we also want to add the `rad` remote, which is the designated
    /// remote the user pushes their own work to update their monorepo for this project.
    /// To do this, we create a `url` that is built using the provided `urn` and the user's `PeerId`
    /// and create the `rad` remote. Finally, we initialise the `default_branch` of the proejct --
    /// think upstream branch in git. We do this by pushing to the `rad` remote. This means that
    /// the working copy will be now setup where when we open it up we see the initial branch as
    /// being `default_branch`.
    ///
    /// To illustrate further, the `config` of the final repository will look similar to:
    ///
    /// ```text
    /// [remote "rad"]
    ///     url = rad://hyymr17h1fg5zk7duikgc7xoqonqorhwnxxs98kdb63f9etnsjxxmo@hwd1yrerzpjbmtshsqw6ajokqtqrwaswty6p7kfeer3yt1n76t46iqggzcr.git
    ///     fetch = +refs/heads/*:refs/remotes/rad/*
    /// [remote "banana@hyy36ey56mfayah398n7w4i8hy5ywci43hbyhwf1krfwonc1ur87ch"]
    ///     url = rad://hyymr17h1fg5zk7duikgc7xoqonqorhwnxxs98kdb63f9etnsjxxmo@hwd1yrerzpjbmtshsqw6ajokqtqrwaswty6p7kfeer3yt1n76t46iqggzcr.git
    ///     fetch = +refs/remotes/hyy36ey56mfayah398n7w4i8hy5ywci43hbyhwf1krfwonc1ur87ch/heads/*:refs/remotes/banana@hyy36ey56mfayah398n7w4i8hy5ywci43hbyhwf1krfwonc1ur87ch/*
    /// [branch "master"]
    ///     remote = rad
    ///     merge = refs/heads/master
    /// [include]
    ///     path = /home/user/.config/radicle/git-includes/hwd1yrerzpjbmtshsqw6ajokqtqrwaswty6p7kfeer3yt1n76t46iqggzcr.inc
    /// ```
    ///
    /// # Errors
    ///  * If the project cloning fails.
    ///  * If we cannot set the upstream branch for the `rad` remote.
    ///  * If we cannot set the include path for the working copy.
    pub fn run<F>(self, open_storage: F, ownership: Ownership) -> Result<PathBuf, Error>
    where
        F: CanOpenStorage + Clone + 'static,
    {
        // Check if the path provided ends in the 'directory_name' provided. If not we create the
        // full path to that name.
        let path = &self.path.as_ref();
        let project_path: PathBuf =
            path.components()
                .next_back()
                .map_or(path.join(&self.name), |destination| {
                    let destination: &ffi::OsStr = destination.as_ref();
                    let name: &ffi::OsStr = self.name.as_ref();
                    if destination == name {
                        path.to_path_buf()
                    } else {
                        path.join(name)
                    }
                });
        crate::project::ensure_directory(&project_path)?
            .ok_or_else(|| Error::AlreadExists(project_path.clone()))?;

        // Clone the repository
        let (repo, rad) =
            ownership.clone(open_storage, self.urn, &self.default_branch, &project_path)?;

        // Set configurations
        super::set_upstream(&repo, &rad, self.default_branch.clone())?;
        include::set_include_path(&repo, self.include_path)?;
        repo.set_head(Qualified::from(self.default_branch).as_str())?;
        repo.checkout_head(None)?;

        Ok(project_path)
    }
}
