//! git-remote-rad git helper related functionality.

use crate::config;

use crate::error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Returns true if it is an executable.
trait IsExecutable {
    fn is_executable(&self) -> bool;
}

/// Implements isExecutable for Path.
impl IsExecutable for Path {
    fn is_executable(&self) -> bool {
        let metadata = match self.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return false,
        };
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    }
}

/// Checks if the git-remote-rad helper is in a stable location and has the
/// executable flag, if not copies the executable to the right place.
pub fn setup() -> Result<(), error::Error> {
    log::info!("Making sure git-remote-rad helper is set up");

    let helper_binary_name = "git-remote-rad";

    let bin_dir = config::bin_dir()?;
    let full_dest_path = bin_dir.join(helper_binary_name);

    if full_dest_path.is_executable() {
        log::debug!("Git helper already exists at: {:?}", full_dest_path);
        return Ok(());
    }

    let proxy_path = config::proxy_path()?;

    fs::create_dir_all(bin_dir)?;
    fs::copy(proxy_path.join(helper_binary_name), full_dest_path.clone())?;
    log::debug!("Copied git-remote-rad helper to: {:?}", full_dest_path);

    Ok(())
}
