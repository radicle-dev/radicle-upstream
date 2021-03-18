use std::time::Duration;

use futures::{future, stream::StreamExt as _};
use tokio::time::timeout;

use coco::{PeerEvent, RunConfig};

#[macro_use]
mod common;
use common::*;

#[tokio::test]
async fn can_observe_timers() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let alice_tmp_dir = tempfile::tempdir()?;
    let alice_peer = build_peer(&alice_tmp_dir, RunConfig::default()).await?;

    let mut alice_events = alice_peer.subscribe();

    tokio::spawn(alice_peer.run());

    let ticked = async_stream::stream! {
        loop { yield alice_events.recv().await }
    }
    .scan(0, |ticked, event| {
        let event = event.unwrap();
        if let PeerEvent::RequestTick = event {
            *ticked += 1;
        }

        future::ready(if *ticked >= 5 { None } else { Some(event) })
    })
    .collect::<Vec<_>>();
    tokio::pin!(ticked);
    timeout(Duration::from_secs(5), ticked).await?;

    Ok(())
}
