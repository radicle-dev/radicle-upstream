//! Abstractions and utilities for git interactions through the API.

pub use librad::hash::Hash;
pub use librad::keys::SignError;
pub use librad::meta::project::Project;
pub use librad::peer::PeerId;
pub use librad::uri::{self, RadUrn as Urn};

pub use radicle_surf::diff::{Diff, FileDiff};
pub use radicle_surf::vcs::git::Stats;

pub mod config;
pub mod control;
mod signer;
pub use signer::{
    Error as SignerError, Reset as ResetSigner, SecUtf8, Signer, Store as StoreSigner,
};
mod peer;
pub use peer::{verify_user, Api, User};

pub mod project;

mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, into_branch_type, local_state, revisions, tags,
    tree, Blob, BlobContent, Branch, Commit, CommitHeader, Info, ObjectType, Person, Revision,
    Revisions, Tag, Tree, TreeEntry,
};
