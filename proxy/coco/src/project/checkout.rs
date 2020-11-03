use std::{
    convert::TryFrom,
    ffi,
    marker::PhantomData,
    path::{self, PathBuf},
};

pub use librad::meta::project::Project;
use librad::{
    git::{
        include,
        local::url::LocalUrl,
        types::{remote::Remote, FlatRef, Force},
    },
    git_ext::{OneLevel, RefLike, RefspecPattern},
    peer::PeerId,
    uri::RadUrn,
};
use radicle_surf::vcs::git::git2;

use crate::config;

/// When checking out a working copy, we can run into several I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Git error when checking out the project.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An error occured building include files.
    #[error(transparent)]
    Include(#[from] include::Error),
}

/// The data necessary for checking out a project.
pub struct Checkout<P>
where
    P: AsRef<path::Path>,
{
    /// The URN identifier for the project we are checking out.
    pub urn: RadUrn,
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

impl Ownership {
    /// Clone a project based off of the `Ownership` value. See [`Checkout::run`] for more details.
    ///
    /// # Errors
    ///   * If the cloning of the working copy fails.
    ///   * In the case of a remote clone, if the pushing of the default branch fails.
    pub fn clone(
        self,
        urn: RadUrn,
        default_branch: &OneLevel,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        match self {
            Self::Local(peer_id) => {
                let url = LocalUrl::from_urn(urn, peer_id);
                Self::local(&url, path, builder)
            },
            Self::Remote {
                handle,
                remote,
                local,
            } => {
                let url = LocalUrl::from_urn(urn, local);
                Self::remote(&handle, remote, url, default_branch, path, builder)
            },
        }
    }

    /// See [`Checkout::run`].
    fn local(
        url: &LocalUrl,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        builder.remote_create(|repo, _remote_name, url| repo.remote(config::RAD_REMOTE, url));
        git2::build::RepoBuilder::clone(builder, &url.to_string(), path)
    }

    /// See [`Checkout::run`].
    fn remote(
        handle: &str,
        peer: PeerId,
        url: LocalUrl,
        default_branch: &OneLevel,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        let name =
            RefLike::try_from(format!("{}@{}", handle, peer)).expect("failed to parse remote name");
        {
            builder.remote_create(move |repo, _remote_name, url| {
                let mut remote = Remote::new(url, name.as_str().to_string());
                let heads: FlatRef<PeerId, _> = FlatRef::heads(PhantomData, peer).with_name(
                    RefspecPattern::try_from("heads/*").expect("'heads/*' failed to parse"),
                );
                let remotes: FlatRef<RefLike, _> = FlatRef::heads(PhantomData, name.clone());
                remote.fetch_spec = Some(remotes.refspec(heads, Force::True).boxed());
                remote.create(repo)
            });
        }

        let repo = git2::build::RepoBuilder::clone(builder, &url.to_string(), path)?;

        // Create a rad remote and push the default branch so we can set it as the
        // upstream.
        {
            let mut remote = Remote::rad_remote(url, None).create(&repo)?;
            remote.push(&[&format!("refs/heads/{}", default_branch.as_str())], None)?;
        }

        Ok(repo)
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
    pub fn run(self, ownership: Ownership) -> Result<PathBuf, Error> {
        // Check if the path provided ends in the 'directory_name' provided. If not we create the
        // full path to that name.
        let path = &self.path.as_ref();
        let project_path = if let Some(destination) = path.components().next_back() {
            let destination: &ffi::OsStr = destination.as_ref();
            let name: &ffi::OsStr = self.name.as_ref();
            if destination == name {
                path.to_path_buf()
            } else {
                path.join(name)
            }
        } else {
            path.join(&self.name)
        };

        // Clone the repository
        let mut builder = git2::build::RepoBuilder::new();
        builder.branch(self.default_branch.as_str());
        let repo = ownership.clone(self.urn, &self.default_branch, &project_path, &mut builder)?;

        // Set configurations
        super::set_rad_upstream(&repo, &self.default_branch)?;
        include::set_include_path(&repo, self.include_path)?;

        Ok(project_path)
    }
}
