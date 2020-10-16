import { derived, get, writable } from "svelte/store";

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
  Announced = "announced",
  PeerSynced = "peerSynced",
  StatusChanged = "statusChanged",
}

export type PeerEvent =
  | { type: Event.Announced; updates: string[] }
  // FIXME(xla): Much like RadUrns peer ids need to be properly typed.
  | { type: Event.PeerSynced; peerId: string }
  | { type: Event.StatusChanged; old: Status; new: Status };

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

export const events = derived(
  eventStore,
  (peerEvent: PeerEvent | null, set: (events: PeerEvent[]) => void): void => {
    if (!peerEvent) return;

    const val = get(events);
    const len = val.push(peerEvent);
    if (len > 1000) {
      val.shift();
    }
    set(val);
  },
  []
);

export const status = derived(
  eventStore,
  (
    peerEvent: PeerEvent | null,
    set: (status: remote.Data<Status>) => void
  ): void => {
    if (peerEvent && peerEvent.type === Event.StatusChanged) {
      set({ status: remote.Status.Success, data: peerEvent.new });
    }
  },
  { status: remote.Status.Loading } as remote.Data<Status>
);
