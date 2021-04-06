import { derived, writable, Readable } from "svelte/store";
import { push } from "svelte-spa-router";

import type * as identity from "./identity";
import * as config from "./config";
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
  ProjectUpdated = "projectUpdated",
  RequestCreated = "requestCreated",
  RequestQueried = "requestQueried",
  RequestCloned = "requestCloned",
  RequestTimedOut = "requestTimedOut",
  StatusChanged = "statusChanged",
}

interface ProjectUpdated {
  type: EventType.ProjectUpdated;
  peer: identity.PeerId;
  urn: urn.Urn;
}

interface RequestCreated {
  type: EventType.RequestCreated;
  urn: urn.Urn;
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

type RequestEvent =
  | RequestCreated
  | RequestCloned
  | RequestQueried
  | RequestTimedOut;

export type Event =
  | ProjectUpdated
  | RequestEvent
  | { type: EventType.StatusChanged; old: Status; new: Status };

let eventSource: EventSource | null = null;

session.session.subscribe(sess => {
  if (
    sess.status === remote.Status.Success &&
    sess.data.status === session.Status.UnsealedSession
  ) {
    if (eventSource === null || eventSource.readyState === EventSource.CLOSED) {
      eventSource = new EventSource(
        `http://${config.proxyAddress}/v1/notifications/local_peer_events`,
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
      notifiation.info({
        message: `Project for "${event.urn}" found and cloned.`,
        actions: [
          {
            label: "Show Project",
            handler: () => push(path.project(event.urn)),
          },
        ],
      });

      break;

    case EventType.RequestTimedOut:
      notifiation.error({ message: `Search for "${event.urn}" failed.` });

      break;
  }
});

export const projectEvents: Readable<ProjectUpdated | null> = derived(
  eventStore,
  (event: Event | null): ProjectUpdated | null => {
    if (!event) {
      return null;
    }

    switch (event.type) {
      case EventType.ProjectUpdated:
        return event;

      default:
        return null;
    }
  }
);

export const requestEvents: Readable<RequestEvent | null> = derived(
  eventStore,
  (event: Event | null): RequestEvent | null => {
    if (!event) {
      return null;
    }

    switch (event.type) {
      case EventType.RequestCloned:
      case EventType.RequestQueried:
      case EventType.RequestTimedOut:
      case EventType.RequestCreated:
        return event;

      default:
        return null;
    }
  }
);

export const status: Readable<remote.Data<Status>> = derived(
  eventStore,
  (event: Event | null, set: (status: remote.Data<Status>) => void): void => {
    if (event && event.type === EventType.StatusChanged) {
      set({ status: remote.Status.Success, data: event.new });
    }
  },
  { status: remote.Status.Loading }
);
