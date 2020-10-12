use std::{
    ffi,
    path::{self, PathBuf},
};

pub use librad::meta::project::Project;
use librad::{
    git::{include, local::url::LocalUrl, types::remote::Remote},
    peer::PeerId,
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

    /// Checkout a working copy of a [`Project`].
    ///
    /// # Errors
    ///
    ///   * The checkout process failed.
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

        let mut builder = git2::build::RepoBuilder::new();
        builder.branch(self.project.default_branch());

        let repo = match ownership {
            Ownership::Local(local) => {
                builder
                    .remote_create(|repo, _remote_name, url| repo.remote(config::RAD_REMOTE, url));
                let url = LocalUrl::from_urn(self.project.urn(), local).to_string();
                git2::build::RepoBuilder::clone(&mut builder, &url, &project_path)
            },
            Ownership::Remote {
                handle,
                remote,
                local,
            } => {
                let name = format!("{}@{}", handle, remote);
                builder.remote_create(move |repo, _remote_name, url| {
                    Remote::new(url, name.clone()).create(repo)
                });
                let remote_url = LocalUrl::from_urn(self.project.urn(), remote).to_string();
                let repo =
                    git2::build::RepoBuilder::clone(&mut builder, &remote_url, &project_path)?;

                // Create a rad remote and push the default branch so we can set it as the
                // upstream.
                {
                    let local_url = LocalUrl::from_urn(self.project.urn(), local);
                    let mut remote = Remote::rad_remote(local_url, None).create(&repo)?;
                    remote.push(
                        &[&format!("refs/heads/{}", self.project.default_branch())],
                        None,
                    )?;
                }

                Ok(repo)
            },
        }?;

        super::set_rad_upstream(&repo, self.project.default_branch())?;
        include::set_include_path(&repo, self.include_path)?;
        Ok(project_path)
    }
}
