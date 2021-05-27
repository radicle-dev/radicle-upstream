//! Machinery to signal significant events to clients.

use radicle_daemon::request::{RequestState, SomeRequest, Status as PeerRequestStatus};
use radicle_git_ext::Oid;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::SystemTime,
};

use serde::Serialize;
use tokio::sync::{mpsc, RwLock};

use radicle_daemon::{convert::MaybeFrom, PeerEvent, PeerId, PeerStatus, Urn};

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
        /// The [`PeerStatus`] before.
        old: PeerStatus,
        /// The new [`PeerStatus`].
        new: PeerStatus,
    },
    WaitingRoomTransition {
        event: radicle_daemon::peer::WaitingRoomEvent,
        state_before: SerializableWaitingRoomState,
        state_after: SerializableWaitingRoomState,
        timestamp: u128,
    },
}

#[allow(clippy::wildcard_enum_match_arm)]
impl MaybeFrom<PeerEvent> for Notification {
    fn maybe_from(event: PeerEvent) -> Option<Self> {
        match event {
            PeerEvent::GossipFetched {
                provider, gossip, ..
            } => Some(Self::LocalPeer(LocalPeer::ProjectUpdated {
                provider: provider.peer_id,
                urn: gossip.urn,
            })),
            PeerEvent::RequestCloned(urn, peer) => {
                Some(Self::LocalPeer(LocalPeer::RequestCloned { peer, urn }))
            },
            PeerEvent::RequestCreated(urn) => {
                Some(Self::LocalPeer(LocalPeer::RequestCreated { urn }))
            },
            PeerEvent::RequestQueried(urn) => {
                Some(Self::LocalPeer(LocalPeer::RequestQueried { urn }))
            },
            PeerEvent::RequestTimedOut(urn) => {
                Some(Self::LocalPeer(LocalPeer::RequestTimedOut { urn }))
            },
            PeerEvent::StatusChanged { old, new } => {
                Some(Self::LocalPeer(LocalPeer::StatusChanged { old, new }))
            },
            PeerEvent::WaitingRoomTransition(t) => {
                let since_the_epoch = t
                    .timestamp
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                Some(Self::LocalPeer(LocalPeer::WaitingRoomTransition {
                    event: t.event,
                    state_before: t.state_before.into(),
                    state_after: t.state_after.into(),
                    timestamp: since_the_epoch.as_millis(),
                }))
            },
            _ => None,
        }
    }
}

/// Manage active subscriptions and broadcast [`Notification`]s.
#[derive(Clone, Debug, Default)]
pub struct Subscriptions {
    /// Generator of unqiue keys for subscriptions.
    next_id: Arc<AtomicUsize>,
    /// Active subscribers.
    subs: Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Notification>>>>,
}

impl Subscriptions {
    /// Broadcast [`Notification`] to all active subscriptions.
    pub async fn broadcast(&self, notification: Notification) {
        // We use retain to discard all closed subscriptions.
        self.subs
            .write()
            .await
            .retain(|_id, sender| sender.send(notification.clone()).is_ok());
    }

    /// Drop all stored senders, which terminates associated receivers and their streams.
    pub async fn clear(&self) {
        self.subs.write().await.clear();
    }

    /// Set up a new subscription, ready to receive [`Notification`].
    pub async fn subscribe(&self) -> mpsc::UnboundedReceiver<Notification> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::unbounded_channel();

        self.subs.write().await.insert(id, sender);

        receiver
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
