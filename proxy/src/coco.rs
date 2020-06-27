//! Abstractions and utilities for git interactions through the API.

use async_trait::async_trait;

use librad::keys;
use librad::meta::entity;
use librad::meta::user;
use librad::surf;
use librad::uri::RadUrn;

mod peer;
pub use peer::{
    create_peer_api, get_project, get_user, init_project, init_user, list_projects, list_users,
    replicate_platinum, setup_fixtures, verify_user, with_browser, PeerApi, User,
};

/// Module that captures all types and functions for source code.
mod source;
pub use source::{
    blob, branches, commit, commit_header, commits, local_state, tags, tree, Blob, BlobContent,
    Branch, Commit, CommitHeader, Info, ObjectType, Person, Tag, Tree, TreeEntry,
};
pub use surf::diff::{Diff, FileDiff};

pub mod config;

/// Acting as a fake resolver where a User resolves to itself.
/// This allows us to check the history status of a single User.
/// TODO(finto): Remove this once Resolvers are complete.
struct FakeUserResolver(user::User<entity::Draft>);

#[async_trait]
impl entity::Resolver<user::User<entity::Draft>> for FakeUserResolver {
    async fn resolve(&self, _uri: &RadUrn) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }

    async fn resolve_revision(
        &self,
        _uri: &RadUrn,
        _revision: u64,
    ) -> Result<user::User<entity::Draft>, entity::Error> {
        Ok(self.0.clone())
    }
}

// TODO(xla): Transform into Peer::create_user.
/// Constructs a fake user to be used as an owner of projects until we have more permanent key and
/// user management.
pub async fn fake_owner(key: keys::SecretKey) -> User {
    let mut user = user::User::<entity::Draft>::create("cloudhead".into(), key.public())
        .expect("unable to create user");
    user.sign_owned(&key).expect("unable to sign user");
    verify_user(user).await.expect("failed to verify user")
}
