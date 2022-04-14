// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Machinery to signal significant events to clients.

use serde::Serialize;
use std::{collections::HashMap, time::SystemTime};

use crate::daemon::request::{RequestState, SomeRequest, Status as PeerRequestStatus};
use link_crypto::PeerId;
use link_identities::git::Urn;
use radicle_git_ext::Oid;

/// Event observed about the local peer.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Notification {
    ProjectUpdated {
        /// URN of the project that was updated
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
        /// The [`crate::daemon::PeerStatus`] before.
        old: crate::daemon::PeerStatus,
        /// The new [`crate::daemon::PeerStatus`].
        new: crate::daemon::PeerStatus,
    },
    WaitingRoomTransition {
        event: crate::daemon::peer::WaitingRoomEvent,
        state_before: SerializableWaitingRoomState,
        state_after: SerializableWaitingRoomState,
        timestamp: u128,
    },
}

pub fn from_peer_event(event: crate::daemon::PeerEvent) -> Option<Notification> {
    match event {
        crate::daemon::PeerEvent::GossipFetched { gossip, .. } => {
            Some(Notification::ProjectUpdated { urn: gossip.urn })
        },
        crate::daemon::PeerEvent::RequestCloned(urn, peer) => {
            Some(Notification::RequestCloned { peer, urn })
        },
        crate::daemon::PeerEvent::RequestCreated(urn) => Some(Notification::RequestCreated { urn }),
        crate::daemon::PeerEvent::RequestQueried(urn) => Some(Notification::RequestQueried { urn }),
        crate::daemon::PeerEvent::RequestTimedOut(urn) => {
            Some(Notification::RequestTimedOut { urn })
        },
        crate::daemon::PeerEvent::StatusChanged { old, new } => {
            Some(Notification::StatusChanged { old, new })
        },
        crate::daemon::PeerEvent::WaitingRoomTransition(t) => {
            let since_the_epoch = t
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            Some(Notification::WaitingRoomTransition {
                event: t.event,
                state_before: t.state_before.into(),
                state_after: t.state_after.into(),
                timestamp: since_the_epoch.as_millis(),
            })
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
                        peers: request.peers().cloned().unwrap_or_default(),
                    },
                )
            })
            .collect();
        Self(inner)
    }
}
