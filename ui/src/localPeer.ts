import { derived, writable } from "svelte/store";

import * as api from "./api";
import * as remote from "./remote";

// TYPES
export enum StatusType {
  Stopped = "stopped",
  Offline = "offline",
  Started = "started",
  Syncing = "syncing",
  Online = "online",
}

interface Stopped {
  kind: StatusType.Stopped;
}

interface Offline {
  kind: StatusType.Offline;
}

interface Started {
  kind: StatusType.Started;
}

interface Syncing {
  kind: StatusType.Syncing;
  syncs: number;
}

interface Online {
  kind: StatusType.Online;
  connected: number;
}

type Status = Stopped | Offline | Started | Syncing | Online;

enum Event {
  StatusChanged = "statusChanged",
}

interface StatusChanged {
  type: Event.StatusChanged;
  old: Status;
  new: Status;
}

export type PeerEvent = StatusChanged;

interface RemotePeer {
  addr: string;
  peerId: string;
}

// STATE
const eventStore = writable<PeerEvent | null>(null, set => {
  const source = new EventSource(
    "http://localhost:8080/v1/notifications/local_peer_events"
  );

  source.addEventListener("message", (msg: MessageEvent) => {
    const peerEvent: PeerEvent = JSON.parse(msg.data);
    set(peerEvent);
  });

  return (): void => source.close();
});

const connectedPeersStore = remote.createStore<RemotePeer[]>();
export const connectedPeers = connectedPeersStore.readable;

connectedPeersStore.start(() => {
  const update = () => {
    api
      .get<RemotePeer[]>(`peer/connected_peers`)
      .then(connectedPeersStore.success)
      .catch(connectedPeersStore.error);
  };

  update();

  return eventStore.subscribe((peerEvent: PeerEvent | null) => {
    if (!peerEvent) return;

    switch (peerEvent.type) {
      case Event.StatusChanged:
        update();

        break;
    }
  });
});

export const status = derived(
  eventStore,
  (peerEvent: PeerEvent | null, set): void => {
    if (!peerEvent) {
      set({ status: remote.Status.Loading });
      return;
    }

    switch (peerEvent.type) {
      case Event.StatusChanged:
        set({ status: remote.Status.Success, data: peerEvent.new });
    }
  },
  { status: remote.Status.Loading } as remote.Data<Status>
);
