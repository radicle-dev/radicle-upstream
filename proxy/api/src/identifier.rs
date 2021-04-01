//! An `Identifier` is the combination of a user handle and the [`radicle_daemon::PeerId`] that
//! identifies the user.

use std::{fmt, str::FromStr};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use radicle_daemon::{identities::Person, librad::peer::conversion, PeerId};

/// Errors captured when parsing a shareable identifier of the form `<handle>@<urn>`.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Could not parse the URN portion of the identifier.
    #[error(transparent)]
    Peer(#[from] conversion::Error),
    /// The identifier contained more than one '@' symbol.
    #[error("shared identifier contains more than one '@' symbol")]
    AtSplitError,
    /// The handle portion of the identifier was missing.
    #[error("shared identifier is missing the handle to the left of the '@' symbol")]
    MissingHandle,
    /// The urn portion of the identifier was missing.
    #[error("shared identifier is missing the URN to the right of the '@' symbol")]
    MissingPeerId,
}

/// The combination of a handle and a urn give user's a structure for sharing their identities.
#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    /// The user's chosen handle.
    pub handle: String,
    /// The unique identifier of the user.
    pub peer_id: PeerId,
}

impl From<(PeerId, Person)> for Identifier {
    fn from((peer_id, user): (PeerId, Person)) -> Self {
        Self {
            handle: user.subject().name.to_string(),
            peer_id,
        }
    }
}

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sub = s.split('@');
        let handle = sub.next();
        let peer_id = sub.next();

        if sub.count() != 0 {
            return Err(ParseError::AtSplitError);
        }

        let handle = handle.ok_or(ParseError::MissingHandle)?.to_string();
        let peer_id = peer_id
            .ok_or(ParseError::MissingPeerId)
            .and_then(|peer_id| Ok(peer_id.parse()?))?;

        Ok(Self { handle, peer_id })
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.handle, self.peer_id)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// A phantom Visitor for serde to deserialize.
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Identifier;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a shared identifier of the form <handle>@<urn>")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(IdVisitor)
    }
}
