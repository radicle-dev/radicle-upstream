//! Proxy library errors usable for caller control flow and additional context for API responses.

use std::time::SystemTimeError;

use librad::meta::common::url;
use librad::meta::entity;
use radicle_registry_client as registry;
use radicle_surf as surf;
use radicle_surf::git::git2;

use crate::coco;
use crate::coco::signer;

/// Project problems.
#[derive(Debug, thiserror::Error)]
pub enum ProjectValidation {
    /// Project names (String32) can only be at most 32 bytes.
    #[error("the Org Name exceeded 32 bytes")]
    NameTooLong,
    /// Org ids (String32) can only be at most 32 bytes.
    #[error("the Org Name exceeded 32 bytes")]
    OrgTooLong,
}

/// Validation errors for inputs of user registrations.
#[derive(Debug, thiserror::Error)]
pub enum UserValidation {
    /// Given handle is too long.
    #[error("the User Handle provided is too long")]
    HandleTooLong,
    /// Given id is too long.
    #[error("the User Id provided is too long")]
    IdTooLong,
}

/// All error variants the API will return.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Returned when an attempt to create an identity was made and there is one present.
    #[error("the identity '{0}' already exits")]
    EntityExists(coco::Urn),

    /// Failed to create a librad project.
    #[error(transparent)]
    ProjectCreation(#[from] coco::project::Error),

    /// FileSystem errors from interacting with code in repository.
    #[error(transparent)]
    FS(#[from] surf::file_system::Error),

    /// Trying to find a file path which could not be found.
    #[error("the path '{0}' was not found")]
    PathNotFound(surf::file_system::Path),

    /// Could not construct a path.
    #[error(transparent)]
    JoinPaths(#[from] std::env::JoinPathsError),

    /// Originated from `radicle_surf`.
    #[error(transparent)]
    Git(#[from] surf::git::error::Error),

    /// Originated from `radicle_surf::git::git2`.
    #[error(transparent)]
    Git2(#[from] git2::Error),

    /// Integer conversion failed.
    #[error(transparent)]
    IntConversion(#[from] std::num::TryFromIntError),

    /// Length limitation on String32 has been exceeded.
    #[error("the provided string's length exceeds 32")]
    InordinateString32(),

    /// Id input is invalid, variant carries the reason.
    #[error("the ID '{0}' is invalid")]
    InvalidId(String),

    /// Project name input is invalid, variant carries the reason.
    #[error("the Project Name '{0}' is invalid")]
    InvalidProjectName(String),

    /// The given account could not be found in the Registry.
    #[error("the given account '{0}' could not be found in the Registry")]
    AccountNotFound(registry::AccountId),

    /// The given block could not be found in the Registry.
    #[error("the given block '{0}' could not be found in the Registry")]
    BlockNotFound(registry::BlockHash),

    /// An error occurred while performing the checkout of a project.
    #[error("checkout of the project failed")]
    Checkout,

    /// Accept error from `librad`.
    #[error(transparent)]
    LibradAccept(#[from] librad::net::peer::AcceptError),

    /// Bootstrap error from `librad`.
    #[error(transparent)]
    LibradBootstrap(#[from] librad::net::peer::BootstrapError),

    /// Originated from `librad`.
    #[error(transparent)]
    LibradRepo(#[from] librad::git::repo::Error),

    /// Originated from `librad::Storage`.
    #[error(transparent)]
    LibradStorage(#[from] librad::git::storage::Error),

    /// Parse error for `librad::uri::path::Path`.
    #[error(transparent)]
    LibradParse(#[from] librad::uri::path::ParseError),

    /// Parse error for [`coco::Urn`].
    #[error(transparent)]
    LibradParseUrn(#[from] coco::uri::rad_urn::ParseError),

    /// Project error from `librad`.
    #[error(transparent)]
    LibradProject(#[from] entity::Error),

    /// Failure to acquire [`std::sync::Mutex`] lock for the peer.
    #[error("failed to acquire lock for peer")]
    LibradLock,

    /// Failure during the verification of a `librad` entity.
    #[error(transparent)]
    LibradVerification(#[from] entity::HistoryVerificationError),

    /// Common I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Url parse error.
    #[error(transparent)]
    Url(#[from] url::ParseError),

    /// Project name validation.
    #[error(transparent)]
    ProjectValidation(#[from] ProjectValidation),

    /// User registration validation errors.
    #[error(transparent)]
    UserValidation(#[from] UserValidation),

    /// Issues with the Radicle protocol.
    #[error(transparent)]
    Protocol(#[from] registry::Error),

    /// Issues with the Radicle runtime.
    #[error("runtime error in registry: {0:?}")]
    Runtime(registry::DispatchError),

    /// Issues when access persistent storage.
    #[error(transparent)]
    Store(#[from] kv::Error),

    /// Errors from handling time.
    #[error(transparent)]
    Time(#[from] SystemTimeError),

    /// Errors from transactions.
    #[error(transparent)]
    Transaction(#[from] registry::TransactionError),

    /// Overflow while incrementing confirmed transaction.
    #[error("while calculating the number of confirmed transactions, we encountered an overflow")]
    TransactionConfirmationOverflow,

    /// We expect at least one [`coco::Revisions`] when looking at a project, however the
    /// computation found none.
    #[error(
        "while trying to get user revisions we could not find any, there should be at least one"
    )]
    EmptyRevisions,

    /// Errors from [`signer::Signer`].
    #[error(transparent)]
    Signer(#[from] signer::Error),
}

impl From<registry::DispatchError> for Error {
    fn from(dispactch: registry::DispatchError) -> Self {
        Self::Runtime(dispactch)
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
