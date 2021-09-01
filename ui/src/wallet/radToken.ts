// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as ethers from "ethers";
import {
  RadicleToken,
  RadicleToken__factory as RadicleTokenFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

import * as ethereum from "ui/src/ethereum";

function radTokenAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return ethereum.contractAddresses.radToken.local;
    case ethereum.Environment.Rinkeby:
      return ethereum.contractAddresses.radToken.rinkeby;
    case ethereum.Environment.Mainnet:
      return ethereum.contractAddresses.radToken.mainnet;
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
