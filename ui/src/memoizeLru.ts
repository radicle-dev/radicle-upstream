// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import LruCache from "lru-cache";

// Creates a functions that memoizes its result using an LRU cache.
//
// The cache key is a string created from the arguments using
// `makeKey`.
export function memoizeLru<Args extends unknown[], V>(
  f: (...args: Args) => Promise<V>,
  makeKey: (...args: Args) => string,
  options?: LruCache.Options<string, { value: V }>
): (...args: Args) => Promise<V> {
  const cache = new LruCache(options || { max: 500 });
  return async function (...args: Args): Promise<V> {
    const key = makeKey(...args);
    const cached = cache.get(key);
    if (cached === undefined) {
      const value = await f(...args);
      cache.set(key, { value });
      return value;
    } else {
      return cached.value;
    }
  };
}
