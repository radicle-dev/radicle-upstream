import { Readable, derived, writable } from "svelte/store";
import * as config from "./config";

export enum Variant {
  Error = "ERROR",
  Info = "INFO",
  // Uses primary color as the notification background
  Primary = "PRIMARY",
}

export interface NotificationParams {
  message: string;
  // If `true`, show an appropriate icon as part of the notification.
  // An icon is only shown for the `Error` and `Info` notification
  // variants. Defaults to `false`.
  showIcon?: boolean;
  // A list of actions to show as part of the notification. If not
  // provided a default action to close the notification will be shown.
  actions?: Action[];
  // If `true`, the notification does not automatically disappear after
  // a certain time. Defaults to `false`.
  persist?: boolean;
}

export interface Notification {
  readonly id: number;
  readonly variant: Variant;
  readonly showIcon: boolean;
  readonly message: string;
  readonly actions: readonly Action[];
  readonly icon: Icon | null;
}

// We can’t use `DesignSystem/Primitives/Icon` directly because this
// file is imported in the Jest tests and they do not support Svelte.
export enum Icon {
  InfoCircle = "InfoCircle",
  ExclamationCircle = "ExclamationCircle",
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

// Only exported for `DesignSystemGuide`.
export const create = (
  variant: Variant,
  params: NotificationParams
): Notification => {
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

  let icon = null;
  if (params.showIcon) {
    switch (variant) {
      case Variant.Info:
        icon = Icon.InfoCircle;
        break;
      case Variant.Error:
        icon = Icon.ExclamationCircle;
        break;
      case Variant.Primary:
        icon = null;
        break;
    }
  }

  return {
    id,
    variant,
    message: params.message,
    showIcon,
    actions,
    icon,
  };
};

const show = (variant: Variant, params: NotificationParams): void => {
  const notification = create(variant, params);
  notificationsStore.update(notifications => [notification, ...notifications]);

  if (params.persist !== true) {
    setTimeout(() => {
      remove(notification.id);
    }, config.NOTIFICATION_TIMEOUT);
  }
};

export const error = (params: NotificationParams): void =>
  show(Variant.Error, params);

export const info = (params: NotificationParams): void =>
  show(Variant.Info, params);

export const primary = (params: NotificationParams): void =>
  show(Variant.Primary, params);

const remove = (id: number): void => {
  notificationsStore.update(notifications => {
    return notifications.filter((n: Notification) => n.id !== id);
  });
};
