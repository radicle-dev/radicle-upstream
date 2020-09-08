use std::time::Duration;

use futures::future;
use futures::{Stream, StreamExt as _};
use tokio::sync::broadcast;
use tokio::time::{timeout, Elapsed};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::keys::SecretKey;
use librad::net::protocol::ProtocolEvent;
use librad::peer::PeerId;
use librad::signer;

use coco::config;
use coco::{Lock, Peer, PeerEvent};

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key.clone());
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

    let conf = config::default(key, tmp_dir.path())?;
    let (peer, state) = coco::into_peer_state(conf, signer.clone(), store).await?;

    Ok((peer, state, signer))
}

pub fn init_logging() {
    if pretty_env_logger::try_init().is_ok() {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting tracing default failed");
    }
}

pub async fn wait_connected(
    receiver: broadcast::Receiver<PeerEvent>,
    expected_id: &PeerId,
) -> Result<(), Elapsed> {
    let filtered = receiver
        .into_stream()
        .filter_map(|res| {
            let event = res.unwrap();
            println!("EVENT {:?}", event);

            match event {
                PeerEvent::Protocol(ProtocolEvent::Connected(remote_id))
                    if remote_id == *expected_id =>
                {
                    future::ready(Some(()))
                }
                _ => future::ready(None),
            }
        })
        .map(|_| ());
    tokio::pin!(filtered);

    timeout(Duration::from_secs(10), filtered.next())
        .await
        .map(|_| ())
}
