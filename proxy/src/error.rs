// use juniper::{
//     FieldError, IntoFieldError, Value,
// };
//
// use radicle_registry_client;
// use radicle_registry_client::{Error as RegistryError};
// use radicle_surf::{
//     file_system::{Path},
// };
//
// #[derive(Debug)]
// /// Validation errors for project names.
// pub enum ProjectNameValidation {
//     /// Error when a project name is too long.
//     TooLong(String)
// }
//
// #[derive(Debug)]
// /// Validation errors for project domains.
// pub enum ProjectDomainValidation {
//     /// Error when a project domain is too long.
//     TooLong(String)
// }
//
// #[derive(Debug)]
// /// Enumerable of expected error types.
// pub enum Error {
//     /// File at a given path was irretrievable.
//     FileNotFound(Path),
//     /// Directory at a given path was irretrievable.
//     DirectoryNotFound(Path),
//     /// Errors originating in radicle-surf's Git adapter.
//     Git(radicle_surf::git::GitError),
//     /// Validation of project name.
//     ProjectName(ProjectNameValidation),
//     /// Validation of project domain.
//     ProjectDomain(ProjectDomainValidation),
//     /// Registry client errors.
//     Registry(RegistryError),
//     /// VCS Browser could not find the last commit of a branch.
//     LastCommitNotFound(Path),
// }
//
// impl From<ProjectNameValidation> for Error {
//     fn from(error: ProjectNameValidation) -> Self {
//         Self::ProjectName(error)
//     }
// }
//
// impl From<ProjectDomainValidation> for Error {
//     fn from(error: ProjectDomainValidation) -> Self {
//         Self::ProjectDomain(error)
//     }
// }
//
// impl From<radicle_surf::git::GitError> for Error {
//     fn from(git_error: radicle_surf::git::GitError) -> Self {
//         Self::Git(git_error)
//     }
// }
//
// impl From<RegistryError> for Error {
//     fn from(error: RegistryError) -> Self {
//         Self::Registry(error)
//     }
// }
//
//
// /// Helper to convert a `radicle_registry_client` error to `FieldError`
// fn convert_registry_error_to_field_error(error: RegistryError) -> FieldError {
//     match error {
//         RegistryError::Codec(codec_error) => {
//             FieldError::new(
//                 format!("Failed to decode data: {:?}", codec_error),
//                 Value::scalar("CODEC_ERROR"),
//             )
//         },
//         RegistryError::Rpc(rpc_error) => {
//             FieldError::new(
//                 format!("RPC error: {:?}", rpc_error),
//                 Value::scalar("RPC_ERROR"),
//             )
//         },
//         RegistryError::InvalidTransaction() => {
//             FieldError::new(
//                 "Invalid transaction.",
//                 Value::scalar("INVALID_TRANSACTION"),
//             )
//         },
//         RegistryError::Other(error) => {
//             FieldError::new(
//                 format!("Registry error: {:?}", error),
//                 Value::scalar("REGISTRY_ERROR"),
//             )
//         },
//     }
// }
//
// impl IntoFieldError for Error {
//     fn into_field_error(self) -> FieldError {
//         match self {
//             Self::Git(git_error) => convert_git_error_to_field_error(git_error),
//             Self::Registry(reg_error) => convert_registry_error_to_field_error(reg_error),
//             Self::DirectoryNotFound(path) => {
//                 FieldError::new(
//                     format!("Directory not found: {:?}", path),
//                     Value::scalar("DIR_NOT_FOUND"),
//                 )
//             },
//             Self::FileNotFound(path) => {
//                 FieldError::new(
//                     format!("File not found: {:?}", path),
//                     Value::scalar("FILE_NOT_FOUND"),
//                 )
//             },
//             Self::LastCommitNotFound(path) => {
//                 FieldError::new(
//                     format!("Last commit not found: {:?}", path),
//                     Value::scalar("LAST_COMMIT_NOT_FOUND"),
//                 )
//             },
//             Self::ProjectName(error) => {
//                 match error {
//                     ProjectNameValidation::TooLong(error) => {
//                         FieldError::new(
//                             error,
//                             Value::scalar("PROJECT_NAME_TOO_LONG"),
//                         )
//                     },
//                 }
//             },
//             Self::ProjectDomain(error) => {
//                 match error {
//                     ProjectDomainValidation::TooLong(error) => {
//                         FieldError::new(
//                             error,
//                             Value::scalar("PROJECT_DOMAIN_TOO_LONG"),
//                         )
//                     },
//                 }
//             },
//         }
//     }
// }
