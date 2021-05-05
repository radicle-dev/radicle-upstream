import { derived, get, Readable, writable, Writable } from "svelte/store";
export { default as Router } from "ui/src/router/Router.svelte";

const writableLocation: Writable<string[] | []> = writable([]);

export const location: Readable<string> = derived(
  writableLocation,
  $store => $store.pop() || ""
);

export const push = (newLocation: string): void => {
  const oldLocation = get(writableLocation);
  writableLocation.set([...oldLocation, newLocation]);
};

export const pop = (): void => {
  const oldLocation = get(writableLocation);
  writableLocation.set(oldLocation.slice(1));
};
