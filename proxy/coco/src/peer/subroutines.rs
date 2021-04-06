//! Management of peer subroutine tasks driven by advancing the core state machine with a stream of
//! inputs, producing commands.

use std::{
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use async_stream::stream;
use futures::stream::{BoxStream, FuturesUnordered, SelectAll, StreamExt as _};
use tokio::{
    sync::{broadcast, mpsc, watch},
    task::{JoinError, JoinHandle},
    time::interval,
};

use librad::{
    git::Urn,
    net::{self, peer::ProtocolEvent},
    peer::PeerId,
    signer::BoxedSigner,
};

use crate::{
    convert::MaybeFrom as _,
    request::{self, waiting_room::WaitingRoom},
    state,
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
    pending_tasks: FuturesUnordered<JoinHandle<()>>,
    /// Stream of inputs to [`RunState`] state machine.
    inputs: SelectAll<BoxStream<'static, Input>>,

    /// [`net::peer::Peer`] for suborutine task fulfillment.
    peer: net::peer::Peer<BoxedSigner>,
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
        peer: net::peer::Peer<BoxedSigner>,
        mut listen_addrs: watch::Receiver<Vec<SocketAddr>>,
        store: kv::Store,
        run_config: &RunConfig,
        protocol_events: BoxStream<'static, Result<ProtocolEvent, net::protocol::RecvError>>,
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
        let mut stats_timer = interval(run_config.stats.interval);
        let sync_timer = if run_config.sync.interval.is_zero() {
            None
        } else {
            Some(interval(run_config.sync.interval))
        };

        let run_state = RunState::new(waiting_room);

        let inputs = {
            let mut coalesced = SelectAll::new();
            coalesced.push(
                // TODO(xla): Ensure stream of Results has significance, or should just signal
                // stream close.
                protocol_events
                    .filter_map(|res| async move {
                        match res {
                            Ok(ev) => Some(Input::Protocol(ev)),
                            Err(err) => {
                                log::warn!("receive error: {}", err);
                                None
                            },
                        }
                    })
                    .boxed(),
            );

            coalesced.push(
                stream! {
                    while listen_addrs.changed().await.is_ok() {
                        let addrs = listen_addrs.borrow().clone();
                        yield Input::ListenAddrs(addrs);
                    }
                }
                .boxed(),
            );

            if let Some(mut timer) = announce_timer {
                coalesced.push(
                    stream! {
                        loop {
                            timer.tick().await;
                            yield Input::Announce(input::Announce::Tick);
                        }
                    }
                    .boxed(),
                );
            }
            if let Some(mut timer) = sync_timer {
                coalesced.push(
                    stream! {
                        loop {
                            timer.tick().await;
                            yield Input::PeerSync(input::Sync::Tick);
                        }
                    }
                    .boxed(),
                );
            }
            coalesced.push(
                stream! {
                    loop {
                        waiting_room_timer.tick().await;
                        yield Input::Request(input::Request::Tick);
                    }
                }
                .boxed(),
            );
            coalesced.push(
                stream! {
                    loop {
                        stats_timer.tick().await;
                        yield Input::Stats(input::Stats::Tick);
                    }
                }
                .boxed(),
            );
            coalesced.push(
                stream! {
                while let Some(request) = control_receiver.recv().await { yield request } }
                .map(|request| match request {
                    control::Request::CurrentStatus(sender) => {
                        Input::Control(input::Control::Status(sender))
                    },
                    control::Request::ListenAddrs(sender) => {
                        Input::Control(input::Control::ListenAddrs(sender))
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
                })
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

            peer,
            store,
            run_state,

            subscriber,
            input_sender,
        }
    }

    /// Map commands produced by the state machine to spawned subroutine tasks.
    fn spawn_command(&self, cmd: Command) -> JoinHandle<()> {
        match cmd {
            Command::Announce => tokio::spawn(announce(
                self.peer.clone(),
                self.store.clone(),
                self.input_sender.clone(),
            )),
            Command::Control(control_command) => match control_command {
                command::Control::Respond(respond_command) => {
                    tokio::spawn(control_respond(respond_command))
                },
            },
            Command::Include(urn) => tokio::spawn(include::update(self.peer.clone(), urn)),
            Command::PersistWaitingRoom(waiting_room) => {
                tokio::spawn(persist_waiting_room(waiting_room, self.store.clone()))
            },
            Command::Request(command::Request::Query(urn)) => {
                tokio::spawn(query(urn, self.peer.clone(), self.input_sender.clone()))
            },
            Command::Request(command::Request::Clone(urn, remote_peer)) => tokio::spawn(clone(
                urn,
                remote_peer,
                self.peer.clone(),
                self.input_sender.clone(),
            )),
            Command::Request(command::Request::TimedOut(urn)) => {
                let sender = self.input_sender.clone();
                tokio::spawn(async move {
                    sender
                        .send(Input::Request(input::Request::TimedOut(urn)))
                        .await
                        .ok();
                })
            },
            Command::Stats => tokio::spawn(get_stats(self.peer.clone(), self.input_sender.clone())),
            Command::SyncPeer(peer_id) => {
                tokio::spawn(sync(self.peer.clone(), peer_id, self.input_sender.clone()))
            },
            Command::EmitEvent(event) => {
                self.subscriber.send(event).ok();
                tokio::spawn(async move {})
            },
        }
    }

    fn handle_input(&mut self, input: Input) {
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
                .send(Event::StatusChanged {
                    old: old_status,
                    new: self.run_state.status.clone(),
                })
                .ok();
        }
    }

    pub async fn run(mut self) -> Result<(), JoinError> {
        #![allow(clippy::mut_mut)]
        loop {
            futures::select! {
                maybe_result = self.pending_tasks.next() => {
                    if let Some(Err(err)) = maybe_result {
                        return Err(err)
                    }
                }
                maybe_input = self.inputs.next() => {
                    if let Some(input) = maybe_input {
                        self.handle_input(input)
                    } else {
                        return Ok(())
                    }
                }
            }
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

/// Run the announcement of updated refs for local projects. On completion report back with the
/// success or failure.
async fn announce(
    peer: net::peer::Peer<BoxedSigner>,
    store: kv::Store,
    sender: mpsc::Sender<Input>,
) {
    match announcement::run(&peer, &store).await {
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
        control::Response::ListenAddrs(sender, addrs) => sender.send(addrs).ok(),
        control::Response::GetSearch(sender, request) => sender.send(request).ok(),
        control::Response::ListSearches(sender, requests) => sender.send(requests).ok(),
        control::Response::StartSearch(sender, request) => sender.send(request).ok(),
    };
}

async fn get_stats(peer: net::peer::Peer<BoxedSigner>, sender: mpsc::Sender<Input>) {
    let connected_peers = peer.connected_peers().await;
    let stats = peer.stats().await;

    sender
        .send(Input::Stats(input::Stats::Values(connected_peers, stats)))
        .await
        .ok();
}

async fn persist_waiting_room(waiting_room: WaitingRoom<SystemTime, Duration>, store: kv::Store) {
    match waiting_room::save(&store, waiting_room) {
        Ok(()) => log::debug!("Successfully persisted the waiting room"),
        Err(err) => log::debug!("Error while persisting the waiting room: {}", err),
    }
}

/// Run the sync with a single peer to reach state parity for locally tracked projects. On
/// completion report back with the success or failure.
async fn sync(peer: net::peer::Peer<BoxedSigner>, peer_id: PeerId, sender: mpsc::Sender<Input>) {
    sender
        .send(Input::PeerSync(input::Sync::Started(peer_id)))
        .await
        .ok();

    match sync::sync(&peer, peer_id).await {
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

/// Send a query on the network for the given urn.
async fn query(urn: Urn, peer: net::peer::Peer<BoxedSigner>, sender: mpsc::Sender<Input>) {
    gossip::query(&peer, &urn, None);
    sender
        .send(Input::Request(input::Request::Queried(urn)))
        .await
        .ok();
}

/// Run a clone for the given `url`. On completion report back with the success or failure.
async fn clone(
    urn: Urn,
    remote_peer: PeerId,
    peer: net::peer::Peer<BoxedSigner>,
    sender: mpsc::Sender<Input>,
) {
    sender
        .send(Input::Request(input::Request::Cloning(
            urn.clone(),
            remote_peer,
        )))
        .await
        .ok();

    match state::clone_project(&peer, urn.clone(), remote_peer, None, None).await {
        Ok(_urn) => {
            sender
                .send(Input::Request(input::Request::Cloned(
                    urn.clone(),
                    remote_peer,
                )))
                .await
                .ok();
            gossip::announce(&peer, &urn, None);
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
