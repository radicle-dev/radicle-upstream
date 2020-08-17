//! git-remote-rad git helper related functionality.

use crate::config;

use crate::error;
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// Filename of the git helper binary.
pub const HELPER_BINARY_NAME: &str = "git-remote-rad";

fn is_executable(path: std::path::PathBuf) -> Result<bool, error::Error> {
    let metadata = path.metadata()?;
    let permissions = metadata.permissions();
    Ok(permissions.mode() & 0o111 != 0)
}

/// Checks if the git-remote-rad helper is in a stable location and has the
/// executable flag, if not copies the executable to the right place.
///
/// # Errors
///
///   * Could not get the path to directory where helper binaries should be stored.
///   * Could not get the current working directory.
///   * Could not create the path to binary directory.
///   * Could not copy helper executable to the binary directory.
pub fn setup() -> Result<(), error::Error> {
    log::info!("Making sure git-remote-rad helper is set up");

    let bin_dir = config::bin_dir()?;
    let full_dest_path = bin_dir.join(HELPER_BINARY_NAME);

    if full_dest_path.exists() && is_executable(full_dest_path.clone())? {
        log::debug!("Git helper already exists at: {:?}", full_dest_path);
        return Ok(());
    }

    let proxy_path = config::proxy_path()?;

    fs::create_dir_all(bin_dir)?;
    fs::copy(proxy_path.join(HELPER_BINARY_NAME), full_dest_path.clone())?;
    log::debug!("Copied git-remote-rad helper to: {:?}", full_dest_path);

    Ok(())
}
