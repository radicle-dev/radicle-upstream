import { Readable, derived, get, writable } from "svelte/store";

interface Navigation<Item> {
  current: Readable<Item>;
  pop(): void;
  push(item: Item): void;
  reset(): void;
}

export const create = <Item>(initial: Item): Navigation<Item> => {
  const store = writable<Item>(initial);
  const current = derived(store, store => store);
  let history: Item[] = [];

  return {
    current,
    pop: (): void => {
      const newItem = history.pop();

      // TODO(xla): Figure out behaviour for empty history.
      if (newItem) {
        store.set(newItem);
      }
    },
    push: (item: Item): void => {
      const currentItem = get(store);
      history.push(currentItem);

      store.set(item);
    },
    reset: (): void => {
      history = [];
      store.set(initial);
    },
  };
};
