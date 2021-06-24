// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// This module adds a helper object to `window.ethereumDebug` that
// allows developers to control an Ethereum development node.
import * as ethers from "ethers";

declare global {
  interface Window {
    ethereumDebug: EthereumDebug;
  }
}

export function install(provider: ethers.providers.Provider): void {
  if (provider instanceof ethers.providers.JsonRpcProvider) {
    window.ethereumDebug = new EthereumDebug(provider);
  }
}

class EthereumDebug {
  private provider: ethers.providers.JsonRpcProvider;

  constructor(provider: ethers.providers.JsonRpcProvider) {
    this.provider = provider;
  }
  async mineBlocks(blocks = 1) {
    while (blocks) {
      blocks -= 1;
      await this.provider.send("evm_mine", []);
    }
  }

  async setBlockTime(seconds = 5) {
    await this.provider.send("evm_setTime", [seconds]);
  }

  async increaseTime(seconds = 5) {
    await this.provider.send("evm_increaseTime", [seconds]);
  }
}
