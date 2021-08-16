// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { BigNumber, Signer } from "ethers";
import type * as ethers from "ethers";

import Big from "big.js";

import * as ethereum from "../ethereum";

import {
  ERC20,
  ERC20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

export { ERC20 };

const addresses = {
  local: "0x1d3e6acf736f4730f709cda657040be1f0d4500f",
  rinkeby: "0x5592ec0cfb4dbc12d3ab100b257153436a1f0fea",
  mainnet: "0x6b175474e89094c44da98b954eedeac495271d0f",
};

// Get the address of the Pool Contract for the given environment
function daiTokenAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.local;
    case ethereum.Environment.Rinkeby:
      return addresses.rinkeby;
    case ethereum.Environment.Mainnet:
      return addresses.mainnet;
  }
}

export function connect(
  signerOrProvider: Signer | ethers.providers.Provider,
  environment: ethereum.Environment
): ERC20 {
  return Erc20Factory.connect(daiTokenAddress(environment), signerOrProvider);
}

// Start watching an allowance on a given token.
// `onUpdated` is called immediately with the latest allowance amount.
// Returns a function, which unwatches the allowance when called.
async function watchDaiTokenAllowance(
  token: ERC20,
  owner: string,
  spender: string,
  onUpdated: (allowance: Big) => void
): Promise<() => void> {
  const filter = token.filters.Approval(owner, spender);
  const listener = (
    _owner: string,
    _spender: string,
    allowance: BigNumber,
    _event: unknown
  ) => {
    onUpdated(Big(allowance.toString()));
  };
  token.on(filter, listener);
  const allowance = await token.allowance(owner, spender);
  onUpdated(Big(allowance.toString()));
  return () => token.off(filter, listener);
}

export { watchDaiTokenAllowance };
