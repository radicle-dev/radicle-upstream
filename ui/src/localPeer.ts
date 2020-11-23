import { derived, writable, Readable } from "svelte/store";
import { push } from "svelte-spa-router";

import type * as identity from "./identity";
import * as notifiation from "./notification";
import * as path from "./path";
import * as remote from "./remote";
import * as session from "./session";
import type * as urn from "./urn";

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

enum EventType {
  RequestCloned = "requestCloned",
  RequestQueried = "requestQueried",
  RequestTimedOut = "requestTimedOut",
  StatusChanged = "statusChanged",
}

interface RequestCloned {
  type: EventType.RequestCloned;
  peer: identity.PeerId;
  urn: urn.Urn;
}

interface RequestQueried {
  type: EventType.RequestQueried;
  urn: urn.Urn;
}

interface RequestTimedOut {
  type: EventType.RequestTimedOut;
  urn: urn.Urn;
}

export type Event =
  | RequestCloned
  | RequestQueried
  | RequestTimedOut
  | { type: EventType.StatusChanged; old: Status; new: Status };

let eventSource: EventSource | null = null;

session.session.subscribe(sess => {
  if (
    sess.status === remote.Status.Success &&
    sess.data.status === session.Status.UnsealedSession
  ) {
    if (eventSource === null || eventSource.readyState === EventSource.CLOSED) {
      eventSource = new EventSource(
        "http://localhost:17246/v1/notifications/local_peer_events",
        { withCredentials: true }
      );
      eventSource.addEventListener("message", msg => {
        const event: Event = JSON.parse(msg.data);
        eventStore.set(event);
      });
    }
  }
});

// STATE
const eventStore = writable<Event | null>(null);

// Event handling.
// FIXME(xla): Formalise event handling.
eventStore.subscribe((event: Event | null): void => {
  if (!event) {
    return;
  }

  switch (event.type) {
    case EventType.RequestCloned:
      notifiation.info(
        `Project for "${event.urn}" found and cloned.`,
        false,
        "Show Project",
        () => push(path.project(event.urn))
      );

      break;

    case EventType.RequestTimedOut:
      notifiation.error(`Search for "${event.urn}" failed.`);

      break;
  }
});

export const requestEvents: Readable<
  RequestCloned | RequestQueried | RequestTimedOut | null
> = derived(eventStore, (event: Event | null):
  | RequestCloned
  | RequestTimedOut
  | RequestQueried
  | null => {
  if (!event) {
    return null;
  }

  switch (event.type) {
    case EventType.RequestCloned:
    case EventType.RequestQueried:
    case EventType.RequestTimedOut:
      return event;

    default:
      return null;
  }
});

export const status: Readable<remote.Data<Status>> = derived(
  eventStore,
  (event: Event | null, set: (status: remote.Data<Status>) => void): void => {
    if (event && event.type === EventType.StatusChanged) {
      set({ status: remote.Status.Success, data: event.new });
    }
  },
  { status: remote.Status.Loading }
);
