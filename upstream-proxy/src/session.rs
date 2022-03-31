// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Legacy "session". Now only holds settings

use serde::{Deserialize, Serialize};

use crate::error;

pub mod settings;

/// Name for the storage bucket used for all session data.
const BUCKET_NAME: &str = "session";
/// Name of the item used for the currently active session.
const KEY_CURRENT: &str = "current";

/// Container for all local state.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// User controlled parameters to control the behaviour and state of the application.
    pub settings: settings::Settings,
}

/// Get the current session if present
///
/// # Errors
///
/// Errors if we cannot read data from the store.
pub fn get_current(store: &kv::Store) -> Result<Option<Session>, error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .get(KEY_CURRENT)?
        .map(|json| json.0))
}

/// Initialize the current session with the given identity, default settings and default seeds.
///
/// # Errors
///
/// * Errors when we cannot write to the store.
pub fn initialize(store: &kv::Store, default_seeds: &[String]) -> Result<Session, error::Error> {
    let mut session = Session {
        settings: settings::Settings::default(),
    };

    session.settings.coco.seeds = default_seeds.to_owned();

    set_current(store, session.clone())?;
    Ok(session)
}

/// Stores the session as the current session
fn set_current(store: &kv::Store, sess: Session) -> Result<(), error::Error> {
    Ok(store
        .bucket::<&str, kv::Json<Session>>(Some(BUCKET_NAME))?
        .set(KEY_CURRENT, kv::Json(sess))?)
}
