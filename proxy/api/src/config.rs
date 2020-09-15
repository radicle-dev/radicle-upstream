//! Configuration vital to the setup and alteration of the application.

use std::{env, ffi, io, path};

use directories::ProjectDirs;

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

/// Returns the directories to locate all application state.
#[must_use]
pub fn dirs() -> ProjectDirs {
    ProjectDirs::from("xyz", "radicle", "radicle-upstream").expect("couldn't build dirs")
}

/// Returns the path to a folder containing helper binaries.
///
/// # Errors
///
///   * Could not get the user home path from the HOME env variable
pub fn bin_dir() -> Result<path::PathBuf, Error> {
    let home_dir = std::env::var("HOME")?;

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

/// Include the path to the current running binary in the global system PATH.
///
/// # Errors
///
///   * Could not determine path to the proxy binary.
///   * Could not join paths.
pub fn default_bin_path() -> Result<ffi::OsString, Error> {
    let proxy_path = proxy_path()?;

    let paths = std::env::var_os("PATH").map_or(vec![proxy_path.to_owned()], |path| {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.push(proxy_path.to_owned());
        paths.reverse();
        paths
    });

    Ok(std::env::join_paths(paths)?)
}
