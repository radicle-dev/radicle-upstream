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
    clippy::multiple_crate_versions,
    clippy::multiple_inherent_impl,
    clippy::similar_names,
    clippy::too_many_lines
)]
#![feature(hash_set_entry)]
#![feature(or_patterns)]

use std::net::SocketAddr;

pub use librad::{
    git::{include, local::url::LocalUrl},
    hash::Hash,
    keys,
    meta::{project::Project, user::User as MetaUser},
    net::{self, discovery},
    paths::Paths,
    peer::PeerId,
    uri::{self, RadUrn as Urn},
};

pub use radicle_git_helpers::remote_helper;

pub use radicle_surf::{
    diff::{Diff, FileDiff},
    vcs::git::Stats,
};

pub mod config;
pub mod control;
mod convert;
pub mod git_helper;
mod identifier;
pub use identifier::Identifier;
pub mod keystore;
pub mod oid;
pub mod peer;
pub use peer::{
    AnnounceConfig, Control as PeerControl, Event as PeerEvent, Peer, RunConfig, SyncConfig,
};
pub mod state;
pub use state::State;
pub mod project;
pub mod request;

pub mod seed;
pub mod signer;

pub mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, into_branch_type, local_state, revisions, tags,
    tree, Blob, BlobContent, Branch, Commit, CommitHeader, Info, ObjectType, Person, Revision,
    Revisions, Tag, Tree, TreeEntry,
};

mod spawn_abortable;
pub use spawn_abortable::{Error as SpawnAbortableError, SpawnAbortable};

pub mod user;

/// Constructs a [`Peer`] and [`State`] pair from a [`net::peer::PeerConfig`].
///
/// # Errors
///
/// * peer construction from config fails.
/// * accept on the peer fails.
pub async fn into_peer_state<I>(
    config: net::peer::PeerConfig<discovery::Static<I, SocketAddr>, keys::SecretKey>,
    signer: librad::signer::BoxedSigner,
    store: kv::Store,
    run_config: RunConfig,
) -> Result<(Peer, State), state::Error>
where
    I: Iterator<Item = (PeerId, SocketAddr)> + Send + 'static,
{
    let peer = config.try_into_peer().await?;
    let (api, run_loop) = peer.accept()?;

    let state = State::new(api, signer);
    let peer = Peer::new(run_loop, state.clone(), store, run_config);

    Ok((peer, state))
}
