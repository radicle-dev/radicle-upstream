// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Machinery to signal significant events to clients.

use serde::Serialize;
use std::{collections::HashMap, time::SystemTime};

use link_crypto::PeerId;
use link_identities::git::Urn;
use radicle_daemon::request::{RequestState, SomeRequest, Status as PeerRequestStatus};
use radicle_git_ext::Oid;

/// Significant events happening during proxy runtime.
#[derive(Clone, Debug)]
pub enum Notification {
    /// Event observed about the local peer.
    LocalPeer(LocalPeer),
}

/// Event observed about the local peer.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LocalPeer {
    ProjectUpdated {
        provider: PeerId,
        urn: Urn,
    },
    /// A request for a project was created and is pending submission to the network
    #[serde(rename_all = "camelCase")]
    RequestCreated {
        /// Urn of the project.
        urn: Urn,
    },
    /// A request for a project was cloned successfully.
    #[serde(rename_all = "camelCase")]
    RequestCloned {
        /// Origin the project was cloned from.
        peer: PeerId,
        /// Urn of the cloned project.
        urn: Urn,
    },
    /// A request for a project was queried on the network.
    #[serde(rename_all = "camelCase")]
    RequestQueried {
        /// Urn of the queried project.
        urn: Urn,
    },
    /// A request for a project timed out.
    #[serde(rename_all = "camelCase")]
    RequestTimedOut {
        /// Urn of the timed out project.
        urn: Urn,
    },
    /// Transition between two statuses occurred.
    #[serde(rename_all = "camelCase")]
    StatusChanged {
        /// The [`radicle_daemon::PeerStatus`] before.
        old: radicle_daemon::PeerStatus,
        /// The new [`radicle_daemon::PeerStatus`].
        new: radicle_daemon::PeerStatus,
    },
    WaitingRoomTransition {
        event: radicle_daemon::peer::WaitingRoomEvent,
        state_before: SerializableWaitingRoomState,
        state_after: SerializableWaitingRoomState,
        timestamp: u128,
    },
}

#[allow(clippy::wildcard_enum_match_arm)]
pub fn from_peer_event(event: radicle_daemon::PeerEvent) -> Option<Notification> {
    match event {
        radicle_daemon::PeerEvent::GossipFetched {
            provider, gossip, ..
        } => Some(Notification::LocalPeer(LocalPeer::ProjectUpdated {
            provider: provider.peer_id,
            urn: gossip.urn,
        })),
        radicle_daemon::PeerEvent::RequestCloned(urn, peer) => {
            Some(Notification::LocalPeer(LocalPeer::RequestCloned {
                peer,
                urn,
            }))
        },
        radicle_daemon::PeerEvent::RequestCreated(urn) => {
            Some(Notification::LocalPeer(LocalPeer::RequestCreated { urn }))
        },
        radicle_daemon::PeerEvent::RequestQueried(urn) => {
            Some(Notification::LocalPeer(LocalPeer::RequestQueried { urn }))
        },
        radicle_daemon::PeerEvent::RequestTimedOut(urn) => {
            Some(Notification::LocalPeer(LocalPeer::RequestTimedOut { urn }))
        },
        radicle_daemon::PeerEvent::StatusChanged { old, new } => {
            Some(Notification::LocalPeer(LocalPeer::StatusChanged {
                old,
                new,
            }))
        },
        radicle_daemon::PeerEvent::WaitingRoomTransition(t) => {
            let since_the_epoch = t
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            Some(Notification::LocalPeer(LocalPeer::WaitingRoomTransition {
                event: t.event,
                state_before: t.state_before.into(),
                state_after: t.state_after.into(),
                timestamp: since_the_epoch.as_millis(),
            }))
        },
        _ => None,
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SerializableWaitingRoomState(HashMap<String, SerializedRequestState>);

#[derive(Debug, Clone, Serialize)]
pub struct SerializedRequestState {
    state: String,
    peers: HashMap<PeerId, PeerRequestStatus>,
}

impl From<HashMap<Oid, SomeRequest<SystemTime>>> for SerializableWaitingRoomState {
    fn from(raw: HashMap<Oid, SomeRequest<SystemTime>>) -> Self {
        let inner: HashMap<String, SerializedRequestState> = raw
            .iter()
            .map(|(urn, request)| {
                (
                    urn.to_string(),
                    SerializedRequestState {
                        state: RequestState::from(request).to_string(),
                        peers: request
                            .peers()
                            .cloned()
                            .unwrap_or_else(std::collections::HashMap::new),
                    },
                )
            })
            .collect();
        Self(inner)
    }
}
