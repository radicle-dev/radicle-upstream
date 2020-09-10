use std::path::PathBuf;
use std::time::Duration;

use futures::future;
use futures::StreamExt as _;
use tokio::sync::broadcast;
use tokio::time::{timeout, Elapsed};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::keys::SecretKey;
use librad::net::protocol::ProtocolEvent;
use librad::peer::PeerId;
use librad::signer;

use coco::config;
use coco::project;
use coco::seed::Seed;
use coco::Paths;
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

#[allow(dead_code)]
pub async fn build_peer_with_seeds(
    tmp_dir: &tempfile::TempDir,
    seeds: Vec<Seed>,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key.clone());
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

    let paths = Paths::from_root(tmp_dir.path())?;
    let conf = config::configure(paths, key, *config::LOCALHOST_ANY, seeds);
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

#[allow(dead_code)]
pub fn radicle_project(path: PathBuf) -> project::Create<PathBuf> {
    project::Create {
        repo: project::Repo::New {
            path,
            name: "radicalise".to_string(),
        },
        description: "the people".to_string(),
        default_branch: "power".to_string(),
    }
}

pub fn shia_le_pathbuf(path: PathBuf) -> project::Create<PathBuf> {
    project::Create {
        repo: project::Repo::New {
            path,
            name: "just".to_string(),
        },
        description: "do".to_string(),
        default_branch: "it".to_string(),
    }
}

#[allow(dead_code)]
pub async fn wait_connected(
    receiver: broadcast::Receiver<PeerEvent>,
    expected_id: &PeerId,
) -> Result<(), Elapsed> {
    let filtered = receiver
        .into_stream()
        .filter_map(|res| match res.unwrap() {
            PeerEvent::Protocol(ProtocolEvent::Connected(remote_id))
                if remote_id == *expected_id =>
            {
                future::ready(Some(()))
            },
            _ => future::ready(None),
        })
        .map(|_| ());
    tokio::pin!(filtered);

    timeout(Duration::from_secs(10), filtered.next())
        .await
        .map(|_| ())
}
