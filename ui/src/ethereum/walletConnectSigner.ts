// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ethers from "ethers";
import * as ethersBytes from "@ethersproject/bytes";
import type {
  Provider,
  TransactionRequest,
  TransactionReceipt,
  TransactionResponse,
} from "@ethersproject/abstract-provider";
import {
  Deferrable,
  defineReadOnly,
  resolveProperties,
} from "@ethersproject/properties";
import * as svelteStore from "svelte/store";

import type { WalletConnect } from "./walletConnect";

export class WalletConnectSigner extends ethers.Signer {
  private walletConnect: WalletConnect;
  private _provider: ethers.providers.Provider;

  constructor(walletConnect: WalletConnect, provider: Provider) {
    super();
    defineReadOnly(this, "provider", provider);
    this._provider = provider;
    this.walletConnect = walletConnect;
  }

  async getAddress(): Promise<string> {
    const accountAddress = svelteStore.get(
      this.walletConnect.connection
    )?.accountAddress;
    if (!accountAddress) {
      throw new Error(
        "The connected wallet has no accounts or there is a connection problem"
      );
    }
    return accountAddress;
  }

  async signMessage(message: ethers.Bytes | string): Promise<string> {
    let messageBytes: Uint8Array;
    if (typeof message === "string") {
      messageBytes = ethers.utils.toUtf8Bytes(message);
    } else {
      messageBytes = ethers.utils.arrayify(message);
    }
    const address = await this.getAddress();
    const signature = await this.walletConnect.signMessage(
      address,
      messageBytes
    );
    return signature;
  }

  async sendTransaction(
    transaction: Deferrable<TransactionRequest>
  ): Promise<TransactionResponse> {
    const tx = await resolveProperties(transaction);
    const from = tx.from || (await this.getAddress());

    const txHash = await this.walletConnect.sendTransaction({
      from,
      to: tx.to,
      value: maybeBigNumberToString(tx.value),
      data: bytesLikeToString(tx.data),
    });

    return {
      from,
      value: ethers.BigNumber.from(tx.value || 0),
      get chainId(): number {
        throw new Error("this should never be called");
      },
      get nonce(): number {
        throw new Error("this should never be called");
      },
      get gasLimit(): ethers.BigNumber {
        throw new Error("this should never be called");
      },
      get gasPrice(): ethers.BigNumber {
        throw new Error("this should never be called");
      },
      data: bytesLikeToString(tx.data) || "",
      hash: txHash,
      confirmations: 1,
      wait: (confirmations: number = 1): Promise<TransactionReceipt> =>
        this._provider.waitForTransaction(txHash, confirmations),
    };
  }

  async signTypedData(address: string, typedData: unknown): Promise<string> {
    return this.walletConnect.signTypedData(address, typedData);
  }

  async signTransaction(
    transaction: Deferrable<TransactionRequest>
  ): Promise<string> {
    const tx = await resolveProperties(transaction);
    const from = tx.from || (await this.getAddress());

    const signedTx = await this.walletConnect.signTransaction({
      from,
      to: tx.to,
      value: maybeBigNumberToString(tx.value),
      gasLimit: maybeBigNumberToString(tx.gasLimit),
      gasPrice: maybeBigNumberToString(tx.gasPrice),
      nonce: maybeBigNumberToString(tx.nonce),
      data: bytesLikeToString(tx.data),
    });
    return signedTx;
  }

  connect(_provider: Provider): ethers.Signer {
    throw new Error("WalletConnectSigner.connect should never be called");
  }
}

function maybeBigNumberToString(
  bn: ethers.BigNumberish | undefined
): string | undefined {
  if (bn === undefined) {
    return undefined;
  } else {
    return ethers.BigNumber.from(bn).toString();
  }
}

function bytesLikeToString(
  bytes: ethersBytes.BytesLike | undefined
): string | undefined {
  if (bytes === undefined) {
    return undefined;
  } else {
    return ethersBytes.hexlify(bytes);
  }
}
