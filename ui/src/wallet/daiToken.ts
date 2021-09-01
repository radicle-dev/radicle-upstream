// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { BigNumber, Signer } from "ethers";
import type * as ethers from "ethers";

import Big from "big.js";

import * as ethereum from "ui/src/ethereum";

import {
  ERC20,
  ERC20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

export { ERC20 };

// Get the address of the Pool Contract for the given environment
function daiTokenAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return ethereum.contractAddresses.daiToken.local;
    case ethereum.Environment.Rinkeby:
      return ethereum.contractAddresses.daiToken.rinkeby;
    case ethereum.Environment.Mainnet:
      return ethereum.contractAddresses.daiToken.mainnet;
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
