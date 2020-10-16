import { derived, writable } from "svelte/store";

const currentStore = writable<HTMLDivElement | undefined>(undefined);
export const current = derived(currentStore, store => store);

export const open = (component: HTMLDivElement): void => {
  currentStore.set(component);
};

export const close = (): void => {
  currentStore.set(undefined);
};
