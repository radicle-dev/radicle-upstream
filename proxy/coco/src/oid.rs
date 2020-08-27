//! Machinery to handle object ids conversion and serialization.

use std::convert::TryFrom;
use std::fmt;
use std::str;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use radicle_surf::vcs::git::git2;

/// Unique identity of any object (commit, tree, blob, tag).
#[derive(Clone, Debug, PartialEq)]
pub struct Oid(git2::Oid);

impl Oid {
    /// Consume to get the inner [`git2::Oid`].
    pub fn inner(self) -> git2::Oid {
        self.0
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

// Conversions.

impl From<git2::Oid> for Oid {
    fn from(oid: git2::Oid) -> Self {
        Self(oid)
    }
}

impl Into<git2::Oid> for Oid {
    fn into(self) -> git2::Oid {
        self.0
    }
}

impl str::FromStr for Oid {
    type Err = git2::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for Oid {
    type Error = git2::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(git2::Oid::from_str(value)?))
    }
}

// Serialization.

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(de::Error::custom)
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
