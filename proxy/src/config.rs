//! Configuration vital to the setup and alteration of the application.

use crate::error;
use directories::ProjectDirs;
use std::ffi;
use std::path;

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
pub fn bin_dir() -> Result<path::PathBuf, error::Error> {
    let home_dir = std::env::var("HOME").expect("Couldn't determine home dir from env");

    Ok(path::Path::new(&home_dir).join(".radicle/bin"))
}

/// Returns path to the directory containing the proxy binary.
///
/// # Errors
///
///   * Could not determine the path of this binary.
pub fn proxy_path() -> Result<path::PathBuf, error::Error> {
    let exe_path = std::env::current_exe()?;

    Ok(exe_path
        .parent()
        .expect("failed to find executable path")
        .to_owned())
}

/// Include the path to the current running binary in the global system PATH.
///
/// # Errors
///
///   * Could not determine path to the proxy binary.
///   * Could not join paths.
pub fn default_bin_path() -> Result<ffi::OsString, error::Error> {
    let proxy_path = proxy_path()?;

    let paths = std::env::var_os("PATH").map_or(vec![proxy_path.to_owned()], |path| {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.push(proxy_path.to_owned());
        paths.reverse();
        paths
    });

    Ok(std::env::join_paths(paths)?)
}
