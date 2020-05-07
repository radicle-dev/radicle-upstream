import { Writable, writable } from "svelte/store";

import * as event from "./event";

// TYPES
export enum Level {
  Error = "ERROR",
  Info = "INFO",
}

interface Notification {
  level: Level;
  message: string;
}

// STATE
export const store: Writable<Notification | null> = writable(null);

// EVENTS
enum Kind {
  ShowError = "SHOW_ERROR",
  ShowInfo = "SHOW_INFO",
}

interface ShowError extends event.Event<Kind> {
  kind: Kind.ShowError;
  message: string;
}

interface ShowInfo extends event.Event<Kind> {
  kind: Kind.ShowInfo;
  message: string;
}

type Msg = ShowError | ShowInfo;

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.ShowError:
      store.set({
        level: Level.Error,
        message: msg.message,
      });
      break;

    case Kind.ShowInfo:
      store.set({
        level: Level.Info,
        message: msg.message,
      });
      break;

  }
}

export const error = event.create<Kind, Msg>(Kind.ShowError, update);
export const info = event.create<Kind, Msg>(Kind.ShowInfo, update);
