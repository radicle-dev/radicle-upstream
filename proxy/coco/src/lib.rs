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
    clippy::clone_on_ref_ptr,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::missing_inline_in_public_items,
    clippy::multiple_crate_versions
)]

pub use librad::git::local::url::LocalUrl;
pub use librad::hash::Hash;
pub use librad::meta::project::Project;
pub use librad::meta::user::User as MetaUser;
pub use librad::paths::Paths;
pub use librad::peer::PeerId;
pub use librad::uri::{self, RadUrn as Urn};

pub use radicle_git_helpers::remote_helper;

pub use radicle_surf::diff::{Diff, FileDiff};
pub use radicle_surf::vcs::git::Stats;

pub mod announcement;
pub mod config;
pub mod control;
mod error;
pub use error::Error;
pub mod git_helper;
mod identifier;
pub use identifier::Identifier;
pub mod keystore;
pub mod oid;
mod peer;
pub use peer::Peer;
mod state;
pub use state::{Lock, State};
pub mod project;

pub mod seed;
pub mod signer;

mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, into_branch_type, local_state, revisions, tags,
    tree, Blob, BlobContent, Branch, Commit, CommitHeader, Info, ObjectType, Person, Revision,
    Revisions, Tag, Tree, TreeEntry,
};

pub mod user;

use librad::keys;
use librad::net::discovery;
use librad::net::peer::PeerConfig;
use std::net::SocketAddr;

/// Constructs a [`Peer`] and [`State`] pair from a [`PeerConfig`].
///
/// # Errors
///
/// * peer construction from config fails.
/// * accept on the peer fails.
pub async fn try_from<I>(
    config: PeerConfig<discovery::Static<I, SocketAddr>, keys::SecretKey>,
    signer: librad::signer::BoxedSigner,
) -> Result<(Peer, Lock), Error>
where
    I: Iterator<Item = (PeerId, SocketAddr)> + Send + 'static,
{
    let peer = config.try_into_peer().await?;
    let (api, run_loop) = peer.accept()?;

    let api_subscriber = api.subscribe();

    let state = State::new(api, signer);
    let state = state::Lock::from(state);
    let peer = Peer::new(run_loop, api_subscriber, state.clone()).await;

    Ok((peer, state))
}
