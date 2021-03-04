//! Validation logic for safely checking that a [`super::Repo`] is valid before setting up the
//! working copy.

use std::{convert::TryFrom, io, path::PathBuf};

use librad::{
    git::{
        local::{transport::CanOpenStorage, url::LocalUrl},
        types::{
            remote::{self, LocalPushspec, Remote},
            Fetchspec, Force, Refspec,
        },
    },
    git_ext::{self, OneLevel},
    reflike, refspec_pattern,
    std_ext::result::ResultExt as _,
};
use nonempty::NonEmpty;
use radicle_surf::vcs::git::git2;

const USER_NAME: &str = "user.name";
const USER_EMAIL: &str = "user.email";

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

    /// When trying to inspect a path, an I/O error occurred.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// When checking the default git config for `user.email` we could not find it.
    #[error("the author email for creating the project could not be found - have you configured your git config?")]
    MissingAuthorEmail,

    /// When checking the default git config for `user.name` we could not find it.
    #[error("the author name for creating the project could not be found - have you configured your git config?")]
    MissingAuthorName,

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

    /// When checking for default git config we could not find it.
    #[error(
        "the git config for creating the project could not be found - have you configured it?"
    )]
    MissingGitConfig,

    /// The `rad` remote was found but it did not have a URL.
    #[error("the `rad` remote exists but is missing its url field")]
    MissingUrl,

    /// The path was expected to point to a git repository but it did not.
    #[error("the path '{0}' does not point to an existing repository")]
    NotARepo(PathBuf),

    /// The path was expected to exist already but does not.
    #[error("the path provided '{0}' does not exist when it was expected to")]
    PathDoesNotExist(PathBuf),

    /// When attempting to find a particular remote that _should_ exist, it did not.
    #[error(transparent)]
    Remote(#[from] remote::FindError),

    /// An internal error occurred when talking to the local transport for git related I/O.
    #[error(transparent)]
    Transport(#[from] librad::git::local::transport::Error),

    /// The `rad` remote was found, but the URL did not match the URL we were expecting.
    #[error("the `rad` remote was found but the url field does not match the provided url, found: '{found}' expected: '{expected}'")]
    UrlMismatch {
        /// The expected URL of the `rad` remote.
        expected: String,
        /// The URL that was found for the `rad` remote.
        found: String,
    },
}

/// The signature of a git author. Used internally to convert into a `git2::Signature`, which
/// _cannot_ be shared between threads.
#[derive(Debug)]
pub struct Signature {
    name: String,
    email: String,
}

impl TryFrom<Signature> for git2::Signature<'static> {
    type Error = git2::Error;

    fn try_from(signature: Signature) -> Result<Self, Self::Error> {
        Self::now(&signature.name, &signature.email)
    }
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
    /// A new repository will be created using these fields.
    New {
        /// The path to the working copy.
        path: PathBuf,
        /// The name of the project.
        name: String,
        /// The URL that will be used for the remote.
        url: LocalUrl,
        /// The default branch the repository should be set up with.
        default_branch: OneLevel,
        /// The signature to be used for creating the first commit.
        signature: Signature,
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
    ///   * The path provided should have at least one component, which forms the name of the
    ///   project. E.g. `Developer/radicle-upstream` is the directory and `radicle-upstream` is the
    ///   project name.
    ///   * The path leads to a git repository
    ///   * The default branch passed exists in the repository
    ///   * If a `rad` remote exists, that it:
    ///         * Has a url field
    ///         * If it does have a url field, that it matches the one provided here
    ///
    /// **New**:
    ///   * The path provided does not exist:
    ///         * If it does exist, it should be a directory and it should be empty
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

                {
                    let _default_branch_ref = Self::existing_branch(&repo, &default_branch)?;
                    let _remote = Self::existing_remote(&repo, &url)?;
                }
                Ok(Self::Existing {
                    repo,
                    url,
                    default_branch,
                })
            },
            super::Repo::New { name, path } => {
                let repo_path = path.join(name.clone());
                let _repo_path = crate::project::ensure_directory(&repo_path)?
                    .ok_or_else(|| Error::AlreadExists(repo_path.clone()))?;

                let signature = Self::existing_author()?;

                Ok(Self::New {
                    name,
                    path,
                    url,
                    default_branch,
                    signature,
                })
            },
        }
    }

    /// Initialise the [`git2::Repository`].
    ///
    /// # Errors
    ///
    ///   * Failed to setup the repository
    pub fn setup_repo<F>(
        self,
        open_storage: F,
        description: &str,
    ) -> Result<git2::Repository, super::Error>
    where
        F: CanOpenStorage + Clone + 'static,
    {
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
                Self::setup_remote(&repo, open_storage, url, &default_branch)?;
                Ok(repo)
            },
            Self::New {
                path,
                name,
                url,
                default_branch,
                signature,
            } => {
                let path = path.join(name);
                log::debug!("Setting up new repository @ '{}'", path.display());
                let repo = Self::initialise(path, description, &default_branch)?;
                Self::initial_commit(
                    &repo,
                    &default_branch,
                    &git2::Signature::try_from(signature)?,
                )?;
                let mut remote =
                    Self::setup_remote(&repo, open_storage.clone(), url, &default_branch)?;
                // Set up the default branch under the remote to allow setting the upstream
                let _fetched = remote
                    .fetch(
                        open_storage,
                        &repo,
                        remote::LocalFetchspec::Specs(NonEmpty::new(Fetchspec::from(Refspec {
                            src: reflike!("refs/heads").join(default_branch.clone()),
                            dst: reflike!("refs/remotes")
                                .join(remote.name.clone())
                                .join(default_branch.clone()),
                            force: Force::False,
                        }))),
                    )
                    .map_err(Error::from)?;

                crate::project::set_upstream(&repo, &remote, default_branch)?;

                Ok(repo)
            },
        }
    }

    fn initialise(
        path: PathBuf,
        description: &str,
        default_branch: &OneLevel,
    ) -> Result<git2::Repository, git2::Error> {
        log::debug!("Setting up new repository @ '{}'", path.display());
        let mut options = git2::RepositoryInitOptions::new();
        options.no_reinit(true);
        options.mkpath(true);
        options.description(description);
        options.initial_head(default_branch.as_str());

        git2::Repository::init_opts(path, &options)
    }

    fn initial_commit(
        repo: &git2::Repository,
        default_branch: &OneLevel,
        signature: &git2::Signature<'static>,
    ) -> Result<(), git2::Error> {
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
                signature,
                signature,
                "Initial commit",
                &tree,
                &[],
            )?;
        }
        Ok(())
    }

    /// Equips a repository with a rad remote for the given id. If the directory at the given path
    /// is not managed by git yet we initialise it first.
    fn setup_remote<F>(
        repo: &git2::Repository,
        open_storage: F,
        url: LocalUrl,
        default_branch: &OneLevel,
    ) -> Result<Remote<LocalUrl>, Error>
    where
        F: CanOpenStorage + 'static,
    {
        let _default_branch_ref = Self::existing_branch(repo, default_branch)?;

        log::debug!("Creating rad remote");

        let fetchspec = Refspec {
            src: refspec_pattern!("refs/heads/*"),
            dst: refspec_pattern!("refs/remotes/rad/*"),
            force: Force::True,
        };
        let mut git_remote = Self::existing_remote(repo, &url)?.map_or_else(
            || {
                let mut rad = Remote::rad_remote(url, fetchspec);
                rad.save(repo)?;
                Ok::<_, Error>(rad)
            },
            Ok,
        )?;
        for pushed in git_remote.push(
            open_storage,
            repo,
            LocalPushspec::Matching {
                pattern: refspec_pattern!("refs/heads/*"),
                force: Force::True,
            },
        )? {
            log::debug!("Pushed local branch `{}`", pushed);
        }
        Ok(git_remote)
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

    fn existing_remote(
        repo: &git2::Repository,
        url: &LocalUrl,
    ) -> Result<Option<Remote<LocalUrl>>, Error> {
        match Remote::<LocalUrl>::find(repo, reflike!("rad")) {
            Err(remote::FindError::ParseUrl(_)) => {
                log::warn!("an old/invalid URL was found when trying to load the `rad` remote");
                log::warn!(
                    "we are going to rename the remote to `rad_old` and create a new `rad` remote"
                );
                repo.remote_rename("rad", "rad_old")?;
                Ok(None)
            },
            Err(err) => Err(err.into()),
            Ok(Some(remote)) if remote.url != *url => Err(Error::UrlMismatch {
                expected: url.to_string(),
                found: remote.url.to_string(),
            }),
            Ok(remote) => Ok(remote),
        }
    }

    fn existing_author() -> Result<Signature, Error> {
        let config = git2::Config::open_default()
            .or_matches(git_ext::is_not_found_err, || Err(Error::MissingGitConfig))?;
        let name = config
            .get_string(USER_NAME)
            .or_matches(git_ext::is_not_found_err, || Err(Error::MissingAuthorName))?;
        let email = config
            .get_string(USER_EMAIL)
            .or_matches(git_ext::is_not_found_err, || Err(Error::MissingAuthorEmail))?;
        Ok(Signature { name, email })
    }
}
