import LruCache from "lru-cache";

// Creates a functions that memoizes its result using an LRU cache.
//
// The cache key is a string created from the arguments using
// `makeKey`.
export function memoizeLru<Args extends unknown[], V>(
  f: (...args: Args) => Promise<V>,
  makeKey: (...args: Args) => string,
  options?: LruCache.Options<string, V>
): (...args: Args) => Promise<V> {
  const cache = new LruCache(options);
  return async function (...args: Args): Promise<V> {
    const key = makeKey(...args);
    const cached = cache.get(key);
    if (cached === undefined) {
      const value = await f(...args);
      cache.set(key, value);
      return value;
    } else {
      return cached;
    }
  };
}
