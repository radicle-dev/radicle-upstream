import { Readable, derived, get, writable } from "svelte/store";

import * as config from "./config";
import * as event from "./event";

// TYPES
type ID = number;

export enum Level {
  Error = "ERROR",
  Info = "INFO",
}

type ActionHandler = () => void;

interface Notification {
  id: ID;
  level: Level;
  showIcon: boolean;
  message: string;
  actionText: string | false;
  actionHandler: ActionHandler | false;
}

type Notifications = Notification[];

// STATE
const notificationsStore = writable<Notification[]>([]);
export const store: Readable<Notifications> = derived(
  notificationsStore,
  (state: Notifications) => state
);

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
  actionText: string | false;
  actionHandler: ActionHandler | false;
}

interface ShowInfo extends event.Event<Kind> {
  kind: Kind.ShowInfo;
  message: string;
  showIcon: boolean;
  actionText: string | false;
  actionHandler: ActionHandler | false;
}

type Msg = Remove | ShowError | ShowInfo;

const filter = (id: ID): void => {
  const notifications = get(notificationsStore).filter(
    (n: Notification) => n.id !== id
  );
  notificationsStore.set(notifications);
};

const show = (
  level: Level,
  showIcon: boolean,
  message: string,
  actionText: string | false,
  actionHandler: ActionHandler | false
): void => {
  const id = Math.random();
  const notification = {
    id,
    level,
    message,
    showIcon,
    actionText,
    actionHandler: () => {
      if (actionHandler) {
        actionHandler();
      }

      remove(id);
    },
  };

  notificationsStore.update(notifications => [notification, ...notifications]);

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
  showIcon: boolean = false,
  actionText: string | false = "Close",
  actionHandler: ActionHandler | false = false
): void =>
  event.create<Kind, Msg>(
    Kind.ShowError,
    update
  )({ message, showIcon, actionText, actionHandler });

export const info = (
  message: string,
  showIcon: boolean = false,
  actionText: string | false = "Close",
  actionHandler: ActionHandler | false = false
): void =>
  event.create<Kind, Msg>(
    Kind.ShowInfo,
    update
  )({ message, showIcon, actionText, actionHandler });
