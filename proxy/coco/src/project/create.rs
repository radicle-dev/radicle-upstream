use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use librad::{
    git::local::url::LocalUrl,
    git_ext::OneLevel,
    keys,
    meta::{entity, project},
};
use radicle_surf::vcs::git::git2;

use crate::user::User;

pub mod validation;

/// Errors that occur when attempting to create a working copy of a project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Internal git error while trying to create the project.
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// Entity meta error.
    #[error(transparent)]
    Meta(#[from] entity::Error),

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
    fn full_path(&self) -> PathBuf {
        match self {
            Self::Existing { path } => path.to_path_buf(),
            Self::New { name, path } => path.join(name),
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
    pub fn validate(self, url: LocalUrl) -> Result<validation::Repository, validation::Error> {
        Ok(validation::Repository::validate(
            self.repo,
            url,
            self.default_branch,
        )?)
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
        let name = self.repo.project_name()?;
        let project = project::Project::<entity::Draft>::create(name, owner.urn())?
            .to_builder()
            .set_description(self.description.clone())
            .set_default_branch(self.default_branch.as_str().to_string())
            .add_key(key)
            .add_certifier(owner.urn())
            .build()?;

        Ok(project)
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
    use assert_matches::assert_matches;

    use librad::{
        git_ext::OneLevel,
        hash::Hash,
        keys::SecretKey,
        peer::PeerId,
        reflike,
        uri::{self, RadUrn},
    };

    use super::*;

    #[test]
    fn validation_fails_on_non_empty_existing_directory() -> Result<(), Box<dyn std::error::Error>>
    {
        let peer_id = PeerId::from(SecretKey::new());
        let url = LocalUrl::from_urn(
            RadUrn::new(Hash::hash(b"geez"), uri::Protocol::Git, uri::Path::empty()),
            peer_id,
        );
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
        let peer_id = PeerId::from(SecretKey::new());
        let url = LocalUrl::from_urn(
            RadUrn::new(Hash::hash(b"geez"), uri::Protocol::Git, uri::Path::empty()),
            peer_id,
        );
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
