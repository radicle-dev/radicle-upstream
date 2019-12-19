use juniper::{FieldError, IntoFieldError};

use radicle_surf::git::git2;

#[derive(Debug)]
pub enum Error {
    Git2Error(git2::Error),
    LibradError(librad::git::Error),
    IoError(std::io::Error),
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

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::IoError(io_error)
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
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
        }
    }
}
