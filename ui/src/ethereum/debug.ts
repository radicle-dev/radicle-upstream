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

  public constructor(provider: ethers.providers.JsonRpcProvider) {
    this.provider = provider;
  }

  public async mineBlocks(blocks = 1): Promise<void> {
    while (blocks) {
      blocks -= 1;
      await this.provider.send("evm_mine", []);
    }
  }

  public async setBlockTime(seconds = 5): Promise<void> {
    await this.provider.send("evm_setTime", [seconds]);
  }

  public async increaseTime(seconds = 5): Promise<void> {
    await this.provider.send("evm_increaseTime", [seconds]);
  }
}
