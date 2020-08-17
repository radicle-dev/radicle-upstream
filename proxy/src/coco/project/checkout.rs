use std::ffi;
use std::io;
use std::path::{self, PathBuf};
use std::process::Command;

use librad::git::local::url::LocalUrl;
use librad::peer::PeerId;

use crate::coco::Project;

/// When checking out a working copy, we can run into several I/O failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The clone process failed.
    #[error("failed to checkout the project")]
    Failed,
    /// We were trying to create the PATH value but it failed.
    #[error(transparent)]
    BinPath(#[from] super::bin_path::Error),
    /// An I/O error occurred while setting up clone.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// The data necessary for checking out a project.
pub struct Checkout<P, ST>
where
    P: AsRef<path::Path>,
{
    /// The credential helper.
    credential: super::Credential,
    /// The project.
    project: Project<ST>,
    /// The path on the filesystem where we're going to checkout to.
    path: P,
    /// The `PATH` environment variable to be used for the checkout. It is safe to leave this
    /// `None` when executing the application for real. However, if we want to run an integration
    /// test we need to tell say where the `git-rad-remote` helper can be found.
    bin_path: Option<ffi::OsString>,
}

impl<P, ST> Checkout<P, ST>
where
    P: AsRef<path::Path>,
    ST: Clone,
{
    /// Create a new `Checkout` with the mock `Credential::Password` helper.
    pub fn new<Bin>(project: Project<ST>, path: P, bin_path: Bin) -> Self
    where
        Bin: Into<Option<ffi::OsString>>,
    {
        Self {
            // TODO(rudolfs): we'll have to figure out how to pass the secret
            // key to git in a safe manner. As it is now it could be sniffed
            // out from the process list while the user is doing a clone.
            //
            // How will we get ahold on the secret key here?
            credential: super::Credential::Password("radicle-upstream".to_owned()),
            project,
            path,
            bin_path: bin_path.into(),
        }
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
        let bin_path = match self.bin_path {
            Some(path) => Ok(path),
            None => super::default_bin_path(),
        }?;

        // Check if the path provided ends in the 'directory_name' provided. If not we create the
        // full path to that name.
        let path = &self.path.as_ref();
        let project_path = if let Some(destination) = path.components().next_back() {
            let destination: &std::ffi::OsStr = destination.as_ref();
            let project_name = self.project.name().to_string();
            let name: &std::ffi::OsStr = project_name.as_ref();
            if destination == name {
                path.to_path_buf()
            } else {
                path.join(name)
            }
        } else {
            path.join(&self.project.name().to_string())
        };

        let mut child_process = Command::new("git")
            .arg("-c")
            .arg(self.credential.to_helper())
            .arg("clone")
            .arg("--origin")
            .arg("rad")
            .arg("--branch")
            .arg(self.project.default_branch())
            .arg(LocalUrl::from_urn(self.project.urn(), peer_id).to_string())
            .arg(project_path.as_os_str())
            .env("PATH", &bin_path)
            .envs(std::env::vars().filter(|(key, _)| key.starts_with("GIT_TRACE")))
            .spawn()?;

        // TODO: Capture the error if any and respond
        let result = child_process.wait()?;

        if result.success() {
            Ok(project_path)
        } else {
            Err(Error::Failed)
        }
    }
}
