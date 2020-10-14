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
  kind: StatusType.Syncing;
  connected: number;
}

type Status = Stopped | Offline | Started | Syncing | Online;

enum EventKind {
  StatusChanged = "LOCAL_PEER_STATUS_CHANGED",
}

interface StatusChanged {
  kind: EventKind.StatusChanged;
  status: Status;
}

export type PeerEvent = StatusChanged;

// STATE
const statusStore = remote.createStore<Status>();
export const status = statusStore.readable;

statusStore.start(() => {
  const source = new EventSource(
    "http://localhost:8080/v1/notifications/local_peer_status"
  );

  source.addEventListener(EventKind.StatusChanged, (event: Event): void => {
    statusStore.success(JSON.parse((event as MessageEvent).data));
  });

  return (): void => source.close();
});
