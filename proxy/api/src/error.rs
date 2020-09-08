//! Proxy library errors usable for caller control flow and additional context for API responses.

use std::time::SystemTimeError;

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
    /// Coco errors.
    #[error(transparent)]
    Coco(#[from] coco::Error),

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

    /// An error occurred while performing the checkout of a project.
    #[error(transparent)]
    Checkout(#[from] coco::project::checkout::Error),

    /// Keystore error.
    #[error(transparent)]
    Keystore(#[from] coco::keystore::Error),

    /// Common I/O errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Project name validation.
    #[error(transparent)]
    ProjectValidation(#[from] ProjectValidation),

    /// User registration validation errors.
    #[error(transparent)]
    UserValidation(#[from] UserValidation),

    /// Issues when access persistent storage.
    #[error(transparent)]
    Store(#[from] kv::Error),

    /// Errors from handling time.
    #[error(transparent)]
    Time(#[from] SystemTimeError),

    /// Overflow while incrementing confirmed transaction.
    #[error("while calculating the number of confirmed transactions, we encountered an overflow")]
    TransactionConfirmationOverflow,

    /// We expect at least one [`coco::Revisions`] when looking at a project, however the
    /// computation found none.
    #[error(
        "while trying to get user revisions we could not find any, there should be at least one"
    )]
    EmptyRevisions,
}
