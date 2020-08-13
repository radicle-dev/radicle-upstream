//! Project creation data and functions.

use std::ffi;

/// Module concerned with creating new projects and repositories.
pub mod create;
pub use create::{Create, Repo};

/// Module concerned with checkout out working copies of projects, as git repositories.
pub mod checkout;
pub use checkout::Checkout;

/// The default name for a user's remote, which is `"rad"`.
const RAD_REMOTE: &str = "rad";

/// Specify how to create the git credential helper argument for a [`Checkout`]
enum Credential {
    /// Plain-text password. You've been warned!
    Password(String),
}

impl Credential {
    /// Convert the `Credential` into the git credential helper.
    fn to_helper(&self) -> String {
        match self {
            Self::Password(pass) => format!(
                "credential.helper=!f() {{ test \"$1\" = get && echo \"password={}\"; }}; f",
                pass
            ),
        }
    }
}

/// Module for separating out the bin path Error.
pub mod bin_path {
    use std::env;
    use std::io;

    /// Failure to calculate the `PATH` for using the rad remote helper.
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        /// We failed to join the paths for the rad remote executable.
        #[error(transparent)]
        Path(#[from] env::JoinPathsError),
        /// There was an I/O error when getting the rad remote helper.
        #[error("failed to get the executable path for the rad remote helper")]
        Io(#[from] io::Error),
        /// The parent of the current exe path was missing.
        #[error("failed to get the executable path for the rad remote helper")]
        MissingExePath,
    }
}

/// Set up the PATH env variable used for running the git command that need the rad remote helper.
fn default_bin_path() -> Result<ffi::OsString, bin_path::Error> {
    let exe_path = std::env::current_exe()?;
    let exe_path = exe_path.parent().ok_or(bin_path::Error::MissingExePath)?;

    let paths = std::env::var_os("PATH").map_or(vec![exe_path.to_path_buf()], |path| {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        paths.push(exe_path.to_path_buf());
        paths.reverse();
        paths
    });

    Ok(std::env::join_paths(paths)?)
}
