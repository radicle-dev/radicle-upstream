import { Readable, derived, get, writable } from "svelte/store";

interface History<Item> {
  readonly current: Readable<Item>;
  pop(): void;
  push(item: Item): void;
  reset(): void;
}

export const create = <Item>(initial: Item): History<Item> => {
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
      const currentItem = safeGet(store);
      if (item === currentItem) {
        return;
      }

      history.push(currentItem);
      store.set(item);
    },
    reset: (): void => {
      history = [];
      store.set(initial);
    },
  };
};

const safeGet = <Item>(store: Readable<Item>): Item => {
  return get(store) as Item;
};
