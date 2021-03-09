use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use librad::{git::local::url::LocalUrl, git_ext::OneLevel};
use radicle_surf::vcs::git::git2;

pub mod validation;

/// Errors that occur when attempting to create a working copy of a project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Internal git error while trying to create the project.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An error occurred while validating input.
    #[error(transparent)]
    Validation(#[from] validation::Error),
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
    pub fn project_name(&self) -> Result<String, validation::Error> {
        match self {
            Self::Existing { path } => path
                .components()
                .next_back()
                .and_then(|component| component.as_os_str().to_str())
                .map(ToString::to_string)
                .ok_or_else(|| validation::Error::EmptyExistingPath(path.to_path_buf())),
            Self::New { name, .. } => Ok(name.to_string()),
        }
    }

    /// Get the full path of the `Repo` creation data.
    #[must_use]
    pub fn full_path(&self) -> PathBuf {
        match self {
            Self::Existing { path } => path.to_path_buf(),
            Self::New { name, path } => path.join(name),
        }
    }
}

/// The data required for creating a new project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    /// Description of the project we want to create.
    pub description: String,
    /// The default branch name for the project.
    pub default_branch: OneLevel,
    /// What kind of working copy we're working with, i.e. new or existing.
    pub repo: Repo,
}

impl Create {
    /// Validate `Create` into a [`validation::Repository`]. This ensures that we have valid
    /// paths when we attempt to create the working copy.
    ///
    /// # Errors
    ///
    /// See [`validation::Repository::validate`]
    pub fn validate(self, url: LocalUrl) -> Result<validation::Repository, validation::Error> {
        validation::Repository::validate(self.repo, url, self.default_branch)
    }
}

// Clippy is stupid and doesn't realise the `Create`s here are different types than `Self`.
#[allow(clippy::use_self)]
impl Create {
    /// Transforms into an existing project.
    #[must_use]
    pub fn into_existing(self) -> Self {
        let path = self.repo.full_path();
        Self {
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

    use librad::{git_ext::OneLevel, identities::Urn, reflike};
    use radicle_git_ext::Oid;

    use super::*;

    #[test]
    fn validation_fails_on_non_empty_existing_directory() -> Result<(), Box<dyn std::error::Error>>
    {
        let url = LocalUrl::from(Urn::new(Oid::try_from(
            "7ab8629dd6da14dcacde7f65b3d58cd291d7e235",
        )?));
        let tmpdir = tempfile::tempdir().expect("failed to create tmp dir");
        let exists = tmpdir.path().join("exists");
        std::fs::create_dir(exists.clone())?;
        std::fs::File::create(exists.join("nonempty.rs"))?;

        let create = Create {
            description: "Radicle".to_string(),
            default_branch: OneLevel::from(reflike!("radicle")),
            repo: Repo::New {
                name: "exists".to_string(),
                path: tmpdir.path().to_path_buf(),
            },
        };
        assert_matches!(
            create.validate(url).err(),
            Some(validation::Error::AlreadExists(_))
        );

        Ok(())
    }

    #[test]
    fn validation_succeeds_on_empty_existing_directory() -> Result<(), Box<dyn std::error::Error>> {
        let url = LocalUrl::from(Urn::new(Oid::try_from(
            "7ab8629dd6da14dcacde7f65b3d58cd291d7e235",
        )?));
        let tmpdir = tempfile::tempdir().expect("failed to create tmp dir");
        let exists = tmpdir.path().join("exists");
        std::fs::create_dir(exists)?;

        let create = Create {
            description: "Radicle".to_string(),
            default_branch: OneLevel::from(reflike!("radicle")),
            repo: Repo::New {
                name: "exists".to_string(),
                path: tmpdir.path().to_path_buf(),
            },
        };
        assert!(create.validate(url).is_ok());

        Ok(())
    }
}
