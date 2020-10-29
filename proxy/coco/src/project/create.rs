use std::{marker::PhantomData, path::PathBuf};

use serde::{Deserialize, Serialize};

use librad::{
    git::{
        ext::{OneLevel, RefLike},
        local::url::LocalUrl,
        types::{remote::Remote, FlatRef},
    },
    keys,
    meta::{entity, project},
};
use radicle_surf::vcs::git::git2;

use crate::{config, user::User};

/// Validation Errors
pub mod validation {
    use std::path::PathBuf;

    /// Errors that occur when validating a [`super::Repo`]'s path.
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        /// The path already existed when trying to create a new project.
        #[error("the path provided '{0}' already exists")]
        AlreadExists(PathBuf),

        /// The path was expected to exist already but does not.
        #[error("the path provided '{0}' does not exist when it was expected to")]
        PathDoesNotExist(PathBuf),

        /// The path was expected to point to a git repository but it did not.
        #[error("the path '{0}' does not point to an existing repository")]
        NotARepo(PathBuf),
    }
}

/// Errors that occur when attempting to create a working copy of a project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An existing project is being created, but we couldn't get the `name` of the project, i.e.
    /// the final suffix of the file path.
    #[error(
        "the existing path provided '{0}' was empty, and we could not get the project name from it"
    )]
    EmptyExistingPath(PathBuf),

    /// Internal git error while trying to create the project.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

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
}

/// The data required to either open an existing repository or create a new one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Repo {
    /// Open an existing repository.
    Existing {
        /// The path to the existing project.
        path: PathBuf,
    },
    /// Create a new project where the final directory path is `<path>/<name>`.
    New {
        /// The name of the project.
        name: String,
        /// The directory where we create the project.
        path: PathBuf,
    },
}

impl Repo {
    /// Get the project name based off of `path` or `path` + `name`.
    ///
    /// # Errors
    ///
    ///   * The existing path provided was empty, so we could not get the project's name.
    pub fn project_name(&self) -> Result<String, Error> {
        match self {
            Self::Existing { path } => path
                .components()
                .next_back()
                .and_then(|component| component.as_os_str().to_str())
                .map(ToString::to_string)
                .ok_or_else(|| Error::EmptyExistingPath(path.to_path_buf())),
            Self::New { name, .. } => Ok(name.to_string()),
        }
    }

    /// Validate `Repo` into a [`ValidatedRepo`], ensuring that path passes certain criteria. These
    /// criteria are documented in the **Errors** section.
    ///
    /// # Errors
    ///
    ///   * If we have `Repo::Existing` and the path does not exist
    ///   * If we have `Repo::Existing` and the path does not point to a repository
    ///   * If we have `Repo::New` and the path already exists
    pub fn validate(self) -> Result<ValidatedRepo, validation::Error> {
        match self {
            Self::Existing { path } => {
                if !path.exists() {
                    return Err(validation::Error::PathDoesNotExist(path));
                }

                // TODO(finto): This may be too permissive. Need to look into git errors.
                if git2::Repository::open(path.clone()).is_err() {
                    return Err(validation::Error::NotARepo(path));
                }

                Ok(ValidatedRepo(Self::Existing { path }))
            },
            Self::New { name, path } => {
                let repo_path = path.join(name.clone());
                if repo_path.exists() {
                    return Err(validation::Error::AlreadExists(repo_path));
                }

                Ok(ValidatedRepo(Self::New { name, path }))
            },
        }
    }

    /// Get the full path of the `Repo` creation data.
    fn full_path(&self) -> PathBuf {
        match self {
            Self::Existing { path } => path.to_path_buf(),
            Self::New { name, path } => path.join(name),
        }
    }
}

/// A `Repo` that has passed through validation, using [`Repo::validate`].
pub struct ValidatedRepo(Repo);

impl ValidatedRepo {
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
        default_branch: &OneLevel,
    ) -> Result<git2::Repository, Error> {
        match &self {
            Self(Repo::Existing { .. }) => {
                let path = self.0.full_path();
                log::debug!("Setting up existing repository @ '{}'", path.display());
                git2::Repository::open(path).map_err(Error::from)
            },
            Self(Repo::New { .. }) => {
                let path = self.0.full_path();

                log::debug!("Setting up new repository @ '{}'", path.display());
                let mut options = git2::RepositoryInitOptions::new();
                options.no_reinit(true);
                options.mkpath(true);
                options.description(description);
                options.initial_head(default_branch.as_str());

                let repo = git2::Repository::init_opts(path, &options)?;
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
                        Some(&format!("refs/heads/{}", default_branch.as_str())),
                        &sig,
                        &sig,
                        "Initial commit",
                        &tree,
                        &[],
                    )?;
                }

                Ok(repo)
            },
        }
    }
}

/// The data required for creating a new project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create<R> {
    /// Description of the project we want to create.
    pub description: String,
    /// The default branch name for the project.
    pub default_branch: OneLevel,
    /// The [`Repo`] to create with. It's left as a generic parameter so that we can have
    /// `Create<Repo>` and `Create<ValidatedRepo>` for that sweet type safety.
    pub repo: R,
}

impl Create<Repo> {
    #![allow(clippy::use_self)]
    /// Validate `Create<Repo>` into a `Create<ValidatedRepo>`. This ensures that we have valid
    /// paths when we attempt to create the working copy.
    ///
    /// It needs to be called before using [`Create::setup_repo`].
    ///
    /// # Errors
    ///
    /// See [`Repo::validate`]
    pub fn validate(self) -> Result<Create<ValidatedRepo>, validation::Error> {
        Ok(Create {
            description: self.description,
            default_branch: self.default_branch,
            repo: self.repo.validate()?,
        })
    }
}

impl Create<ValidatedRepo> {
    /// Initialise the [`git2::Repository`] for the project found at `urn` in the `monorepo`.
    ///
    /// # Errors
    ///
    ///   * Failed to setup the repository
    ///   * Failed to build the project entity
    pub fn setup_repo(&self, url: LocalUrl) -> Result<git2::Repository, Error> {
        let repo = self.repo.create(&self.description, &self.default_branch)?;

        // Test if the repo has setup rad remote.
        match repo.find_remote(config::RAD_REMOTE) {
            Ok(mut remote) => {
                // Send a warning if the remote urls don't match
                if let Some(remote_url) = remote.url() {
                    if remote_url == url.to_string() {
                        Self::push_default(&mut remote, &self.default_branch)?;
                    } else {
                        log::warn!("Remote URL Mismatch: '{}' /= '{}'", remote_url, url);
                        log::warn!("Deleting original '{}' remote", config::RAD_REMOTE);
                        repo.remote_delete(config::RAD_REMOTE)?;
                        Self::setup_remote(&repo, url, &self.default_branch)?;
                    }
                };
            },
            Err(_err) => {
                Self::setup_remote(&repo, url, &self.default_branch)?;
            },
        }

        Ok(repo)
    }

    /// Build a [`project::Project`], where the provided [`User`] is the owner, and the set of
    /// keys starts with the provided [`keys::PublicKey`].
    ///
    /// # Errors
    ///
    ///   * Failed to build the project entity.
    pub fn build(
        &self,
        owner: &User,
        key: keys::PublicKey,
    ) -> Result<project::Project<entity::Draft>, Error> {
        let name = self.repo.0.project_name()?;
        let project = project::Project::<entity::Draft>::create(name, owner.urn())?
            .to_builder()
            .set_description(self.description.clone())
            .set_default_branch(self.default_branch.as_str().to_string())
            .add_key(key)
            .add_certifier(owner.urn())
            .build()?;

        Ok(project)
    }

    /// Equips a repository with a rad remote for the given id. If the directory at the given path
    /// is not managed by git yet we initialise it first.
    fn setup_remote(
        repo: &git2::Repository,
        url: LocalUrl,
        default_branch: &OneLevel,
    ) -> Result<(), Error> {
        if let Err(err) = repo.resolve_reference_from_short_name(default_branch.as_str()) {
            log::error!("error while trying to find default branch: {:?}", err);
            return Err(Error::MissingDefaultBranch {
                repo_path: repo.path().to_path_buf(),
                branch: default_branch.as_str().to_string(),
            });
        }

        log::debug!("Creating rad remote");
        let remote = Remote::rad_remote(url, None);
        let mut git_remote = remote.create(repo)?;

        Self::push_default(&mut git_remote, default_branch)?;

        log::debug!("Setting upstream to default branch");
        super::set_rad_upstream(repo, default_branch)?;

        Ok(())
    }

    /// Push the default branch to the provided remote.
    fn push_default(remote: &mut git2::Remote, default_branch: &OneLevel) -> Result<(), Error> {
        let default: FlatRef<RefLike, _> =
            FlatRef::head(PhantomData, None, default_branch.clone().into());
        log::debug!("Pushing default branch '{}'", default);
        remote.push(&[default.to_string()], None)?;
        Ok(())
    }
}

// Clippy is stupid and doesn't realise the `Create`s here are different types than `Self`.
#[allow(clippy::use_self)]
impl Create<Repo> {
    /// Transforms into an existing project.
    #[must_use]
    pub fn into_existing(self) -> Create<Repo> {
        let path = self.repo.full_path();
        Create {
            repo: Repo::Existing { path },
            description: self.description,
            default_branch: self.default_branch,
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom as _;

    use assert_matches::assert_matches;

    use librad::git::ext::{OneLevel, RefLike};

    use super::*;

    #[test]
    fn validation_fails_on_existing_directory() -> Result<(), Box<dyn std::error::Error>> {
        let tmpdir = tempfile::tempdir().expect("failed to create tmp dir");
        let exists = tmpdir.path().join("exists");
        std::fs::create_dir(exists)?;

        let create = Create {
            description: "Radicle".to_string(),
            default_branch: OneLevel::from(
                RefLike::try_from("radicle").expect("failed to parse ref"),
            ),
            repo: Repo::New {
                name: "exists".to_string(),
                path: tmpdir.path().to_path_buf(),
            },
        };
        assert_matches!(
            create.validate().err(),
            Some(validation::Error::AlreadExists(_))
        );

        Ok(())
    }
}
