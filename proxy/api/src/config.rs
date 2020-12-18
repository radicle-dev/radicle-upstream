//! Configuration vital to the setup and alteration of the application.

use std::{env, io, path};

/// Errors when setting up configuration paths and variables.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Exception during I/O actions.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Couldn't join file paths to calculate a new PATH variable.
    #[error(transparent)]
    Path(#[from] env::JoinPathsError),
    /// Couldn't get an environment variable's value.
    #[error(transparent)]
    Var(#[from] env::VarError),
    /// We couldn't get the executable path.
    #[error("we were not able to find the executable path's parent directory")]
    MissingExePath(path::PathBuf),
}

/// Returns the directories to locate all application state specific to the current identity.
pub fn id_dirs() -> Result<path::PathBuf, Error> {
    let home_dir = std::env::var("RAD_HOME").unwrap_or(std::env::var("HOME")?);

    Ok(path::Path::new(&home_dir).join(".radicle/identities/current"))
}

/// Returns the path to a folder containing helper binaries.
///
/// # Errors
///
///   * Could not get the user home path from the HOME env variable
pub fn bin_dir() -> Result<path::PathBuf, Error> {
    let home_dir = std::env::var("RAD_HOME").unwrap_or(std::env::var("HOME")?);

    Ok(path::Path::new(&home_dir).join(".radicle/bin"))
}

/// Returns path to the directory containing the proxy binary.
///
/// # Errors
///
///   * Could not determine the path of this binary.
pub fn proxy_path() -> Result<path::PathBuf, Error> {
    let exe_path = std::env::current_exe()?;

    Ok(exe_path
        .parent()
        .ok_or_else(|| Error::MissingExePath(exe_path.clone()))?
        .to_owned())
}
