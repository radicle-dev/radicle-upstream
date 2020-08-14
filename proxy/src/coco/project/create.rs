use std::ffi;
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::path::{self, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use librad::git::local::url::LocalUrl;
use librad::git::types::{remote::Remote, FlatRef, Force, NamespacedRef, Refspec};
use librad::keys;
use librad::meta::entity;
use librad::meta::project;
use librad::uri::RadUrn;
use radicle_surf::vcs::git::git2;

use crate::coco::User;

/// Errors that can occur during project creation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An existing project is being created, but we couldn't get the `name` of the project, i.e.
    /// the final suffix of the file path.
    #[error(
        "the existing path provided '{0}' was empty, and we could not get the project name from it"
    )]
    EmptyExistingPath(PathBuf),
    /// An error occurred when performing git operations.
    #[error(transparent)]
    Git(#[from] git2::Error),
    /// Repository already has a 'rad' remote.
    #[error("a remote named 'rad' already exists for the repository '{0}'")]
    RadRemoteExists(PathBuf),
    /// Configured default branch for the project is missing.
    #[error(
        "the default branch '{branch}' supplied was not found for the repository at '{repo_path}'"
    )]
    MissingDefaultBranch {
        /// The repository path we're setting up.
        repo_path: PathBuf,
        /// The default branch that was expected to be found.
        branch: String,
    },
    /// An error occurred setting up the project entity.
    #[error(transparent)]
    Entity(#[from] entity::Error),
    /// An error occurred when trying to push the default branch
    #[error("failed to push the refspec '{0}'")]
    PushFailed(String),
    #[error(transparent)]
    /// An I/O error occurred during push.
    Io(#[from] io::Error),
    /// TODO
    #[error(transparent)]
    Binpath(#[from] super::bin_path::Error),
}

/// The data required to either open an existing repository or create a new one.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Repo<Path> {
    /// Open an existing repository.
    Existing {
        /// The path to the existing project.
        path: Path,
    },
    /// Create a new project where the final directory path is `<path>/<name>`.
    New {
        /// The name of the project.
        name: String,
        /// The directory where we create the project.
        path: Path,
    },
}

impl<Path: AsRef<path::Path>> Repo<Path> {
    /// Get the project name based off of `path` or `path` + `name`.
    ///
    /// # Errors
    ///
    ///   * The existing path provided was empty, so we could not get the project's name.
    pub fn project_name(&self) -> Result<String, Error> {
        match self {
            Self::Existing { path } => path
                .as_ref()
                .components()
                .next_back()
                .and_then(|component| component.as_os_str().to_str())
                .map(ToString::to_string)
                .ok_or_else(|| Error::EmptyExistingPath(path.as_ref().to_path_buf())),
            Self::New { name, .. } => Ok(name.to_string()),
        }
    }

    /// If we pass `Existing`, we're opening a repository at the provided path.
    ///
    /// If we pass `New`, we're creating a repository in the provided directory path, where the new
    /// folder is called after `name`. We also write an initial commit to the repository to set it
    /// up for browsing.
    ///
    /// # Errors
    ///
    ///   * Failed to find the repository at the provided path.
    ///   * Failed to initialise the repository.
    pub fn create(
        &self,
        description: &str,
        default_branch: &str,
    ) -> Result<git2::Repository, git2::Error> {
        match &self {
            Self::Existing { .. } => git2::Repository::open(self.full_path()),
            Self::New { .. } => {
                let mut options = git2::RepositoryInitOptions::new();
                options.no_reinit(true);
                options.mkpath(true);
                options.description(description);
                options.initial_head(default_branch);

                let repo = git2::Repository::init_opts(self.full_path(), &options)?;
                // First use the config to initialize a commit signature for the user.
                let sig = repo.signature()?;
                // Now let's create an empty tree for this commit
                let tree_id = {
                    let mut index = repo.index()?;

                    // For our purposes, we'll leave the index empty for now.
                    index.write_tree()?
                };
                {
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

                Ok(repo)
            }
        }
    }

    /// Get the full path of the `Repo` creation data.
    fn full_path(&self) -> PathBuf {
        match self {
            Self::Existing { path } => path.as_ref().to_path_buf(),
            Self::New { name, path } => path.as_ref().join(name),
        }
    }
}

/// The data required for creating a new project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create<Path> {
    /// Description of the project we want to create.
    pub description: String,
    /// The default branch name for the project.
    pub default_branch: String,
    /// How the repository should be created or opened.
    pub repo: Repo<Path>,
}

impl<Path: AsRef<path::Path>> Create<Path> {
    /// Initialise the [`git2::Repository`] for the project found at `urn` in the `monorepo`.
    ///
    /// # Errors
    ///
    ///   * Failed to setup the repository
    ///   * Failed to build the project entity
    pub fn setup_repo(
        &self,
        urn: &RadUrn,
        bin_path: Option<ffi::OsString>,
    ) -> Result<git2::Repository, Error> {
        let repo = self.repo.create(&self.description, &self.default_branch)?;

        // Test if the repo has setup rad remote.
        match repo.find_remote(super::RAD_REMOTE) {
            Ok(_) => return Err(Error::RadRemoteExists(repo.path().to_path_buf())),
            Err(_err) => {
                Self::setup_remote(&repo, urn, &self.default_branch, bin_path)?;
            }
        }

        Ok(repo)
    }

    /// Build a [`project::Project`], where the provided [`User`] is the owner, and the set of keys
    /// starts with the provided [`keys::PublicKey`].
    ///
    /// # Errors
    ///
    ///   * Failed to build the project entity.
    pub fn build(
        &self,
        owner: &User,
        key: keys::PublicKey,
    ) -> Result<project::Project<entity::Draft>, Error> {
        let name = self.repo.project_name()?;
        let project = project::Project::<entity::Draft>::create(name, owner.urn())?
            .to_builder()
            .set_description(self.description.clone())
            .set_default_branch(self.default_branch.clone())
            .add_key(key)
            .add_certifier(owner.urn())
            .build()?;

        Ok(project)
    }

    /// Equips a repository with a rad remote for the given id. If the directory at the given path
    /// is not managed by git yet we initialise it first.
    fn setup_remote(
        repo: &git2::Repository,
        urn: &RadUrn,
        default_branch: &str,
        bin_path: Option<ffi::OsString>,
    ) -> Result<(), Error> {
        // TODO(finto): Need to check that Hash is the same for this repository. So basically we
        // initialise the remote or update it.

        if let Err(err) = repo.resolve_reference_from_short_name(default_branch) {
            log::error!("error while trying to find default branch: {:?}", err);
            return Err(Error::MissingDefaultBranch {
                repo_path: repo.path().to_path_buf(),
                branch: default_branch.to_string(),
            });
        }

        let working_copy_heads: FlatRef<String, _> = FlatRef::heads(PhantomData, None);
        let namespace_heads = NamespacedRef::heads(urn.id.clone(), None);
        let fetch = working_copy_heads
            .clone()
            .refspec(namespace_heads.clone(), Force::True);
        let push = namespace_heads.refspec(working_copy_heads, Force::True);

        let local_url: LocalUrl = urn.clone().into();
        let mut remote = Remote::rad_remote(local_url, fetch.into_dyn());
        remote.add_pushes(vec![push.into_dyn()].into_iter());
        let mut git_remote = remote.create(repo)?;

        /* TODO(finto): Pushing isn't working and is possibly failing silently.
         * When I inspect the monorepo the default branch isn't pushed.
         * This could be due to the remote helper needing credentials, which I attempted to fix
         * below, but no luck...
         */
        let default: FlatRef<String, _> = FlatRef::head(PhantomData, None, default_branch);
        // let namespace_default = NamespacedRef::head(urn.id.clone(), None, default_branch);
        git_remote.push(&[default.to_string()], None)?;

        Ok(())
    }
}

impl Create<PathBuf> {
    #[cfg(test)]
    #[must_use]
    pub fn into_existing(self) -> Self {
        Self {
            repo: Repo::Existing {
                path: self.repo.full_path(),
            },
            ..self
        }
    }
}

// TODO
fn push_spec<R, L>(
    repo: &git2::Repository,
    spec: &Refspec<R, L>,
    credential: super::Credential,
    bin_path: Option<ffi::OsString>,
) -> Result<(), Error>
where
    R: fmt::Display + ToString + Clone,
    L: fmt::Display + ToString + Clone,
{
    let bin_path = match bin_path {
        Some(path) => Ok(path),
        None => super::default_bin_path(),
    }?;

    let mut child_process = Command::new("git")
        .current_dir(repo.path())
        .arg("-c")
        .arg(credential.to_helper())
        .arg("push")
        .arg(super::RAD_REMOTE)
        //         .arg(format!("{}", repo.path().display()))
        .arg(spec.to_string())
        .env("PATH", &bin_path)
        .envs(std::env::vars().filter(|(key, _)| key.starts_with("GIT_TRACE")))
        .spawn()?;

    let result = child_process.wait()?;
    if result.success() {
        Ok(())
    } else {
        Err(Error::PushFailed(spec.to_string()))
    }
}
