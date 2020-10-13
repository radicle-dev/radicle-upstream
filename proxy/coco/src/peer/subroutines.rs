//! Management of peer subroutine tasks driven by advancing the core state machine with a stream of
//! inputs, producing commands.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::stream::{BoxStream, FuturesUnordered, SelectAll, StreamExt as _};
use tokio::{
    sync::{broadcast, mpsc},
    time::interval,
};

use librad::{
    net::{peer::Gossip, protocol::ProtocolEvent},
    peer::PeerId,
    uri::{RadUrl, RadUrn},
};

use crate::{
    convert::MaybeFrom as _,
    spawn_abortable::{self, SpawnAbortable},
    state::State,
};

use super::{
    announcement, control, gossip,
    run_state::{
        AnnounceInput, Command, Config as RunConfig, ControlCommand, Event, Input, RequestCommand,
        RequestInput, RunState, SyncInput, TimeoutInput,
    },
    sync, RECEIVER_CAPACITY,
};

/// Management of "subroutine" tasks.
pub struct Subroutines {
    /// Set of handles of spawned subroutine tasks. Draining them will ensure resources are
    /// release.
    pending_tasks: FuturesUnordered<SpawnAbortable<()>>,
    /// Stream of inputs to [`RunState`] state machine.
    inputs: SelectAll<BoxStream<'static, Input>>,

    /// [`State`] for suborutine task fulfillment.
    state: State,
    /// [`kv::Store`] for suborutine task fulfillment.
    store: kv::Store,

    /// Main peer state machine.
    run_state: RunState,

    /// Feedback channel for subroutine tasks send new inputs to the state machine.
    input_sender: mpsc::Sender<Input>,
    /// Channel for public subscribers to get to know of significant events of the peer machinery.
    subscriber: broadcast::Sender<Event>,
}

impl Subroutines {
    /// Constructs a new subroutines manager.
    pub fn new(
        state: State,
        store: kv::Store,
        run_config: RunConfig,
        protocol_events: BoxStream<'static, ProtocolEvent<Gossip>>,
        subscriber: broadcast::Sender<Event>,
        control_receiver: mpsc::Receiver<control::Request>,
    ) -> Self {
        let announce_timer = interval(run_config.announce.interval);
        let waiting_room_timer = interval(run_config.waiting_room.interval);
        let (input_sender, inputs) = mpsc::channel::<Input>(RECEIVER_CAPACITY);
        let run_state = RunState::from(run_config);

        let inputs = {
            let mut coalesced = SelectAll::new();
            coalesced.push(protocol_events.map(Input::Protocol).boxed());
            coalesced.push(
                announce_timer
                    .map(|_tick| Input::Announce(AnnounceInput::Tick))
                    .boxed(),
            );
            coalesced.push(
                waiting_room_timer
                    .map(|_tick| Input::Request(RequestInput::Tick))
                    .boxed(),
            );
            coalesced.push(
                control_receiver
                    .map(|request| match request {
                        control::Request::Urn(urn, time, sender) => {
                            Input::Request(RequestInput::Requested(urn, time, Some(sender)))
                        },
                    })
                    .boxed(),
            );
            coalesced.push(inputs.boxed());

            coalesced
        };

        Self {
            pending_tasks: FuturesUnordered::new(),
            inputs,

            state,
            store,
            run_state,

            subscriber,
            input_sender,
        }
    }

    /// Map commands produced by the state machine to spawned subroutine tasks.
    fn spawn_command(&self, cmd: Command) -> SpawnAbortable<()> {
        match cmd {
            Command::Announce => SpawnAbortable::new(announce(
                self.state.clone(),
                self.store.clone(),
                self.input_sender.clone(),
            )),
            Command::Control(control_command) => match control_command {
                ControlCommand::Respond(respond_command) => {
                    SpawnAbortable::new(control_respond(respond_command))
                },
            },
            Command::SyncPeer(peer_id) => {
                SpawnAbortable::new(sync(self.state.clone(), peer_id, self.input_sender.clone()))
            },
            Command::StartSyncTimeout(sync_period) => {
                SpawnAbortable::new(start_sync_timeout(sync_period, self.input_sender.clone()))
            },
            Command::Request(RequestCommand::Query(urn)) => {
                SpawnAbortable::new(query(urn, self.state.clone(), self.input_sender.clone()))
            },
            Command::Request(RequestCommand::Clone(url)) => {
                SpawnAbortable::new(clone(url, self.state.clone(), self.input_sender.clone()))
            },
            Command::Request(RequestCommand::TimedOut(urn)) => {
                let mut sender = self.input_sender.clone();
                SpawnAbortable::new(async move {
                    sender
                        .send(Input::Request(RequestInput::TimedOut(urn)))
                        .await
                        .ok();
                })
            },
        }
    }
}

impl Drop for Subroutines {
    fn drop(&mut self) {
        for task in self.pending_tasks.iter_mut() {
            task.abort()
        }
    }
}

impl Future for Subroutines {
    type Output = Result<(), spawn_abortable::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut tasks_initial_empty = true;

        // Drain the task queue.
        loop {
            match self.pending_tasks.poll_next_unpin(cx) {
                Poll::Ready(Some(Err(e))) => {
                    log::warn!("error in spawned subroutine task: {:?}", e);
                    return Poll::Ready(Err(e));
                },
                Poll::Ready(Some(Ok(()))) => continue,
                // Either pending, or FuturesUnordered thinks it's done, but
                // we'll enqueue new tasks below
                Poll::Ready(None) => break,
                Poll::Pending => {
                    tasks_initial_empty = false;
                    break;
                },
            }
        }

        // Drain all pending inputs, feed them to the [`RunState`] and execute the returned
        // commands as async tasks.
        loop {
            match self.inputs.poll_next_unpin(cx) {
                Poll::Ready(Some(input)) => {
                    log::debug!("handling subroutine input: {:?}", input);

                    if let Some(event) = Event::maybe_from(&input) {
                        // Ignore if there are no subscribers.
                        self.subscriber.send(event).ok();
                    }

                    for cmd in self.run_state.transition(input) {
                        let task = self.spawn_command(cmd);

                        self.pending_tasks.push(task);
                    }
                },
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => {
                    if tasks_initial_empty && !self.pending_tasks.is_empty() {
                        cx.waker().wake_by_ref()
                    }
                    return Poll::Pending;
                },
            }
        }
    }
}

/// Run the announcement of updated refs for local projects. On completion report back with the
/// success or failure.
async fn announce(state: State, store: kv::Store, mut sender: mpsc::Sender<Input>) {
    match announcement::run(&state, &store).await {
        Ok(updates) => {
            sender
                .send(Input::Announce(AnnounceInput::Succeeded(updates)))
                .await
                .ok();
        },
        Err(err) => {
            log::error!("announce error: {:?}", err);
            sender
                .send(Input::Announce(AnnounceInput::Failed))
                .await
                .ok();
        },
    }
}

/// Fulfill control requests by sending the scheduled responses.
async fn control_respond(cmd: control::Response) {
    match cmd {
        control::Response::Urn(sender, request) => sender.send(request).ok(),
    };
}

/// Run the sync with a single peer to reach state parity for locally tracked projects. On
/// completion report back with the success or failure.
async fn sync(state: State, peer_id: PeerId, mut sender: mpsc::Sender<Input>) {
    sender
        .send(Input::PeerSync(SyncInput::Started(peer_id)))
        .await
        .ok();

    match sync::sync(&state, peer_id).await {
        Ok(_) => {
            sender
                .send(Input::PeerSync(SyncInput::Succeeded(peer_id)))
                .await
                .ok();
        },
        Err(err) => {
            log::error!("sync error for {}: {:?}", peer_id, err);
            sender
                .send(Input::PeerSync(SyncInput::Failed(peer_id)))
                .await
                .ok();
        },
    }
}

/// Send a timeout input once the `sync_period` has elapsed.
async fn start_sync_timeout(sync_period: Duration, mut sender: mpsc::Sender<Input>) {
    tokio::time::delay_for(sync_period).await;
    sender
        .send(Input::Timeout(TimeoutInput::SyncPeriod))
        .await
        .ok();
}

/// Send a query on the network for the given urn.
async fn query(urn: RadUrn, state: State, mut sender: mpsc::Sender<Input>) {
    gossip::query(&state, urn.clone()).await;
    sender
        .send(Input::Request(RequestInput::Queried(urn)))
        .await
        .ok();
}

/// Run a clone for the given `url`. On completion report back with the success or failure.
async fn clone(url: RadUrl, state: State, mut sender: mpsc::Sender<Input>) {
    sender
        .send(Input::Request(RequestInput::Cloning(url.clone())))
        .await
        .ok();

    match state.clone_project(url.clone(), None).await {
        Ok(_urn) => {
            sender
                .send(Input::Request(RequestInput::Cloned(url)))
                .await
                .ok();
        },
        Err(err) => {
            log::warn!(
                "an error occurred for the command 'Clone' for the URL '{}':\n{}",
                url,
                err
            );
            sender
                .send(Input::Request(RequestInput::Failed {
                    url,
                    reason: err.to_string(),
                }))
                .await
                .ok();
        },
    }
}
