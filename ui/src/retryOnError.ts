// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { sleep } from "ui/src/sleep";

// Retries calling `action` if it throws an error that matches
// `retryPredicate`. Otherwise the result from `action()` is returned
// or the unmatched error is re-thrown.
//
// `delayMs` is the delay between retries in milliseconds. The first
// try is called immediately.
//
// `tryCount` is the maximum number of times that `action` is called.
export async function retryOnError<T>(
  action: () => Promise<T>,
  retryPredicate: (error: unknown) => boolean,
  delayMs: number,
  tryCount: number
): Promise<T> {
  for (; ; tryCount--) {
    try {
      return await action();
    } catch (error: unknown) {
      if (!retryPredicate(error) || tryCount <= 1) {
        throw error;
      }
    }
    await sleep(delayMs);
  }
}

// Convenience wrapper for `retryOnError` that retries on `fetch()`
// errors.
export async function retryFetch<T>(
  action: () => Promise<T>,
  delayMs: number,
  tryCount: number
): Promise<T> {
  return retryOnError(
    action,
    error => error instanceof Error && error.message === "Failed to fetch",
    delayMs,
    tryCount
  );
}
