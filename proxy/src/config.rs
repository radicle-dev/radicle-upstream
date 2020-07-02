//! Configuration vital to the setup and alteration of the application.

use directories::ProjectDirs;

/// Returns the directories to locate all application state.
#[must_use]
pub fn dirs() -> ProjectDirs {
    ProjectDirs::from("xyz", "radicle", "radicle-upstream").expect("couldn't build dirs")
}
