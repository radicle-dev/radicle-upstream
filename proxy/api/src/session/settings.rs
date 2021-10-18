// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! User controlled parameters for application appearance, behaviour and state.
use serde::{Deserialize, Serialize};

/// User controlled parameters for application appearance, behaviour and state.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// User-determined p2p parameters.
    pub coco: CoCo,
}

/// `CoCo` config parameters subject to user preferences
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CoCo {
    /// Peers to connect to at startup.
    pub seeds: Vec<String>,
}

/// Default seeds placeholder, don't use this for setting default seeds.
/// Set the default seeds by passing a "--default-seed" CLI flag.
impl Default for CoCo {
    fn default() -> Self {
        Self { seeds: vec![] }
    }
}
