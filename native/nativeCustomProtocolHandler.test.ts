import { parseRadicleUrl } from "./nativeCustomProtocolHandler";

describe("parseRadicleURL", () => {
  it("passes valid URLs", () => {
    const validUrl =
      "radicle://link/v1/rad:git:hnrkj7qjesxx4omprbj5c6apd97ebc9e5izoo";
    expect(parseRadicleUrl(validUrl)).toEqual(validUrl);
  });

  it("rejects empty strings", () => {
    expect(parseRadicleUrl("")).toEqual(undefined);
  });

  it("passes URLs that are exactly 1024 bytes long", () => {
    const longUrl = `radicle://${"x".repeat(1014)}`;
    expect(parseRadicleUrl(longUrl)).toEqual(longUrl);
  });

  it("rejects handling URLs longer than 1024 bytes", () => {
    const tooLongUrl = `radicle://${"x".repeat(1015)}`;
    expect(parseRadicleUrl(tooLongUrl)).toEqual(undefined);
  });

  it("rejects URLs that are not prefixed with radicle://", () => {
    expect(parseRadicleUrl("upstream://")).toEqual(undefined);
  });
});
