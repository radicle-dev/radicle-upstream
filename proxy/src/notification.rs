//! Infrastructure to signal to clients significant events.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::registry;

/// All variants of notifications the proxy can emit.
#[derive(Clone, Debug)]
pub enum Notification {
    /// Carries a updated [`registry::Transaction`].
    Transaction(registry::Transaction),
}

/// Manage and active subscriptions and broadcast new notifications.
#[derive(Clone, Default)]
pub struct Subscriptions {
    /// We use this to have unique keys for the stored subscriptions.
    next_id: Arc<AtomicUsize>,
    /// The active subscriptions.
    subs: Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Notification>>>>,
}

impl Subscriptions {
    /// Distribute a new [`Notification`] to all active subscriptions.
    pub async fn broadcast(&self, notification: Notification) {
        // We use retain to remove all closed senders.
        self.subs
            .lock()
            .await
            .retain(|_id, sender| sender.send(notification.clone()).is_ok());
    }

    /// Set up a new subscription.
    pub async fn subscribe(&self) -> mpsc::UnboundedReceiver<Notification> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (sender, receiver) = mpsc::unbounded_channel();

        self.subs.lock().await.insert(id, sender);

        receiver
    }
}
