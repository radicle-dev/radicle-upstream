use std::ffi;
use std::path::{self, PathBuf};

use librad::git::local::url::LocalUrl;
use librad::git::types::remote::Remote;
use librad::peer::PeerId;
use radicle_surf::vcs::git::git2;

use crate::coco::Project;

/// When checking out a working copy, we can run into several I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Git error when checking out the project.
    #[error(transparent)]
    Git(#[from] git2::Error),
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
}

impl<P, ST> Checkout<P, ST>
where
    P: AsRef<path::Path>,
    ST: Clone,
{
    /// Create a new `Checkout` with the mock `Credential::Password` helper.
    pub fn new(project: Project<ST>, path: P) -> Self {
        Self { project, path }
    }

    /// Checkout a working copy of a [`Project`].
    ///
    /// NOTE: `RAD_HOME` should be expected to be set if using a custom root for
    /// [`librad::paths::Paths`]. If it is not set the underlying binary will delegate to the
    /// `ProjectDirs` setup of the `Paths`.
    ///
    /// # Errors
    ///
    ///   * We couldn't resolve the executable path.
    ///   * The checkout process failed.
    pub fn run(self, peer_id: PeerId) -> Result<PathBuf, Error> {
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
        builder.remote_create(|repo, _, url| {
            let remote = Remote::rad_remote(url, None).create(repo)?;
            Ok(remote)
        });
        let _repo = git2::build::RepoBuilder::clone(
            &mut builder,
            &LocalUrl::from_urn(self.project.urn(), peer_id).to_string(),
            &project_path,
        )?;

        Ok(project_path)
    }
}
