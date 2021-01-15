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
}

/// Knobs for the look and feel.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    /// Currently active color scheme.
    pub theme: Theme,
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
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
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

impl Default for CoCo {
    fn default() -> Self {
        Self { seeds: vec![] }
    }
}
