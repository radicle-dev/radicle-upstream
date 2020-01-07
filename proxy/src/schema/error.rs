use juniper::{FieldError, IntoFieldError};

use radicle_surf::git::git2;

#[derive(Debug)]
pub enum Error {
    Git(radicle_surf::git::GitError),
    Git2(git2::Error),
    Librad(librad::git::Error),
    LibradParse(librad::project::projectid::ParseError),
    LibradProject(librad::project::Error),
    Io(std::io::Error),
    Url(url::ParseError),
}

impl From<radicle_surf::git::GitError> for Error {
    fn from(git_error: radicle_surf::git::GitError) -> Self {
        Self::Git(git_error)
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

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Self::Git(git_error) => FieldError::new(
                format!("{:?}", git_error),
                graphql_value!({
                    "type": "GIT_ERROR",
                }),
            ),
            Self::Git2(git2_error) => FieldError::new(
                git2_error.to_string(),
                graphql_value!({
                    "type": "GIT2_ERROR",
                }),
            ),
            Self::Io(io_error) => FieldError::new(
                io_error.to_string(),
                graphql_value!({
                    "type": "IO_ERROR",
                }),
            ),
            Self::Librad(librad_error) => FieldError::new(
                librad_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_ERROR",
                }),
            ),
            Self::LibradParse(parse_error) => FieldError::new(
                parse_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PARSE_ERROR",
                }),
            ),
            Self::LibradProject(project_error) => FieldError::new(
                project_error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PROJECT_ERROR",
                }),
            ),
            Self::Url(url_error) => FieldError::new(
                url_error.to_string(),
                graphql_value!({
                    "type": "URL_ERROR",
                }),
            ),
        }
    }
}
