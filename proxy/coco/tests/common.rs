use std::{path::PathBuf, time::Duration};

use futures::{future, StreamExt as _};
use tokio::{
    sync::broadcast,
    time::{error::Elapsed, timeout},
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::{
    git::Urn, git_ext::OneLevel, keys::SecretKey, net::discovery, peer::PeerId, reflike, signer,
};

use coco::{config, project, seed::Seed, Paths, Peer, PeerEvent, PeerStatus, RunConfig};

#[doc(hidden)]
#[macro_export]
macro_rules! await_event {
    ( $receiver:expr , $filter:expr ) => {{
        let filtered = async_stream::stream! { loop { yield $receiver.recv().await } }
            .filter_map($filter)
            .map(|_| ());
        tokio::pin!(filtered);
        timeout(Duration::from_secs(2), filtered.next())
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

/// Assert that we received a cloned event for the expected `RadUrl`.
#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn assert_cloned(
    mut receiver: broadcast::Receiver<PeerEvent>,
    expected_urn: &Urn,
    expected_remote: PeerId,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::RequestCloned(urn, remote_peer) if urn == *expected_urn && remote_peer == expected_remote
    )
}

/// Assert that we received a query event for the expected `RadUrn`.
#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn requested(
    mut receiver: broadcast::Receiver<PeerEvent>,
    expected: &Urn,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::RequestQueried(urn) if urn == *expected
    )
}

/// Assert that the `PeerStatus` transitions to `Online` and the number of connected peers is equal
/// to or more than `min_connected`.
#[allow(dead_code)]
pub async fn connected(
    mut receiver: broadcast::Receiver<PeerEvent>,
    min_connected: usize,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::StatusChanged { new: PeerStatus::Online { connected }, .. } if connected >= min_connected
    )
}

#[allow(dead_code)]
pub async fn started(mut receiver: broadcast::Receiver<PeerEvent>) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::StatusChanged {
            new: PeerStatus::Started,
            ..
        }
    )
}

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
    run_config: RunConfig,
) -> Result<Peer<discovery::Static>, Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
    let conf = config::default(signer, tmp_dir.path())?;
    let disco = config::static_seed_discovery(&[]);
    let peer = coco::Peer::new(conf, disco, store, run_config);

    Ok(peer)
}

#[allow(dead_code)]
pub async fn build_peer_with_seeds(
    tmp_dir: &tempfile::TempDir,
    seeds: Vec<Seed>,
    run_config: RunConfig,
) -> Result<Peer<discovery::Static>, Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
    let paths = Paths::from_root(tmp_dir.path())?;
    let conf = config::configure(paths, signer, *config::LOCALHOST_ANY);
    let disco = config::static_seed_discovery(&seeds);
    let peer = coco::Peer::new(conf, disco, store, run_config);

    Ok(peer)
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
pub fn radicle_project(path: PathBuf) -> project::Create {
    project::Create {
        repo: project::Repo::New {
            path,
            name: "radicalise".to_string(),
        },
        description: "the people".to_string(),
        default_branch: OneLevel::from(reflike!("power")),
    }
}

#[allow(dead_code)]
pub fn shia_le_pathbuf(path: PathBuf) -> project::Create {
    project::Create {
        repo: project::Repo::New {
            path,
            name: "just".to_string(),
        },
        description: "do".to_string(),
        default_branch: OneLevel::from(reflike!("it")),
    }
}
