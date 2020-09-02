//! Abstractions and utilities to run and interact with link and surf.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::unwrap_used,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![allow(
    clippy::expect_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::missing_inline_in_public_items,
    clippy::multiple_crate_versions
)]

pub use librad::hash::Hash;
pub use librad::meta::project::Project;
pub use librad::peer::PeerId;
pub use librad::uri::{self, RadUrn as Urn};

pub use radicle_surf::diff::{Diff, FileDiff};
pub use radicle_surf::vcs::git::Stats;

pub mod announcement;
pub mod config;
pub mod control;

mod error;
pub use error::Error;

pub mod git_helper;
pub mod oid;
mod peer;
pub use peer::{verify_user, Api, User};
pub mod project;

pub mod seed;

mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, into_branch_type, local_state, revisions, tags,
    tree, Blob, BlobContent, Branch, Commit, CommitHeader, Info, ObjectType, Person, Revision,
    Revisions, Tag, Tree, TreeEntry,
};
