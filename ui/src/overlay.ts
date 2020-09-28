import { SvelteComponent } from "svelte";
import { derived, writable } from "svelte/store";

const currentStore = writable<SvelteComponent | undefined>(undefined);
export const current= derived(currentStore, store => store);

export const open = (component: SvelteComponent) => {
  currentStore.set(component);
};

export const close = () => {
  currentStore.set(undefined);
};
