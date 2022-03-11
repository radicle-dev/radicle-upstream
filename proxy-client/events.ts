// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import * as bacon from "baconjs";

import * as waitingRoom from "./events/waitingRoom";

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

export enum EventType {
  ProjectUpdated = "projectUpdated",
  RequestCreated = "requestCreated",
  RequestQueried = "requestQueried",
  RequestCloned = "requestCloned",
  RequestTimedOut = "requestTimedOut",
  StatusChanged = "statusChanged",
  WaitingRoomTransition = "waitingRoomTransition",
}

export interface ProjectUpdated {
  type: EventType.ProjectUpdated;
  urn: string;
}

interface RequestCreated {
  type: EventType.RequestCreated;
  urn: string;
}

interface RequestCloned {
  type: EventType.RequestCloned;
  peer: string;
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
  event: waitingRoom.Event;
  timestamp: number;
  state_before: waitingRoom.RoomState;
  state_after: waitingRoom.RoomState;
}

export type RequestEvent =
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
    state_before: waitingRoom.roomStateSchema,
    state_after: waitingRoom.roomStateSchema,
    event: waitingRoom.eventSchema,
  }),
  zod.object({
    type: zod.literal(EventType.StatusChanged),
    old: statusSchema,
    new: statusSchema,
  }),
]);

interface EventSource {
  addEventListener(
    type: "error",
    listener: (event: globalThis.Event) => unknown
  ): void;
  addEventListener(
    type: "message",
    listener: (event: globalThis.MessageEvent) => unknown
  ): void;
  close(): void;
}

export interface EventSourceConstructor {
  new (url: string, options?: { withCredentials: boolean }): EventSource;
}

export function events(
  EventSource: EventSourceConstructor,
  baseUrl: string
): bacon.EventStream<Event> {
  return bacon.fromBinder(send => {
    const eventSource = new EventSource(
      `${baseUrl}/v1/notifications/local_peer_events`,
      { withCredentials: true }
    );
    eventSource.addEventListener("error", event => {
      send(new bacon.Error(event));
      send(new bacon.End());
    });
    eventSource.addEventListener("message", msg => {
      const data = JSON.parse(msg.data);
      const result = eventSchema.safeParse(data);
      if (result.success) {
        send(new bacon.Next(result.data));
      } else {
        send(new bacon.Error(new Error("Failed to parse proxy event")));
      }
    });
    return function () {
      eventSource.close();
    };
  });
}
