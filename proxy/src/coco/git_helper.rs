//! git-remote-rad git helper related functionality.

use crate::error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path;

/// Filename of the git helper binary.
pub const GIT_REMOTE_RAD: &str = "git-remote-rad";

/// Check if a path contains an executable file.
fn is_executable(path: &std::path::PathBuf) -> Result<bool, error::Error> {
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
pub fn setup(src_dir: &path::PathBuf, dst_dir: &path::PathBuf) -> Result<(), error::Error> {
    log::info!("Making sure git-remote-rad helper is set up");

    let helper_bin_src = src_dir.join(GIT_REMOTE_RAD);
    let helper_bin_dst = dst_dir.join(GIT_REMOTE_RAD);

    if helper_bin_dst.exists() && is_executable(&helper_bin_dst)? {
        log::debug!("Git helper already exists at: {:?}", helper_bin_dst);
        return Ok(());
    }

    fs::create_dir_all(dst_dir)?;
    fs::copy(helper_bin_src, helper_bin_dst.clone())?;
    log::debug!("Copied git-remote-rad helper to: {:?}", helper_bin_dst);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::coco;
    use crate::error;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // when the ~/.radicle/bin directory doesn't exist at all
    #[tokio::test]
    async fn setup_creates_destination_directory_if_none_exists() -> Result<(), error::Error> {
        let tmp_src_dir = tempfile::tempdir().expect("failed to create source tempdir");
        let src_git_helper_bin_path = tmp_src_dir.path().join(coco::git_helper::GIT_REMOTE_RAD);
        let file = fs::File::create(src_git_helper_bin_path.clone())
            .expect("failed to create mock binary");
        let mut src_permissions = file.metadata()?.permissions();
        src_permissions.set_mode(0o755);

        fs::set_permissions(src_git_helper_bin_path, src_permissions)?;

        let tmp_dst_dir = tempfile::tempdir().expect("failed to create destination tempdir");
        let dst_full_path = tmp_dst_dir.path().join(".radicle/bin");
        coco::git_helper::setup(&tmp_src_dir.path().to_path_buf(), &dst_full_path)?;

        let dst_git_helper_bin_path = dst_full_path.join(coco::git_helper::GIT_REMOTE_RAD);

        let dst_metadata = dst_git_helper_bin_path.metadata()?;
        let dst_permissions = dst_metadata.permissions();
        assert_eq!(dst_permissions.mode(), 0o100_755);

        Ok(())
    }

    // the ~/.radicle/bin directory exists, but there is no helper binary in it
    #[tokio::test]
    async fn setup_copies_binary_if_none_exists() -> Result<(), error::Error> {
        let tmp_src_dir = tempfile::tempdir().expect("failed to create source tempdir");
        let src_git_helper_bin_path = tmp_src_dir.path().join(coco::git_helper::GIT_REMOTE_RAD);
        let file = fs::File::create(src_git_helper_bin_path.clone())
            .expect("failed to create mock binary");
        let mut src_permissions = file.metadata()?.permissions();
        src_permissions.set_mode(0o755);

        fs::set_permissions(src_git_helper_bin_path, src_permissions)?;

        let tmp_dst_dir = tempfile::tempdir().expect("failed to create destination tempdir");

        let dst_full_path = tmp_dst_dir.path().join(".radicle/bin");
        fs::create_dir_all(dst_full_path.clone())?;

        coco::git_helper::setup(&tmp_src_dir.path().to_path_buf(), &dst_full_path)?;

        let dst_git_helper_bin_path = dst_full_path.join(coco::git_helper::GIT_REMOTE_RAD);

        let dst_metadata = dst_git_helper_bin_path.metadata()?;
        let dst_permissions = dst_metadata.permissions();
        assert_eq!(dst_permissions.mode(), 0o100_755);

        Ok(())
    }

    // the ~/.radicle/bin directory exists, and the binary is present, but not executable
    #[tokio::test]
    async fn setup_makes_binary_executable() -> Result<(), error::Error> {
        let tmp_src_dir = tempfile::tempdir().expect("failed to create source tempdir");
        let src_git_helper_bin_path = tmp_src_dir.path().join(coco::git_helper::GIT_REMOTE_RAD);
        let file = fs::File::create(src_git_helper_bin_path.clone())
            .expect("failed to create mock binary");
        let mut src_permissions = file.metadata()?.permissions();
        src_permissions.set_mode(0o755);

        fs::set_permissions(src_git_helper_bin_path, src_permissions)?;

        let tmp_dst_dir = tempfile::tempdir().expect("failed to create destination tempdir");

        let dst_full_path = tmp_dst_dir.path().join(".radicle/bin");
        fs::create_dir_all(dst_full_path.clone())?;

        let dst_file_path = dst_full_path.join(coco::git_helper::GIT_REMOTE_RAD);
        let dst_file =
            fs::File::create(dst_file_path.clone()).expect("failed to create mock binary");
        let mut dst_permissions = dst_file.metadata()?.permissions();
        dst_permissions.set_mode(0o644);

        fs::set_permissions(dst_file_path, dst_permissions)?;

        coco::git_helper::setup(&tmp_src_dir.path().to_path_buf(), &dst_full_path)?;

        let dst_git_helper_bin_path = dst_full_path.join(coco::git_helper::GIT_REMOTE_RAD);

        let dst_metadata = dst_git_helper_bin_path.metadata()?;
        let dst_permissions = dst_metadata.permissions();
        assert_eq!(dst_permissions.mode(), 0o100_755);

        Ok(())
    }
}
