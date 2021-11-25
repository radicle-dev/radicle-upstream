// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Readable } from "svelte/store";

export * from "svelte/store";

// Calls `matcher(value)` whenever `value` in `store` changes. Returns the
// result of `matcher(value)` when it returns a value that is not `undefined`
// for the first time.
//
// If `matcher` throws then the error is rethrown by the returned
// promise.
export function waitUntil<T, S>(
  store: Readable<T>,
  matcher: (t: T) => S | undefined
): Promise<S> {
  return new Promise((resolve, reject) => {
    let resolvedNow = false;
    let unsubscribe: () => void | undefined;
    // We’re using `let` so that we can access `unsubscribe` if the
    // `susbscribe` callback is called synchronously.
    // eslint-disable-next-line prefer-const
    unsubscribe = store.subscribe(value => {
      let matched;
      try {
        matched = matcher(value);
      } catch (err: unknown) {
        reject(err);
        return;
      }

      if (matched !== undefined) {
        if (unsubscribe) {
          unsubscribe();
        } else {
          resolvedNow = true;
        }
        resolve(matched);
      }
    });
    if (resolvedNow) {
      unsubscribe();
    }
  });
}
