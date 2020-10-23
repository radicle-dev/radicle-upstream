use futures::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, Notify};

#[derive(Clone)]
pub struct Config {
    pub key: Option<coco::keys::SecretKey>,
}

enum Message {
    Reset,
    SetSecretKey { key: coco::keys::SecretKey },
    Seal,
}

pub struct Manager {
    reload_notify: Arc<Notify>,
    message_sender: mpsc::Sender<Message>,
    message_receiver: mpsc::Receiver<Message>,
    config: Config,
}

impl Manager {
    pub fn new() -> Self {
        let (message_sender, message_receiver) = mpsc::channel(10);
        Self {
            reload_notify: Arc::new(Notify::new()),
            message_sender,
            message_receiver,
            config: Config { key: None },
        }
    }

    pub fn reload(&self) {
        self.reload_notify.notify();
    }

    pub fn handle(&self) -> Handle {
        Handle {
            reload_notify: self.reload_notify.clone(),
            message_sender: self.message_sender.clone(),
        }
    }

    pub async fn config(&mut self) -> Config {
        loop {
            let message = match self.message_receiver.try_recv() {
                Ok(message) => message,
                Err(_) => break,
            };

            match message {
                Message::Reset => self.config = Config { key: None },
                Message::SetSecretKey { key } => self.config.key = Some(key),
                Message::Seal => self.config.key = None,
            }
        }

        self.config.clone()
    }

    pub fn notified_restart(&mut self) -> impl Future<Output = ()> + Send + 'static {
        let reload_notify = Arc::new(Notify::new());
        self.reload_notify = reload_notify.clone();
        async move { reload_notify.notified().await }
    }
}

#[derive(Clone)]
pub struct Handle {
    reload_notify: Arc<Notify>,
    message_sender: mpsc::Sender<Message>,
}

impl Handle {
    pub fn reset(&mut self) {
        self.send_message(Message::Reset)
    }

    pub fn set_secret_key(&mut self, key: coco::keys::SecretKey) {
        self.send_message(Message::SetSecretKey { key })
    }

    pub fn seal(&mut self) {
        self.send_message(Message::Seal)
    }

    fn send_message(&mut self, message: Message) {
        match self.message_sender.try_send(message) {
            Ok(()) => {},
            Err(err) => match err {
                mpsc::error::TrySendError::Full(_) => panic!("full"),
                mpsc::error::TrySendError::Closed(_) => panic!("closed"),
            },
        }
        self.reload_notify.notify();
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        let (message_sender, mut message_receiver) = mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                if message_receiver.recv().await.is_none() {
                    break;
                }
            }
        });
        Self {
            reload_notify: Arc::new(Notify::new()),
            message_sender,
        }
    }
}
