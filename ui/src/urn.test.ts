/**
 * The jsdom environment lacks `TextEncoder` required by `multibase`,
 * see https://github.com/multiformats/js-multibase/issues/90,
 * will be solved by https://github.com/jsdom/jsdom/issues/2524
 *
 * @jest-environment node
 *
 */

import { parseIdentitySha1 } from "./urn";
import * as ethers from "ethers";

describe("parseIdentitySha1", () => {
  function expectSuccess(urn: string, expectedRoot: string) {
    const actual = parseIdentitySha1(urn);
    expect(ethers.utils.hexlify(actual)).toEqual(expectedRoot);
  }

  function expectFailure(urn: string, expectedError: string) {
    expect(() => parseIdentitySha1(urn)).toThrow(expectedError);
  }

  it("parses a valid identity URN", () => {
    expectSuccess(
      "rad:git:hnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "0x4d964d1bfa90593ba290dfb2b969717894868f70"
    );
  });

  it("rejects a non-rad URN", () => {
    expectFailure(
      "bad:git:hnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN malformed"
    );
  });

  it("rejects a non-git URN", () => {
    expectFailure(
      "rad:svn:hnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN malformed"
    );
  });

  it("rejects a payload with non-multibase characters", () => {
    expectFailure(
      "rad:git:lhnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN malformed"
    );
  });

  it("rejects a malformed multibase payload", () => {
    expectFailure(
      "rad:git:xhnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN has a malformed multibase payload"
    );
  });

  it("rejects a multibase payload of invalid size", () => {
    expectFailure(
      "rad:git:hnrkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay1",
      "URN has an invalid payload size: 23"
    );
  });

  it("rejects a non-SHA1 multihash payload", () => {
    expectFailure(
      "rad:git:horkr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN has an invalid multihash algorithm identifier: 129"
    );
  });

  it("rejects a non-SHA1 multihash payload", () => {
    expectFailure(
      "rad:git:hnrmr5f1pdx7jysj5wkep9ci3pfaztfrgt7ay",
      "URN has an invalid multihash payload size: 22"
    );
  });
});
