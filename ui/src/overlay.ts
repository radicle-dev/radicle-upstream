import { derived, writable } from "svelte/store";
import { SvelteComponent } from "svelte";

const currentlyOpenStore = writable<SvelteComponent | undefined>(undefined);
export const currentlyOpen = derived(currentlyOpenStore, store => store);

export const openDropdown = (component: SvelteComponent) => {
  currentlyOpenStore.set(component);
};

export const closeCurrentDropdown = () => {
  currentlyOpenStore.set(undefined);
};
