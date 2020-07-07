//! Abstractions and utilities for git interactions through the API.

pub use librad::uri::rad_urn::ParseError;
pub use librad::uri::RadUrn as Urn;

pub use radicle_surf::diff::{Diff, FileDiff};
pub use radicle_surf::vcs::git::Stats;

/// Module that captures all the functions for working with `librad`'s [`PeerApi`].
mod peer;
pub use peer::{
    create_peer_api, default_owner, get_project, get_user, init_owner, init_project, init_user,
    list_projects, list_users, set_default_owner, verify_user, with_browser, PeerApi, User,
};

/// Module that captures all types and functions for source code.
mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, local_state, tags, tree, Blob, BlobContent,
    Branch, Commit, CommitHeader, Info, ObjectType, Person, Tag, Tree, TreeEntry,
};

pub mod config;

/// Moule that captures fixture setup.
pub mod control;
