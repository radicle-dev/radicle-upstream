// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as router from "ui/src/router";
import type { Readable, Writable } from "svelte/store";
import { writable } from "svelte/store";
import * as svelteStore from "svelte/store";

import {
  EventType,
  Event,
  ProjectUpdated,
  RequestEvent,
  type Status,
  StatusType,
  WaitingRoomTransition,
} from "proxy-client/events";
import type { RoomState } from "proxy-client/events/waitingRoom";

import * as notification from "./notification";
import * as remote from "./remote";
import * as session from "./session";
import * as error from "./error";
import * as bacon from "./bacon";
import * as Proxy from "./proxy";

export { type Status, StatusType };

let events: bacon.EventStream<Event> | null = null;

session.session.subscribe(sess => {
  if (
    sess.status === remote.Status.Success &&
    sess.data.status === session.Status.UnsealedSession
  ) {
    if (events === null) {
      events = Proxy.client.events();
      events.subscribe(event => {
        if (event instanceof bacon.End) {
          events = null;
        } else if (event instanceof bacon.Next) {
          eventBus.push(event.value);
        } else if (event instanceof bacon.Error) {
          notification.showException(
            new error.Error({
              message: "Received proxy peer event",
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
