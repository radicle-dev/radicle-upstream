//! Abstractions and utilities for git interactions through the API.

/// Module that captures all the functions for working with `librad`'s [`PeerApi`].
mod peer;
pub use peer::{
    create_peer_api, get_project, get_user, init_project, init_user, list_projects, list_users,
    verify_user, with_browser, PeerApi, User,
};

/// Module that captures all types and functions for source code.
mod source;
pub use radicle_surf::diff::{Diff, FileDiff};
pub use source::{
    blob, branches, commit, commit_header, commits, local_state, tags, tree, Blob, BlobContent,
    Branch, Commit, CommitHeader, Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

pub mod config;

/// Moule that captures fixture setup.
pub mod control;
