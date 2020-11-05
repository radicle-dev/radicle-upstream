//! Validation logic for safely checking that a [`super::Repo`] is valid before setting up the
//! working copy.

use std::{io, marker::PhantomData, path::PathBuf};

use librad::{
    git::{
        local::url::LocalUrl,
        types::{remote::Remote, FlatRef},
    },
    git_ext::{self, OneLevel, RefLike},
    std_ext::result::ResultExt as _,
};
use radicle_surf::vcs::git::git2;

use crate::config;

/// Errors that occur when validating a [`super::Repo`]'s path.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The path already existed when trying to create a new project.
    #[error("the path provided '{0}' already exists")]
    AlreadExists(PathBuf),

    /// An existing project is being created, but we couldn't get the `name` of the project, i.e.
    /// the final suffix of the file path.
    #[error(
        "the existing path provided '{0}' was empty, and we could not get the project name from it"
    )]
    EmptyExistingPath(PathBuf),

    /// An error occurred in `git2` that we could not handle.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// When trying to inspect a path, and I/O error occurred.
    #[error(transparent)]
    Io(#[from] io::Error),

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

    /// The `rad` remote was found but it did not have a URL.
    #[error("the `rad` remote exists but is missing its url field")]
    MissingUrl,

    /// The path was expected to point to a git repository but it did not.
    #[error("the path '{0}' does not point to an existing repository")]
    NotARepo(PathBuf),

    /// The path was expected to exist already but does not.
    #[error("the path provided '{0}' does not exist when it was expected to")]
    PathDoesNotExist(PathBuf),

    /// The `rad` remote was found, but the URL did not match the URL we were expecting.
    #[error("the `rad` remote was found but the url field does not match the provided url, found: '{found}' expected: '{expected}'")]
    UrlMismatch {
        /// The expected URL of the `rad` remote.
        expected: String,
        /// The URL that was found for the `rad` remote.
        found: String,
    },
}

/// A `Repository` represents the validated information for setting up a working copy.
///
/// We can get a `Repository` by calling [`Repository::validate`].
pub enum Repository {
    /// The existing repository.
    Existing {
        /// Le [`git2::Repository`] that exists.
        repo: git2::Repository,
        /// The URL that will be used for the remote.
        url: LocalUrl,
        /// The default branch the repository should be set up with.
        default_branch: OneLevel,
    },
    /// A new repository will be created with using these fields.
    New {
        /// The path to the working copy.
        path: PathBuf,
        /// The name of the project.
        name: String,
        /// The URL that will be used for the remote.
        url: LocalUrl,
        /// The default branch the repository should be set up with.
        default_branch: OneLevel,
    },
}

impl Repository {
    /// Validate a [`super::Repo`] to construct a `Repository`.
    ///
    /// This ensures that when setting up a working copy, that there should be no errors.
    /// The following are validated for each case:
    ///
    /// **Existing**:
    ///   * The path provided should exist
    ///   * The path provided should have at least one components, which forms the name of the
    ///   project. E.g. `Developer/radicle-upstream` is the directory and `radicle-upstream` is the
    ///   project name.
    ///   * The path leads to a git repository
    ///   * The default branch passed exists in the repository
    ///   * If a `rad` remote exists, that it: a. Has a url b. If it does have a url, that it
    ///     matches the one provided here
    ///
    /// **New**:
    ///   * The path provided does not exist a. If it does exist, it should be a directory and it
    ///     should be empty
    ///
    /// # Errors
    ///
    /// If any of the criteria outlined above are violated, this will result in an [`Error`].
    pub fn validate(
        repo: super::Repo,
        url: LocalUrl,
        default_branch: OneLevel,
    ) -> Result<Self, Error> {
        match repo {
            super::Repo::Existing { path } => {
                if !path.exists() {
                    return Err(Error::PathDoesNotExist(path));
                }

                let _ = path
                    .components()
                    .next_back()
                    .and_then(|component| component.as_os_str().to_str())
                    .map(ToString::to_string)
                    .ok_or_else(|| Error::EmptyExistingPath(path.to_path_buf()))?;

                let repo = git2::Repository::open(path.clone())
                    .or_matches(git_ext::is_not_found_err, || Err(Error::NotARepo(path)))?;

                let _ = Self::existing_branch(&repo, &default_branch)?;
                let _ = Self::existing_remote(&repo, &url)?;
                Ok(Self::Existing {
                    repo,
                    url,
                    default_branch,
                })
            },
            super::Repo::New { name, path } => {
                let repo_path = path.join(name.clone());

                if repo_path.is_file() {
                    return Err(Error::AlreadExists(repo_path));
                }

                if repo_path.exists()
                    && repo_path.is_dir()
                    && repo_path.read_dir()?.next().is_some()
                {
                    return Err(Error::AlreadExists(repo_path));
                }

                Ok(Self::New {
                    name,
                    path,
                    url,
                    default_branch,
                })
            },
        }
    }

    /// Initialise the [`git2::Repository`] for the project found at `urn` in the `monorepo`.
    ///
    /// # Errors
    ///
    ///   * Failed to setup the repository
    ///   * Failed to build the project entity
    pub fn setup_repo(self, description: &str) -> Result<git2::Repository, Error> {
        match self {
            Self::Existing {
                repo,
                url,
                default_branch,
            } => {
                log::debug!(
                    "Setting up existing repository @ '{}'",
                    repo.path().display()
                );
                Self::setup_remote(&repo, url, &default_branch)?;
                Ok(repo)
            },
            Self::New {
                path,
                name,
                url,
                default_branch,
            } => {
                let path = path.join(name);
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
                Self::setup_remote(&repo, url, &default_branch)?;

                Ok(repo)
            },
        }
    }

    /// Equips a repository with a rad remote for the given id. If the directory at the given path
    /// is not managed by git yet we initialise it first.
    fn setup_remote(
        repo: &git2::Repository,
        url: LocalUrl,
        default_branch: &OneLevel,
    ) -> Result<(), Error> {
        let _ = Self::existing_branch(repo, default_branch)?;

        log::debug!("Creating rad remote");
        let mut git_remote = Self::existing_remote(repo, &url)?
            .map_or_else(|| Remote::rad_remote(url, None).create(repo), Ok)?;
        Self::push_default(&mut git_remote, default_branch)?;

        log::debug!("Setting upstream to default branch");
        crate::project::set_rad_upstream(repo, default_branch)?;

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

    fn existing_branch<'a>(
        repo: &'a git2::Repository,
        default_branch: &OneLevel,
    ) -> Result<git2::Reference<'a>, Error> {
        repo.resolve_reference_from_short_name(default_branch.as_str())
            .or_matches(git_ext::is_not_found_err, || {
                Err(Error::MissingDefaultBranch {
                    repo_path: repo.path().to_path_buf(),
                    branch: default_branch.as_str().to_string(),
                })
            })
    }

    fn existing_remote<'a>(
        repo: &'a git2::Repository,
        url: &LocalUrl,
    ) -> Result<Option<git2::Remote<'a>>, Error> {
        match repo.find_remote(config::RAD_REMOTE) {
            Err(err) if git_ext::is_not_found_err(&err) => Ok(None),
            Err(err) => Err(err.into()),
            Ok(remote) => match remote.url() {
                None => Err(Error::MissingUrl),
                Some(remote_url) if remote_url != url.to_string() => Err(Error::UrlMismatch {
                    expected: url.to_string(),
                    found: remote_url.to_string(),
                }),
                Some(_) => Ok(Some(remote)),
            },
        }
    }
}
