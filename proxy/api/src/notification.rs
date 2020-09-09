//! Machinery to signal significant events to clients.

use std::{
    collections::HashMap,
    net::SocketAddr,
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
    /// Our local peer started listening on a local socket.
    LocalPeerListening(SocketAddr),
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

    /// Set up a new subscription, ready to receive [`Notification`].
    pub async fn subscribe(&self) -> mpsc::UnboundedReceiver<Notification> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::unbounded_channel();

        self.subs.write().await.insert(id, sender);

        receiver
    }
}
