// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Identity } from "ui/src/identity";
import type { UnsealedSession } from "ui/src/session";
import type { Wallet } from "ui/src/wallet";

import { Signer, ethers } from "ethers";
import { claimsAddress, ClaimsContract } from "./contract";
import { parseIdentitySha1 } from "ui/src/urn";
import * as session from "ui/src/session";
import * as svelteStore from "svelte/store";

export enum AttestationStatus {
  Fetching = "Fetching",
  Incomplete = "Incomplete",
  Expired = "Expired",
  UnmatchingEth = "UnmatchingEth",
  UnmatchingRoot = "UnmatchingRoot",
  Valid = "Valid",
}

export const attestationStatus: svelteStore.Writable<AttestationStatus> =
  svelteStore.writable(AttestationStatus.Fetching);

let watching = false;

export async function watchAttestationStatus(
  walletStore: svelteStore.Readable<Wallet>
): Promise<void> {
  if (watching) {
    return;
  }
  watching = true;

  const wallet = svelteStore.get(walletStore);
  const claimWatcher = new ClaimWatcher(
    wallet.signer,
    claimsAddress(wallet.environment)
  );
  await updateAttesttationStatus(walletStore, claimWatcher);

  // FIXME: we don't want to poll, but to re-check the status based on
  // changes to the wallet(new account selected, new environment, etc).
  // We can't have updateAttesttationStatus derive wallet.store because of
  // initialization issues tho.
  const POLL_INTERVAL_MILLIS = 1000;
  setInterval(
    () => updateAttesttationStatus(walletStore, claimWatcher),
    POLL_INTERVAL_MILLIS
  );
}

async function updateAttesttationStatus(
  walletStore: svelteStore.Readable<Wallet>,
  claimWatcher: ClaimWatcher
): Promise<void> {
  const wallet = svelteStore.get(walletStore);
  const sess: UnsealedSession = session.unsealed();
  const address = wallet.getAddress();
  if (address) {
    attestationStatus.set(
      await getAttestationStatus(sess.identity, address, claimWatcher)
    );
  } else {
    attestationStatus.set(AttestationStatus.Fetching);
  }
}

async function getAttestationStatus(
  identity: Identity,
  ethAddress: string,
  claimWatcher: ClaimWatcher
): Promise<AttestationStatus> {
  const optEthClaim = identity.metadata.ethereum;
  if (optEthClaim === null) {
    return AttestationStatus.Incomplete;
  }
  const expiration = Date.parse(optEthClaim.expiration);
  if (isNaN(expiration) || expiration <= new Date().valueOf()) {
    return AttestationStatus.Expired;
  }
  if (optEthClaim.address.toLowerCase() !== ethAddress.toLowerCase()) {
    return AttestationStatus.UnmatchingEth;
  }
  const claimed = await claimWatcher.claimed(ethAddress);
  if (claimed === undefined) {
    return AttestationStatus.Incomplete;
  }
  const expected = parseIdentitySha1(identity.urn);
  if (ethers.utils.hexlify(claimed) !== ethers.utils.hexlify(expected)) {
    return AttestationStatus.UnmatchingRoot;
  }
  return AttestationStatus.Valid;
}

class ClaimWatcher {
  private contract: ClaimsContract;
  private ethAddr?: string;
  private unwatch?: () => void;
  private lastClaimed?: Uint8Array;

  constructor(signer: Signer, contractAddr: string) {
    this.contract = new ClaimsContract(signer, contractAddr);
    this.ethAddr = undefined;
    this.lastClaimed = undefined;
  }

  async claimed(ethAddr: string): Promise<Uint8Array | undefined> {
    if (ethAddr !== this.ethAddr) {
      this.ethAddr = ethAddr;
      if (this.unwatch) {
        this.unwatch();
      }
      const [lastClaimed, unwatch] = await this.contract.watchClaimed(
        ethAddr,
        claimed => {
          this.lastClaimed = claimed;
        }
      );
      this.lastClaimed = lastClaimed;
      this.unwatch = unwatch;
    }
    return this.lastClaimed;
  }
}
