import type { Readable } from "svelte/store";

export * from "svelte/store";

// Waits until the value in `store` matches the predicate and resolves
// the promise with the value.
//
// If `predicate` throws then the error is rethrown by the returned
// promise.
export function waitUntil<T>(
  store: Readable<T>,
  predicate: (t: T) => boolean
): Promise<T> {
  return new Promise((resolve, reject) => {
    let resolvedNow = false;
    let unsubscribe: () => void | undefined;
    // We’re using `let` so that we can access `unsubscribe` if the
    // `susbscribe` callback is called synchronously.
    // eslint-disable-next-line prefer-const
    unsubscribe = store.subscribe(value => {
      let matched;
      try {
        matched = predicate(value);
      } catch (err) {
        reject(err);
        return;
      }

      if (matched) {
        if (unsubscribe) {
          unsubscribe();
        } else {
          resolvedNow = true;
        }
        resolve(value);
      }
    });
    if (resolvedNow) {
      unsubscribe();
    }
  });
}
