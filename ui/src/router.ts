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

function getLocation() {
  const hashPosition = window.location.href.indexOf("#/");
  let location =
    hashPosition > -1 ? window.location.href.substr(hashPosition + 1) : "/";
  // Check if there's a querystring
  const qsPosition = location.indexOf("?");
  let querystring = "";
  if (qsPosition > -1) {
    querystring = location.substr(qsPosition + 1);
    location = location.substr(0, qsPosition);
  }
  return { location, querystring };
}
/**
 * Readable store that returns the current full location (incl. querystring)
 */
export const loc = svelteStore.readable(
  null,
  // eslint-disable-next-line prefer-arrow-callback
  function start(set) {
    set(getLocation());
    const update = () => {
      set(getLocation());
    };
    window.addEventListener("hashchange", update, false);
    return function stop() {
      window.removeEventListener("hashchange", update, false);
    };
  }
);
/**
 * Readable store that returns the current location
 */
export const location = svelteStore.derived(loc, $loc => $loc.location);
