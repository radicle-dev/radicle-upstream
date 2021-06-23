// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Facade for the `baconjs` package with extra functions.
//
// Re-exports everything from `baconjs`.

import * as bacon from "baconjs";
export * from "baconjs";

// Creates a new stream that calls `f` on items on the original stream
// and emits the result if it is not undefined.
//
// This is equivalent to
// ```
// stream.filter((x) => f(x) !== undefined).map(f)
// ```
// but type checks and calls `f` only once.
export function filterMap<S, T>(
  stream: bacon.EventStream<S>,
  f: (s: S) => T | undefined
): bacon.EventStream<T> {
  return stream.transform((event, sink) => {
    if (bacon.hasValue(event)) {
      const next = f(event.value);
      if (next === undefined) {
        return;
      } else {
        return sink(new bacon.Next(next));
      }
    } else {
      return sink(event as unknown as bacon.Event<T>);
    }
  });
}
