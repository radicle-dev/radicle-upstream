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
  showIcon: boolean;
  message: string;
  actionText: string;
  actionHandler: () => void;
}

type Notifications = Notification[];

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
  showIcon: boolean;
  actionText?: string;
  actionHandler?: () => void;
}

interface ShowInfo extends event.Event<Kind> {
  kind: Kind.ShowInfo;
  message: string;
  showIcon: boolean;
  actionText?: string;
  actionHandler?: () => void;
}

type Msg = Remove | ShowError | ShowInfo;

const filter = (id: ID): void => {
  notifications = notifications.filter(n => n.id !== id);
  store.set(notifications);
};

const show = (
  level: Level,
  showIcon: boolean,
  message: string,
  actionText?: string,
  actionHandler?: () => void
): void => {
  const id = Math.random();
  notifications = [
    ...notifications,
    {
      id,
      level,
      message,
      showIcon,
      actionText: actionText || "Close",
      actionHandler: () => {
        if (actionHandler) {
          actionHandler();
        }

        remove(id);
      },
    },
  ];
  store.set(notifications);

  setTimeout(() => {
    filter(id);
  }, config.NOTIFICATION_TIMEOUT);
};

const update = (msg: Msg): void => {
  switch (msg.kind) {
    case Kind.ShowError:
      show(
        Level.Error,
        msg.showIcon,
        msg.message,
        msg.actionText,
        msg.actionHandler
      );

      break;

    case Kind.ShowInfo:
      show(
        Level.Info,
        msg.showIcon,
        msg.message,
        msg.actionText,
        msg.actionHandler
      );

      break;

    case Kind.Remove:
      filter(msg.id);

      break;
  }
};

const remove = (id: ID): void =>
  event.create<Kind, Msg>(Kind.Remove, update)({ id });

export const error = (
  message: string,
  showIcon = false,
  actionText?: string,
  actionHandler?: () => void
): void =>
  event.create<Kind, Msg>(
    Kind.ShowError,
    update
  )({ message, showIcon, actionText, actionHandler });

export const info = (
  message: string,
  showIcon = false,
  actionText?: string,
  actionHandler?: () => void
): void =>
  event.create<Kind, Msg>(
    Kind.ShowInfo,
    update
  )({ message, showIcon, actionText, actionHandler });
