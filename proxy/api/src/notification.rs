//! Machinery to signal significant events to clients.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use serde::Serialize;
use tokio::sync::{mpsc, RwLock};

use coco::{convert::MaybeFrom, PeerEvent, PeerStatus};

/// Significant events happening during proxy runtime.
#[derive(Clone, Debug)]
pub enum Notification {
    /// Event observed about the local peer.
    LocalPeer(LocalPeer),
}

/// Event observed about the local peer.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LocalPeer {
    /// Announced updates on the network.
    #[serde(rename_all = "camelCase")]
    Announced {
        /// List of new refs that have been announced.
        updates: coco::AnnouncementUpdates,
    },
    /// Sync with a peer has completed.
    #[serde(rename_all = "camelCase")]
    PeerSynced {
        /// [`PeerId`] of the synced peer.
        peer_id: coco::PeerId,
    },
    /// Transition between two statuses occurred.
    #[serde(rename_all = "camelCase")]
    StatusChanged {
        /// The [`PeerStatus`] before.
        old: PeerStatus,
        /// The new [`PeerStatus`].
        new: PeerStatus,
    },
}

#[allow(clippy::wildcard_enum_match_arm)]
impl MaybeFrom<PeerEvent> for Notification {
    fn maybe_from(event: PeerEvent) -> Option<Self> {
        match event {
            PeerEvent::Announced(updates) => {
                Some(Self::LocalPeer(LocalPeer::Announced { updates }))
            },
            PeerEvent::PeerSynced(peer_id) => {
                Some(Self::LocalPeer(LocalPeer::PeerSynced { peer_id }))
            },
            PeerEvent::StatusChanged(old, new) => {
                Some(Self::LocalPeer(LocalPeer::StatusChanged { old, new }))
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
