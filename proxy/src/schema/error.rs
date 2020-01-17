//! Domain errors returned by the API.

use juniper::{FieldError, IntoFieldError};
use librad::meta::common::url;
use radicle_surf as surf;
use radicle_surf::git::git2;

/// All error variants the API will return.
#[derive(Debug)]
pub enum Error {
    /// FileSystem errors from interacting with code in repository.
    FS(radicle_surf::file_system::error::Error),
    /// Originated from `radicle_surf`.
    Git(surf::git::error::Error),
    /// Originated from `radicle_surf::git::git2`.
    Git2(git2::Error),
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
}

impl From<radicle_surf::file_system::error::Error> for Error {
    fn from(fs_error: radicle_surf::file_system::error::Error) -> Self {
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

/// Helper to convert `radicle_surf` `FileSystem` error to `FieldError`.
fn convert_fs(error: &radicle_surf::file_system::error::Error) -> FieldError {
    let error_str = match &error {
        radicle_surf::file_system::error::Error::Label(label_error) => match label_error {
            radicle_surf::file_system::error::Label::ContainsSlash => "Label contains slashes",
            radicle_surf::file_system::error::Label::Empty => "Label is empty",
            radicle_surf::file_system::error::Label::InvalidUTF8 => "Label is not valid utf8",
        },
        radicle_surf::file_system::error::Error::Path(path_error) => match path_error {
            radicle_surf::file_system::error::Path::Empty => "Path is empty",
        },
    };

    FieldError::new(
        error_str,
        graphql_value!({
            "type": "FS"
        }),
    )
}

/// Helper to convert `std::io::Error` to `FieldError`.
fn convert_io(error: &std::io::Error) -> FieldError {
    FieldError::new(
        error.to_string(),
        graphql_value!({
            "type": "IO_ERROR",
        }),
    )
}

/// Helper to convert a `radicle_surf` Git error to `FieldError`.
fn convert_git(error: &surf::git::error::Error) -> FieldError {
    match error {
        surf::git::error::Error::EmptyCommitHistory => FieldError::new(
            "Repository has an empty commit history.",
            graphql_value!({
                "type": "GIT_EMPTY_COMMIT_HISTORY"
            }),
        ),
        surf::git::error::Error::NotBranch => FieldError::new(
            "Not a known branch.",
            graphql_value!({
                "type": "GIT_NOT_BRANCH"
            }),
        ),
        surf::git::error::Error::NotTag => FieldError::new(
            "Not a known tag.",
            graphql_value!({
                "type": "GIT_NOT_TAG"
            }),
        ),
        surf::git::error::Error::Utf8Error(_utf8_error) => FieldError::new(
            "String conversion error",
            graphql_value!({
                "type": "STRING_CONVERSION",
            }),
        ),
        surf::git::error::Error::FileSystem(fs_error) => convert_fs(fs_error),
        surf::git::error::Error::FileDiffException => FieldError::new(
            "Diff failed.",
            graphql_value!({
                "type": "GIT_FILE_DIFF"
            }),
        ),
        surf::git::error::Error::Internal(error) => FieldError::new(
            format!("Internal Git error: {:?}", error),
            graphql_value!({
                "type": "GIT_INTERNAL"
            }),
        ),
    }
}

/// Helper to convert a `git2::error::Error` to `FieldError`.
fn convert_git2(error: &git2::Error) -> FieldError {
    FieldError::new(
        error.to_string(),
        graphql_value!({
            "type": "GIT2_ERROR"
        }),
    )
}

/// Helper to convert `librad::git::Error` to `FieldError`.
fn convert_librad_git(error: &librad::git::Error) -> FieldError {
    match error {
        librad::git::Error::MissingPgpAddr => FieldError::new(
            "Missing PGP address.",
            graphql_value!({
                "type": "LIBRAD_MISSING_PGP_ADDRESS"
            }),
        ),
        librad::git::Error::MissingPgpUserId => FieldError::new(
            "Missing PGP user ID.",
            graphql_value!({
                "type": "LIBRAD_MISSING_PGP_USER_ID"
            }),
        ),
        librad::git::Error::ProjectExists => FieldError::new(
            "Project already exists.",
            graphql_value!({
                "type": "LIBRAD_PROJECT_EXISTS"
            }),
        ),
        librad::git::Error::NoSuchProject => FieldError::new(
            "No such project exists.",
            graphql_value!({
                "type": "LIBRAD_NO_SUCH_PROJECT"
            }),
        ),
        librad::git::Error::Libgit(git2_error) => convert_git2(git2_error),
        librad::git::Error::Io(io_error) => convert_io(io_error),
        librad::git::Error::Serde(json_error) => FieldError::new(
            json_error.to_string(),
            graphql_value!({
                "type": "LIBRAD_JSON_ERROR"
            }),
        ),
        librad::git::Error::Pgp(pgp_error) => FieldError::new(
            pgp_error.to_string(),
            graphql_value!({
                "type": "LIBRAD_PGP_ERROR"
            }),
        ),
        librad::git::Error::Surf(surf_error) => convert_git(surf_error),
        librad::git::Error::MissingDefaultBranch(branch, _) => FieldError::new(
            format!(
                "Branch {} specified as default_branch does not exist in the source repo",
                branch
            ),
            graphql_value!({
                "type": "LIBRAD_MISING_DEFAULT_BRANCH"
            }),
        ),
    }
}

/// Helper to convert `librad::project::projectid::ParseError` to `FieldError`.
fn convert_librad_parse_error_to_field_error(
    error: &librad::project::projectid::ParseError,
) -> FieldError {
    match error {
        librad::project::projectid::ParseError::Git(parse_error) => match parse_error {
            librad::git::projectid::ParseError::InvalidBackend(error) => FieldError::new(
                error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PARSE_INVALID_BACKEND"
                }),
            ),
            librad::git::projectid::ParseError::InvalidFormat(error) => FieldError::new(
                error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PARSE_INVALID_FORMAT"
                }),
            ),
            librad::git::projectid::ParseError::InvalidOid(_, git2_error) => {
                convert_git2(git2_error)
            }
        },
    }
}

/// Helper to convert `url::ParseError` to `FieldError`.
fn convert_url_parse_error_to_field_error(error: url::ParseError) -> FieldError {
    FieldError::new(error.to_string(), graphql_value!({ "type": "URL_PARSE" }))
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Self::FS(fs_error) => convert_fs(&fs_error),
            Self::Git(git_error) => convert_git(&git_error),
            Self::Git2(git2_error) => convert_git2(&git2_error),
            Self::Io(io_error) => convert_io(&io_error),
            Self::Librad(librad_error) => convert_librad_git(&librad_error),
            Self::LibradParse(parse_error) => {
                convert_librad_parse_error_to_field_error(&parse_error)
            }
            Self::LibradProject(project_error) => match project_error {
                librad::project::Error::Git(librad_error) => convert_librad_git(&librad_error),
            },
            Self::Url(url_error) => convert_url_parse_error_to_field_error(url_error),
        }
    }
}
