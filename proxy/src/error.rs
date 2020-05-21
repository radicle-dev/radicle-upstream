//! Proxy library errors usable for caller control flow and additional context for API responses.

use librad::meta::common::url;
use librad::meta::entity;
use librad::surf;
use librad::surf::git::git2;
use radicle_registry_client as registry;
use std::time::SystemTimeError;
use thiserror;

/// Project problems.
#[derive(Debug)]
pub enum ProjectValidation {
    /// Project names (String32) can only be at most 32 bytes.
    NameTooLong,
    /// Org ids (String32) can only be at most 32 bytes.
    OrgTooLong,
}

/// Validation errors for inputs of user registrations.
#[derive(Debug)]
pub enum UserValidation {
    /// Given handle is too long.
    HandleTooLong,
    /// Given id is too long.
    IdTooLong,
}

/// All error variants the API will return.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Returned when an attempt to create an identity was made and there is one present.
    IdentityExists(String),
    /// FileSystem errors from interacting with code in repository.
    FS(#[from] surf::file_system::Error),
    /// TODO(fintan)
    PathNotFound,
    /// Originated from `radicle_surf`.
    Git(surf::git::error::Error),
    /// Originated from `radicle_surf::git::git2`.
    Git2(git2::Error),
    /// Integer conversion failed.
    IntConversion(std::num::TryFromIntError),
    /// Length limitation on String32 has been exceeded.
    InordinateString32(),
    /// Id input is invalid, variant carries the reason.
    InvalidId(String),
    /// Project name input is invalid, variant carries the reason.
    InvalidProjectName(String),
    /// Originated from `librad`.
    LibradRepo(librad::git::repo::Error),
    /// Originated from `librad::Storage`.
    LibradStorage(librad::git::storage::Error),
    /// Parse error for `librad::uri::path::Path`.
    LibradParse(librad::uri::path::ParseError),
    /// Parse error for `RadUrn`
    LibradParseUrn(librad::uri::rad_urn::ParseError),
    /// Project error from `librad`.
    LibradProject(entity::Error),
    /// Common I/O errors.
    Io(std::io::Error),
    /// Url parse error.
    Url(url::ParseError),
    /// Project name validation.
    ProjectValidation(ProjectValidation),
    /// User registration validation errors.
    UserValidation(UserValidation),
    /// Issues with the Radicle protocol.
    Protocol(registry::Error),
    /// Issues with the Radicle runtime.
    Runtime(registry::DispatchError),
    /// Issues when access persistent storage.
    Store(kv::Error),
    /// Errors from handling time.
    Time(SystemTimeError),
    /// Errors from transactions.
    Transaction(registry::TransactionError),
}

impl From<surf::file_system::Error> for Error {
    fn from(fs_error: surf::file_system::Error) -> Self {
        Self::FS(fs_error)
    }
}

impl From<surf::git::error::Error> for Error {
    fn from(surf_error: surf::git::error::Error) -> Self {
        Self::Git(surf_error)
    }
}

impl From<git2::Error> for Error {
    fn from(git2_error: git2::Error) -> Self {
        Self::Git2(git2_error)
    }
}

impl From<kv::Error> for Error {
    fn from(kv_error: kv::Error) -> Self {
        Self::Store(kv_error)
    }
}

impl From<librad::git::repo::Error> for Error {
    fn from(librad_error: librad::git::repo::Error) -> Self {
        Self::LibradRepo(librad_error)
    }
}

impl From<librad::git::storage::Error> for Error {
    fn from(librad_error: librad::git::storage::Error) -> Self {
        Self::LibradStorage(librad_error)
    }
}

impl From<entity::Error> for Error {
    fn from(project_error: entity::Error) -> Self {
        Self::LibradProject(project_error)
    }
}

impl From<librad::uri::path::ParseError> for Error {
    fn from(parse_error: librad::uri::path::ParseError) -> Self {
        Self::LibradParse(parse_error)
    }
}

impl From<librad::uri::rad_urn::ParseError> for Error {
    fn from(parse_error: librad::uri::rad_urn::ParseError) -> Self {
        Self::LibradParseUrn(parse_error)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(int_error: std::num::TryFromIntError) -> Self {
        Self::IntConversion(int_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::Io(io_error)
    }
}

impl From<url::ParseError> for Error {
    fn from(url_error: url::ParseError) -> Self {
        Self::Url(url_error)
    }
}

impl From<registry::DispatchError> for Error {
    fn from(error: registry::DispatchError) -> Self {
        Self::Runtime(error)
    }
}

impl From<registry::Error> for Error {
    fn from(error: registry::Error) -> Self {
        Self::Protocol(error)
    }
}

impl From<registry::InvalidIdError> for Error {
    fn from(invalid_id: registry::InvalidIdError) -> Self {
        Self::InvalidId(invalid_id.to_string())
    }
}

impl From<registry::InvalidProjectNameError> for Error {
    fn from(invalid_project_name: registry::InvalidProjectNameError) -> Self {
        Self::InvalidProjectName(invalid_project_name.to_string())
    }
}

impl From<registry::TransactionError> for Error {
    fn from(tx_err: registry::TransactionError) -> Self {
        Self::Transaction(tx_err)
    }
}

impl From<ProjectValidation> for Error {
    fn from(error: ProjectValidation) -> Self {
        Self::ProjectValidation(error)
    }
}

impl From<UserValidation> for Error {
    fn from(error: UserValidation) -> Self {
        Self::UserValidation(error)
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Self::Time(error)
    }
}
