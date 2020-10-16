import { derived } from "svelte/store";

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

// STATE
const eventStore = remote.createStore<PeerEvent>();
eventStore.start(() => {
  const source = new EventSource(
    "http://localhost:8080/v1/notifications/local_peer_events"
  );

  source.addEventListener("message", (msg: MessageEvent) => {
    const peerEvent: PeerEvent = JSON.parse(msg.data);
    eventStore.success(peerEvent);
  });

  return (): void => source.close();
});

export const status = derived(
  eventStore,
  (data: remote.Data<PeerEvent>): remote.Data<Status> => {
    if (data.status === remote.Status.Success) {
      const peerEvent = data.data;
      switch (peerEvent.type) {
        case Event.StatusChanged:
          return { status: remote.Status.Success, data: peerEvent.new };
      }
    }

    return data;
  }
);
