use juniper::{FieldError, IntoFieldError};

use radicle_surf::git::git2;

#[derive(Debug)]
pub enum Error {
    GitError(radicle_surf::git::GitError),
    Git2Error(git2::Error),
    LibradError(librad::git::Error),
    LibradParseError(librad::project::projectid::ParseError),
    LibradProjectError(librad::project::Error),
    IoError(std::io::Error),
    UrlError(url::ParseError),
}

impl From<radicle_surf::git::GitError> for Error {
    fn from(git_error: radicle_surf::git::GitError) -> Self {
        Error::GitError(git_error)
    }
}

impl From<git2::Error> for Error {
    fn from(git2_error: git2::Error) -> Self {
        Error::Git2Error(git2_error)
    }
}

impl From<librad::git::Error> for Error {
    fn from(librad_error: librad::git::Error) -> Self {
        Error::LibradError(librad_error)
    }
}

impl From<librad::project::Error> for Error {
    fn from(project_error: librad::project::Error) -> Self {
        Error::LibradProjectError(project_error)
    }
}

impl From<librad::project::projectid::ParseError> for Error {
    fn from(parse_error: librad::project::projectid::ParseError) -> Self {
        Error::LibradParseError(parse_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::IoError(io_error)
    }
}

impl From<url::ParseError> for Error {
    fn from(url_error: url::ParseError) -> Self {
        Error::UrlError(url_error)
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Error::GitError(git_error) => FieldError::new(
                format!("{:?}", git_error),
                graphql_value!({
                    "type": "GIT_ERROR",
                }),
            ),
            Error::Git2Error(git2_error) => FieldError::new(
                git2_error.to_string(),
                graphql_value!({
                    "type": "GIT2_ERROR",
                }),
            ),
            Error::IoError(io_error) => FieldError::new(
                io_error.to_string(),
                graphql_value!({
                    "type": "IO_ERROR",
                }),
            ),
            Error::LibradError(librad_error) => FieldError::new(
                librad_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_ERROR",
                }),
            ),
            Error::LibradParseError(parse_error) => FieldError::new(
                parse_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PARSE_ERROR",
                }),
            ),
            Error::LibradProjectError(project_error) => FieldError::new(
                project_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PROJECT_ERROR",
                }),
            ),
            Error::UrlError(url_error) => FieldError::new(
                url_error.to_string(),
                graphql_value!({
                    "type": "URL_ERROR",
                }),
            ),
        }
    }
}
