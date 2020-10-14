use std::{
    ffi,
    path::{self, PathBuf},
};

pub use librad::meta::project::Project;
use librad::{
    git::{include, local::url::LocalUrl, types::remote::Remote},
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
pub struct Checkout<P, ST>
where
    P: AsRef<path::Path>,
{
    /// The project.
    project: Project<ST>,
    /// The path on the filesystem where we're going to checkout to.
    path: P,
    /// Absolute path of the include file that will be set in the working copy config.
    include_path: PathBuf,
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
        default_branch: &str,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        match self {
            Self::Local(peer_id) => Self::local(peer_id, urn, path, builder),
            Self::Remote {
                handle,
                remote,
                local,
            } => Self::remote(&handle, remote, local, urn, default_branch, path, builder),
        }
    }

    /// See [`Checkout::run`].
    fn local(
        local: PeerId,
        urn: RadUrn,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        builder.remote_create(|repo, _remote_name, url| repo.remote(config::RAD_REMOTE, url));
        let url = LocalUrl::from_urn(urn, local).to_string();
        git2::build::RepoBuilder::clone(builder, &url, path)
    }

    /// See [`Checkout::run`].
    fn remote(
        handle: &str,
        remote: PeerId,
        local: PeerId,
        urn: RadUrn,
        default_branch: &str,
        path: &path::Path,
        builder: &mut git2::build::RepoBuilder,
    ) -> Result<git2::Repository, git2::Error> {
        let name = format!("{}@{}", handle, remote);
        builder.remote_create(move |repo, _remote_name, url| {
            Remote::new(url, name.clone()).create(repo)
        });
        let remote_url = LocalUrl::from_urn(urn.clone(), remote).to_string();
        let repo = git2::build::RepoBuilder::clone(builder, &remote_url, path)?;

        // Create a rad remote and push the default branch so we can set it as the
        // upstream.
        {
            let local_url = LocalUrl::from_urn(urn, local);
            let mut remote = Remote::rad_remote(local_url, None).create(&repo)?;
            remote.push(&[&format!("refs/heads/{}", default_branch)], None)?;
        }

        Ok(repo)
    }
}

impl<P, ST> Checkout<P, ST>
where
    P: AsRef<path::Path>,
    ST: Clone,
{
    /// Create a new `Checkout` with the mock `Credential::Password` helper.
    pub fn new(project: Project<ST>, path: P, include_path: PathBuf) -> Self {
        Self {
            project,
            path,
            include_path,
        }
    }

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
    /// project. The `url` to clone from is made up of the provided `urn` and the remote's
    /// `PeerId`.
    /// Due to the semantics of cloning in `git`, this means that the remote that is
    /// created points to this `url`. We keep with the format of `librad::git::include` for the
    /// name of the remote as `<peer_handle>@<peer_id>`.
    /// To finalise the setup of the clone, we also want to add the `rad` remote, which is the
    /// designated remote the user pushes their own work to update their monorepo for this project.
    /// To do this, we create `url` that is built using the provided `urn` and the user's `PeerId`
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
    ///     url = rad://hyy36ey56mfayah398n7w4i8hy5ywci43hbyhwf1krfwonc1ur87ch@hwd1yrerzpjbmtshsqw6ajokqtqrwaswty6p7kfeer3yt1n76t46iqggzcr.git
    ///     fetch = +refs/heads/*:refs/remotes/banana@hyy36ey56mfayah398n7w4i8hy5ywci43hbyhwf1krfwonc1ur87ch/*
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
            let project_name = self.project.name().to_string();
            let name: &ffi::OsStr = project_name.as_ref();
            if destination == name {
                path.to_path_buf()
            } else {
                path.join(name)
            }
        } else {
            path.join(&self.project.name().to_string())
        };

        // Clone the repository
        let mut builder = git2::build::RepoBuilder::new();
        builder.branch(self.project.default_branch());
        let repo = ownership.clone(
            self.project.urn(),
            self.project.default_branch(),
            &project_path,
            &mut builder,
        )?;

        // Set configurations
        super::set_rad_upstream(&repo, self.project.default_branch())?;
        include::set_include_path(&repo, self.include_path)?;

        Ok(project_path)
    }
}
