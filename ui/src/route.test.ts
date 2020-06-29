import qs from "qs";
import regexparam from "regexparam";

import * as route from "./route";

interface Result {
  keys: string[];
  pattern: RegExp;
}

interface Out {
  [key: string]: string | null;
}

const exec = (path: string, result: Result): object => {
  const out = {} as Out;
  const matches = result.pattern.exec(path);

  if (!matches) {
    return out;
  }

  let i = 0;

  while (i < result.keys.length) {
    out[result.keys[i]] = matches[++i] || null;
  }
  return out;
};

describe("calling modal", () => {
  describe("with no query params", () => {
    it("returns the correct route", () => {
      console.log(exec("/foo/cha/baz/nah", regexparam("/foo/:bar/baz/:fuz")));

      const loc = {
        location: "/foo/bar",
        querystring: "",
      };

      expect(route.modal(loc, route.Modal.IdentityCreation)).toEqual(
        "/foo/bar?modal=%2Fidentity%2Fnew"
      );
    });
  });

  describe("with an existing modal", () => {
    it("overwrites it", () => {
      const loc = {
        location: "/foo/bar",
        querystring: qs.stringify({ modal: "/an/exisiting/one" }),
      };

      expect(route.modal(loc, route.Modal.IdentityCreation)).toEqual(
        "/foo/bar?modal=%2Fidentity%2Fnew"
      );
    });
  });

  describe("with existing query params", () => {
    it("preserves them", () => {
      const loc = {
        location: "/foo/bar",
        querystring: qs.stringify({ theme: "green", tree: "expand" }),
      };

      expect(route.modal(loc, route.Modal.IdentityCreation)).toEqual(
        "/foo/bar?theme=green&tree=expand&modal=%2Fidentity%2Fnew"
      );
    });
  });
});
