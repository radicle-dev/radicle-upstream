use std::{path::PathBuf, time::Duration};

use futures::{future, StreamExt as _};
use tokio::{
    sync::broadcast,
    time::{timeout, Elapsed},
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::{
    git_ext::OneLevel,
    keys::SecretKey,
    peer::PeerId,
    reflike, signer,
    uri::{RadUrl, RadUrn},
};

use coco::{config, project, seed::Seed, Paths, Peer, PeerEvent, RunConfig, State};

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
#[allow(dead_code)]
pub async fn connected(
    receiver: broadcast::Receiver<PeerEvent>,
    expected_id: &PeerId,
) -> Result<(), Elapsed> {
    assert_event!(
        receiver,
        PeerEvent::PeerConnected(remote_id) if remote_id == *expected_id
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
        PeerEvent::RequestCloned(url) if url == *expected
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
        PeerEvent::RequestQueried(urn) if urn == *expected
    )
}

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
    run_config: RunConfig,
) -> Result<(Peer, State), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;
    let conf = config::default(key, tmp_dir.path())?;
    let (peer, state) = coco::into_peer_state(conf, signer.clone(), store, run_config).await?;

    Ok((peer, state))
}

#[allow(dead_code)]
pub async fn build_peer_with_seeds(
    tmp_dir: &tempfile::TempDir,
    seeds: Vec<Seed>,
    run_config: RunConfig,
) -> Result<(Peer, State), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key);
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

    let paths = Paths::from_root(tmp_dir.path())?;
    let conf = config::configure(
        paths,
        key,
        *config::LOCALHOST_ANY,
        config::static_seed_discovery(seeds),
    );

    let (peer, state) = coco::into_peer_state(conf, signer.clone(), store, run_config).await?;

    Ok((peer, state))
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
