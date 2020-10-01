import { derived, writable } from "svelte/store";

const currentStore = writable<HTMLDivElement | undefined>(undefined);
export const current = derived(currentStore, store => store);

export const open = (component: HTMLDivElement) => {
  currentStore.set(component);
};

export const close = () => {
  currentStore.set(undefined);
};
