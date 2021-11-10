// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import * as error from "ui/src/error";
import * as svelteStore from "svelte/store";

import * as browserStore from "./browserStore";

afterEach(() => {
  window.localStorage.clear();
});

test("persist value in storage", () => {
  const store = browserStore.create("radicle.foo", false, zod.boolean());

  expect(window.localStorage.getItem("radicle.foo")).toBe(
    JSON.stringify(false)
  );
  expect(svelteStore.get(store)).toBe(false);

  store.set(true);

  expect(window.localStorage.getItem("radicle.foo")).toBe(JSON.stringify(true));
  expect(svelteStore.get(store)).toBe(true);
});

test("update value", () => {
  const store = browserStore.create("radicle.foo", false, zod.boolean());

  expect(window.localStorage.getItem("radicle.foo")).toBe(
    JSON.stringify(false)
  );
  expect(svelteStore.get(store)).toBe(false);

  const updater = jest.fn((x: boolean) => !x);
  store.update(updater);

  expect(window.localStorage.getItem("radicle.foo")).toBe(JSON.stringify(true));
  expect(svelteStore.get(store)).toBe(true);
  expect(updater).toHaveBeenLastCalledWith(false);
});

test("invalid value returns initial value and shows error", () => {
  const onError = jest.fn();
  error.notifications.onValue(onError);

  window.localStorage.setItem("radicle.foo", "0");
  const store = browserStore.create("radicle.foo", false, zod.boolean());

  expect(svelteStore.get(store)).toBe(false);

  expect(onError.mock.calls[0][0].message).toBe(
    "Stored data does not match schema"
  );
});

test("invalid value shows error only once", () => {
  const onError = jest.fn();
  error.notifications.onValue(onError);

  window.localStorage.setItem("radicle.foo", "0");
  const store = browserStore.create("radicle.foo", false, zod.boolean());

  expect(svelteStore.get(store)).toBe(false);
  expect(svelteStore.get(store)).toBe(false);

  expect(onError.mock.calls.length).toBe(1);
});
