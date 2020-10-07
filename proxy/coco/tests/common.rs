use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use futures::{future, StreamExt as _};
use tokio::{
    sync::broadcast,
    time::{timeout, Elapsed},
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::{
    keys::SecretKey,
    net::protocol::ProtocolEvent,
    peer::PeerId,
    signer,
    uri::{RadUrl, RadUrn},
};

use coco::{
    config, project, request::waiting_room::WaitingRoom, seed::Seed, shared::Shared, Paths, Peer,
    PeerEvent, RequestEvent, RunConfig, State,
};

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

/// Given one peers stream of events and another peers id, it will succeed once a connection from
/// the given id has been observed.
///
/// # Errors
///
/// * if the timeout waiting for the [`ProtocolEvent::Connected`] has been reached.
#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn connected(
    receiver: broadcast::Receiver<PeerEvent>,
    expected_id: &PeerId,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::Protocol(ProtocolEvent::Connected(remote_id)) if remote_id == *expected_id
    )
}

/// Assert that we received a cloned event for the expected `RadUrl`.
#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn assert_cloned(
    receiver: broadcast::Receiver<PeerEvent>,
    expected: &RadUrl,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::Request(RequestEvent::Cloned(url)) if url == *expected
    )
}

/// Assert that we received a query event for the expected `RadUrn`.
#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn requested(
    receiver: broadcast::Receiver<PeerEvent>,
    expected: &RadUrn,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::Request(RequestEvent::Query(urn)) if urn == *expected
    )
}

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    run_config: RunConfig,
) -> Result<(Peer, State, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
    let conf = config::default(key, tmp_dir.path())?;
    let (peer, state) =
        coco::into_peer_state(conf, signer.clone(), store, waiting_room, run_config).await?;

    Ok((peer, state, signer))
}

#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
pub async fn build_peer_with_seeds(
    tmp_dir: &tempfile::TempDir,
    seeds: Vec<Seed>,
    waiting_room: Shared<WaitingRoom<Instant, Duration>>,
    run_config: RunConfig,
) -> Result<(Peer, State, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

    let paths = Paths::from_root(tmp_dir.path())?;
    let conf = config::configure(paths, key, *config::LOCALHOST_ANY, seeds);

    let (peer, state) =
        coco::into_peer_state(conf, signer.clone(), store, waiting_room, run_config).await?;

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

#[allow(dead_code)] // NOTE(finto): this is used in integrations tests.
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
