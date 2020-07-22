import { get } from "svelte/store";

import * as navigation from "./navigation";

import Foo from "../Theme.svelte";

console.log(Foo);

describe("navigation", () => {
  describe("initial state", () => {
    it("is the one passed on initialisation", () => {
      const initial = { component: {} };
      const nav = navigation.create(initial);

      expect(get(nav.current)).toEqual(initial);
    });
  });
});
