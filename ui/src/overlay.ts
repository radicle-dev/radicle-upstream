import { derived, writable } from "svelte/store";
import { SvelteComponent } from "svelte";

const currentlyOpenStore = writable<SvelteComponent | undefined>(undefined);
export const currentlyOpen = derived(currentlyOpenStore, store => store);

export const openOverlay = (component: SvelteComponent) => {
  currentlyOpenStore.set(component);
};

export const closeOverlay = () => {
  currentlyOpenStore.set(undefined);
};
