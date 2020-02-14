use librad::meta::common::url;
use librad::surf;
use librad::surf::git::git2;

use crate::error;

impl juniper::IntoFieldError for error::Error {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            Self::FS(fs_error) => convert_fs(&fs_error),
            Self::Git(git_error) => convert_git(&git_error),
            Self::Git2(git2_error) => convert_git2(&git2_error),
            Self::IntConversion(int_error) => juniper::FieldError::new(
                int_error.to_string(),
                graphql_value!({
                    "type": "INT_CONVERSION",
                }),
            ),
            Self::Io(io_error) => convert_io(&io_error),
            Self::Librad(librad_error) => convert_librad_git(&librad_error),
            Self::LibradParse(parse_error) => {
                convert_librad_parse_error_to_field_error(&parse_error)
            },
            Self::LibradProject(project_error) => match project_error {
                librad::project::Error::Git(librad_error) => convert_librad_git(&librad_error),
            },
            Self::Url(url_error) => convert_url_parse_error_to_field_error(&url_error),
            Self::ProjectValidation(project_error) => match project_error {
                error::ProjectValidation::NameTooLong(error) => juniper::FieldError::new(
                    error,
                    graphql_value!({ "type": "PROJECT_NAME_TOO_LONG" }),
                ),
                error::ProjectValidation::OrgTooLong(error) => juniper::FieldError::new(
                    error,
                    graphql_value!({ "type": "PROJECT_DOMAIN_TOO_LONG" }),
                ),
            },
            // TODO(garbados): expand via sub-match
            Self::Protocol(error) => juniper::FieldError::new(
                error.to_string(),
                graphql_value!({ "type": "RADICLE_PROTOCOL" }),
            ),
            // TODO(garbados): expand via sub-match
            Self::Runtime(error) => juniper::FieldError::new(
                format!("{:?}", error),
                graphql_value!({ "type": "RADICLE_RUNTIME" }),
            ),
            Self::Time(error) => {
                juniper::FieldError::new(error.to_string(), graphql_value!({ "type": "TIME" }))
            },
        }
    }
}

/// Helper to convert `radicle_surf` `FileSystem` error to `juniper::FieldError`.
fn convert_fs(error: &surf::file_system::error::Error) -> juniper::FieldError {
    let error_str = match &error {
        surf::file_system::error::Error::Label(label_error) => match label_error {
            surf::file_system::error::Label::ContainsSlash => "Label contains slashes",
            surf::file_system::error::Label::Empty => "Label is empty",
            surf::file_system::error::Label::InvalidUTF8 => "Label is not valid utf8",
        },
        surf::file_system::error::Error::Path(path_error) => match path_error {
            surf::file_system::error::Path::Empty => "Path is empty",
        },
    };

    juniper::FieldError::new(
        error_str,
        graphql_value!({
            "type": "FS"
        }),
    )
}

/// Helper to convert `std::io::Error` to `juniper::FieldError`.
fn convert_io(error: &std::io::Error) -> juniper::FieldError {
    juniper::FieldError::new(
        error.to_string(),
        graphql_value!({
            "type": "IO_ERROR",
        }),
    )
}

/// Helper to convert a `radicle_surf` Git error to `juniper::FieldError`.
fn convert_git(error: &surf::git::error::Error) -> juniper::FieldError {
    match error {
        surf::git::error::Error::NotBranch(branch) => juniper::FieldError::new(
            format!("branch '{}' not known", branch.name()),
            graphql_value!({
                "type": "GIT_NOT_BRANCH"
            }),
        ),
        surf::git::error::Error::NotTag(tag) => juniper::FieldError::new(
            format!("tag '{}' not known", tag.name()),
            graphql_value!({
                "type": "GIT_NOT_TAG"
            }),
        ),
        surf::git::error::Error::RevParseFailure(rev) => juniper::FieldError::new(
            format!("revspec '{}' malformed", rev),
            graphql_value!({
                "type": "GIT_REV_PARSE"
            }),
        ),
        surf::git::error::Error::Utf8Error(_utf8_error) => juniper::FieldError::new(
            "String conversion error",
            graphql_value!({
                "type": "STRING_CONVERSION",
            }),
        ),
        surf::git::error::Error::FileSystem(fs_error) => convert_fs(fs_error),
        surf::git::error::Error::LastCommitException => juniper::FieldError::new(
            "last commit failed",
            graphql_value!({
                "type": "GIT_LAST_COMMIT"
            }),
        ),
        surf::git::error::Error::Git(error) => juniper::FieldError::new(
            format!("Internal Git error: {:?}", error),
            graphql_value!({
                "type": "GIT_INTERNAL"
            }),
        ),
    }
}

/// Helper to convert a `git2::error::Error` to `juniper::FieldError`.
fn convert_git2(error: &git2::Error) -> juniper::FieldError {
    juniper::FieldError::new(
        error.to_string(),
        graphql_value!({
            "type": "GIT2_ERROR"
        }),
    )
}

/// Helper to convert `librad::git::Error` to `juniper::FieldError`.
fn convert_librad_git(error: &librad::git::Error) -> juniper::FieldError {
    match error {
        librad::git::Error::MissingPgpAddr => juniper::FieldError::new(
            "Missing PGP address.",
            graphql_value!({
                "type": "LIBRAD_MISSING_PGP_ADDRESS"
            }),
        ),
        librad::git::Error::MissingPgpUserId => juniper::FieldError::new(
            "Missing PGP user ID.",
            graphql_value!({
                "type": "LIBRAD_MISSING_PGP_USER_ID"
            }),
        ),
        librad::git::Error::ProjectExists => juniper::FieldError::new(
            "Project already exists.",
            graphql_value!({
                "type": "LIBRAD_PROJECT_EXISTS"
            }),
        ),
        librad::git::Error::NoSuchProject => juniper::FieldError::new(
            "No such project exists.",
            graphql_value!({
                "type": "LIBRAD_NO_SUCH_PROJECT"
            }),
        ),
        librad::git::Error::Libgit(git2_error) => convert_git2(git2_error),
        librad::git::Error::Io(io_error) => convert_io(io_error),
        librad::git::Error::Serde(json_error) => juniper::FieldError::new(
            json_error.to_string(),
            graphql_value!({
                "type": "LIBRAD_JSON_ERROR"
            }),
        ),
        librad::git::Error::Pgp(pgp_error) => juniper::FieldError::new(
            pgp_error.to_string(),
            graphql_value!({
                "type": "LIBRAD_PGP_ERROR"
            }),
        ),
        librad::git::Error::Surf(surf_error) => convert_git(surf_error),
        librad::git::Error::MissingDefaultBranch(branch, _) => juniper::FieldError::new(
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

/// Helper to convert `librad::project::projectid::ParseError` to `juniper::FieldError`.
fn convert_librad_parse_error_to_field_error(
    error: &librad::project::projectid::ParseError,
) -> juniper::FieldError {
    match error {
        librad::project::projectid::ParseError::Git(parse_error) => match parse_error {
            librad::git::projectid::ParseError::InvalidBackend(error) => juniper::FieldError::new(
                error.to_string(),
                graphql_value!({
                    "type": "LIBRAD_PARSE_INVALID_BACKEND"
                }),
            ),
            librad::git::projectid::ParseError::InvalidFormat(error) => juniper::FieldError::new(
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

/// Helper to convert `url::ParseError` to `juniper::FieldError`.
fn convert_url_parse_error_to_field_error(error: &url::ParseError) -> juniper::FieldError {
    juniper::FieldError::new(error.to_string(), graphql_value!({ "type": "URL_PARSE" }))
}
