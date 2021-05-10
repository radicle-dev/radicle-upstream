import type { SvelteComponent } from "svelte";
import * as svelteStore from "svelte/store";

export { default as Router } from "ui/src/router/Router.svelte";

export type State = {
  component: typeof SvelteComponent | null;
  // any object | empty object
  props: Record<string, unknown> | Record<string, never>;
};

const writableHistory: svelteStore.Writable<State[]> = svelteStore.writable([]);
const emptyState = { component: null, props: {} };

export const push = (newState: State): void => {
  const oldHistory = svelteStore.get(writableHistory);
  writableHistory.set([...oldHistory, newState]);
};

export const pop = (): void => {
  const oldHistory = svelteStore.get(writableHistory);
  writableHistory.set(oldHistory.slice(0, -1));
};

export const state: svelteStore.Readable<State> = svelteStore.derived(
  writableHistory,
  state => {
    if (state.length === 0) {
      return emptyState;
    } else {
      return state.slice(-1)[0];
    }
  }
);
