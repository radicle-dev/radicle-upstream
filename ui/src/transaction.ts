// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as svelteStore from "svelte/store";
import { writable as persistentStore } from "svelte-persistent-store/dist/local";

import type { ContractTransaction } from "ethers";
import type { TransactionReceipt } from "@ethersproject/abstract-provider";

import * as error from "./error";
import { store as walletStore } from "./wallet";
import * as notification from "ui/src/notification";

// The store where all managed transactions are stored.
export const store = persistentStore<Tx[]>("transactions", []);

export type Tx = TxData & MetaTx;
export type { ContractTransaction };

// The data shared across all types of transactions
// that we are to deal with.
export interface TxData {
  // The hash of the transaction
  hash: string;

  // The status of the transaction
  status: TxStatus;

  // The date in which this transaction was created.
  // In milliseconds since epoch.
  date: number;

  // The sender of this transaction
  from: string;

  // The destination of this transaction
  to?: string;
}

// The meta transactions that we provide to the user.
type MetaTx =
  | AnchorProject
  | ClaimRadicleIdentity
  | CommitEnsName
  | CreateOrg
  | LinkEnsNameToOrg
  | RegisterEnsName
  | UpdateEnsMetadata;

interface AnchorProject {
  kind: TxKind.AnchorProject;
}

interface CreateOrg {
  kind: TxKind.CreateOrg;
}

interface RegisterEnsName {
  kind: TxKind.RegisterEnsName;
}

interface CommitEnsName {
  kind: TxKind.CommitEnsName;
}

interface UpdateEnsMetadata {
  kind: TxKind.UpdateEnsMetadata;
}

interface LinkEnsNameToOrg {
  kind: TxKind.LinkEnsNameToOrg;
}

interface ClaimRadicleIdentity {
  kind: TxKind.ClaimRadicleIdentity;
  // The claimed Radicle identity root
  root: string;
}

export enum TxKind {
  AnchorProject = "Anchor Project",
  ClaimRadicleIdentity = "Claim Identity",
  CommitEnsName = "Commit ENS name",
  CreateOrg = "Create Org",
  LinkEnsNameToOrg = "Link Ens Name to Org",
  RegisterEnsName = "Register ENS name",
  UpdateEnsMetadata = "Update ENS metadata",
}

export enum TxStatus {
  // The transaction as been approved and is awaiting to be included in a block.
  AwaitingInclusion = "Awaiting inclusion",
  // The transaction as been included in the block. End of its life cycle.
  Included = "Included",
  // The transaction as been rejected.
  Rejected = "Rejected",
}

/* Smart constructors for `Tx` values */

export function anchorProject(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.AnchorProject };
}

export function createOrg(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.CreateOrg };
}

export function registerEnsName(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.RegisterEnsName };
}

export function commitEnsName(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.CommitEnsName };
}

export function linkEnsNameToOrg(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.LinkEnsNameToOrg };
}

export function updateEnsMetadata(txc: ContractTransaction): Tx {
  return { ...txData(txc), kind: TxKind.UpdateEnsMetadata };
}

export function claimRadicleIdentity(
  txc: ContractTransaction,
  root: string
): Tx {
  return { ...txData(txc), kind: TxKind.ClaimRadicleIdentity, root };
}

function txData(txc: ContractTransaction, date: number = Date.now()): TxData {
  return {
    hash: txc.hash,
    status: txc.blockNumber ? TxStatus.Included : TxStatus.AwaitingInclusion,
    date,
    from: txc.from,
    to: txc.to,
  };
}

function registerTxUpdateCallback(tx: Tx): void {
  if (tx.status === TxStatus.AwaitingInclusion) {
    const provider = svelteStore.get(walletStore).provider;
    provider
      .waitForTransaction(tx.hash)
      .then(txReceipt => {
        store.update((txs: Tx[]) => {
          const tx_ = txs.find(tx_ => tx_.hash === tx.hash);
          if (tx_) {
            tx_.status = status(txReceipt);
          }
          return txs;
        });
      })
      .catch(err => {
        notification.showException(
          new error.Error({
            message: "Failed to update transaction status",
            source: err,
          })
        );
      });
  }
}

export function add(tx: Tx): void {
  store.update((txs: Tx[]) => {
    txs.unshift(tx);
    return txs;
  });
  registerTxUpdateCallback(tx);
  cap();
}

// Cap the amount of managed transactions
function cap(length = 7): void {
  store.update((txs: Tx[]) => {
    txs.length = Math.min(txs.length, length);
    return txs;
  });
}

async function updateStatuses(): Promise<void> {
  const txs = svelteStore.get(store);
  txs.forEach(tx => {
    registerTxUpdateCallback(tx);
  });
}

function status(receipt: TransactionReceipt): TxStatus {
  if (receipt.blockNumber === null || receipt.blockNumber === 0) {
    return TxStatus.AwaitingInclusion;
  } else if (receipt.status === 1) {
    return TxStatus.Included;
  } else {
    return TxStatus.Rejected;
  }
}

/* UI helper functions */

// Middleware criterion to check for an ongoing transaction of a given TxKind.
export function ongoing(txKind: TxKind): (tx: Tx) => boolean {
  return (tx: Tx) =>
    tx.status === TxStatus.AwaitingInclusion && tx.kind === txKind;
}

// Convert a transaction-related error to `error.Error`.
export function convertError(e: unknown, label: string): error.Error {
  let code = error.Code.UnkownTransactionFailure;
  let message = "an unkown transaction error occurred";

  if (e instanceof globalThis.Error) {
    if (e.message.includes("gas")) {
      code = error.Code.InsufficientGas;
      message = "you seem to have insufficient gas to cover this transaction.";
    } else if (e.message.toLowerCase().includes("rejected request")) {
      code = error.Code.FailedOrRejectedTransaction;
      message =
        "you have rejected this transaction or it has failed for some unkown reason.";
    }
  }

  return new error.Error({
    code,
    message: `${label}: ${message}`,
    source: e,
  });
}

export function initialize(): void {
  updateStatuses();
}
