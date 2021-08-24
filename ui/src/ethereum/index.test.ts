// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ethers from "ethers";
import { formatTokenAmount } from "./index";
import Big from "big.js";

describe("formatTokenAmount", () => {
  it("formats values", () => {
    expect(formatTokenAmount(toEther("10"))).toEqual("10");
    expect(formatTokenAmount(toEther("10.01"))).toEqual("10.01");
  });

  it("rounds when significant digits are exceeded", () => {
    expect(formatTokenAmount(toEther("1,000,000.001"))).toEqual(
      "1,000,000.001"
    );
    expect(formatTokenAmount(toEther("1,000,000.0001"))).toEqual("1,000,000");
  });

  it("throws on large numbers", () => {
    expect(formatTokenAmount(toEther("9,007,199,254"))).toEqual(
      "9,007,199,254"
    );
    const n = toEther("9,007,199,255");
    expect(() => formatTokenAmount(n)).toThrowError(/overflow/);
  });

  it("rounds after more than 6 fraction digits", () => {
    expect(formatTokenAmount(toEther("0.12345"))).toEqual("0.12345");
    expect(formatTokenAmount(toEther("0.123456"))).toEqual("0.123456");
    expect(formatTokenAmount(toEther("0.1234567"))).toEqual("0.123456");
  });

  function toEther(n: string): ethers.BigNumber {
    n = n.replace(/,/g, "");
    const scale = new Big(10).pow(18);
    const scaled = new Big(n).times(scale).round(0);
    return ethers.BigNumber.from(scaled.toString());
  }
});
