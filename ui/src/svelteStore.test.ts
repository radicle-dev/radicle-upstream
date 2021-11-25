// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as sinon from "sinon";
import { sleep } from "ui/src/sleep";

import * as svelteStore from "./svelteStore";

describe("waitUntil", () => {
  test("current value matches", async () => {
    const store = svelteStore.writable(true);
    const value = await svelteStore.waitUntil(store, x => x.toString());
    expect(value).toBe("true");
  });

  test("future value matches", async () => {
    const store = svelteStore.writable(false);
    const promise = svelteStore.waitUntil(store, x =>
      x ? x.toString() : undefined
    );
    const resolved = sinon.spy();
    promise.then(resolved);

    store.set(false);
    await sleep(5);
    store.set(false);
    await sleep(5);
    expect(resolved.called).toBe(false);
    store.set(true);
    expect(await promise).toBe("true");
  });

  test("predicate throws", async () => {
    const store = svelteStore.writable(false);
    const error = new Error();
    const promise = svelteStore.waitUntil(store, x => {
      if (x === true) {
        throw error;
      }
    });

    store.set(true);

    expect(await promise.catch(e => e)).toBe(error);
  });
});
