// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { memoizeLru } from "./memoizeLru";

test("it caches undefined return values", async () => {
  const inner = jest.fn(async (_: string) => undefined);
  const memoized = memoizeLru(inner, key => key);

  expect(await memoized("a")).toBe(undefined);
  expect(await memoized("a")).toBe(undefined);

  expect(inner).toHaveBeenCalledTimes(1);
});
