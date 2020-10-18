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

enum EventKind {
  StatusChanged = "LOCAL_PEER_STATUS_CHANGED",
}

interface StatusChanged {
  kind: EventKind.StatusChanged;
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

  return (): void => source.close();
});
