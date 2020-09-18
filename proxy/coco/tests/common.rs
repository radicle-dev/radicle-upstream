use std::{path::PathBuf, time::Duration};

use futures::{future, StreamExt as _};
use tokio::{
    sync::broadcast,
    time::{timeout, Elapsed},
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::{keys::SecretKey, net::protocol::ProtocolEvent, peer::PeerId, signer};

use coco::{config, project, seed::Seed, Lock, Paths, Peer, PeerEvent};

#[doc(hidden)]
#[macro_export]
macro_rules! await_event {
    ( $receiver:expr , $filter:expr ) => {{
        let filtered = $receiver.into_stream().filter_map($filter).map(|_| ());
        tokio::pin!(filtered);
        timeout(Duration::from_secs(1), filtered.next())
            .await
            .map(|_| ())
    }};
}

macro_rules! assert_event {
    ( $receiver:expr , $pattern:pat ) => {{
        $crate::await_event!($receiver, |res| match res.unwrap() {
            $pattern => future::ready(Some(())),
            _ => future::ready(None),
        })
    }};
    ( $receiver:expr , $pattern:pat if $cond:expr ) => {{
        $crate::await_event!($receiver, |res| match res.unwrap() {
            $pattern if $cond => future::ready(Some(())),
            _ => future::ready(None),
        })
    }};
}

#[allow(dead_code)]
pub async fn connected(
    receiver: broadcast::Receiver<PeerEvent>,
    expected_id: &PeerId,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::Protocol(ProtocolEvent::Connected(remote_id)) if remote_id == *expected_id
    )
}

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    build_peer_with_seeds(tmp_dir, vec![]).await
}

#[allow(dead_code)]
pub async fn build_peer_with_seeds(
    tmp_dir: &tempfile::TempDir,
    seeds: Vec<Seed>,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key.clone());

    let paths = Paths::from_root(tmp_dir.path())?;
    let conf = config::configure(paths, key, *config::LOCALHOST_ANY, seeds);
    let (peer, state) = coco::into_peer_state(conf, signer.clone()).await?;

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
