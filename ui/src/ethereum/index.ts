// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import Big from "big.js";
import * as ethers from "ethers";
import persistentStore from "svelte-persistent-store/dist";
import { config, isCypressTestEnv } from "ui/src/config";
import * as error from "ui/src/error";

import { Environment, Network, supportedNetwork } from "./environment";

export { Environment, supportedNetwork, Network };

// The store where the selected Ethereum environment is persisted.
export const selectedEnvironment = persistentStore.local.writable<Environment>(
  "ethereum-environment-v0",
  config.isDev ? Environment.Rinkeby : Environment.Mainnet
);

if (
  ![Environment.Mainnet, Environment.Rinkeby, Environment.Local].includes(
    svelteStore.get(selectedEnvironment)
  )
) {
  selectedEnvironment.set(
    config.isDev ? Environment.Rinkeby : Environment.Mainnet
  );
}

if (isCypressTestEnv) {
  selectedEnvironment.set(Environment.Local);
}

// EIP-20 token decimals for the tokens we operate with across
// the diferent environments. We hardcode this value since it
// is well-settled and since we would need to request it from
// the token contract for each number conversion otherwise.
// We have, however, to keep in mind that new versions of the
// token might change it.
const TOKEN_DECIMALS = Big(10).pow(18);

// Big.PE determines the exponent at which its `toString()` representation
// starts being displayed in exponential notation. We never want to do that.
Big.PE = Number.MAX_SAFE_INTEGER;

export function toBaseUnit(n: ethers.BigNumber | Big): Big {
  return Big(n.toString()).div(TOKEN_DECIMALS).round(2);
}

export function fromBaseUnit(n: Big): ethers.BigNumber {
  return ethers.BigNumber.from(n.mul(TOKEN_DECIMALS).round().toString());
}

// Format a token amount with conventional units, limited
// significant digits, and thousand separators.
//
// `amount` is in the base unit (e.g. Wei) and the output is expressed
// in the conventional unit (e.g. Ether) using 18 decimals.
//
// The number of fraction digits is limited to 6 and the number of
// siginificant digits is 10.
//
// Throws when `amount` exceeds `Number.MAX_SAFE_INTEGER * 10 ** 12` or
// 9,007,199,254 in conventional units.
export function formatTokenAmount(amount: ethers.BigNumberish): string {
  const amountBn = ethers.BigNumber.from(amount);
  const decimals = 18;
  const fractionDigits = 6;
  const base = ethers.BigNumber.from(10).pow(decimals - fractionDigits);
  const scaled = amountBn.div(base).toNumber() / Math.pow(10, fractionDigits);
  return scaled.toLocaleString("en-US", {
    maximumFractionDigits: fractionDigits,
    maximumSignificantDigits: 10,
  });
}

export const VALID_ADDRESS_MATCH = /^0x[a-fA-F0-9]{40}$/;

export function etherscanUrl(ethEnv: Environment, query: string): string {
  switch (ethEnv) {
    case Environment.Local:
      throw new error.Error({
        code: error.Code.FeatureNotAvailableForGivenNetwork,
        message: "Etherscan links are not supported on the Local environment",
      });
    case Environment.Rinkeby:
      return `https://rinkeby.etherscan.io/search?f=0&q=${query}`;
    case Environment.Mainnet:
      return `https://etherscan.io/search?f=0&q=${query}`;
  }
}

export function ensAddress(env: Environment): string {
  if (env === Environment.Local) {
    throw new error.Error({
      code: error.Code.FeatureNotAvailableForGivenNetwork,
      message: "ENS is not supported on the Local environment",
    });
  } else {
    // https://docs.ens.domains/ens-deployments
    return "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
  }
}
