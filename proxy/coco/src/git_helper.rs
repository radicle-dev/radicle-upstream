//! git-remote-rad git helper related functionality.

use std::{fs, io, os::unix::fs::PermissionsExt as _, os::unix::fs::symlink, path};

/// Git helper errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors from I/O operations.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Filename of the git helper binary.
pub const GIT_REMOTE_RAD: &str = "git-remote-rad";

/// Checks if the git-remote-rad helper is in a stable location and has the
/// executable flag, if not copies the executable to the right place.
///
/// # Errors
///
///   * Could not get the path to directory where helper binaries should be stored.
///   * Could not get the current working directory.
///   * Could not create the path to binary directory.
///   * Could not copy helper executable to the binary directory.
pub fn setup(src_dir: &path::PathBuf, dst_dir: &path::PathBuf) -> Result<(), Error> {
    let helper_bin_src = src_dir.join(GIT_REMOTE_RAD);
    let helper_bin_dst = dst_dir.join(GIT_REMOTE_RAD);

    fs::create_dir_all(dst_dir)?;
    fs::copy(helper_bin_src, helper_bin_dst.clone())?;
    let mut permissions = helper_bin_dst.metadata()?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&helper_bin_dst, permissions)?;

    log::info!("Copied git remote helper to: {:?}", helper_bin_dst);

    Ok(())
}

// TODO: this should live somewhere else, just here out of convinience
/// Set up electron and identities directory and current symlink
pub fn setup_directories(dst_dir: &path::PathBuf) -> Result<(), Error> {
    let electron_bin_dst = dst_dir.join("electron");
    fs::create_dir_all(electron_bin_dst.clone())?;
    log::info!("Created electron directory: {:?}", electron_bin_dst);
    // TODO: Obviously this should be the specific identity directory and not created here
    let id_bin_dst = dst_dir.join("identities/bla");
    fs::create_dir_all(id_bin_dst.clone())?;
    let symlink_dir = dst_dir.join("identities/current");
    // TODO: `symlink` does not work on windows, we'll need to add specific functions for it
    if !path::Path::new(&symlink_dir.clone()).exists() {
        symlink(id_bin_dst.clone(), symlink_dir)?;
        log::info!("Created current identities symlink to: {:?}", id_bin_dst);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, os::unix::fs::PermissionsExt as _};

    use super::Error;

    #[tokio::test]
    async fn ensure_setup_sets_up_remote_helper() -> Result<(), Error> {
        let tmp_src_dir = tempfile::tempdir().expect("failed to create source tempdir");
        let src_git_helper_bin_path = tmp_src_dir.path().join(super::GIT_REMOTE_RAD);
        let file = fs::File::create(src_git_helper_bin_path.clone())
            .expect("failed to create mock binary");
        let mut src_permissions = file.metadata()?.permissions();
        src_permissions.set_mode(0o644);

        fs::set_permissions(src_git_helper_bin_path, src_permissions)?;

        let tmp_dst_dir = tempfile::tempdir().expect("failed to create destination tempdir");
        let dst_full_path = tmp_dst_dir.path().join(".radicle/bin");
        super::setup(&tmp_src_dir.path().to_path_buf(), &dst_full_path)?;

        let dst_git_helper_bin_path = dst_full_path.join(super::GIT_REMOTE_RAD);

        let dst_metadata = dst_git_helper_bin_path.metadata()?;
        let dst_permissions = dst_metadata.permissions();
        assert_eq!(dst_permissions.mode(), 0o100_755);

        Ok(())
    }
}
