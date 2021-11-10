// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import lodash from "lodash";

import * as browserStore from "ui/src/browserStore";
import * as error from "ui/src/error";
import * as ipc from "ui/src/ipc";
import * as svelteStore from "ui/src/svelteStore";

type NotificationType = "error" | "info" | "primary";

interface SerializedNotification {
  readonly timestamp: number;
  readonly type: string;
  readonly message: string;
  readonly details?: unknown;
}

export interface Notification {
  readonly id: number;
  readonly type: NotificationType;
  readonly message: string;
  readonly actions: readonly Action[];
  readonly bypassLockedScreen?: boolean;
  readonly details?: unknown;
  hideTimerHandle?: number;
  timesShownStore: svelteStore.Writable<number>;
}

export interface Action {
  readonly label: string;
  readonly handler: () => void;
}

// Handle to remove notifications. Returned when a notification is created.
export interface Handle {
  // Don’t show the notification anymore.
  remove(): void;
}

export const notificationStore = svelteStore.writable<Notification[]>([]);

export const notificationHistory = browserStore.create<
  SerializedNotification[]
>(
  "radicle.notificationHistory",
  [],
  zod.array(
    zod.object({
      timestamp: zod.number(),
      type: zod.string(),
      message: zod.string(),
      details: zod.unknown(),
    })
  )
);

error.notifications.onValue(error => {
  showException(error);
});

const closeAction: Action = {
  label: "Close",
  handler: () => {},
};

export function removeHideTimer(notification: Notification) {
  window.clearTimeout(notification.hideTimerHandle);
  notification.hideTimerHandle = undefined;
}

export function attachHideTimer(notification: Notification) {
  if (notification.hideTimerHandle) {
    removeHideTimer(notification);
  }

  notification.hideTimerHandle = window.setTimeout(() => {
    remove(notification.id);
  }, 8000);
}

export function show(params: {
  type: NotificationType;
  message: string;
  // A list of actions to show as part of the notification. If not
  // provided a default action to close the notification will be shown.
  actions?: Action[];
  // If `true`, the notification does not automatically disappear after
  // a certain time. Defaults to `false`.
  persist?: boolean;
  // If `true`, the user is allowed to interact with the notification even when
  // the screen is in a waiting waiting state, i.e. cursor is a spinner and
  // mouse clicks are disabled.
  bypassLockedScreen?: boolean;
  // Any additional metadata that can be serialized with `JSON.serialize`, e.g.
  // the exception stack for error notifications.
  details?: unknown;
}): Handle {
  notificationHistory.update(
    history =>
      [
        ...history,
        {
          timestamp: Date.now(),
          type: params.type,
          message: params.message,
          details: params.details,
        },
      ].slice(-20) // Limit history to a maximum of 20 items.
  );

  const existingNotification = svelteStore
    .get(notificationStore)
    .find(
      storedNotification =>
        storedNotification.type === params.type &&
        storedNotification.message === params.message &&
        lodash.isEqual(storedNotification.details, params.details)
    );

  if (existingNotification) {
    existingNotification.timesShownStore.update(n => n + 1);
    attachHideTimer(existingNotification);
    return {
      remove: () => remove(existingNotification.id),
    };
  } else {
    const id = Math.random();

    let actions = params.actions || [closeAction];
    actions = actions.map(action => ({
      label: action.label,
      handler: () => {
        action.handler();
        remove(id);
      },
    }));

    const notification: Notification = {
      id,
      actions,
      timesShownStore: svelteStore.writable<number>(1),
      type: params.type,
      message: params.message,
      bypassLockedScreen: params.bypassLockedScreen,
      details: params.details,
    };

    if (params.persist !== true) {
      attachHideTimer(notification);
    }

    notificationStore.update(notifications => [notification, ...notifications]);

    return {
      remove: () => remove(notification.id),
    };
  }
}

function remove(id: number): void {
  notificationStore.update(notifications => {
    return notifications.filter(notification => {
      if (notification.id === id) {
        removeHideTimer(notification);
        return false;
      } else {
        return true;
      }
    });
  });
}

// Show an error notification and log the error to the console.
export function showException(exception: error.Error): Handle {
  error.log(exception);

  return show({
    type: "error",
    message: exception.message,
    persist: false,
    details: exception,
    actions: [
      {
        label: "Copy error",
        handler: () => {
          ipc.copyToClipboard(JSON.stringify(exception, null, 2));
        },
      },
      {
        label: "Dismiss",
        handler: () => {},
      },
    ],
  });
}
