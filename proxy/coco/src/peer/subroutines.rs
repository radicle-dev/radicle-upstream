//! Management of peer subroutine tasks driven by advancing the core state machine with a stream of
//! inputs, producing commands.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, SystemTime},
};

use async_stream::stream;
use futures::stream::{BoxStream, FuturesUnordered, SelectAll, StreamExt as _};
use tokio::{
    sync::{broadcast, mpsc},
    time::interval,
};

use librad::{
    identities::Urn,
    net::{
        peer::{Gossip, PeerEvent},
        protocol::ProtocolEvent,
    },
    peer::PeerId,
};

use crate::{
    convert::MaybeFrom as _,
    request::{self, waiting_room::WaitingRoom},
    spawn_abortable::{self, SpawnAbortable},
    state::State,
};

use super::{
    announcement, control, gossip, include,
    run_state::{command, config, input, Command, Config as RunConfig, Event, Input, RunState},
    sync, waiting_room, RECEIVER_CAPACITY,
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
        peer_events: BoxStream<'static, PeerEvent>,
        protocol_events: BoxStream<'static, ProtocolEvent<Gossip>>,
        subscriber: broadcast::Sender<Event>,
        mut control_receiver: mpsc::Receiver<control::Request>,
    ) -> Self {
        let announce_timer = if run_config.announce.interval.is_zero() {
            None
        } else {
            Some(interval(run_config.announce.interval))
        };
        let waiting_room = match waiting_room::load(&store) {
            Err(err) => {
                log::warn!("Failed to load waiting room: {}", err);
                WaitingRoom::new(request::waiting_room::Config {
                    delta: config::DEFAULT_WAITING_ROOM_TIMEOUT,
                    ..request::waiting_room::Config::default()
                })
            },
            Ok(None) => WaitingRoom::new(request::waiting_room::Config {
                delta: config::DEFAULT_WAITING_ROOM_TIMEOUT,
                ..request::waiting_room::Config::default()
            }),
            Ok(Some(room)) => room,
        };
        let mut waiting_room_timer = interval(run_config.waiting_room.interval);
        let (input_sender, mut external_inputs) = mpsc::channel::<Input>(RECEIVER_CAPACITY);
        let run_state = RunState::new(run_config, waiting_room);

        let inputs = {
            let mut coalesced = SelectAll::new();
            coalesced.push(peer_events.map(Input::Peer).boxed());
            coalesced.push(protocol_events.map(Input::Protocol).boxed());

            if let Some(mut timer) = announce_timer {
                coalesced.push(
                    stream! {
                        while let _instant = timer.tick().await {
                            yield Input::Announce(input::Announce::Tick);
                        }
                    }
                    .boxed(),
                );
            }
            coalesced.push(
                stream! {
                    while let _instant = waiting_room_timer.tick().await {
                        yield Input::Request(input::Request::Tick);
                    }
                }
                .boxed(),
            );
            coalesced.push(
                stream! {
                    while let Some(request) = control_receiver.recv().await {
                        let input = match request {
                        control::Request::CurrentStatus(sender) => {
                            Input::Control(input::Control::Status(sender))
                        },
                        control::Request::CancelSearch(urn, time, sender) => {
                            Input::Control(input::Control::CancelRequest(urn, time, sender))
                        },
                        control::Request::GetSearch(urn, sender) => {
                            Input::Control(input::Control::GetRequest(urn, sender))
                        },
                        control::Request::ListSearches(sender) => {
                            Input::Control(input::Control::ListRequests(sender))
                        },
                        control::Request::StartSearch(urn, time, sender) => {
                            Input::Control(input::Control::CreateRequest(urn, time, sender))
                        },
                    };
                        yield input;
                    }
                }
                .boxed(),
            );
            coalesced.push(
                stream! {
                    while let Some(input) = external_inputs.recv().await {
                        yield input;
                    }
                }
                .boxed(),
            );

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
                command::Control::Respond(respond_command) => {
                    SpawnAbortable::new(control_respond(respond_command))
                },
            },
            Command::Include(urn) => SpawnAbortable::new(include::update(self.state.clone(), urn)),
            Command::PersistWaitingRoom(waiting_room) => {
                SpawnAbortable::new(persist_waiting_room(waiting_room, self.store.clone()))
            },
            Command::Request(command::Request::Query(urn)) => {
                SpawnAbortable::new(query(urn, self.state.clone(), self.input_sender.clone()))
            },
            Command::Request(command::Request::Clone(urn, remote_peer)) => {
                SpawnAbortable::new(clone(
                    urn,
                    remote_peer,
                    self.state.clone(),
                    self.input_sender.clone(),
                ))
            },
            Command::Request(command::Request::TimedOut(urn)) => {
                let sender = self.input_sender.clone();
                SpawnAbortable::new(async move {
                    sender
                        .send(Input::Request(input::Request::TimedOut(urn)))
                        .await
                        .ok();
                })
            },
            Command::StartSyncTimeout(sync_period) => {
                SpawnAbortable::new(start_sync_timeout(sync_period, self.input_sender.clone()))
            },
            Command::SyncPeer(peer_id) => {
                SpawnAbortable::new(sync(self.state.clone(), peer_id, self.input_sender.clone()))
            },
            Command::EmitEvent(event) => {
                self.subscriber.send(event).ok();
                SpawnAbortable::new(async move {})
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

                    let old_status = self.run_state.status.clone();

                    if let Some(event) = Event::maybe_from(&input) {
                        // Ignore if there are no subscribers.
                        self.subscriber.send(event).ok();
                    }

                    for cmd in self.run_state.transition(input) {
                        let task = self.spawn_command(cmd);

                        self.pending_tasks.push(task);
                    }

                    if old_status != self.run_state.status {
                        self.subscriber
                            .send(Event::StatusChanged(
                                old_status,
                                self.run_state.status.clone(),
                            ))
                            .ok();
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
async fn announce(state: State, store: kv::Store, sender: mpsc::Sender<Input>) {
    match announcement::run(&state, &store).await {
        Ok(updates) => {
            sender
                .send(Input::Announce(input::Announce::Succeeded(updates)))
                .await
                .ok();
        },
        Err(err) => {
            log::error!("announce error: {:?}", err);
            sender
                .send(Input::Announce(input::Announce::Failed))
                .await
                .ok();
        },
    }
}

/// Fulfill control requests by sending the scheduled responses.
async fn control_respond(cmd: control::Response) {
    match cmd {
        control::Response::CurrentStatus(sender, status) => sender.send(status).ok(),
        control::Response::CancelSearch(sender, request) => sender.send(request).ok(),
        control::Response::GetSearch(sender, request) => sender.send(request).ok(),
        control::Response::ListSearches(sender, requests) => sender.send(requests).ok(),
        control::Response::StartSearch(sender, request) => sender.send(request).ok(),
    };
}

async fn persist_waiting_room(waiting_room: WaitingRoom<SystemTime, Duration>, store: kv::Store) {
    match waiting_room::save(&store, waiting_room) {
        Ok(()) => log::debug!("Successfully persisted the waiting room"),
        Err(err) => log::debug!("Error while persisting the waiting room: {}", err),
    }
}

/// Run the sync with a single peer to reach state parity for locally tracked projects. On
/// completion report back with the success or failure.
async fn sync(state: State, peer_id: PeerId, sender: mpsc::Sender<Input>) {
    sender
        .send(Input::PeerSync(input::Sync::Started(peer_id)))
        .await
        .ok();

    match sync::sync(&state, peer_id).await {
        Ok(_) => {
            sender
                .send(Input::PeerSync(input::Sync::Succeeded(peer_id)))
                .await
                .ok();
        },
        Err(err) => {
            log::error!("sync error for {}: {:?}", peer_id, err);
            sender
                .send(Input::PeerSync(input::Sync::Failed(peer_id)))
                .await
                .ok();
        },
    }
}

/// Send a timeout input once the `sync_period` has elapsed.
async fn start_sync_timeout(sync_period: Duration, sender: mpsc::Sender<Input>) {
    tokio::time::sleep(sync_period).await;
    sender
        .send(Input::Timeout(input::Timeout::SyncPeriod))
        .await
        .ok();
}

/// Send a query on the network for the given urn.
async fn query(urn: Urn, state: State, sender: mpsc::Sender<Input>) {
    gossip::query(&state, urn.clone(), None).await;
    sender
        .send(Input::Request(input::Request::Queried(urn)))
        .await
        .ok();
}

/// Run a clone for the given `url`. On completion report back with the success or failure.
async fn clone(urn: Urn, remote_peer: PeerId, state: State, sender: mpsc::Sender<Input>) {
    sender
        .send(Input::Request(input::Request::Cloning(
            urn.clone(),
            remote_peer,
        )))
        .await
        .ok();

    match state
        .clone_project(urn.clone(), remote_peer, None, None)
        .await
    {
        Ok(_urn) => {
            sender
                .send(Input::Request(input::Request::Cloned(urn, remote_peer)))
                .await
                .ok();
        },
        Err(err) => {
            log::warn!(
                "an error occurred for the command 'Clone' for the URN '{}' from {}:\n{}",
                urn,
                remote_peer,
                err,
            );
            sender
                .send(Input::Request(input::Request::Failed {
                    urn,
                    remote_peer,
                    reason: err.to_string(),
                }))
                .await
                .ok();
        },
    }
}
