import { get } from "svelte/store";

import * as navigation from "./navigation";

interface Item {
  inner: number;
}

describe("navigation", () => {
  describe("initial state", () => {
    it("is the one passed to the constructor", () => {
      const initial = { inner: 12 };
      const nav = navigation.create<Item>(initial);

      expect(get(nav.current)).toEqual(initial);
    });
  });

  describe("push", () => {
    it("sets the new state", () => {
      const initial = { inner: -1 };
      const next = { inner: 111 };
      const nav = navigation.create<Item>(initial);

      nav.push(next);

      expect(get(nav.current)).toEqual(next);
    });
  });

  describe("pop", () => {
    it("goes back to the old state", () => {
      const initial = { inner: -1 };
      const prev = { inner: 3 };
      const next = { inner: 5 };
      const nav = navigation.create<Item>(initial);

      nav.push(prev);
      expect(get(nav.current)).toEqual(prev);

      nav.push(next);
      expect(get(nav.current)).toEqual(next);

      nav.pop();
      expect(get(nav.current)).toEqual(prev);
    });
  });
});
