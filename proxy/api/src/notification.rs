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

/// Significant events happening during proxy runtime.
#[derive(Clone, Debug, Serialize)]
pub enum Notification {
    /// Event for peer status updates.
    LocalPeerStatus(coco::Status),
}

/// Manage active subscriptions and broadcast [`Notification`]s.
#[derive(Clone, Debug, Default)]
pub struct Subscriptions {
    /// Generator of unqiue keys for subscriptions.
    next_id: Arc<AtomicUsize>,
    /// Active subscribers.
    subs: Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Notification>>>>,
    /// Cache for last event sent through this stream. This allows us to replay the last event
    /// whenever a new consumer starts listening on this stream.
    last_state: Arc<RwLock<Option<Notification>>>,
}

impl Subscriptions {
    /// Broadcast [`Notification`] to all active subscriptions.
    pub async fn broadcast(&self, notification: Notification) {
        *self.last_state.write().await = Some(notification.clone());
        // We use retain to discard all closed subscriptions.
        self.subs
            .write()
            .await
            .retain(|_id, sender| sender.send(notification.clone()).is_ok());
    }

    /// Set up a new subscription, ready to receive [`Notification`].
    pub async fn subscribe(&self) -> mpsc::UnboundedReceiver<Notification> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::unbounded_channel();

        if let Some(last_state) = &*self.last_state.read().await {
            sender.send(last_state.clone()).ok();
        };
        self.subs.write().await.insert(id, sender);

        receiver
    }
}
