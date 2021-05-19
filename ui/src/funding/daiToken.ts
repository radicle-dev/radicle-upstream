import type { BigNumber, Signer } from "ethers";

import Big from "big.js";

import * as ethereum from "../ethereum";

import {
  ERC20,
  ERC20__factory as Erc20Factory,
} from "radicle-contracts/build/contract-bindings/ethers";

export { ERC20 };

const addresses = {
  local: "0xff1d4d289bf0aaaf918964c57ac30481a67728ef",
  ropsten: "0x31f42841c2db5173425b5223809cf3a38fede360",
};

// Get the address of the Pool Contract for the given environment
export function daiTokenAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.local;
    case ethereum.Environment.Ropsten:
      return addresses.ropsten;
  }
}

export function connect(signer: Signer, address: string): ERC20 {
  return Erc20Factory.connect(address, signer);
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
  // console.log("ERC20FILTERS " + JSON.stringify(token.eventstoken.events));
  const filter = token.filters.Approval(owner, spender);
  const listener = (_owner: unknown, _spender: unknown, allowance: BigNumber) =>
    onUpdated(Big(allowance.toString()));
  token.on(filter, listener);
  const allowance = await token.allowance(owner, spender);
  onUpdated(Big(allowance.toString()));
  return () => token.off(filter, listener);
}

export { watchDaiTokenAllowance };
