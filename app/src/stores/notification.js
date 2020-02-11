import { writable } from "svelte/store";

export const notificationStore = writable(null);

export const showNotification = message => {
  notification.set(message);
};
