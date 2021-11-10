// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as zod from "zod";
import persistentStore from "svelte-persistent-store/dist";

import * as error from "ui/src/error";
import * as svelteStore from "ui/src/svelteStore";

const VALID_STORAGE_KEY_RE = /radicle(\.[a-zA-Z])+/;

// Create a store that is backed by the browsers local storage.
//
// The raw storage value must conform to `schema`. If the value does
// not parse, `initialValue` is returned and an error is shown to the
// user.
//
// `key` must be a dot-separated list letters that starts with
// "radicle". For example "radicle.foo.bar".
//
// `initialValue` is the value assigned to the storage if it is not
// set.
export function create<T>(
  key: string,
  initialValue: T,
  schema: zod.Schema<T>
): svelteStore.Writable<T> {
  let errorShown = false;
  if (!VALID_STORAGE_KEY_RE.test(key)) {
    throw new Error(
      `invalid storage key ${key}. Must match ${VALID_STORAGE_KEY_RE}`
    );
  }

  function parseOrInitial(value: unknown): T {
    const result = schema.safeParse(value);
    if (result.success) {
      return result.data;
    } else {
      if (!errorShown) {
        errorShown = true;
        error.showNotification(
          new error.Error({
            message: "Stored data does not match schema",
            details: {
              key,
              value,
              zodIssues: result.error.issues,
            },
          })
        );
      }
      return initialValue;
    }
  }

  const raw = persistentStore.local.writable<unknown>(key, initialValue);
  const parsed = svelteStore.derived(raw, parseOrInitial);
  return {
    set: raw.set,
    update: (updater: svelteStore.Updater<T>) => {
      raw.update(value => {
        return updater(parseOrInitial(value));
      });
    },
    subscribe: parsed.subscribe,
  };
}
