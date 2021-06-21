// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Client representation of proxy waiting room requests, subscriptions, etc.
import * as zod from "zod";

export enum Status {
  Created = "created",
  Requested = "requested",
  Found = "found",
  Cloning = "cloning",
  Cloned = "cloned",
  Cancelled = "cancelled",
  Failed = "failed",
  TimedOut = "timedOut",
}

export interface ProjectRequest {
  type: Status;
  urn: string;
}

export enum PeerStatus {
  Available = "available",
  InProgress = "inProgress",
  Failed = "failed",
}

interface TickEvent {
  type: "tick";
}
interface CreatedEvent {
  type: "created";
}
interface QueriedEvent {
  type: "queried";
}
interface FoundEvent {
  type: "found";
}
interface CloningEvent {
  type: "cloning";
}
interface CloningFailedEvent {
  type: "cloningFailed";
}
interface ClonedEvent {
  type: "cloned";
}
interface CanceledEvent {
  type: "canceled";
}
export type Event =
  | TickEvent
  | CreatedEvent
  | QueriedEvent
  | FoundEvent
  | CloningEvent
  | CloningFailedEvent
  | ClonedEvent
  | CanceledEvent;

export const eventSchema: zod.Schema<Event> = zod.union([
  zod.object({
    type: zod.literal("tick"),
  }),
  zod.object({
    type: zod.literal("created"),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal("queried"),
    urn: zod.string(),
  }),
  zod.object({
    type: zod.literal("found"),
    urn: zod.string(),
    peer: zod.string(),
  }),
  zod.object({
    type: zod.literal("cloning"),
    urn: zod.string(),
    peer: zod.string(),
  }),
  zod.object({
    type: zod.literal("cloningFailed"),
    urn: zod.string(),
    peer: zod.string(),
    reason: zod.string(),
  }),
  zod.object({
    type: zod.literal("cloned"),
    urn: zod.string(),
    peer: zod.string(),
  }),
  zod.object({
    type: zod.literal("canceled"),
    urn: zod.string(),
  }),
]);

export interface RoomState {
  [revision: string]: {
    state:
      | "Created"
      | "Requested"
      | "Found"
      | "Cloning"
      | "Cloned"
      | "Cancelled"
      | "Failed"
      | "TimedOut";
    peers: {
      [peerId: string]:
        | "available"
        | "inProgress"
        | { failed: { reason: string } };
    };
  };
}

export const roomStateSchema: zod.Schema<RoomState> = zod.record(
  zod.object({
    state: zod.union([
      zod.literal("Created"),
      zod.literal("Requested"),
      zod.literal("Found"),
      zod.literal("Cloning"),
      zod.literal("Cloned"),
      zod.literal("Cancelled"),
      zod.literal("Failed"),
      zod.literal("TimedOut"),
    ]),
    peers: zod.record(
      zod.union([
        zod.literal("available"),
        zod.literal("inProgress"),
        zod.object({
          failed: zod.object({
            reason: zod.string(),
          }),
        }),
      ])
    ),
  })
);
