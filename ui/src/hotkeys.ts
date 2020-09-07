import { get, writable } from "svelte/store";

const state = writable(true);

export const areEnabled = (): boolean => {
  return get(state);
};

export const enable = (): void => {
  state.set(true);
};

export const disable = (): void => {
  state.set(false);
};
