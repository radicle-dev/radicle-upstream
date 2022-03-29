// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Signer } from "ethers";
import type * as wallet from "ui/src/wallet";

import LruCache from "lru-cache";
import * as ethers from "ethers";

import {
  Claims,
  Claims__factory as ClaimsFactory,
} from "radicle-contracts/build/contract-bindings/ethers";

import * as error from "ui/src/error";
import * as ethereum from "ui/src/ethereum";
import * as mutexExecutor from "ui/src/mutexExecutor";
import * as transaction from "ui/src/transaction";
import * as Urn from "ui/src/urn";

// Get the address of the Claims Contract for the given environment
export function claimsAddress(environment: ethereum.Environment): string {
  switch (environment) {
    case ethereum.Environment.Local:
      return ethereum.contractAddresses.claims.local;
    case ethereum.Environment.Rinkeby:
      return ethereum.contractAddresses.claims.rinkeby;
    case ethereum.Environment.Mainnet:
      return ethereum.contractAddresses.claims.mainnet;
  }
}

const FORMAT_SHA1 = ethers.BigNumber.from(1);
const FORMAT_SHA1_LENGTH = 20;

export function claims(signer: Signer, address: string): ClaimsContract {
  return new ClaimsContract(signer, address);
}

interface ClaimsContractCacheEntry {
  value: Uint8Array | undefined;
}

const claimsContractCache = new LruCache<string, ClaimsContractCacheEntry>({
  max: 1000,
  ttl: 60 * 1000, // 1 minute
});

export class ClaimsContract {
  #contract: Claims;

  public constructor(
    signerOrProvider: Signer | wallet.Provider,
    address: string
  ) {
    this.#contract = ClaimsFactory.connect(address, signerOrProvider);
  }

  public async claim(urn: string): Promise<void> {
    const payload = Urn.urnToSha1(urn);
    const tx = await this.#contract.claim(FORMAT_SHA1, payload);
    transaction.add(transaction.claimRadicleIdentity(tx, urn));
  }

  // Fetches the identity claimed by the given Ethereum address.
  // Returns `undefined` if currently there's no valid claim.
  public async getClaimed(address: string): Promise<Uint8Array | undefined> {
    const normalisedAddress = address.toLowerCase();
    const cached: ClaimsContractCacheEntry | undefined =
      claimsContractCache.get(normalisedAddress);

    if (cached) {
      return cached.value;
    }

    const filter = this.#contract.filters.Claimed(address);
    const lastEvent = (await this.#contract.queryFilter(filter)).pop();
    if (!lastEvent) {
      claimsContractCache.set(normalisedAddress, { value: undefined });
      return undefined;
    }
    let claimed;
    try {
      claimed = await this.getClaimedByTx(lastEvent.transactionHash, address);
    } catch {
      claimsContractCache.set(normalisedAddress, { value: undefined });
      // Lack of a valid claim on the Ethereum side, the whole claim is invalid
      return undefined;
    }
    // Claim of hash `0`, the identity has been explicitly unclaimed
    if (claimed.every(hashByte => hashByte === 0)) {
      claimsContractCache.set(normalisedAddress, { value: undefined });
      return undefined;
    }
    claimsContractCache.set(normalisedAddress, { value: claimed });
    return claimed;
  }

  // Start watching claims of a given Ethereum address.
  // `onClaimed` is called whenever the claim for `address` is updated.
  // Returns the current claim (or `undefined` if the address hasn’t
  // claimed anything) and a function that stops watching the claims.
  // Throws if the current claim is invalid.
  public async watchClaimed(
    address: string,
    onClaimed: (claimed?: Uint8Array) => void
  ): Promise<[claimed: Uint8Array | undefined, unwatch: () => void]> {
    const filter = this.#contract.filters.Claimed(address);

    const getClaim = mutexExecutor.createWorker((txHash: string) => {
      return this.getClaimedByTx(txHash, address);
    });

    const listener = async (_: unknown, event: ethers.Event): Promise<void> => {
      getClaim.submit(event.transactionHash);
    };
    this.#contract.on(filter, listener);

    const lastEvent = (await this.#contract.queryFilter(filter)).pop();
    let claimed;
    if (lastEvent) {
      getClaim.submit(lastEvent.transactionHash);
      claimed = await getClaim.output.firstToPromise();
    }
    const unsubOnClaimed = getClaim.output.onValue(onClaimed);

    const unwatch = (): void => {
      unsubOnClaimed();
      this.#contract.off(filter, listener);
    };
    return [claimed, unwatch];
  }

  // Extracts the claimed identity root from the transaction sent by the address
  // Throws if the current claim is invalid.
  private async getClaimedByTx(
    txHash: string,
    address: string
  ): Promise<Uint8Array> {
    const tx = await this.#contract.provider.getTransaction(txHash);
    if (tx === null) {
      throw new error.Error({ message: "Claim transaction not found" });
    }
    // TODO(igor) add more checks for malicious client (e.g. TX signature, network, etc.)
    if (tx.from.toLowerCase() !== address.toLowerCase()) {
      throw new error.Error({
        message: "Claim transaction sent by an invalid address",
        details: {
          tx: txHash,
          address,
        },
      });
    }
    if (tx.to?.toLowerCase() !== this.#contract.address.toLowerCase()) {
      throw new error.Error({
        message: "Claim transaction sent to an invalid contract",
        details: { tx: txHash },
      });
    }
    const args = this.#contract.interface.decodeFunctionData("claim", tx.data);
    if (FORMAT_SHA1.eq(args[0]) === false) {
      throw new error.Error({
        message: "Bad claim transaction payload format version",
        details: { data: args[0].toString() },
      });
    }
    const payload = ethers.utils.arrayify(args[1]);
    if (payload.length !== FORMAT_SHA1_LENGTH) {
      throw new error.Error({
        message: "Bad claim transaction payload size",
        details: { payload },
      });
    }
    return payload;
  }
}
