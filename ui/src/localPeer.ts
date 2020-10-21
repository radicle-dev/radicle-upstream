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
  type: StatusType.Stopped;
}

interface Offline {
  type: StatusType.Offline;
}

interface Started {
  type: StatusType.Started;
}

interface Syncing {
  type: StatusType.Syncing;
  syncs: number;
}

interface Online {
  type: StatusType.Online;
  connected: number;
}

type Status = Stopped | Offline | Started | Syncing | Online;

enum EventKind {
  StatusChanged = "LOCAL_PEER_STATUS_CHANGED",
}

interface StatusChanged {
  type: EventKind.StatusChanged;
  old: Status;
  new: Status;
}

export type PeerEvent = StatusChanged;

// STATE
const statusStore = remote.createStore<Status>();
export const status = statusStore.readable;

statusStore.start(() => {
  const source = new EventSource(
    "http://localhost:8080/v1/notifications/local_peer_events"
  );

  source.addEventListener(EventKind.StatusChanged, (event: Event): void => {
    const changed = JSON.parse((event as MessageEvent).data);
    statusStore.success(changed.new);
  });

  // Switch the connection status indicator icon to "Stopped" when we get
  // disconnected from the SSE endpoint during peer configuration reload.
  source.addEventListener("error", (): void => {
    statusStore.success({ type: StatusType.Stopped });
  });

  return (): void => source.close();
});
