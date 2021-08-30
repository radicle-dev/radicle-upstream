// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as ethers from "ethers";
import * as ethereum from "ui/src/ethereum";

import {
  RadicleToken,
  RadicleToken__factory as RadicleTokenFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

const addresses = {
  local: "0x1d3e6acf736f4730f709cda657040be1f0d4500f",
  rinkeby: "0x7b6CbebC5646D996d258dcD4ca1d334B282e9948",
  mainnet: "0x31c8EAcBFFdD875c74b94b077895Bd78CF1E64A3",
};

function radTokenAddress(environment: ethereum.Environment): string {
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
  signerOrProvider: ethers.Signer | ethers.providers.Provider,
  environment: ethereum.Environment
): RadicleToken {
  return RadicleTokenFactory.connect(
    radTokenAddress(environment),
    signerOrProvider
  );
}
