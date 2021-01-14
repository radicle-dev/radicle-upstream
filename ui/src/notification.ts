import { Readable, derived, get, writable } from "svelte/store";

import * as config from "./config";

export enum Level {
  Error = "ERROR",
  Info = "INFO",
}

export interface NotificationParams {
  message: string;
  showIcon?: boolean;
  // A list of actions to show as part of the notification. If not
  // provided a default action to close the notification will be shown.
  actions?: Action[];
}

export interface Notification {
  readonly id: number;
  readonly level: Level;
  readonly showIcon: boolean;
  readonly message: string;
  readonly actions: readonly Action[];
}

export interface Action {
  readonly label: string;
  readonly handler: () => void;
}

const notificationsStore = writable<Notification[]>([]);

export const store: Readable<Notification[]> = derived(
  notificationsStore,
  (state: Notification[]) => state
);

const closeAction: Action = {
  label: "Close",
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  handler: () => {},
};

const show = (level: Level, params: NotificationParams): void => {
  const id = Math.random();
  const showIcon = params.showIcon || false;

  let actions = params.actions || [closeAction];
  actions = actions.map(action => ({
    label: action.label,
    handler: () => {
      action.handler();
      remove(id);
    },
  }));

  const notification = {
    id,
    level,
    message: params.message,
    showIcon,
    actions,
  };

  notificationsStore.update(notifications => [notification, ...notifications]);

  setTimeout(() => {
    remove(id);
  }, config.NOTIFICATION_TIMEOUT);
};

export const error = (params: NotificationParams): void =>
  show(Level.Error, params);

export const info = (params: NotificationParams): void =>
  show(Level.Info, params);

const remove = (id: number): void => {
  const notifications = get(notificationsStore).filter(
    (n: Notification) => n.id !== id
  );
  notificationsStore.set(notifications);
};
