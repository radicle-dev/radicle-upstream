// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { radicleUrlSchema } from "./nativeCustomProtocolHandler";

describe("parseRadicleURL", () => {
  it("passes valid URLs", () => {
    const validUrl =
      "radicle://link/v1/rad:git:hnrkj7qjesxx4omprbj5c6apd97ebc9e5izoo";
    expect(radicleUrlSchema.safeParse(validUrl).success).toEqual(true);
  });

  it("rejects empty strings", () => {
    expect(radicleUrlSchema.safeParse("").success).toEqual(false);
  });

  it("passes URLs that are exactly 1024 bytes long", () => {
    const longUrl = `radicle://${"x".repeat(1014)}`;
    expect(radicleUrlSchema.safeParse(longUrl).success).toEqual(true);
  });

  it("rejects handling URLs longer than 1024 bytes", () => {
    const tooLongUrl = `radicle://${"x".repeat(1015)}`;
    expect(radicleUrlSchema.safeParse(tooLongUrl).success).toEqual(false);
  });

  it("rejects URLs that are not prefixed with radicle://", () => {
    expect(radicleUrlSchema.safeParse("upstream://").success).toEqual(false);
  });
});
