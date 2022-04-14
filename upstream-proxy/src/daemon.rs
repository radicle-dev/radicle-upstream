// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Abstractions and utilities to run and interact with link and surf.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::unwrap_used,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![allow(clippy::similar_names, clippy::too_many_lines)]

pub use librad::{
    self, crypto,
    git::{self, identities::local::LocalIdentity, include, local::url::LocalUrl, Urn},
    identities::{self, Person, Project},
    net::{self, discovery},
    paths::Paths,
    profile, PeerId,
};

pub use librad::git_ext;

pub use radicle_git_helpers::remote_helper;

pub mod config;
pub mod convert;
pub mod peer;
pub mod project;
pub mod request;
pub mod state;
pub use peer::{Control as PeerControl, Event as PeerEvent, Peer, RunConfig, Status as PeerStatus};
