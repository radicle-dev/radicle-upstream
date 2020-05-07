import { Writable, writable } from "svelte/store";

import * as config from "./config";
import * as event from "./event";

// TYPES
type ID = number;

export enum Level {
  Error = "ERROR",
  Info = "INFO",
}

interface Notification {
  id: ID;
  level: Level;
  message: string;
}

type Notifications = Notification[]

// STATE
let notifications: Notifications = [];
export const store: Writable<Notifications> = writable([]);

// EVENTS
enum Kind {
  Remove = "REMOVE",
  ShowError = "SHOW_ERROR",
  ShowInfo = "SHOW_INFO",
}

interface Remove extends event.Event<Kind> {
  kind: Kind.Remove;
  id: ID;
}

interface ShowError extends event.Event<Kind> {
  kind: Kind.ShowError;
  message: string;
}

interface ShowInfo extends event.Event<Kind> {
  kind: Kind.ShowInfo;
  message: string;
}

type Msg = Remove | ShowError | ShowInfo;

const filter = (id: ID): void => {
  notifications = notifications.filter((n) => n.id !== id);
  store.set(notifications);
}

const show = (level: Level, message: string): void => {
  const id = Math.random();
  notifications = [
    ...notifications,
    {
      id,
      level,
      message,
    }
  ];
  store.set(notifications);

  setTimeout(() => {
    filter(id);
  }, config.NOTIFICATION_TIMEOUT);
}

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.ShowError:
      show(Level.Error, msg.message);

      break;

    case Kind.ShowInfo:
      show(Level.Info, msg.message);

      break;

    case Kind.Remove:
      console.log("remove", msg.id);
      filter(msg.id);

      break;
  }
}

export const error = (message: string): void =>
  event.create<Kind, Msg>(Kind.ShowError, update)({ message });
export const info = (message: string): void =>
  event.create<Kind, Msg>(Kind.ShowInfo, update)({ message });
export const remove = (id: ID): void =>
  event.create<Kind, Msg>(Kind.Remove, update)({ id });
