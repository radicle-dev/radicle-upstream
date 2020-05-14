//! Proxy library errors usable for caller control flow and additional context for API responses.

use librad::meta::common::url;
use librad::surf;
use librad::surf::git::git2;
use radicle_registry_client as registry;
use std::time::SystemTimeError;

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
#[derive(Debug)]
pub enum Error {
    /// Returned when an attempt to create an identity was made and there is one present.
    IdentityExists(String),
    /// FileSystem errors from interacting with code in repository.
    FS(surf::file_system::error::Error),
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
    Librad(librad::git::Error),
    /// Parse error for `librad::project::ProjectId`.
    LibradParse(librad::project::projectid::ParseError),
    /// Project error from `librad`.
    LibradProject(librad::project::Error),
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

impl From<surf::file_system::error::Error> for Error {
    fn from(fs_error: surf::file_system::error::Error) -> Self {
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

impl From<librad::git::Error> for Error {
    fn from(librad_error: librad::git::Error) -> Self {
        Self::Librad(librad_error)
    }
}

impl From<librad::project::Error> for Error {
    fn from(project_error: librad::project::Error) -> Self {
        Self::LibradProject(project_error)
    }
}

impl From<librad::project::projectid::ParseError> for Error {
    fn from(parse_error: librad::project::projectid::ParseError) -> Self {
        Self::LibradParse(parse_error)
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
