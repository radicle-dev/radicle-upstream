// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as ethers from "ethers";
import type { Signer } from "ethers";

import {
  Claims,
  Claims__factory as ClaimsFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

import * as ethereum from "../ethereum";
import * as transaction from "../transaction";
import { parseIdentitySha1 } from "../urn";

const addresses = {
  claims: {
    local: "0x785e8de68df899d77ce689f863e4166849c8bfd5",
    rinkeby: "0x6c7b50EA0AFB02d73AE3846B3B9EBC31808300a6",
    mainnet: "0x4a7DFda4F2e9F062965cC87f775841fB58AEA83e",
  },
};

// Get the address of the Claims Contract for the given environment
export function claimsAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return addresses.claims.local;
    case ethereum.Environment.Rinkeby:
      return addresses.claims.rinkeby;
    case ethereum.Environment.Mainnet:
      return addresses.claims.mainnet;
  }
}

const FORMAT_SHA1 = ethers.BigNumber.from(1);
const FORMAT_SHA1_LENGTH = 20;

export function claims(signer: Signer, address: string): ClaimsContract {
  return new ClaimsContract(signer, address);
}

export class ClaimsContract {
  contract: Claims;

  constructor(signer: Signer, address: string) {
    this.contract = ClaimsFactory.connect(address, signer);
  }

  async claim(urn: string): Promise<void> {
    const payload = parseIdentitySha1(urn);
    const tx = await this.contract.claim(FORMAT_SHA1, payload);
    transaction.add(transaction.claimRadicleIdentity(tx, urn));
  }

  // Fetches the identity claimed by the given Ethereum address.
  // Returns `undefined` if currently there's no valid claim.
  async getClaimed(address: string): Promise<Uint8Array | undefined> {
    const filter = this.contract.filters.Claimed(address);
    const lastEvent = (await this.contract.queryFilter(filter)).pop();
    if (!lastEvent) {
      return undefined;
    }
    let claimed;
    try {
      claimed = await this.getClaimedByTx(lastEvent.transactionHash, address);
    } catch {
      // Lack of a valid claim on the Ethereum side, the whole claim is invalid
      return undefined;
    }
    // Claim of hash `0`, the identity has been explicitly unclaimed
    if (claimed.every(hashByte => hashByte === 0)) {
      return undefined;
    }
    return claimed;
  }

  // Start watching claims of a given Ethereum address.
  // `onClaimed` is called immediately with the latest claim or `undefined` if there was none.
  // Returns a function, which unwatches claims when called.
  // Throws if the current claim is invalid.
  async watchClaimed(
    address: string,
    onClaimed: (claimed?: Uint8Array) => void
  ): Promise<() => void> {
    const filter = this.contract.filters.Claimed(address);

    const listener = async (_: unknown, event: ethers.Event) => {
      const claimed = await this.getClaimedByTx(event.transactionHash, address);
      onClaimed(claimed);
    };
    await this.contract.on(filter, listener);

    const lastEvent = (await this.contract.queryFilter(filter)).pop();
    if (lastEvent) {
      const lastClaimed = await this.getClaimedByTx(
        lastEvent.transactionHash,
        address
      );
      onClaimed(lastClaimed);
    } else {
      onClaimed(undefined);
    }
    return () => {
      this.contract.off(filter, listener);
    };
  }

  // Extracts the claimed identity root from the transaction sent by the address
  // Throws if the current claim is invalid.
  private async getClaimedByTx(
    txHash: string,
    address: string
  ): Promise<Uint8Array> {
    const tx = await this.contract.provider.getTransaction(txHash);
    if (tx === null) {
      throw new Error("Claim transaction not found");
    }
    // TODO(igor) add more checks for malicious client (e.g. TX signature, network, etc.)
    if (tx.from.toLowerCase() !== address.toLowerCase()) {
      throw new Error("Claim transaction sent by an invalid address");
    }
    if (tx.to.toLowerCase() !== this.contract.address.toLowerCase()) {
      throw new Error("Claim transaction sent to an invalid contract");
    }
    const [format, payloadRaw]: [ethers.BigNumberish, ethers.BytesLike] =
      this.contract.interface.decodeFunctionData("claim", tx.data);
    if (FORMAT_SHA1.eq(format) === false) {
      throw new Error(
        `Bad claim transaction payload format version ${format.toString()}`
      );
    }
    const payload = ethers.utils.arrayify(payloadRaw);
    if (payload.length !== FORMAT_SHA1_LENGTH) {
      throw new Error(`Bad claim transaction payload size ${payload.length}`);
    }
    return payload;
  }
}
