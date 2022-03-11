// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as router from "ui/src/router";
import type { Readable, Writable } from "svelte/store";
import { writable } from "svelte/store";
import * as zod from "zod";
import * as svelteStore from "svelte/store";

import type * as identity from "./identity";
import { config } from "./config";
import * as notification from "./notification";
import * as remote from "./remote";
import * as session from "./session";
import * as error from "./error";
import * as bacon from "./bacon";
import type { Event as RoomEvent, RoomState } from "./waitingRoom";
import { eventSchema as roomEventSchema, roomStateSchema } from "./waitingRoom";

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
  connectedPeers: { [peerId: string]: string[] };
}

export type Status = Stopped | Offline | Started | Syncing | Online;

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
    connectedPeers: zod.record(zod.array(zod.string())),
  }),
]);

enum EventType {
  ProjectUpdated = "projectUpdated",
  RequestCreated = "requestCreated",
  RequestQueried = "requestQueried",
  RequestCloned = "requestCloned",
  RequestTimedOut = "requestTimedOut",
  StatusChanged = "statusChanged",
  WaitingRoomTransition = "waitingRoomTransition",
}

interface ProjectUpdated {
  type: EventType.ProjectUpdated;
  urn: string;
}

interface RequestCreated {
  type: EventType.RequestCreated;
  urn: string;
}

interface RequestCloned {
  type: EventType.RequestCloned;
  peer: identity.PeerId;
  urn: string;
}

interface RequestQueried {
  type: EventType.RequestQueried;
  urn: string;
}

interface RequestTimedOut {
  type: EventType.RequestTimedOut;
  urn: string;
}

export interface WaitingRoomTransition {
  type: EventType.WaitingRoomTransition;
  event: RoomEvent;
  timestamp: number;
  state_before: RoomState;
  state_after: RoomState;
}

type RequestEvent =
  | RequestCreated
  | RequestCloned
  | RequestQueried
  | RequestTimedOut;

export type Event =
  | ProjectUpdated
  | RequestEvent
  | WaitingRoomTransition
  | { type: EventType.StatusChanged; old: Status; new: Status };

const eventSchema: zod.Schema<Event> = zod.union([
  zod.object({
    type: zod.literal(EventType.ProjectUpdated),
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
    type: zod.literal(EventType.WaitingRoomTransition),
    timestamp: zod.number(),
    state_before: roomStateSchema,
    state_after: roomStateSchema,
    event: roomEventSchema,
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
          notification.showException(
            new error.Error({
              code: error.Code.ProxyEventParseFailure,
              message: "Failed to parse proxy event",
              details: {
                event: data,
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
      notification.show({
        type: "info",
        message: `Project for "${event.urn}" found and cloned.`,
        actions: [
          {
            label: "Show Project",
            handler: () =>
              router.push({
                type: "project",
                params: {
                  urn: event.urn,
                  activeView: { type: "files" },
                },
              }),
          },
        ],
      });

      break;

    case EventType.RequestTimedOut:
      notification.show({
        type: "error",
        message: `Search for "${event.urn}" failed.`,
      });

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

const internalWaitingRoomState = svelteStore.writable<RoomState | null>(null);
export const waitingRoomState: Readable<RoomState | null> =
  internalWaitingRoomState;

eventBus.onValue(event => {
  if (event.type === EventType.StatusChanged) {
    status.set(event.new);
  }
  if (event.type === EventType.WaitingRoomTransition) {
    internalWaitingRoomState.set(event.state_after);
  }
});

const waitingRoomEvents: bacon.Property<WaitingRoomTransition[]> = bacon
  .filterMap(eventBus, event => {
    if (
      event.type === EventType.WaitingRoomTransition &&
      event.event.type !== "tick"
    ) {
      return event;
    } else {
      return undefined;
    }
  })
  .scan<WaitingRoomTransition[]>([], (acc, event) =>
    [...acc, event].slice(-200)
  );

const waitingRoomEventLogStore: Writable<WaitingRoomTransition[]> = writable(
  []
);
waitingRoomEvents.onValue(events => waitingRoomEventLogStore.set(events));
export const waitingRoomEventLog: Readable<WaitingRoomTransition[]> =
  waitingRoomEventLogStore;
