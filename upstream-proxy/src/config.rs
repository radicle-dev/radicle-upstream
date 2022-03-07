// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Configuration vital to the setup and alteration of the application.

use std::{env, io, path};

use directories::ProjectDirs;

use librad::profile::ProfileId;

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
}

/// Returns the directories to locate all application state.
#[must_use]
pub fn dirs() -> ProjectDirs {
    ProjectDirs::from("xyz", "radicle", "radicle-upstream").expect("couldn't build dirs")
}

/// Returns the directory for the application store
pub fn store_dir(profile_id: &ProfileId, lnk_home: Option<&path::Path>) -> path::PathBuf {
    let store_root = match lnk_home {
        None => {
            let dirs = dirs();
            dirs.data_dir().to_path_buf()
        },
        Some(root) => root.to_path_buf(),
    };
    store_root.join(profile_id.as_str()).join("store")
}
