import { push } from "svelte-spa-router";
import * as zod from "zod";
import * as svelteStore from "svelte/store";

import type * as identity from "./identity";
import * as config from "./config";
import * as notifiation from "./notification";
import * as path from "./path";
import * as remote from "./remote";
import * as session from "./session";
import type * as urn from "./urn";
import * as error from "./error";
import * as bacon from "./bacon";

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

const statusSchema = zod.union([
  zod.object({
    type: zod.literal(StatusType.Stopped),
  }),
  zod.object({
    type: zod.literal(StatusType.Offline),
  }),
  zod.object({
    type: zod.literal(StatusType.Started),
  }),
  zod.object({
    type: zod.literal(StatusType.Syncing),
    syncs: zod.number(),
  }),
  zod.object({
    type: zod.literal(StatusType.Online),
    connected: zod.number(),
  }),
]);

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
  provider: identity.PeerId;
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

const eventSchema: zod.Schema<Event> = zod.union([
  zod.object({
    type: zod.literal(EventType.ProjectUpdated),
    provider: zod.string(),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal(EventType.RequestCreated),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal(EventType.RequestCloned),
    peer: zod.string(),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal(EventType.RequestQueried),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal(EventType.RequestTimedOut),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal(EventType.StatusChanged),
    old: statusSchema,
    new: statusSchema,
  }),
]);

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
        const data = JSON.parse(msg.data);
        const result = eventSchema.safeParse(data);
        if (result.success) {
          eventBus.push(result.data);
        } else {
          error.show(
            new error.Error({
              code: error.Code.ProxyEventParseFailure,
              message: "Failed to parse proxy event",
              details: {
                errors: result.error.errors,
              },
            })
          );
        }
      });
    }
  }
});

const eventBus = new bacon.Bus<Event>();

eventBus.onValue(event => {
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

export const projectEvents: bacon.EventStream<ProjectUpdated> = bacon.filterMap(
  eventBus,
  event => {
    if (event.type === EventType.ProjectUpdated) {
      return event;
    }
  }
);

export const requestEvents: bacon.EventStream<RequestEvent> = bacon.filterMap(
  eventBus,
  event => {
    switch (event.type) {
      case EventType.RequestCloned:
      case EventType.RequestQueried:
      case EventType.RequestTimedOut:
      case EventType.RequestCreated:
        return event;

      default:
        return undefined;
    }
  }
);

export const status = svelteStore.writable<Status>({
  type: StatusType.Offline,
});

eventBus.onValue(event => {
  if (event.type === EventType.StatusChanged) {
    status.set(event.new);
  }
});
