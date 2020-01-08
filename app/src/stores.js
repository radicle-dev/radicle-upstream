import { writable } from "svelte/store";

export const revision = writable(null);
export const objectPath = writable(null);
export const objectType = writable(null);

export const notification = writable(null);

export const showNotification = message => {
  notification.set(message);
};
