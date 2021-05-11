import type { SvelteComponent } from "svelte";
import * as svelteStore from "svelte/store";

export { default as Router } from "ui/src/router/Router.svelte";

export type Route = {
  component: typeof SvelteComponent | null;
  // any object | empty object
  props: Record<string, unknown> | Record<string, never>;
};

const writableHistory: svelteStore.Writable<Route[]> = svelteStore.writable([]);
const emptyRoute = { component: null, props: {} };

export const push = (newRoute: Route): void => {
  const oldHistory = svelteStore.get(writableHistory);
  writableHistory.set([...oldHistory, newRoute]);
};

export const pop = (): void => {
  const oldHistory = svelteStore.get(writableHistory);
  writableHistory.set(oldHistory.slice(0, -1));
};

export const routeStore: svelteStore.Readable<Route> = svelteStore.derived(
  writableHistory,
  state => {
    if (state.length === 0) {
      return emptyRoute;
    } else {
      return state.slice(-1)[0];
    }
  }
);
