//! Machinery to signal significant events to clients.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Serialize)]
pub enum Notification {}

pub struct Subscriptions {
    next_id: Arc<AtomicUsize>,
    subs: Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Notification>>>>,
}

impl Subscriptions {
    pub async fn broadcast(&self, notification: Notification) {
        // We use retain to remain all closed subscriptions.
        self.subs
            .write()
            .await
            .retain(|_id, sender| sender.send(notification.clone()).is_ok());
    }

    pub async fn subscribe(&self) -> mpsc::UnboundedReceiver<Notification> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::unbounded_channel();

        self.subs.write().await.insert(id, sender);

        receiver
    }
}
