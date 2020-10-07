use std::time::{Duration, Instant};

use futures::{future, stream::StreamExt as _};
use tokio::time::timeout;

use coco::{
    request::waiting_room::{self, WaitingRoom},
    shared::Shared,
    PeerEvent, RunConfig,
};

#[macro_use]
mod common;
use common::*;

#[tokio::test]
async fn can_observe_timers() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let waiting_room: Shared<WaitingRoom<Instant, Duration>> =
        Shared::from(WaitingRoom::new(waiting_room::Config::default()));
    let (alice_peer, _alice_state, _alice_signer) =
        build_peer(&alice_tmp_dir, waiting_room, RunConfig::default()).await?;

    let alice_events = alice_peer.subscribe();

    tokio::spawn(alice_peer.into_running());

    let pinged = alice_events
        .into_stream()
        .scan(0, |pinged, event| {
            let event = event.unwrap();
            if let PeerEvent::Ping = event {
                *pinged += 1;
            }

            future::ready(if *pinged >= 10 { None } else { Some(event) })
        })
        .collect::<Vec<_>>();
    tokio::pin!(pinged);
    timeout(Duration::from_secs(5), pinged).await?;

    Ok(())
}
