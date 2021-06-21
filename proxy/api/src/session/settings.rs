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
    /// Currently set appearance parameters.
    pub appearance: Appearance,
    /// User-determined p2p parameters.
    pub coco: CoCo,
    #[serde(default)]
    pub feature_flags: FeatureFlags,
}

/// Knobs for the look and feel.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    /// Currently active color scheme.
    pub theme: Theme,
    /// Currently active UI font.
    pub ui_font: UIFont,
    /// Currently active code font.
    pub code_font: CodeFont,
    /// User dismissable hints.
    pub hints: Hints,
}

/// Color schemes available.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    /// A dark theme.
    Dark,
    /// A light theme.
    Light,
    /// A h4x0r theme.
    H4x0r,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

/// uiFont schemes available.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UIFont {
    /// The Inter font.
    Inter,
    /// The system font.
    System,
}

impl Default for UIFont {
    fn default() -> Self {
        Self::Inter
    }
}

/// codeFont schemes available.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CodeFont {
    /// The Source Code font.
    SourceCode,
    /// The system font.
    System,
}

impl Default for CodeFont {
    fn default() -> Self {
        Self::SourceCode
    }
}

/// User dismissable textual hints.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hints {
    /// Whether to show hints about how to set up the remote helper.
    pub show_remote_helper: bool,
}

impl Default for Hints {
    fn default() -> Self {
        Self {
            show_remote_helper: true,
        }
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FeatureFlags {
    /// Whether the funding feature is enabled or disabled.
    pub funding: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self { funding: false }
    }
}
