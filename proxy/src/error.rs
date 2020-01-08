use juniper::{
    FieldError, IntoFieldError, Value,
};

use radicle_registry_client;
use radicle_registry_client::{Error as RegistryError};
use radicle_surf::{
    file_system::{Path},
};

use crate::source::{Error as SourceError};

#[derive(Debug)]
/// Enumerable of expected error types.
pub enum Error {
    /// File at a given path was irretrievable.
    FileNotFound(Path),
    /// Directory at a given path was irretrievable.
    DirectoryNotFound(Path),
    /// Project name exceeded 32 characters.
    BadProjectName(String),
    /// Project domain exceeded 32 characters.
    BadProjectDomain(String),
    /// Errors originating in radicle-surf's Git adapter.
    Git(radicle_surf::git::GitError),
    /// Registry client errors.
    Registry(RegistryError),
    /// VCS Browser could not find the last commit of a branch.
    LastCommitNotFound(Path),
}

impl From<radicle_surf::git::GitError> for Error {
    fn from(git_error: radicle_surf::git::GitError) -> Self {
        Self::Git(git_error)
    }
}

impl From<SourceError> for Error {
    fn from(source_error: SourceError) -> Self {
        match source_error {
            SourceError::BadProjectName(name) => Self::BadProjectName(name),
            SourceError::BadProjectDomain(domain) => Self::BadProjectDomain(domain),
            SourceError::Registry(error) => Self::Registry(error),
        }
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Self::Git(git_error) => {
                match &git_error {
                    radicle_surf::git::GitError::EmptyCommitHistory => {
                        FieldError::new(
                            "Repository has an empty commit history.",
                            graphql_value!({
                                "type": "EMPTY_COMMIT_HISTORY"
                            })
                        )
                    },
                    radicle_surf::git::GitError::BranchDecode => {
                        FieldError::new(
                            "Unable to decode the given branch.",
                            graphql_value!({
                                "type": "BRANCH_DECODE"
                            })
                        )
                    },
                    radicle_surf::git::GitError::NotBranch => {
                        FieldError::new(
                            "Not a known branch.",
                            graphql_value!({
                                "type": "NOT_BRANCH"
                            })
                        )
                    },
                    radicle_surf::git::GitError::NotTag => {
                        FieldError::new(
                            "Not a known tag.",
                            graphql_value!({
                                "type": "NOT_TAG"
                            })
                        )
                    },
                    radicle_surf::git::GitError::Internal(error) => {
                        FieldError::new(
                            format!("Internal Git error: {:?}", error),
                            graphql_value!({
                                "type": "INTERNAL"
                            })
                        )
                    },
                }
            },
            Self::Registry(reg_error) => {
                match reg_error {
                    RegistryError::Codec(codec_error) => {
                        FieldError::new(
                            format!("Failed to decode data: {:?}", codec_error),
                            Value::scalar("CODEC_ERROR"),
                        )
                    },
                    RegistryError::Rpc(rpc_error) => {
                        FieldError::new(
                            format!("RPC error: {:?}", rpc_error),
                            Value::scalar("RPC_ERROR"),
                        )
                    },
                    RegistryError::InvalidTransaction() => {
                        FieldError::new(
                            "Invalid transaction.",
                            Value::scalar("INVALID_TRANSACTION"),
                        )
                    },
                    RegistryError::Other(error) => {
                        FieldError::new(
                            format!("Registry error: {:?}", error),
                            Value::scalar("REGISTRY_ERROR"),
                        )
                    },
                }
            },
            Self::DirectoryNotFound(path) => {
                FieldError::new(
                    format!("Directory not found: {:?}", path),
                    Value::scalar("DIR_NOT_FOUND"),
                )
            },
            Self::FileNotFound(error) => {
                FieldError::new(
                    format!("File not found: {:?}", error),
                    Value::scalar("FILE_NOT_FOUND"),
                )
            },
            Self::LastCommitNotFound(error) => {
                FieldError::new(
                    format!("Last commit not found: {:?}", error),
                    Value::scalar("LAST_COMMIT_NOT_FOUND"),
                )
            },
            Self::BadProjectName(error) => {
                FieldError::new(
                    error,
                    Value::scalar("BAD_PROJECT_NAME"),
                )
            },
            Self::BadProjectDomain(error) => {
                FieldError::new(
                    error,
                    Value::scalar("BAD_PROJECT_DOMAIN"),
                )
            }
        }
    }
}
